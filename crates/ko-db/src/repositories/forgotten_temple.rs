//! Forgotten Temple (Monster Challenge) stage and summon repository.
//!
//! C++ Reference: `CGameServerDlg::LoadForgettenTempleStages()`,
//!                `CGameServerDlg::LoadForgettenTempleSummon()`
//!
//! FT options and event rewards are loaded via `EventScheduleRepository`.

use crate::models::forgotten_temple::{FtStageRow, FtSummonRow};
use crate::DbPool;

/// Repository for Forgotten Temple stage/summon data.
pub struct ForgottenTempleRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ForgottenTempleRepository<'a> {
    /// Create a new forgotten temple repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all FT stage definitions from the database.
    ///
    /// C++ Reference: `CGameServerDlg::LoadForgettenTempleStages()`
    pub async fn load_all_stages(&self) -> Result<Vec<FtStageRow>, sqlx::Error> {
        sqlx::query_as::<_, FtStageRow>(
            "SELECT n_index, event_type, stage, time_offset \
             FROM ft_stages ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all FT monster summon definitions from the database.
    ///
    /// C++ Reference: `CGameServerDlg::LoadForgettenTempleSummon()`
    pub async fn load_all_summons(&self) -> Result<Vec<FtSummonRow>, sqlx::Error> {
        sqlx::query_as::<_, FtSummonRow>(
            "SELECT b_index, event_type, stage, sid_id, sid_count, \
             pos_x, pos_z, spawn_range, summon_name \
             FROM ft_summon_list ORDER BY b_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
