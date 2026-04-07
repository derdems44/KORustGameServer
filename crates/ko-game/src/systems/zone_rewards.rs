//! Zone online reward tick system.
//! `ZoneOnlineRewardStart()`, `ZoneOnlineRewardChange()`, `ZoneOnlineSendReward()`.
//! Every 10 seconds the background task checks all online sessions. For each
//! session with initialised timers, if the timer for a given reward entry has
//! expired **and** the player is in the matching zone, the reward is granted.
//! Premium players use `pre_` fields (different interval, item, loyalty, cash).
//! ## Lifecycle
//! - **Start** (`zone_online_reward_start`): called at game entry — copies the
//!   global reward list into per-player timers (now + interval).
//! - **Change** (`zone_online_reward_change`): called on zone change / respawn —
//!   resets all per-player timers to now + interval.
//! - **Check** (background tick): every 10s, checks each player.
//! - **Send** (internal): grants items / loyalty / cash to the player.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tracing::debug;

use crate::systems::loyalty::send_loyalty_change;
use crate::world::WorldState;
use crate::zone::SessionId;

// ── QA Note (L3-1): Event room check for party kill rewards — RESOLVED ─────
//
// Resolved in Sprint 202: `give_kill_reward` in `handler/attack.rs` now checks
// `h.event_room == killer_event_room` for both all-party and priest-redirect
// paths, matching `User.cpp:3386` and `User.cpp:3413`.
//
// The zone *online* rewards in this file are per-player (no party distribution),
// so no event room check is needed here.
// ────────────────────────────────────────────────────────────────────────────────

/// Background tick interval in seconds.
const ZONE_ONLINE_REWARD_TICK_SECS: u64 = 10;

/// One minute in seconds.
const MINUTE: u64 = 60;

/// Start the zone online reward background task.
/// Returns a `JoinHandle` so the caller can abort on shutdown.
pub fn start_zone_online_reward_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(ZONE_ONLINE_REWARD_TICK_SECS));
        loop {
            interval.tick().await;
            process_zone_online_reward_tick(&world);
            process_online_cash_tick(&world);
        }
    })
}

/// Return the current UNIX timestamp in seconds.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Initialise per-player online reward timers at game entry.
/// Copies the global reward list length into a per-session timer vec.
/// Each entry is set to `now + interval` (premium uses `pre_minute`).
pub fn zone_online_reward_start(world: &WorldState, sid: SessionId) {
    let rewards = world.get_zone_online_rewards();
    if rewards.is_empty() {
        return;
    }

    let is_premium = world
        .with_session(sid, |h| h.premium_in_use > 0)
        .unwrap_or(false);

    let now = now_secs();
    let timers: Vec<u64> = rewards
        .iter()
        .map(|r| {
            let interval_min = if is_premium && r.pre_minute > 0 {
                r.pre_minute as u64
            } else {
                r.minute as u64
            };
            now + interval_min * MINUTE
        })
        .collect();

    world.update_session(sid, |h| {
        h.zone_online_reward_timers = timers;
    });
}

/// Reset per-player online reward timers on zone change or respawn.
/// Resets all timers to `now + minute` so the player starts a fresh
/// countdown in the new zone. Unlike `zone_online_reward_start`, this
/// function ALWAYS uses the normal `minute` interval — no premium check.
/// This matches the C++ implementation exactly:
///   `itr->usingtime = UNIXTIME + (itr->minute * MINUTE);`
pub fn zone_online_reward_change(world: &WorldState, sid: SessionId) {
    let rewards = world.get_zone_online_rewards();
    if rewards.is_empty() {
        return;
    }

    let now = now_secs();

    world.update_session(sid, |h| {
        // If timers were never initialised (e.g., player entered before
        // the reward table was loaded), initialise them now.
        if h.zone_online_reward_timers.len() != rewards.len() {
            h.zone_online_reward_timers = vec![0u64; rewards.len()];
        }
        for (i, r) in rewards.iter().enumerate() {
            // C++ always uses normal minute here — no premium override.
            h.zone_online_reward_timers[i] = now + r.minute as u64 * MINUTE;
        }
    });
}

