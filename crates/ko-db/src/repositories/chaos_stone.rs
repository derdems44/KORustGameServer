//! Chaos Stone event data repository.
//!
//! C++ Reference: `CGameServerDlg::ChaosStoneLoad()`,
//!                `CDBAgent::LoadChaosStoneFamilyStage()` in `ChaosStone.cpp`

use crate::models::chaos_stone::{
    ChaosStoneSpawnRow, ChaosStoneSummonListRow, ChaosStoneSummonStageRow, EventChaosRewardRow,
};
use crate::DbPool;

/// Repository for Chaos Stone event data.
pub struct ChaosStoneRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ChaosStoneRepository<'a> {
    /// Create a new chaos stone repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all chaos stone spawn points.
    ///
    /// C++ Reference: `CChaosStoneCoordinate::Fetch()` in `ChaosStoneRespawn.h`
    pub async fn load_all_spawns(&self) -> Result<Vec<ChaosStoneSpawnRow>, sqlx::Error> {
        sqlx::query_as::<_, ChaosStoneSpawnRow>(
            "SELECT s_index, zone_id, is_open, rank, chaos_id, count, \
             spawn_x, spawn_z, spawn_time, direction, radius_range \
             FROM chaos_stone_spawn ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all chaos stone summon list entries.
    ///
    /// C++ Reference: `CChaosStoneSummonListSet::Fetch()` in `ChaosStoneSummonList.h`
    pub async fn load_all_summon_list(&self) -> Result<Vec<ChaosStoneSummonListRow>, sqlx::Error> {
        sqlx::query_as::<_, ChaosStoneSummonListRow>(
            "SELECT n_index, zone_id, sid, monster_spawn_family \
             FROM chaos_stone_summon_list ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all chaos stone summon stage definitions.
    ///
    /// C++ Reference: `CDBAgent::LoadChaosStoneFamilyStage()` in `ChaosStone.cpp:317-347`
    pub async fn load_all_stages(&self) -> Result<Vec<ChaosStoneSummonStageRow>, sqlx::Error> {
        sqlx::query_as::<_, ChaosStoneSummonStageRow>(
            "SELECT n_index, zone_id, index_family \
             FROM chaos_stone_summon_stage ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all chaos event reward definitions.
    ///
    /// C++ Reference: `EVENT_CHAOS_REWARDS` table.
    pub async fn load_all_rewards(&self) -> Result<Vec<EventChaosRewardRow>, sqlx::Error> {
        sqlx::query_as::<_, EventChaosRewardRow>(
            "SELECT rank_id, item_id1, item_count1, item_expiration1, \
             item_id2, item_count2, item_expiration2, \
             item_id3, item_count3, item_expiration3, \
             item_id4, item_count4, item_expiration4, \
             item_id5, item_count5, item_expiration5, \
             experience, loyalty, cash, noah \
             FROM event_chaos_rewards ORDER BY rank_id",
        )
        .fetch_all(self.pool)
        .await
    }
}
