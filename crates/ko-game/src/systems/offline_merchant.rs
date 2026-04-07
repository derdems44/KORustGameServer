//! Offline merchant tick system.
//!
//! C++ Reference: `User.cpp:1174-1192` — offline check inside `CUser::Update()`
//!
//! Runs every 10 seconds, checking all offline sessions.  When a session's
//! `offline_next_check` has elapsed, the remaining minutes counter is
//! decremented.  When it reaches 0 the session is disconnected and the
//! merchant is closed.
//!
//! The C++ server ticks each user at ~100 ms but the offline check only fires
//! every 60 seconds (via `m_bOfflineCheck`).  We replicate that by storing the
//! next-check instant on the session handle and polling at a coarser 10-second
//! granularity — accurate enough and lighter on CPU.

use std::sync::Arc;
use std::time::Duration;

use ko_protocol::{Opcode, Packet};
use tracing::{debug, info};

use crate::world::WorldState;

/// Tick interval for the offline merchant background task (seconds).
///
/// We poll at 10 s; actual per-session decrement happens based on the
/// `offline_next_check` instant stored on each session handle (60 s cadence).
const OFFLINE_TICK_INTERVAL_SECS: u64 = 10;

/// Start the offline merchant background task.
///
/// Returns a `JoinHandle` so the caller can abort on shutdown.
pub fn start_offline_merchant_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(OFFLINE_TICK_INTERVAL_SECS));
        loop {
            interval.tick().await;
            process_offline_tick(&world).await;
        }
    })
}

/// Process one offline merchant tick.
///
/// Collects expired sessions from `WorldState::tick_offline_merchants()` and
/// performs the cleanup sequence for each: deactivate offline status, close the
/// merchant, broadcast MERCHANT_CLOSE, broadcast USER_INOUT_OUT, remove from
/// zone region, and finally unregister the session.
async fn process_offline_tick(world: &WorldState) {
    let expired = world.tick_offline_merchants();

    for sid in expired {
        info!(
            "[sid={}] Offline merchant time expired — disconnecting",
            sid
        );
        cleanup_offline_session(world, sid).await;
    }
}