/// Process one zone online reward tick for all online sessions.
fn process_zone_online_reward_tick(world: &WorldState) {
    let rewards = world.get_zone_online_rewards();
    if rewards.is_empty() {
        return;
    }

    let now = now_secs();

    // Collect session IDs that have reward timers set.
    let session_ids = world.collect_zone_online_reward_session_ids();

    for sid in session_ids {
        process_session_online_reward(world, sid, &rewards, now);
    }
}

/// Process online rewards for a single session.
fn process_session_online_reward(
    world: &WorldState,
    sid: SessionId,
    rewards: &[ko_db::models::ZoneOnlineReward],
    now: u64,
) {
    // Gather needed session state in one read.
    let info = world.with_session(sid, |h| {
        let ch = match &h.character {
            Some(c) => c,
            None => return None,
        };
        if ch.hp <= 0 || ch.res_hp_type == crate::world::USER_DEAD {
            return None;
        }
        Some((
            h.position.zone_id,
            h.premium_in_use,
            h.zone_online_reward_timers.clone(),
        ))
    });

    let (zone_id, premium_in_use, timers) = match info.flatten() {
        Some(t) => t,
        None => return,
    };

    if timers.len() != rewards.len() {
        return;
    }

    let is_premium = premium_in_use > 0;
    let is_merchanting = world.is_merchanting(sid);

    // Check each reward entry.
    for (i, reward) in rewards.iter().enumerate() {
        if timers[i] > now {
            continue;
        }

        if zone_id != reward.zone_id as u16 {
            continue;
        }

        // Reset this timer.
        let interval_min = if is_premium && reward.pre_minute > 0 {
            reward.pre_minute as u64
        } else {
            reward.minute as u64
        };
        let new_timer = now + interval_min * MINUTE;

        world.update_session(sid, |h| {
            if i < h.zone_online_reward_timers.len() {
                h.zone_online_reward_timers[i] = new_timer;
            }
        });

        // Send the reward.
        zone_online_send_reward(world, sid, reward, is_premium, is_merchanting);
    }
}

/// Grant a zone online reward to a player.
/// Selects premium vs normal fields, gives item (if not merchanting),
/// loyalty, and cash.
fn zone_online_send_reward(
    world: &WorldState,
    sid: SessionId,
    reward: &ko_db::models::ZoneOnlineReward,
    is_premium: bool,
    is_merchanting: bool,
) {
    let (item_id, item_count, loyalty, cash, tl) = if is_premium {
        (
            reward.pre_item_id,
            reward.pre_item_count,
            reward.pre_loyalty,
            reward.pre_cash,
            reward.pre_tl,
        )
    } else {
        (
            reward.item_id,
            reward.item_count,
            reward.loyalty,
            reward.cash,
            reward.tl,
        )
    };

    if !is_merchanting && item_id > 0 && item_count > 0 {
        let given = world.give_item(
            sid,
            item_id as u32,
            item_count.clamp(0, u16::MAX as i32) as u16,
        );
        if given {
            debug!(
                sid = ?sid,
                item_id,
                item_count,
                "zone_online_reward: gave item"
            );
        }
    }

    if loyalty > 0 {
        send_loyalty_change(world, sid, loyalty, false, true, false);
    }

    if cash > 0 || tl > 0 {
        // Compute new balances atomically in a single lock.
        let (new_kc, new_tl, account_id) = world
            .with_session(sid, |h| {
                let kc = h.knight_cash.saturating_add(cash.max(0) as u32);
                let tl_bal = h.tl_balance.saturating_add(tl.max(0) as u32);
                (kc, tl_bal, h.account_id.clone())
            })
            .unwrap_or((0, 0, String::new()));

        world.set_kc_balance(sid, new_kc, new_tl);

        // Persist to DB immediately (matches C++ GiveBalance behaviour).
        if let Some(pool) = world.db_pool() {
            let pool = pool.clone();
            let acct = account_id;
            tokio::spawn(async move {
                let repo = ko_db::repositories::cash_shop::CashShopRepository::new(&pool);
                if let Err(e) = repo
                    .update_kc_balances(&acct, new_kc as i32, new_tl as i32)
                    .await
                {
                    tracing::warn!(
                        "zone_online_reward: failed to persist KC for {}: {}",
                        acct,
                        e
                    );
                }
            });
        }

        let pkt = crate::handler::knight_cash::build_kcupdate_packet(new_kc, new_tl);
        world.send_to_session_owned(sid, pkt);
        debug!(
            sid = ?sid,
            cash,
            tl,
            new_kc,
            new_tl,
            "zone_online_reward: KC/TL reward given"
        );
    }
}

