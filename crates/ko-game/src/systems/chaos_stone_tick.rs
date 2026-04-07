//! Chaos Stone respawn timer tick system.
//!
//! C++ Reference: `CGameServerDlg::ChaosStoneRespawnTimer()` in `ChaosStone.cpp:53-76`
//!
//! Runs once per second, decrementing respawn countdowns for killed chaos
//! stones. When a countdown reaches zero the stone is flagged for respawn
//! and the NPC AI tick will re-spawn it on the next pass.

use std::sync::Arc;
use std::time::Duration;

use crate::world::WorldState;

/// Chaos stone respawn timer tick interval (once per second).
///
/// C++ Reference: `ChaosStone.cpp` — `ChaosStoneRespawnTimer()` called from 1-second timer.
const CHAOS_STONE_TICK_SECS: u64 = 1;

/// Start the chaos stone respawn timer background task.
///
/// Returns a `JoinHandle` so the caller can abort on shutdown.
pub fn start_chaos_stone_tick_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(CHAOS_STONE_TICK_SECS));
        loop {
            interval.tick().await;
            process_chaos_stone_tick(&world);
        }
    })
}

/// Process one second of chaos stone respawn timers.
///
/// C++ Reference: `CGameServerDlg::ChaosStoneRespawnTimer()` in `ChaosStone.cpp:53-76`
fn process_chaos_stone_tick(world: &WorldState) {
    let results = {
        let infos = world.chaos_stone_infos();
        crate::handler::chaos_stone::respawn_timer_tick(&infos)
    };

    for result in results {
        use crate::handler::chaos_stone::ChaosStoneTimerResult;
        if let ChaosStoneTimerResult::Respawn(chaos_id, rank, zone_id) = result {
            tracing::debug!(
                chaos_id,
                rank,
                zone_id,
                "Chaos stone respawn timer expired — stone ready to respawn"
            );
            // The actual NPC re-spawn is performed by the NPC AI tick system
            // when it detects a Dead NPC whose respawn timer has elapsed.
            // Here we only log the event; in a full implementation this would
            // call the NPC spawn path with the data from chaos_stone_spawns.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::chaos_stone::{ChaosStoneInfo, ChaosStoneTimerResult};
    use std::collections::HashMap;
    use std::sync::atomic::Ordering;

    /// Helper: create a ChaosStoneInfo with specific flags.
    fn make_info(on: bool, timer_active: bool, killed: bool, spawn_time: u32) -> ChaosStoneInfo {
        let info = ChaosStoneInfo::new();
        info.chaos_stone_on.store(on, Ordering::Relaxed);
        info.is_on_res_timer.store(timer_active, Ordering::Relaxed);
        info.is_chaos_stone_killed.store(killed, Ordering::Relaxed);
        info.spawn_time.store(spawn_time, Ordering::Relaxed);
        info.chaos_id.store(100, Ordering::Relaxed);
        info.rank.store(2, Ordering::Relaxed);
        info.zone_id.store(21, Ordering::Relaxed);
        info
    }

    #[test]
    fn test_tick_interval_is_one_second() {
        assert_eq!(CHAOS_STONE_TICK_SECS, 1);
    }

    #[test]
    fn test_respawn_timer_tick_empty_map() {
        let infos: HashMap<u8, ChaosStoneInfo> = HashMap::new();
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert!(results.is_empty());
    }

    #[test]
    fn test_respawn_timer_tick_stone_off() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(false, true, true, 5));
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        // Stone is off → skipped, no Respawn
        assert!(results.is_empty());
    }

    #[test]
    fn test_respawn_timer_tick_not_killed() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(true, true, false, 5));
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert!(results.is_empty());
    }

    #[test]
    fn test_respawn_timer_tick_no_res_timer() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(true, false, true, 5));
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert!(results.is_empty());
    }

    #[test]
    fn test_respawn_timer_tick_countdown() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(true, true, true, 3));
        // Tick: spawn_time 3 → 2 (still counting, no respawn)
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert!(results.is_empty());
        assert_eq!(infos[&0].spawn_time.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_respawn_timer_tick_expires_at_zero() {
        let mut infos = HashMap::new();
        let info = make_info(true, true, true, 0);
        info.rank.store(3, Ordering::Relaxed);
        info.zone_id.store(51, Ordering::Relaxed);
        info.chaos_id.store(200, Ordering::Relaxed);
        infos.insert(0, info);
        // spawn_time == 0 → timer expired → Respawn
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert_eq!(results.len(), 1);
        match &results[0] {
            ChaosStoneTimerResult::Respawn(id, rank, zone) => {
                assert_eq!(*id, 200);
                assert_eq!(*rank, 3);
                assert_eq!(*zone, 51);
            }
            _ => panic!("expected Respawn result"),
        }
        // After respawn: flags reset
        assert!(!infos[&0].is_chaos_stone_killed.load(Ordering::Relaxed));
        assert!(!infos[&0].is_on_res_timer.load(Ordering::Relaxed));
    }

    #[test]
    fn test_respawn_timer_resets_boss_count() {
        let mut infos = HashMap::new();
        let info = make_info(true, true, true, 0);
        info.boss_killed_count.store(5, Ordering::Relaxed);
        infos.insert(0, info);
        crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert_eq!(infos[&0].boss_killed_count.load(Ordering::Relaxed), 0);
    }

    // ── Sprint 934: Additional coverage ──────────────────────────────

    /// Countdown decrements from 5 to 4 on one tick.
    #[test]
    fn test_respawn_timer_tick_decrement_from_5() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(true, true, true, 5));
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        assert!(results.is_empty());
        assert_eq!(infos[&0].spawn_time.load(Ordering::Relaxed), 4);
    }

    /// Multiple stones can tick independently in the same pass.
    #[test]
    fn test_respawn_timer_multiple_stones() {
        let mut infos = HashMap::new();
        infos.insert(0, make_info(true, true, true, 2));
        infos.insert(1, make_info(true, true, true, 0));
        let results = crate::handler::chaos_stone::respawn_timer_tick(&infos);
        // Stone 0: 2→1 (still counting)
        assert_eq!(infos[&0].spawn_time.load(Ordering::Relaxed), 1);
        // Stone 1: expired → Respawn
        assert_eq!(results.len(), 1);
    }
}
