//! Check account repository — account ban/check records.
//!
//! Source: MSSQL `CHECK_ACCOUNT` (150 rows)

use crate::models::check_account::CheckAccount;
use crate::DbPool;

/// Repository for `check_account` table access.
pub struct CheckAccountRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> CheckAccountRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Check if an account is banned (login_time_status != 1).
    pub async fn is_banned(&self, account_id: &str) -> Result<bool, sqlx::Error> {
        let row: Option<CheckAccount> = sqlx::query_as::<_, CheckAccount>(
            "SELECT account_id, gm, login_time_status, reason, ban_count, open_count, updated_at \
             FROM check_account WHERE account_id = $1",
        )
        .bind(account_id)
        .fetch_optional(self.pool)
        .await?;

        Ok(row.is_some_and(|r| r.login_time_status != 1))
    }

    /// Get ban record for an account.
    pub async fn get(&self, account_id: &str) -> Result<Option<CheckAccount>, sqlx::Error> {
        sqlx::query_as::<_, CheckAccount>(
            "SELECT account_id, gm, login_time_status, reason, ban_count, open_count, updated_at \
             FROM check_account WHERE account_id = $1",
        )
        .bind(account_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Ban an account.
    pub async fn ban(&self, account_id: &str, gm: &str, reason: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO check_account (account_id, gm, login_time_status, reason, ban_count, updated_at) \
             VALUES ($1, $2, 0, $3, 1, NOW()) \
             ON CONFLICT (account_id) DO UPDATE SET \
             gm = $2, login_time_status = 0, reason = $3, \
             ban_count = check_account.ban_count + 1, updated_at = NOW()",
        )
        .bind(account_id)
        .bind(gm)
        .bind(reason)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Unban an account.
    pub async fn unban(&self, account_id: &str, gm: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE check_account SET login_time_status = 1, gm = $2, \
             open_count = open_count + 1, updated_at = NOW() \
             WHERE account_id = $1",
        )
        .bind(account_id)
        .bind(gm)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