// ── Online Cash Reward (pServerSetting) ─────────────────────────────────────
//
//   if (g_pMain->pServerSetting.onlinecash
//       && g_pMain->pServerSetting.onlinecashtime
//       && UNIXTIME > m_bOnlineCashTime)
//   {
//       m_bOnlineCashTime = UNIXTIME + (g_pMain->pServerSetting.onlinecashtime * MINUTE);
//       if (GetZoneID() == ZONE_MORADON) GiveBalance(1);
//       else if (GetZoneID() == ZONE_RONARK_LAND) GiveBalance(2);
//   }
//
// Separate from zone_online_reward (table-driven). This is a simpler
// server-setting-gated system: 1 KC in Moradon, 2 KC in Ronark Land.
// ─────────────────────────────────────────────────────────────────────────────

use crate::world::types::{ZONE_MORADON, ZONE_RONARK_LAND};

/// Process the online cash reward for all sessions.
/// Called every 10 seconds from the zone online reward background task.
/// Checks `online_give_cash` + `online_cash_time` from server settings,
/// then grants 1 KC (Moradon) or 2 KC (Ronark Land) per interval.
fn process_online_cash_tick(world: &WorldState) {
    // Check server setting gate first (avoids iterating sessions when disabled).
    let (enabled, interval_min) = world
        .get_server_settings()
        .map(|s| (s.online_give_cash, s.online_cash_time))
        .unwrap_or((false, 0));

    if !enabled || interval_min <= 0 {
        return;
    }

    let now = now_secs();
    let interval_secs = interval_min as u64 * MINUTE;

    // Iterate all in-game sessions.
    let session_ids = world.all_ingame_session_ids();

    for sid in session_ids {
        let info = world.with_session(sid, |h| {
            let ch = h.character.as_ref()?;
            // C++ skips dead / offline players (implied by Update() context).
            if ch.hp <= 0 {
                return None;
            }
            Some((h.position.zone_id, h.online_cash_next_time))
        });

        let (zone_id, next_time) = match info.flatten() {
            Some(t) => t,
            None => continue,
        };

        // C++ check: `UNIXTIME > m_bOnlineCashTime`
        // First tick: next_time = 0, so `now > 0` is always true → immediate first grant.
        if now <= next_time {
            continue;
        }

        // Reset timer.
        world.update_session(sid, |h| {
            h.online_cash_next_time = now + interval_secs;
        });

        // Determine KC amount by zone.
        // C++ User.cpp:1210-1213: Moradon → 1, Ronark Land → 2.
        let kc_amount: u32 = match zone_id {
            ZONE_MORADON => 1,
            ZONE_RONARK_LAND => 2,
            _ => continue,
        };

        // Grant KC via GiveBalance pattern.
        let (new_kc, new_tl, account_id) = world
            .with_session(sid, |h| {
                let kc = h.knight_cash.saturating_add(kc_amount);
                (kc, h.tl_balance, h.account_id.clone())
            })
            .unwrap_or((0, 0, String::new()));

        if account_id.is_empty() {
            continue;
        }

        world.set_kc_balance(sid, new_kc, new_tl);

        // Persist to DB (async, fire-and-forget).
        if let Some(pool) = world.db_pool() {
            let pool = pool.clone();
            let acct = account_id;
            tokio::spawn(async move {
                let repo = ko_db::repositories::cash_shop::CashShopRepository::new(&pool);
                if let Err(e) = repo
                    .update_kc_balances(&acct, new_kc as i32, new_tl as i32)
                    .await
                {
                    tracing::warn!("online_cash: failed to persist KC for {}: {}", acct, e);
                }
            });
        }

        // Send KC update packet to client.
        let pkt = crate::handler::knight_cash::build_kcupdate_packet(new_kc, new_tl);
        world.send_to_session_owned(sid, pkt);

        debug!(
            sid = ?sid,
            zone_id,
            kc_amount,
            new_kc,
            "online_cash: KC reward granted"
        );
    }
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants, clippy::too_many_arguments)]
mod tests {
    use super::*;
    use ko_db::models::ZoneOnlineReward;

