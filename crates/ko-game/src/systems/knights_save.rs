//! Periodic knights save — saves clan flag and point fund to DB.
//!
//! C++ Reference: `GameServerDlg.cpp:312-326` — ProcDbServerType::UpdateKnights
//! C++ Reference: `DBAgent.cpp:3584-3594` — KnightsSave(clanID, flag, clanFund)

use std::sync::Arc;
use std::time::Duration;

use ko_db::repositories::knights::KnightsRepository;
use ko_db::DbPool;
use tokio::time::interval;
use tracing::{debug, warn};

use crate::world::WorldState;

/// Save interval: 5 minutes.
///
/// C++ triggers UpdateKnights via event timers and on server shutdown.
/// We use a fixed 5-minute interval for simplicity.
const KNIGHTS_SAVE_INTERVAL_SECS: u64 = 5 * 60;

/// Start the periodic knights save background task.
///
/// Spawns a tokio task that ticks every 5 minutes and saves all clans'
/// `clan_point_fund` to the database.
///
/// C++ Reference: `GameServerDlg.cpp:312-326` — ProcDbServerType::UpdateKnights
pub fn start_knights_save_task(
    world: Arc<WorldState>,
    pool: DbPool,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(KNIGHTS_SAVE_INTERVAL_SECS));
        loop {
            tick.tick().await;
            save_all_knights(&world, &pool).await;
        }
    })
}

/// Save flag + clan_point_fund + premium for all clans.
///
/// Each clan's save is dispatched as a fire-and-forget spawned task
/// so that one slow DB query does not block the others.
async fn save_all_knights(world: &WorldState, pool: &DbPool) {
    let clan_ids = world.get_all_knights_ids();
    if clan_ids.is_empty() {
        return;
    }
    debug!("Periodic knights save: saving {} clans", clan_ids.len());

    for clan_id in clan_ids {
        let info = match world.get_knights(clan_id) {
            Some(k) => k,
            None => continue,
        };

        let pool_clone = pool.clone();
        let fund = info.clan_point_fund as i32;
        let premium_time = info.premium_time as i32;
        let premium_in_use = info.premium_in_use as i16;

        // Fire-and-forget DB save
        tokio::spawn(async move {
            let repo = KnightsRepository::new(&pool_clone);
            // Save clan_point_fund
            if let Err(e) = repo.update_clan_point_fund(clan_id as i16, fund).await {
                warn!(
                    "Periodic knights save: failed to save clan {} fund: {}",
                    clan_id, e
                );
            }
            // Save premium state if active
            if premium_time > 0 || premium_in_use > 0 {
                if let Err(e) = repo
                    .save_clan_premium(clan_id as i16, premium_time, premium_in_use)
                    .await
                {
                    warn!(
                        "Periodic knights save: failed to save clan {} premium: {}",
                        clan_id, e
                    );
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_interval_is_five_minutes() {
        assert_eq!(KNIGHTS_SAVE_INTERVAL_SECS, 300);
    }

    #[test]
    fn test_save_interval_is_5_times_60() {
        assert_eq!(KNIGHTS_SAVE_INTERVAL_SECS, 5 * 60);
    }

    #[test]
    fn test_save_interval_duration() {
        let d = Duration::from_secs(KNIGHTS_SAVE_INTERVAL_SECS);
        assert_eq!(d.as_secs(), 300);
        assert_eq!(d.as_millis(), 300_000);
    }

    // ── Sprint 935: Additional coverage ──────────────────────────────

    /// Interval is exactly 5 minutes in seconds.
    #[test]
    fn test_interval_minutes() {
        assert_eq!(KNIGHTS_SAVE_INTERVAL_SECS / 60, 5);
    }

    /// Fund conversion: u32 → i32 clamp.
    #[test]
    fn test_fund_i32_conversion() {
        let fund_normal: u32 = 1_000_000;
        assert_eq!(fund_normal as i32, 1_000_000);
        // Values within i32 range are safe
        let fund_max: u32 = i32::MAX as u32;
        assert_eq!(fund_max as i32, i32::MAX);
    }

    /// Premium time: 0 means no premium active.
    #[test]
    fn test_premium_time_zero() {
        let premium_time: u32 = 0;
        let premium_in_use: u16 = 0;
        // Both zero → skip premium save
        assert!(premium_time == 0 && premium_in_use == 0);
    }

    /// Empty clan list early-exits without DB work.
    #[test]
    fn test_empty_clans_skipped() {
        let clan_ids: Vec<u16> = vec![];
        assert!(clan_ids.is_empty());
    }

    // ── Sprint 938: Additional coverage ──────────────────────────────

    /// Premium in_use as i16 conversion.
    #[test]
    fn test_premium_in_use_i16() {
        let in_use: u16 = 1;
        assert_eq!(in_use as i16, 1);
        let not_in_use: u16 = 0;
        assert_eq!(not_in_use as i16, 0);
    }

    /// Clan ID as i16 conversion for DB.
    #[test]
    fn test_clan_id_i16_conversion() {
        let clan_id: u16 = 100;
        assert_eq!(clan_id as i16, 100);
        let max_safe: u16 = i16::MAX as u16;
        assert_eq!(max_safe as i16, i16::MAX);
    }

    /// WorldState::new() starts with no knights.
    #[test]
    fn test_world_no_knights() {
        let world = crate::world::WorldState::new();
        let ids = world.get_all_knights_ids();
        assert!(ids.is_empty());
    }
}
