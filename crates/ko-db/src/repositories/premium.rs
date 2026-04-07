//! Premium repository — loads premium type definitions, XP bonus table,
//! and per-account premium subscriptions from PostgreSQL.
//!
//! C++ Reference:
//! - `LoadServerData.cpp` — `LoadPremiumItemTable()`, `LoadPremiumItemExpTable()`
//! - `DBAgent.cpp` — `AccountPremiumData` load/save

use crate::models::premium::{
    AccountPremiumRow, PremiumGiftItemRow, PremiumItemExpRow, PremiumItemRow,
};
use crate::DbPool;

/// Repository for premium system table access.
pub struct PremiumRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> PremiumRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all premium item type definitions (bulk load at startup).
    ///
    /// Returns one row per premium type (13 rows).
    /// C++ Reference: `CGameServerDlg::LoadPremiumItemTable()`
    pub async fn load_all_premium_types(&self) -> Result<Vec<PremiumItemRow>, sqlx::Error> {
        sqlx::query_as::<_, PremiumItemRow>(
            "SELECT premium_type, name, exp_restore_pct, noah_pct, drop_pct, \
             bonus_loyalty, repair_disc_pct, item_sell_pct \
             FROM premium_item_types ORDER BY premium_type",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all premium gift items (bulk load at startup).
    ///
    /// Returns bonus items grouped by premium type, sent via letter on activation.
    /// C++ Reference: `CGameServerDlg::LoadPremiumGiftItemTable()`
    pub async fn load_all_premium_gift_items(
        &self,
    ) -> Result<Vec<PremiumGiftItemRow>, sqlx::Error> {
        sqlx::query_as::<_, PremiumGiftItemRow>(
            "SELECT id, premium_type, bonus_item_num, count, sender, subject, message, item_name \
             FROM premium_gift_item ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all premium XP bonus entries (bulk load at startup).
    ///
    /// Returns level-range-based XP bonus percentages per premium type.
    /// C++ Reference: `CGameServerDlg::LoadPremiumItemExpTable()`
    pub async fn load_all_premium_exp(&self) -> Result<Vec<PremiumItemExpRow>, sqlx::Error> {
        sqlx::query_as::<_, PremiumItemExpRow>(
            "SELECT n_index, premium_type, min_level, max_level, s_percent \
             FROM premium_item_exp ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load premium subscriptions for a specific account.
    ///
    /// Returns up to 6 slots with their premium type and expiry.
    /// C++ Reference: `CDBAgent::AccountPremiumData()` load path
    pub async fn load_account_premium(
        &self,
        account_id: &str,
    ) -> Result<Vec<AccountPremiumRow>, sqlx::Error> {
        sqlx::query_as::<_, AccountPremiumRow>(
            "SELECT account_id, slot, premium_type, expiry_time \
             FROM account_premium \
             WHERE account_id = $1 \
             ORDER BY slot",
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
    }

    /// Save (upsert) premium subscriptions for an account.
    ///
    /// Writes all active slots. Called on premium change and periodic save.
    /// C++ Reference: `CDBAgent::AccountPremiumData()` save path
    pub async fn save_account_premium(
        &self,
        account_id: &str,
        slots: &[(i16, i16, i32)], // (slot, premium_type, expiry_time)
    ) -> Result<(), sqlx::Error> {
        if slots.is_empty() {
            return Ok(());
        }

        let mut builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            "INSERT INTO account_premium (account_id, slot, premium_type, expiry_time) ",
        );
        builder.push_values(slots, |mut b, &(slot, premium_type, expiry_time)| {
            b.push_bind(account_id)
                .push_bind(slot)
                .push_bind(premium_type)
                .push_bind(expiry_time);
        });
        builder.push(
            " ON CONFLICT (account_id, slot) DO UPDATE \
             SET premium_type = EXCLUDED.premium_type, expiry_time = EXCLUDED.expiry_time",
        );
        builder.build().execute(self.pool).await?;
        Ok(())
    }
}
