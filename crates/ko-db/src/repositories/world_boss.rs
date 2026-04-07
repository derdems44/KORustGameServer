//! World Boss repository — boss configuration and ranking persistence.
//!
//! v2525 WIZ_WORLD_BOSS (0xD5/0xD6) — boss tracking and rankings.

use crate::models::world_boss::{WorldBossConfigRow, WorldBossRankingRow};
use crate::DbPool;

/// Repository for world_boss tables.
pub struct WorldBossRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> WorldBossRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    // ── Config ───────────────────────────────────────────────────────────

    /// Load all boss slot configurations.
    pub async fn load_configs(&self) -> Result<Vec<WorldBossConfigRow>, sqlx::Error> {
        sqlx::query_as::<_, WorldBossConfigRow>(
            "SELECT slot_id, boss_name, npc_proto_id, boss_type, boss_info_id, \
             spawn_zone, spawn_x, spawn_z, enabled \
             FROM world_boss_config ORDER BY slot_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load a single boss slot config.
    pub async fn load_config(
        &self,
        slot_id: i16,
    ) -> Result<Option<WorldBossConfigRow>, sqlx::Error> {
        sqlx::query_as::<_, WorldBossConfigRow>(
            "SELECT slot_id, boss_name, npc_proto_id, boss_type, boss_info_id, \
             spawn_zone, spawn_x, spawn_z, enabled \
             FROM world_boss_config WHERE slot_id = $1",
        )
        .bind(slot_id)
        .fetch_optional(self.pool)
        .await
    }

    // ── Rankings ──────────────────────────────────────────────────────────

    /// Record or update a player's damage for a boss event.
    pub async fn upsert_damage(
        &self,
        slot_id: i16,
        character_id: &str,
        damage: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO world_boss_ranking (slot_id, character_id, damage_dealt) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (slot_id, character_id, event_time) DO UPDATE SET \
             damage_dealt = world_boss_ranking.damage_dealt + $3",
        )
        .bind(slot_id)
        .bind(character_id)
        .bind(damage)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Get top N rankings for a boss slot (by total damage).
    pub async fn top_rankings(
        &self,
        slot_id: i16,
        limit: i64,
    ) -> Result<Vec<WorldBossRankingRow>, sqlx::Error> {
        sqlx::query_as::<_, WorldBossRankingRow>(
            "SELECT id, slot_id, character_id, damage_dealt, kill_count, last_hit \
             FROM world_boss_ranking WHERE slot_id = $1 \
             ORDER BY damage_dealt DESC LIMIT $2",
        )
        .bind(slot_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await
    }

    /// Mark the last-hit player for a boss kill.
    pub async fn mark_last_hit(&self, slot_id: i16, character_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE world_boss_ranking SET last_hit = TRUE, kill_count = kill_count + 1 \
             WHERE slot_id = $1 AND character_id = $2",
        )
        .bind(slot_id)
        .bind(character_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
