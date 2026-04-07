//! Auto mining/fishing background tick system.
//! When a player has an auto-mining or auto-fishing item equipped in the
//! SHOULDER slot (active) or CFAIRY slot (offline merchant), the server
//! automatically performs mining/fishing every 5 seconds, depositing
//! rewards into the player's warehouse.
//! ## Active Mode (SHOULDER slot)
//! - `AUTOMATIC_MINING (501610000)` or `MINING_ROBIN_ITEM (510000000)` → mining
//! - `AUTOMATIC_FISHING (501620000)` or `FISHING_ROBIN_ITEM (520000000)` → fishing
//! ## Offline Mode (CFAIRY slot) — Sprint 447
//! - `MERCHANT_AUTO_MANING (700049758)` or `OFFLINE_MINNING (700059759)` → mining
//! - `MERCHANT_AUTO_FISHING (700099755)` or `OFFLINE_FISHING (700069754)` → fishing

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tracing::debug;

use crate::handler::mining::weighted_random_item;
use crate::world::WorldState;
use crate::zone::SessionId;

// ── Item ID constants ─────────────────────────────────────────────────

/// Auto-mining shoulder item.
const AUTOMATIC_MINING: u32 = 501_610_000;

/// Auto-fishing shoulder item.
const AUTOMATIC_FISHING: u32 = 501_620_000;

/// Robin mining item (also triggers auto-mining from shoulder).
const MINING_ROBIN_ITEM: u32 = 510_000_000;

/// Robin fishing item (also triggers auto-fishing from shoulder).
const FISHING_ROBIN_ITEM: u32 = 520_000_000;

/// Offline mining item (CFAIRY slot).
const OFFLINE_MINNING: u32 = 700_059_759;

/// Offline fishing item (CFAIRY slot).
const OFFLINE_FISHING: u32 = 700_069_754;

use crate::world::ITEM_EXP;

/// Auto-harvest tick interval (seconds).
const AUTO_HARVEST_INTERVAL: u64 = 5;

/// Minimum free warehouse slots required to proceed.
/// the `ITEMS_IN_EXCHANGE_GROUP` (5) is only an early-exit optimization for
/// the counting loop, NOT a minimum threshold.
const MIN_FREE_WAREHOUSE_SLOTS: u8 = 1;

/// SHOULDER equipment slot index.
const SHOULDER_SLOT: usize = 5;

/// CFAIRY cospre slot index.
const CFAIRY_SLOT: usize = 48;

use crate::inventory_constants::WAREHOUSE_MAX;

// ── Background tick task ──────────────────────────────────────────────

/// Start the auto-harvest background task.
/// Ticks every 5 seconds, checking all in-game sessions for auto-mining
/// and auto-fishing equipment.
pub fn start_auto_harvest_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(AUTO_HARVEST_INTERVAL));
        loop {
            interval.tick().await;
            process_auto_harvest_tick(&world);
        }
    })
}

/// Process one auto-harvest tick — check all in-game sessions.
fn process_auto_harvest_tick(world: &Arc<WorldState>) {
    let now = current_unix_time();
    let session_ids = world.get_in_game_session_ids();

    for sid in session_ids {
        // Skip if player is actively mining/fishing (manual mode)
        let (is_mining, is_fishing, is_offline) =
            match world.with_session(sid, |h| (h.is_mining, h.is_fishing, h.is_offline)) {
                Some(v) => v,
                None => continue,
            };

        if is_mining || is_fishing {
            continue;
        }

        if !is_offline {
            // Active mode: check SHOULDER slot
            process_active_mining(world, sid, now);
            process_active_fishing(world, sid, now);
        } else {
            // Offline mode: check CFAIRY slot
            process_offline_mining(world, sid, now);
            process_offline_fishing(world, sid, now);
        }
    }
}

// ── Active mode (SHOULDER slot) ───────────────────────────────────────

/// Process auto-mining for active (non-offline) players.
fn process_active_mining(world: &Arc<WorldState>, sid: SessionId, now: u64) {
    let last_tick = world.with_session(sid, |h| h.auto_mining_time).unwrap_or(0);
    if now < last_tick + AUTO_HARVEST_INTERVAL {
        return;
    }

    // Check SHOULDER slot for auto-mining items
    let shoulder_item = world
        .get_inventory_slot(sid, SHOULDER_SLOT)
        .map(|s| s.item_id);
    let has_auto_mining = matches!(
        shoulder_item,
        Some(AUTOMATIC_MINING) | Some(MINING_ROBIN_ITEM)
    );
    if !has_auto_mining {
        return;
    }

    // Update timer
    world.update_session(sid, |h| {
        h.auto_mining_time = now + AUTO_HARVEST_INTERVAL;
    });

    // Get mining items (nTableType=0, UseItemType=4, nWarStatus=0)
    let items = world.get_mining_fishing_items(0, 4, 0);
    do_auto_harvest(world, sid, &items, "Auto Mining");
}

