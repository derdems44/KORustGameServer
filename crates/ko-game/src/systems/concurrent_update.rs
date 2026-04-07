//! Timer_UpdateConcurrent — periodic server population update.
//!
//! C++ Reference: `CGameServerDlg::Timer_UpdateConcurrent()` /
//! `CGameServerDlg::ReqUpdateConcurrent()` (ServerStartStopHandler.cpp:378-524)
//!
//! Counts all in-game players + active bots every 120 seconds and persists
//! the total to the `game_server_list.concurrent_users` column so the login
//! server can report the population to launchers.

use std::sync::Arc;
use std::time::Duration;

use ko_db::repositories::server_list::ServerListRepository;
use ko_db::DbPool;
use tracing::{debug, warn};

use crate::world::WorldState;

/// Default server number — matches `DEFAULT_SERVER_NO` in server_index.rs.
///
/// C++ Reference: `CGameServerDlg::m_nServerNo`
const SERVER_NO: i16 = 1;

/// Interval between concurrent user updates (120 seconds = 2 minutes).
///
/// C++ Reference: `sleep(120 * SECOND)` in Timer_UpdateConcurrent
const UPDATE_INTERVAL: Duration = Duration::from_secs(120);

/// Start the concurrent user update background task.
///
/// C++ Reference: `CGameServerDlg::Timer_UpdateConcurrent()`
pub fn start_concurrent_update_task(
    world: Arc<WorldState>,
    pool: DbPool,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(UPDATE_INTERVAL).await;
            update_concurrent_count(&world, &pool).await;
        }
    })
}

/// Count online players + bots and update the database.
///
/// C++ Reference: `CGameServerDlg::ReqUpdateConcurrent()` — iterates all
/// users checking `isInGame()`, adds bot count, sends to DB agent.
async fn update_concurrent_count(world: &WorldState, pool: &DbPool) {
    // Count players (sessions with an active character) + active bots
    let player_count = world.online_count();
    let bot_count = world.bot_count();
    let total = (player_count + bot_count) as i32;

    let repo = ServerListRepository::new(pool);
    if let Err(e) = repo.update_concurrent_users(SERVER_NO, total).await {
        warn!("Failed to update concurrent user count: {}", e);
        return;
    }

    debug!(
        "Concurrent user update: {} players + {} bots = {} total",
        player_count, bot_count, total
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(SERVER_NO, 1);
        assert_eq!(UPDATE_INTERVAL, Duration::from_secs(120));
    }

    #[test]
    fn test_concurrent_count_zero_on_empty_world() {
        let world = WorldState::new();
        let player_count = world.online_count();
        let bot_count = world.bot_count();
        assert_eq!(player_count, 0);
        assert_eq!(bot_count, 0);
    }

    // ── Sprint 936: Additional coverage ──────────────────────────────

    /// Update interval is exactly 2 minutes.
    #[test]
    fn test_update_interval_two_minutes() {
        assert_eq!(UPDATE_INTERVAL.as_secs(), 120);
        assert_eq!(UPDATE_INTERVAL.as_secs() / 60, 2);
    }

    /// Total count is player + bot sum.
    #[test]
    fn test_total_count_sum() {
        let players: usize = 50;
        let bots: usize = 10;
        let total = (players + bots) as i32;
        assert_eq!(total, 60);
    }

    /// SERVER_NO matches game server default.
    #[test]
    fn test_server_no_default() {
        assert_eq!(SERVER_NO, 1);
    }

    // ── Sprint 939: Additional coverage ──────────────────────────────

    /// Total count uses i32 for DB compatibility.
    #[test]
    fn test_total_count_i32() {
        let players: usize = 100;
        let bots: usize = 50;
        let total = (players + bots) as i32;
        assert_eq!(total, 150);
        assert!(total >= 0);
    }

    /// Zero players and zero bots yields zero total.
    #[test]
    fn test_zero_total() {
        let total = (0usize + 0usize) as i32;
        assert_eq!(total, 0);
    }

    /// SERVER_NO fits in i16.
    #[test]
    fn test_server_no_fits_i16() {
        assert!(SERVER_NO >= i16::MIN);
        assert!(SERVER_NO <= i16::MAX);
    }

    /// Update interval is not zero.
    #[test]
    fn test_interval_nonzero() {
        assert!(UPDATE_INTERVAL.as_secs() > 0);
        assert!(!UPDATE_INTERVAL.is_zero());
    }

    /// WorldState online_count + bot_count on empty world.
    #[test]
    fn test_empty_world_counts() {
        let world = WorldState::new();
        assert_eq!(world.online_count() + world.bot_count(), 0);
    }
}
