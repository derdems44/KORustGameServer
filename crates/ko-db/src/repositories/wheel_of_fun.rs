//! Wheel of Fun event repository.
//!
//! Source: MSSQL `WHEEL_OF_FUN_ITEM` + `WHEEL_SETTINGS`

use crate::models::wheel_of_fun::{WheelOfFunItem, WheelOfFunSettings};
use crate::DbPool;

/// Repository for wheel of fun event table access.
pub struct WheelOfFunRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> WheelOfFunRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all wheel prize items.
    pub async fn load_items(&self) -> Result<Vec<WheelOfFunItem>, sqlx::Error> {
        sqlx::query_as::<_, WheelOfFunItem>(
            "SELECT id, name, num, count, percent, days \
             FROM wheel_of_fun_item ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all wheel drop settings.
    pub async fn load_settings(&self) -> Result<Vec<WheelOfFunSettings>, sqlx::Error> {
        sqlx::query_as::<_, WheelOfFunSettings>(
            "SELECT idx, item_name, item_id, item_count, rental_time, flag, drop_rate \
             FROM wheel_of_fun_settings ORDER BY idx",
        )
        .fetch_all(self.pool)
        .await
    }
}