/// Process auto-fishing for active (non-offline) players.
fn process_active_fishing(world: &Arc<WorldState>, sid: SessionId, now: u64) {
    let last_tick = world
        .with_session(sid, |h| h.auto_fishing_time)
        .unwrap_or(0);
    if now < last_tick + AUTO_HARVEST_INTERVAL {
        return;
    }

    // Check SHOULDER slot for auto-fishing items
    let shoulder_item = world
        .get_inventory_slot(sid, SHOULDER_SLOT)
        .map(|s| s.item_id);
    let has_auto_fishing = matches!(
        shoulder_item,
        Some(AUTOMATIC_FISHING) | Some(FISHING_ROBIN_ITEM)
    );
    if !has_auto_fishing {
        return;
    }

    // Update timer
    world.update_session(sid, |h| {
        h.auto_fishing_time = now + AUTO_HARVEST_INTERVAL;
    });

    // Get fishing items (nTableType=1, UseItemType=4, nWarStatus=0)
    let items = world.get_mining_fishing_items(1, 4, 0);
    do_auto_harvest(world, sid, &items, "Auto Fishing");
}

// ── Offline mode (CFAIRY slot) ────────────────────────────────────────

/// Process auto-mining for offline merchant players.
fn process_offline_mining(world: &Arc<WorldState>, sid: SessionId, now: u64) {
    let last_tick = world.with_session(sid, |h| h.auto_mining_time).unwrap_or(0);
    if now < last_tick + AUTO_HARVEST_INTERVAL {
        return;
    }

    // Check CFAIRY slot for offline mining items
    use crate::world::types::MERCHANT_AUTO_MANING;
    let cfairy_item = world
        .get_inventory_slot(sid, CFAIRY_SLOT)
        .map(|s| s.item_id);
    let has_offline_mining = matches!(
        cfairy_item,
        Some(MERCHANT_AUTO_MANING) | Some(OFFLINE_MINNING)
    );
    if !has_offline_mining {
        return;
    }

    world.update_session(sid, |h| {
        h.auto_mining_time = now + AUTO_HARVEST_INTERVAL;
    });

    let items = world.get_mining_fishing_items(0, 4, 0);
    do_auto_harvest(world, sid, &items, "Offline Mining");
}

/// Process auto-fishing for offline merchant players.
fn process_offline_fishing(world: &Arc<WorldState>, sid: SessionId, now: u64) {
    let last_tick = world
        .with_session(sid, |h| h.auto_fishing_time)
        .unwrap_or(0);
    if now < last_tick + AUTO_HARVEST_INTERVAL {
        return;
    }

    use crate::world::types::MERCHANT_AUTO_FISHING;
    let cfairy_item = world
        .get_inventory_slot(sid, CFAIRY_SLOT)
        .map(|s| s.item_id);
    let has_offline_fishing = matches!(
        cfairy_item,
        Some(MERCHANT_AUTO_FISHING) | Some(OFFLINE_FISHING)
    );
    if !has_offline_fishing {
        return;
    }

    world.update_session(sid, |h| {
        h.auto_fishing_time = now + AUTO_HARVEST_INTERVAL;
    });

    let items = world.get_mining_fishing_items(1, 4, 0);
    do_auto_harvest(world, sid, &items, "Offline Fishing");
}

// ── Shared harvest logic ──────────────────────────────────────────────

/// Perform the actual auto-harvest: weighted random item selection, warehouse
/// space check, and deposit (or EXP grant for ITEM_EXP).
fn do_auto_harvest(
    world: &Arc<WorldState>,
    sid: SessionId,
    items: &[ko_db::models::MiningFishingItemRow],
    source: &str,
) {
    if items.is_empty() {
        return;
    }

    let item_id = match weighted_random_item(items) {
        Some(id) => id,
        None => return,
    };

    // Check warehouse free slots (need at least MIN_FREE_WAREHOUSE_SLOTS)
    let warehouse = world.get_warehouse(sid);
    let free_slots = warehouse
        .iter()
        .take(WAREHOUSE_MAX)
        .filter(|s| s.item_id == 0)
        .count();
    if free_slots < MIN_FREE_WAREHOUSE_SLOTS as usize {
        return;
    }

    if item_id == ITEM_EXP {
        // EXP reward based on player level
        let level = world
            .get_character_info(sid)
            .map(|ch| ch.level)
            .unwrap_or(1);
        let exp_amount = exp_for_level(level);
        if exp_amount > 0 {
            // Fire-and-forget EXP grant
            let world_c = world.clone();
            tokio::spawn(async move {
                crate::handler::level::exp_change(&world_c, sid, exp_amount as i64).await;
            });
        }
        debug!(
            "[sid={}] {}: ITEM_EXP → {}xp (level={})",
            sid,
            source,
            exp_for_level(level),
            level
        );
    } else {
        // Give item to warehouse
        let gave = world.give_warehouse_item(sid, item_id, 1, 0);
        if gave {
            debug!(
                "[sid={}] {}: gave item {} to warehouse",
                sid, source, item_id
            );
        }
    }
}