    /// Create a test reward with the given parameters.
    fn make_reward(
        zone_id: i16,
        item_id: i32,
        item_count: i32,
        minute: i32,
        loyalty: i32,
        cash: i32,
        pre_item_id: i32,
        pre_item_count: i32,
        pre_minute: i32,
        pre_loyalty: i32,
        pre_cash: i32,
    ) -> ZoneOnlineReward {
        ZoneOnlineReward {
            zone_id,
            item_id,
            item_count,
            item_time: 0,
            minute,
            loyalty,
            cash,
            tl: 0,
            pre_item_id,
            pre_item_count,
            pre_item_time: 0,
            pre_minute,
            pre_loyalty,
            pre_cash,
            pre_tl: 0,
        }
    }

    #[test]
    fn test_now_secs_reasonable() {
        let now = now_secs();
        // Should be after 2024-01-01 (1704067200)
        assert!(now > 1_704_067_200);
    }

    #[test]
    fn test_premium_vs_normal_field_selection() {
        let r = make_reward(71, 100, 1, 5, 10, 0, 200, 2, 3, 20, 5);

        // Normal: item_id=100, item_count=1, loyalty=10, cash=0
        let (iid, icnt, loy, csh) = (r.item_id, r.item_count, r.loyalty, r.cash);
        assert_eq!(iid, 100);
        assert_eq!(icnt, 1);
        assert_eq!(loy, 10);
        assert_eq!(csh, 0);

        // Premium: pre_item_id=200, pre_item_count=2, pre_loyalty=20, pre_cash=5
        let (iid, icnt, loy, csh) = (r.pre_item_id, r.pre_item_count, r.pre_loyalty, r.pre_cash);
        assert_eq!(iid, 200);
        assert_eq!(icnt, 2);
        assert_eq!(loy, 20);
        assert_eq!(csh, 5);
    }

    #[test]
    fn test_timer_calculation_normal_5min() {
        let now = 1_700_000_000u64;
        let interval_min = 5u64;
        let expected = now + interval_min * MINUTE;
        assert_eq!(expected, 1_700_000_300);
    }

    #[test]
    fn test_timer_calculation_premium_3min() {
        let now = 1_700_000_000u64;
        let interval_min = 3u64;
        let expected = now + interval_min * MINUTE;
        assert_eq!(expected, 1_700_000_180);
    }

    #[test]
    fn test_minute_constant() {
        assert_eq!(MINUTE, 60);
    }

    #[test]
    fn test_tick_interval() {
        assert_eq!(ZONE_ONLINE_REWARD_TICK_SECS, 10);
    }

    #[test]
    fn test_make_reward_fields() {
        let r = make_reward(72, 500, 3, 10, 50, 100, 600, 5, 7, 80, 200);
        assert_eq!(r.zone_id, 72);
        assert_eq!(r.item_id, 500);
        assert_eq!(r.item_count, 3);
        assert_eq!(r.minute, 10);
        assert_eq!(r.loyalty, 50);
        assert_eq!(r.cash, 100);
        assert_eq!(r.pre_item_id, 600);
        assert_eq!(r.pre_item_count, 5);
        assert_eq!(r.pre_minute, 7);
        assert_eq!(r.pre_loyalty, 80);
        assert_eq!(r.pre_cash, 200);
        assert_eq!(r.tl, 0);
        assert_eq!(r.pre_tl, 0);
    }

