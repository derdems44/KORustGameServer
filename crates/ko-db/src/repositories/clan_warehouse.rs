//! Clan warehouse repository — clan_warehouse_items + knights.n_money access.

use sqlx::PgPool;

use crate::models::ClanWarehouseItemRow;

/// Parameters for saving a clan warehouse item slot.
pub struct SaveClanWarehouseItemParams {
    pub clan_id: i16,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}

/// Repository for clan warehouse DB operations.
pub struct ClanWarehouseRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ClanWarehouseRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all clan warehouse items for a clan.
    pub async fn load_items(&self, clan_id: i16) -> Result<Vec<ClanWarehouseItemRow>, sqlx::Error> {
        sqlx::query_as::<_, ClanWarehouseItemRow>(
            "SELECT * FROM clan_warehouse_items WHERE clan_id = $1 ORDER BY slot_index",
        )
        .bind(clan_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load clan warehouse gold (n_money from knights table).
    pub async fn load_gold(&self, clan_id: i16) -> Result<i32, sqlx::Error> {
        let result: Option<(i32,)> =
            sqlx::query_as("SELECT n_money FROM knights WHERE id_num = $1")
                .bind(clan_id)
                .fetch_optional(self.pool)
                .await?;
        Ok(result.map(|r| r.0).unwrap_or(0))
    }

    /// Save a single clan warehouse item slot (upsert).
    pub async fn save_item(&self, params: &SaveClanWarehouseItemParams) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO clan_warehouse_items (clan_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (clan_id, slot_index) DO UPDATE SET
                item_id = $3, durability = $4, count = $5, flag = $6, original_flag = $7, serial_num = $8, expire_time = $9",
        )
        .bind(params.clan_id)
        .bind(params.slot_index)
        .bind(params.item_id)
        .bind(params.durability)
        .bind(params.count)
        .bind(params.flag)
        .bind(params.original_flag)
        .bind(params.serial_num)
        .bind(params.expire_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save clan warehouse gold (update knights.n_money).
    pub async fn save_gold(&self, clan_id: i16, gold: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE knights SET n_money = $2 WHERE id_num = $1")
            .bind(clan_id)
            .bind(gold)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
