//! Guild Bank repository — clan shared storage persistence.
//!
//! v2525 WIZ_GUILD_BANK (0xD0) — clan storage with tabs, items, logs.

use crate::models::guild_bank::{GuildBankItemRow, GuildBankLogRow, GuildBankRow};
use crate::DbPool;

/// Repository for guild_bank tables.
pub struct GuildBankRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> GuildBankRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    // ── Bank Settings ────────────────────────────────────────────────────

    /// Load guild bank settings for a clan, creating defaults if missing.
    pub async fn load_or_create(&self, knights_id: i32) -> Result<GuildBankRow, sqlx::Error> {
        let row = sqlx::query_as::<_, GuildBankRow>(
            "SELECT knights_id, gold, max_tabs, permissions \
             FROM guild_bank WHERE knights_id = $1",
        )
        .bind(knights_id)
        .fetch_optional(self.pool)
        .await?;

        if let Some(r) = row {
            return Ok(r);
        }

        sqlx::query("INSERT INTO guild_bank (knights_id) VALUES ($1) ON CONFLICT DO NOTHING")
            .bind(knights_id)
            .execute(self.pool)
            .await?;

        Ok(GuildBankRow {
            knights_id,
            gold: 0,
            max_tabs: 1,
            permissions: 0,
        })
    }

    /// Update guild bank gold.
    pub async fn update_gold(&self, knights_id: i32, gold: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE guild_bank SET gold = $1 WHERE knights_id = $2")
            .bind(gold)
            .bind(knights_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    // ── Items ────────────────────────────────────────────────────────────

    /// Load all items in a guild bank.
    pub async fn load_items(&self, knights_id: i32) -> Result<Vec<GuildBankItemRow>, sqlx::Error> {
        sqlx::query_as::<_, GuildBankItemRow>(
            "SELECT id, knights_id, tab_index, slot_id, item_id, item_count, \
             max_durability, cur_durability, flag, expiry_time \
             FROM guild_bank_item WHERE knights_id = $1 \
             ORDER BY tab_index, slot_id",
        )
        .bind(knights_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load items in a specific tab.
    pub async fn load_tab_items(
        &self,
        knights_id: i32,
        tab_index: i16,
    ) -> Result<Vec<GuildBankItemRow>, sqlx::Error> {
        sqlx::query_as::<_, GuildBankItemRow>(
            "SELECT id, knights_id, tab_index, slot_id, item_id, item_count, \
             max_durability, cur_durability, flag, expiry_time \
             FROM guild_bank_item WHERE knights_id = $1 AND tab_index = $2 \
             ORDER BY slot_id",
        )
        .bind(knights_id)
        .bind(tab_index)
        .fetch_all(self.pool)
        .await
    }

    /// Upsert an item into a guild bank slot.
    #[allow(clippy::too_many_arguments)]
    pub async fn upsert_item(
        &self,
        knights_id: i32,
        tab_index: i16,
        slot_id: i32,
        item_id: i32,
        item_count: i16,
        max_durability: i16,
        cur_durability: i16,
        flag: i16,
        expiry_time: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO guild_bank_item \
             (knights_id, tab_index, slot_id, item_id, item_count, \
              max_durability, cur_durability, flag, expiry_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             ON CONFLICT (knights_id, tab_index, slot_id) DO UPDATE SET \
             item_id = $4, item_count = $5, max_durability = $6, \
             cur_durability = $7, flag = $8, expiry_time = $9",
        )
        .bind(knights_id)
        .bind(tab_index)
        .bind(slot_id)
        .bind(item_id)
        .bind(item_count)
        .bind(max_durability)
        .bind(cur_durability)
        .bind(flag)
        .bind(expiry_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Remove an item from a guild bank slot.
    pub async fn remove_item(
        &self,
        knights_id: i32,
        tab_index: i16,
        slot_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM guild_bank_item \
             WHERE knights_id = $1 AND tab_index = $2 AND slot_id = $3",
        )
        .bind(knights_id)
        .bind(tab_index)
        .bind(slot_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    // ── Logs ─────────────────────────────────────────────────────────────

    /// Insert a transaction log entry.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_log(
        &self,
        knights_id: i32,
        character_id: &str,
        tab_index: i16,
        item_id: i32,
        quantity: i16,
        price: i32,
        action_type: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO guild_bank_log \
             (knights_id, character_id, tab_index, item_id, quantity, price, action_type) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(knights_id)
        .bind(character_id)
        .bind(tab_index)
        .bind(item_id)
        .bind(quantity)
        .bind(price)
        .bind(action_type)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Load recent log entries for a clan (paginated, newest first).
    pub async fn load_logs(
        &self,
        knights_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GuildBankLogRow>, sqlx::Error> {
        sqlx::query_as::<_, GuildBankLogRow>(
            "SELECT id, knights_id, character_id, tab_index, item_id, \
             quantity, price, action_type \
             FROM guild_bank_log WHERE knights_id = $1 \
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(knights_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await
    }

    /// Count total log entries for a clan.
    pub async fn count_logs(&self, knights_id: i32) -> Result<i64, sqlx::Error> {
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM guild_bank_log WHERE knights_id = $1")
                .bind(knights_id)
                .fetch_one(self.pool)
                .await?;
        Ok(row.0)
    }
}
