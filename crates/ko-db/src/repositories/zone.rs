//! Zone repository — loads zone configuration and events from PostgreSQL.
//!
//! C++ Reference:
//! - `GameServer/Map.cpp:Initialize()` — loads _ZONE_INFO from DB
//! - `shared/database/EventSet.h` — CEventSet (loads EVENT table)

use crate::models::{GameEventRow, ObjectEventRow, ZoneInfoRow};
use crate::DbPool;

/// Repository for `zone_info` and `game_event` table access.
pub struct ZoneRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ZoneRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all zone configuration entries.
    ///
    /// C++ Reference: `CGameServerDlg::LoadZoneInfoFromDB()`
    pub async fn load_all_zones(&self) -> Result<Vec<ZoneInfoRow>, sqlx::Error> {
        sqlx::query_as::<_, ZoneInfoRow>(
            "SELECT zone_no, smd_name, zone_name, zone_type, min_level, max_level, \
             init_x, init_z, init_y, \
             trade_other_nation, talk_other_nation, attack_other_nation, attack_same_nation, \
             friendly_npc, war_zone, clan_updates, \
             teleport, gate, escape, calling_friend, teleport_friend, blink, \
             pet_spawn, exp_lost, give_loyalty, guard_summon, \
             military_zone, mining_zone, blink_zone, auto_loot, gold_lose, status \
             FROM zone_info ORDER BY zone_no",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all events for a specific zone.
    ///
    /// C++ Reference: `CEventSet::Read()` — loads from EVENT table per zone
    pub async fn load_events(&self, zone_no: i16) -> Result<Vec<GameEventRow>, sqlx::Error> {
        sqlx::query_as::<_, GameEventRow>(
            "SELECT zone_no, event_num, event_type, \
             cond1, cond2, cond3, cond4, cond5, \
             exec1, exec2, exec3 \
             FROM game_event WHERE zone_no = $1 ORDER BY event_num",
        )
        .bind(zone_no)
        .fetch_all(self.pool)
        .await
    }

    /// Load all events for all zones at once (bulk load for startup).
    pub async fn load_all_events(&self) -> Result<Vec<GameEventRow>, sqlx::Error> {
        sqlx::query_as::<_, GameEventRow>(
            "SELECT zone_no, event_num, event_type, \
             cond1, cond2, cond3, cond4, cond5, \
             exec1, exec2, exec3 \
             FROM game_event ORDER BY zone_no, event_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all object events (interactive objects) for all zones.
    ///
    /// C++ Reference: `CGameServerDlg::LoadObjectPosTable()` — loads `K_OBJECTPOS` table
    pub async fn load_all_object_events(&self) -> Result<Vec<ObjectEventRow>, sqlx::Error> {
        sqlx::query_as::<_, ObjectEventRow>(
            "SELECT id, zone_id, belong, s_index, obj_type, control_npc, status, \
             pos_x, pos_y, pos_z, by_life \
             FROM object_event_pos ORDER BY zone_id, s_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
