//! Knights castellan repository — Castle Siege winner cape bonus.
//!
//! Source: MSSQL `KNIGHTS_CASTELLAN` (0 rows)

use crate::models::knights_castellan::KnightsCastellan;
use crate::DbPool;

/// Repository for `knights_castellan` table access.
pub struct KnightsCastellanRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> KnightsCastellanRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load castellan data by ID.
    pub async fn get(&self, id_num: i16) -> Result<Option<KnightsCastellan>, sqlx::Error> {
        sqlx::query_as::<_, KnightsCastellan>(
            "SELECT id_num, cape, cape_r, cape_g, cape_b, is_active, remaining_time \
             FROM knights_castellan WHERE id_num = $1",
        )
        .bind(id_num)
        .fetch_optional(self.pool)
        .await
    }

    /// Load all castellan entries.
    pub async fn load_all(&self) -> Result<Vec<KnightsCastellan>, sqlx::Error> {
        sqlx::query_as::<_, KnightsCastellan>(
            "SELECT id_num, cape, cape_r, cape_g, cape_b, is_active, remaining_time \
             FROM knights_castellan ORDER BY id_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Upsert castellan cape data.
    pub async fn upsert(&self, data: &KnightsCastellan) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO knights_castellan (id_num, cape, cape_r, cape_g, cape_b, is_active, remaining_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             ON CONFLICT (id_num) DO UPDATE SET \
             cape = $2, cape_r = $3, cape_g = $4, cape_b = $5, \
             is_active = $6, remaining_time = $7",
        )
        .bind(data.id_num)
        .bind(data.cape)
        .bind(data.cape_r)
        .bind(data.cape_g)
        .bind(data.cape_b)
        .bind(data.is_active)
        .bind(data.remaining_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