/// EXP amount by player level for auto-mining/fishing.
/// ```text
/// Level 1-34:   50 EXP
/// Level 35-59:  100 EXP
/// Level 60-69:  200 EXP
/// Level 70-83:  300 EXP
/// ```
fn exp_for_level(level: u8) -> u32 {
    match level {
        1..=34 => 50,
        35..=59 => 100,
        60..=69 => 200,
        70..=83 => 300,
        _ => 0,
    }
}

/// Get the current unix timestamp in seconds.
fn current_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp_for_level_brackets() {
        assert_eq!(exp_for_level(1), 50);
        assert_eq!(exp_for_level(10), 50);
        assert_eq!(exp_for_level(34), 50);
        assert_eq!(exp_for_level(35), 100);
        assert_eq!(exp_for_level(50), 100);
        assert_eq!(exp_for_level(59), 100);
        assert_eq!(exp_for_level(60), 200);
        assert_eq!(exp_for_level(69), 200);
        assert_eq!(exp_for_level(70), 300);
        assert_eq!(exp_for_level(83), 300);
        assert_eq!(exp_for_level(0), 0);
    }

    #[test]
    fn test_auto_harvest_constants() {
        assert_eq!(AUTOMATIC_MINING, 501_610_000);
        assert_eq!(AUTOMATIC_FISHING, 501_620_000);
        assert_eq!(MINING_ROBIN_ITEM, 510_000_000);
        assert_eq!(FISHING_ROBIN_ITEM, 520_000_000);
        assert_eq!(OFFLINE_MINNING, 700_059_759);
        assert_eq!(OFFLINE_FISHING, 700_069_754);
        assert_eq!(ITEM_EXP, 900_001_000);
        assert_eq!(AUTO_HARVEST_INTERVAL, 5);
        assert_eq!(MIN_FREE_WAREHOUSE_SLOTS, 1);
        assert_eq!(SHOULDER_SLOT, 5);
        assert_eq!(CFAIRY_SLOT, 48);
        assert_eq!(WAREHOUSE_MAX, 192);
    }

    #[test]
    fn test_exp_for_level_boundary_values() {
        // Boundary between brackets
        assert_eq!(exp_for_level(34), 50);
        assert_eq!(exp_for_level(35), 100);
        assert_eq!(exp_for_level(59), 100);
        assert_eq!(exp_for_level(60), 200);
        assert_eq!(exp_for_level(69), 200);
        assert_eq!(exp_for_level(70), 300);
    }

    // ── Sprint 935: Additional coverage ──────────────────────────────

    /// Levels above 83 yield 0 EXP.
    #[test]
    fn test_exp_for_level_above_83() {
        assert_eq!(exp_for_level(84), 0);
        assert_eq!(exp_for_level(100), 0);
        assert_eq!(exp_for_level(255), 0);
    }

    /// Active and offline mining items are distinct IDs.
    #[test]
    fn test_active_vs_offline_items_distinct() {
        assert_ne!(AUTOMATIC_MINING, OFFLINE_MINNING);
        assert_ne!(AUTOMATIC_FISHING, OFFLINE_FISHING);
        assert_ne!(MINING_ROBIN_ITEM, OFFLINE_MINNING);
        assert_ne!(FISHING_ROBIN_ITEM, OFFLINE_FISHING);
    }

    /// Auto-harvest interval is 5 seconds as Duration.
    #[test]
    fn test_harvest_interval_duration() {
        let d = Duration::from_secs(AUTO_HARVEST_INTERVAL);
        assert_eq!(d.as_secs(), 5);
    }

    /// current_unix_time returns a reasonable timestamp.
    #[test]
    fn test_current_unix_time_reasonable() {
        let t = current_unix_time();
        // Must be after 2024-01-01 (1704067200)
        assert!(t > 1_704_067_200);
    }

    /// CFAIRY slot is in cospre range (>= INVENTORY_COSP=42).
    #[test]
    fn test_cfairy_in_cospre_range() {
        assert!(CFAIRY_SLOT >= 42);
    }

    // ── Sprint 938: Additional coverage ──────────────────────────────

    /// Mining and fishing robin items are in 5xx range.
    #[test]
    fn test_robin_item_ranges() {
        assert!(MINING_ROBIN_ITEM >= 500_000_000 && MINING_ROBIN_ITEM < 600_000_000);
        assert!(FISHING_ROBIN_ITEM >= 500_000_000 && FISHING_ROBIN_ITEM < 600_000_000);
    }

    /// Offline items are in 7xx range.
    #[test]
    fn test_offline_item_ranges() {
        assert!(OFFLINE_MINNING >= 700_000_000 && OFFLINE_MINNING < 800_000_000);
        assert!(OFFLINE_FISHING >= 700_000_000 && OFFLINE_FISHING < 800_000_000);
    }
}