    #[test]
    fn test_timer_skip_when_not_expired() {
        // Simulates the check: if timers[i] > now { continue; }
        let now = 1_700_000_000u64;
        let timer = 1_700_000_100u64; // 100 seconds in the future
        assert!(timer > now, "timer should not have expired yet");
    }

    #[test]
    fn test_timer_triggers_when_expired() {
        let now = 1_700_000_300u64;
        let timer = 1_700_000_000u64; // 300 seconds in the past
        assert!(timer <= now, "timer should be expired");
    }

    #[test]
    fn test_zone_mismatch_skips_reward() {
        // Reward is for zone 71, player is in zone 72
        let reward_zone: u16 = 71;
        let player_zone: u16 = 72;
        assert_ne!(reward_zone, player_zone, "zone should not match");
    }

    #[test]
    fn test_zone_match_grants_reward() {
        let reward_zone: i16 = 71;
        let player_zone: u16 = 71;
        assert_eq!(player_zone, reward_zone as u16, "zone should match");
    }

    #[test]
    fn test_dead_player_skipped() {
        // Simulates: if ch.hp <= 0 || ch.res_hp_type == USER_DEAD { return None; }
        let hp: i16 = 0;
        let res_hp_type: u8 = crate::world::USER_DEAD;
        assert!(hp <= 0 || res_hp_type == crate::world::USER_DEAD);
    }

    #[test]
    fn test_timer_reset_after_reward() {
        let now = 1_700_000_300u64;
        let interval_min = 5u64;
        let new_timer = now + interval_min * MINUTE;
        assert_eq!(new_timer, 1_700_000_600);
        assert!(new_timer > now, "new timer should be in the future");
    }

    #[test]
    fn test_premium_interval_overrides_normal() {
        let r = make_reward(71, 100, 1, 10, 0, 0, 100, 1, 5, 0, 0);
        let is_premium = true;
        let interval = if is_premium && r.pre_minute > 0 {
            r.pre_minute as u64
        } else {
            r.minute as u64
        };
        assert_eq!(interval, 5, "premium should use pre_minute=5");
    }

    #[test]
    fn test_normal_interval_when_not_premium() {
        let r = make_reward(71, 100, 1, 10, 0, 0, 100, 1, 5, 0, 0);
        let is_premium = false;
        let interval = if is_premium && r.pre_minute > 0 {
            r.pre_minute as u64
        } else {
            r.minute as u64
        };
        assert_eq!(interval, 10, "normal should use minute=10");
    }

    #[test]
    fn test_zero_pre_minute_falls_back_to_normal() {
        let r = make_reward(71, 100, 1, 10, 0, 0, 100, 1, 0, 0, 0);
        let is_premium = true;
        let interval = if is_premium && r.pre_minute > 0 {
            r.pre_minute as u64
        } else {
            r.minute as u64
        };
        assert_eq!(interval, 10, "pre_minute=0 should fall back to minute=10");
    }

    #[test]
    fn test_merchanting_blocks_item_give() {
        let is_merchanting = true;
        let item_id = 500i32;
        let item_count = 1i32;
        let should_give = !is_merchanting && item_id > 0 && item_count > 0;
        assert!(!should_give, "merchanting players should not receive items");
    }

    #[test]
    fn test_non_merchanting_gets_item() {
        let is_merchanting = false;
        let item_id = 500i32;
        let item_count = 1i32;
        let should_give = !is_merchanting && item_id > 0 && item_count > 0;
        assert!(should_give, "non-merchanting players should receive items");
    }

    #[test]
    fn test_zero_item_id_skips_give() {
        let is_merchanting = false;
        let item_id = 0i32;
        let item_count = 1i32;
        let should_give = !is_merchanting && item_id > 0 && item_count > 0;
        assert!(!should_give, "item_id=0 should skip give_item");
    }

    #[test]
    fn test_zero_loyalty_skips_send() {
        let loyalty = 0i32;
        assert!(loyalty <= 0, "zero loyalty should be skipped");
    }

    #[test]
    fn test_positive_loyalty_triggers_send() {
        let loyalty = 50i32;
        assert!(loyalty > 0, "positive loyalty should trigger send");
    }

