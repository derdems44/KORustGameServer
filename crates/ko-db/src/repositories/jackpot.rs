//! JackPot repository — `jackpot_settings` table access.
//!
//! C++ Reference: `CGameServerDlg::LoadJackPotSettingTable` in `LoadServerData.cpp:1052`.

use sqlx::PgPool;

use crate::models::jackpot::JackPotSettingRow;

/// Repository for jackpot system database operations.
pub struct JackPotRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> JackPotRepository<'a> {
    /// Create a new jackpot repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all jackpot settings (2 rows: type 0=EXP, type 1=Noah).
    ///
    /// C++ Reference: `CJackPotSettingSet::Fetch` in `thykedb_class.h:76`.
    pub async fn load_all(&self) -> Result<Vec<JackPotSettingRow>, sqlx::Error> {
        sqlx::query_as::<_, JackPotSettingRow>(
            "SELECT i_type, rate, x_1000, x_500, x_100, x_50, x_10, x_2 \
             FROM jackpot_settings \
             WHERE i_type <= 1 \
             ORDER BY i_type",
        )
        .fetch_all(self.pool)
        .await
    }
}