/// Full cleanup for an expired or forcefully closed offline merchant session.
///
/// C++ Reference: `CUser::goDisconnect()` — closes socket, triggers cleanup.
///
/// This replicates the essential parts of `ClientSession::cleanup()` but
/// operates purely through the `WorldState` since there is no live TCP session
/// for an offline merchant.
pub async fn cleanup_offline_session(world: &WorldState, sid: crate::zone::SessionId) {
    // 1. Deactivate offline flag
    world.deactivate_offline_status(sid);

    // 2. Close merchant and broadcast
    let is_selling = world.is_selling_merchant(sid);
    let is_buying = world.is_buying_merchant(sid);

    if is_selling || is_buying {
        world.close_merchant(sid);

        let mut close_pkt = Packet::new(Opcode::WizMerchant as u8);
        close_pkt.write_u8(2); // MERCHANT_CLOSE
        close_pkt.write_u32(sid as u32);

        if let Some((pos, event_room)) = world.with_session(sid, |h| (h.position, h.event_room)) {
            world.broadcast_to_3x3(
                pos.zone_id,
                pos.region_x,
                pos.region_z,
                Arc::new(close_pkt),
                None,
                event_room,
            );
        }
    }

    // 3. Party cleanup
    world.cleanup_party_on_disconnect(sid);

    // 4. Knights/Clan offline notification
    if let Some(ch) = world.get_character_info(sid) {
        if ch.knights_id > 0 {
            world.knights_clan_buff_update(ch.knights_id, false, sid);
            crate::handler::knights::send_clan_offline_notification(
                world,
                ch.knights_id,
                &ch.name,
                sid,
            );
        }
    }

    // 5. Zone-wide logout notification
    if let Some(ch) = world.get_character_info(sid) {
        if let Some(pos) = world.get_position(sid) {
            let region_del_pkt = crate::handler::user_info::build_region_delete_packet(&ch.name);
            world.broadcast_to_zone(pos.zone_id, Arc::new(region_del_pkt), Some(sid));
        }
    }

    // 6. Zone region removal + INOUT_OUT broadcast
    if let Some((pos, event_room)) = world.with_session(sid, |h| (h.position, h.event_room)) {
        if let Some(zone) = world.get_zone(pos.zone_id) {
            zone.remove_user(pos.region_x, pos.region_z, sid);
        }

        let out_pkt = crate::handler::region::build_user_inout(
            crate::handler::region::INOUT_OUT,
            sid,
            None,
            &Default::default(),
        );
        world.broadcast_to_3x3(
            pos.zone_id,
            pos.region_x,
            pos.region_z,
            Arc::new(out_pkt),
            Some(sid),
            event_room,
        );
    }

    // 7. Ranking cleanup
    world.pk_zone_remove_player(sid);
    world.zindan_remove_player(sid);
    world.bdw_remove_player(sid);
    world.chaos_remove_player(sid);

    // 8. DB character save (via character_save system)
    if let Some(pool) = world.db_pool() {
        crate::systems::character_save::save_single_character_sync(
            world,
            pool,
            sid,
            "Offline merchant expire save",
        )
        .await;
    }

    // 9. Unregister session from world
    debug!("[sid={}] Offline merchant session unregistered", sid);
    world.unregister_session(sid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::types::{
        MERCHANT_STATE_SELLING, OFFLINE_CHECK_INTERVAL_SECS, OFFLINE_DEFAULT_MINUTES,
    };
    use crate::world::WorldState;
    use ko_protocol::{Opcode, PacketReader};
    use std::time::{Duration, Instant};

    /// Helper: register a session and set it as an offline selling merchant.
    fn setup_offline_merchant(world: &WorldState) -> crate::zone::SessionId {
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        let sid: crate::zone::SessionId = 100;
        world.register_session(sid, tx);
        // Set up as selling merchant + offline
        world.update_session(sid, |h| {
            h.merchant_state = MERCHANT_STATE_SELLING;
            h.is_offline = true;
            h.offline_remaining_minutes = 2; // 2 minutes remaining
            h.offline_next_check = Some(Instant::now()); // due immediately
        });
        sid
    }

    #[test]
    fn test_tick_offline_merchants_decrements_minutes() {
        let world = WorldState::new();
        let sid = setup_offline_merchant(&world);

        // First tick: decrement from 2 → 1 (not expired yet)
        let expired = world.tick_offline_merchants();
        assert!(expired.is_empty(), "Should not expire with 1 minute left");

        // Verify minutes decremented
        let remaining = world
            .with_session(sid, |h| h.offline_remaining_minutes)
            .unwrap();
        assert_eq!(remaining, 1);
    }

    #[test]
    fn test_tick_offline_merchants_expires_at_zero() {
        let world = WorldState::new();
        let sid = setup_offline_merchant(&world);

        // Set to 1 minute — next tick should expire
        world.update_session(sid, |h| {
            h.offline_remaining_minutes = 1;
            h.offline_next_check = Some(Instant::now());
        });

        let expired = world.tick_offline_merchants();
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], sid);
    }

    #[test]
    fn test_tick_offline_merchants_ignores_non_offline() {
        let world = WorldState::new();
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        let sid: crate::zone::SessionId = 200;
        world.register_session(sid, tx);
        // Not offline — should be ignored
        world.update_session(sid, |h| {
            h.is_offline = false;
            h.offline_remaining_minutes = 1;
            h.offline_next_check = Some(Instant::now());
        });

        let expired = world.tick_offline_merchants();
        assert!(expired.is_empty());
    }

    #[test]
    fn test_tick_offline_merchants_respects_next_check_time() {
        let world = WorldState::new();
        let sid = setup_offline_merchant(&world);

        // Set next check far in the future
        world.update_session(sid, |h| {
            h.offline_next_check = Some(Instant::now() + Duration::from_secs(3600));
        });

        let expired = world.tick_offline_merchants();
        assert!(expired.is_empty(), "Should not tick before next_check time");

        // Minutes should be unchanged (2)
        let remaining = world
            .with_session(sid, |h| h.offline_remaining_minutes)
            .unwrap();
        assert_eq!(remaining, 2);
    }

    #[test]
    fn test_tick_offline_merchants_already_zero_expires_immediately() {
        let world = WorldState::new();
        let sid = setup_offline_merchant(&world);

        // Already at 0 minutes
        world.update_session(sid, |h| {
            h.offline_remaining_minutes = 0;
            h.offline_next_check = Some(Instant::now());
        });

        let expired = world.tick_offline_merchants();
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], sid);
    }

    #[test]
    fn test_deactivate_offline_status() {
        let world = WorldState::new();
        let sid = setup_offline_merchant(&world);

        world.deactivate_offline_status(sid);

        let (is_offline, remaining) = world
            .with_session(sid, |h| (h.is_offline, h.offline_remaining_minutes))
            .unwrap();
        assert!(!is_offline);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn test_merchant_close_packet_format() {
        // Verify the MERCHANT_CLOSE packet format matches C++ expectation
        let mut close_pkt = Packet::new(Opcode::WizMerchant as u8);
        close_pkt.write_u8(2); // MERCHANT_CLOSE
        close_pkt.write_u32(42); // session_id

        assert_eq!(close_pkt.opcode, Opcode::WizMerchant as u8);
        let mut r = PacketReader::new(&close_pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // sub = MERCHANT_CLOSE
        assert_eq!(r.read_u32(), Some(42)); // session_id
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_offline_default_minutes_value() {
        // C++ Reference: default offline time is ~23 hours (1400 minutes)
        assert_eq!(OFFLINE_DEFAULT_MINUTES, 1400);
    }

    #[test]
    fn test_offline_check_interval_value() {
        // C++ Reference: offline check fires every 60 seconds
        assert_eq!(OFFLINE_CHECK_INTERVAL_SECS, 60);
    }

    #[test]
    fn test_tick_interval_constant() {
        // Polling interval is 10 seconds (coarser than per-session 60s cadence)
        assert_eq!(OFFLINE_TICK_INTERVAL_SECS, 10);
    }

    #[test]
    fn test_multiple_offline_sessions_tick_independently() {
        let world = WorldState::new();
        let sid1 = setup_offline_merchant(&world);

        // Create second offline merchant with more time
        let (tx2, _rx2) = tokio::sync::mpsc::unbounded_channel();
        let sid2: crate::zone::SessionId = 101;
        world.register_session(sid2, tx2);
        world.update_session(sid2, |h| {
            h.merchant_state = MERCHANT_STATE_SELLING;
            h.is_offline = true;
            h.offline_remaining_minutes = 5;
            h.offline_next_check = Some(Instant::now());
        });

        // Set sid1 to 1 minute (will expire), sid2 to 5 minutes (won't)
        world.update_session(sid1, |h| {
            h.offline_remaining_minutes = 1;
            h.offline_next_check = Some(Instant::now());
        });

        let expired = world.tick_offline_merchants();
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], sid1);

        // sid2 should have decremented to 4
        let remaining = world
            .with_session(sid2, |h| h.offline_remaining_minutes)
            .unwrap();
        assert_eq!(remaining, 4);
    }
}