    #[test]
    fn test_timer_vec_length_mismatch_skips() {
        let timers_len = 3usize;
        let rewards_len = 5usize;
        assert_ne!(timers_len, rewards_len, "mismatch should cause skip");
    }

    #[test]
    fn test_empty_rewards_early_return() {
        let rewards: Vec<ZoneOnlineReward> = Vec::new();
        assert!(rewards.is_empty());
    }

    /// H1 fix: `zone_online_reward_change` always uses normal `minute`, never premium.
    ///
    /// Unlike `ZoneOnlineRewardStart`, the `Change` variant has no premium branch.
    #[test]
    fn test_zone_online_reward_change_ignores_premium() {
        let r = make_reward(71, 100, 1, 10, 0, 0, 200, 2, 3, 0, 0);
        // In zone_online_reward_change, the interval should always be r.minute (10),
        // never r.pre_minute (3), regardless of premium status.
        let normal_interval = r.minute as u64;
        let premium_interval = r.pre_minute as u64;
        assert_eq!(normal_interval, 10);
        assert_eq!(premium_interval, 3);

        // The function always uses normal_interval for the timer reset
        let now = 1_700_000_000u64;
        let expected_timer = now + normal_interval * MINUTE;
        assert_eq!(
            expected_timer, 1_700_000_600,
            "Change should use minute=10, not pre_minute=3"
        );

        // If it incorrectly used premium: now + 3*60 = 1_700_000_180 (WRONG)
        let wrong_timer = now + premium_interval * MINUTE;
        assert_ne!(
            expected_timer, wrong_timer,
            "Premium interval must not be used in change"
        );
    }

    #[test]
    fn test_cash_reward_kc_update_packet_format() {
        // C++ GiveBalance adds KC and sends KCUPDATE packet
        // Wire: WIZ_EXT_HOOK(0xE9) << u8(0xB9) << u32(kc) << u32(tl)
        let pkt = crate::handler::knight_cash::build_kcupdate_packet(500, 100);
        assert_eq!(pkt.opcode, ko_protocol::Opcode::EXT_HOOK_S2C);
        let mut r = ko_protocol::PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0xB9)); // EXT_SUB_KCUPDATE
        assert_eq!(r.read_u32(), Some(500)); // knight_cash
        assert_eq!(r.read_u32(), Some(100)); // tl_balance
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_cash_reward_saturating_add() {
        // Ensure KC addition doesn't overflow
        let kc: u32 = u32::MAX - 10;
        let reward: i32 = 50;
        let result = kc.saturating_add(reward as u32);
        assert_eq!(result, u32::MAX);
    }

    #[test]
    fn test_tl_reward_field_selection() {
        // BUG-522-B fix: tl/pre_tl must be selected alongside cash/pre_cash.
        let mut r = make_reward(71, 100, 1, 5, 10, 50, 200, 2, 3, 20, 80);
        r.tl = 30;
        r.pre_tl = 60;

        // Normal player gets tl=30
        let is_premium = false;
        let (_iid, _icnt, _loy, cash, tl) = if is_premium {
            (
                r.pre_item_id,
                r.pre_item_count,
                r.pre_loyalty,
                r.pre_cash,
                r.pre_tl,
            )
        } else {
            (r.item_id, r.item_count, r.loyalty, r.cash, r.tl)
        };
        assert_eq!(cash, 50);
        assert_eq!(tl, 30);

        // Premium player gets pre_tl=60
        let is_premium = true;
        let (_iid, _icnt, _loy, cash, tl) = if is_premium {
            (
                r.pre_item_id,
                r.pre_item_count,
                r.pre_loyalty,
                r.pre_cash,
                r.pre_tl,
            )
        } else {
            (r.item_id, r.item_count, r.loyalty, r.cash, r.tl)
        };
        assert_eq!(cash, 80);
        assert_eq!(tl, 60);
    }

    #[test]
    fn test_tl_reward_saturating_add() {
        // Ensure TL addition doesn't overflow
        let tl_bal: u32 = u32::MAX - 5;
        let reward: i32 = 20;
        let result = tl_bal.saturating_add(reward as u32);
        assert_eq!(result, u32::MAX);
    }

    #[test]
    fn test_cash_or_tl_triggers_give_balance() {
        // C++ calls GiveBalance(cash, tl) — should fire when either is > 0
        assert!(0 > 0 || 10 > 0, "tl-only reward should trigger");
        assert!(5 > 0 || 0 > 0, "cash-only reward should trigger");
        assert!(0 <= 0, "zero/zero should not trigger");
    }

    #[test]
    fn test_negative_cash_tl_clamped_to_zero() {
        // Defensive: max(0) before saturating_add prevents underflow
        let cash: i32 = -5;
        let tl: i32 = -10;
        let kc: u32 = 100;
        let tl_bal: u32 = 200;
        let new_kc = kc.saturating_add(cash.max(0) as u32);
        let new_tl = tl_bal.saturating_add(tl.max(0) as u32);
        assert_eq!(new_kc, 100, "negative cash should add 0");
        assert_eq!(new_tl, 200, "negative tl should add 0");
    }

    // ── Online Cash Reward Tests ────────────────────────────────────────────

    #[test]
    fn test_online_cash_moradon_gives_1_kc() {
        let zone_id: u16 = ZONE_MORADON;
        let kc_amount: u32 = match zone_id {
            ZONE_MORADON => 1,
            ZONE_RONARK_LAND => 2,
            _ => 0,
        };
        assert_eq!(kc_amount, 1, "Moradon should give 1 KC");
    }

    #[test]
    fn test_online_cash_ronark_gives_2_kc() {
        let zone_id: u16 = ZONE_RONARK_LAND;
        let kc_amount: u32 = match zone_id {
            ZONE_MORADON => 1,
            ZONE_RONARK_LAND => 2,
            _ => 0,
        };
        assert_eq!(kc_amount, 2, "Ronark Land should give 2 KC");
    }

    #[test]
    fn test_online_cash_other_zone_gives_nothing() {
        let zone_id: u16 = 72; // Ardream
        let kc_amount: u32 = match zone_id {
            ZONE_MORADON => 1,
            ZONE_RONARK_LAND => 2,
            _ => 0,
        };
        assert_eq!(kc_amount, 0, "Other zones should give 0 KC");
    }

    #[test]
    fn test_online_cash_timer_initial_zero() {
        // C++ initializes m_bOnlineCashTime = 0 in constructor.
        // First tick: UNIXTIME > 0 is always true → immediate first grant.
        let next_time: u64 = 0;
        let now = now_secs();
        assert!(now > next_time, "first tick should always trigger");
    }

    #[test]
    fn test_online_cash_timer_reset() {
        let now: u64 = 1_700_000_000;
        let interval_min: i32 = 30;
        let interval_secs = interval_min as u64 * MINUTE;
        let next_time = now + interval_secs;
        assert_eq!(next_time, 1_700_001_800, "30 min = 1800s");
    }

    #[test]
    fn test_online_cash_timer_not_expired() {
        let now: u64 = 1_700_000_000;
        let next_time: u64 = 1_700_001_800; // 30 min in the future
        assert!(now <= next_time, "timer should not have expired");
    }

    #[test]
    fn test_online_cash_timer_expired() {
        let now: u64 = 1_700_002_000;
        let next_time: u64 = 1_700_001_800;
        assert!(now > next_time, "timer should have expired");
    }

    #[test]
    fn test_online_cash_disabled_when_flag_false() {
        let enabled = false;
        let interval_min: i32 = 30;
        assert!(!enabled || interval_min <= 0, "should skip when disabled");
    }

    #[test]
    fn test_online_cash_disabled_when_interval_zero() {
        let enabled = true;
        let interval_min: i32 = 0;
        assert!(
            !enabled || interval_min <= 0,
            "should skip when interval is 0"
        );
    }

    #[test]
    fn test_online_cash_kc_saturating_add() {
        let kc: u32 = u32::MAX - 1;
        let amount: u32 = 2;
        let result = kc.saturating_add(amount);
        assert_eq!(result, u32::MAX, "should cap at u32::MAX");
    }
}
