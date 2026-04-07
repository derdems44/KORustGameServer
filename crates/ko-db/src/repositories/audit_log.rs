//! Audit Log repository — game event logging to PostgreSQL.
//! Rust approach: Unified `game_audit_log` table with `event_type` discriminator and
//! TEXT `details` field for event-specific data. All inserts are fire-and-forget via
//! `tokio::spawn` (non-blocking).

use crate::DbPool;

/// Repository for `game_audit_log` table access.
pub struct AuditLogRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> AuditLogRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Insert a full audit log entry with position and details.
    ///
    /// This is the primary write method. Callers should wrap in `tokio::spawn`
    /// to avoid blocking the game loop.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_log(
        &self,
        event_type: i16,
        account_name: &str,
        char_name: &str,
        remote_ip: &str,
        zone_id: i16,
        pos_x: i16,
        pos_z: i16,
        details: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO game_audit_log \
             (event_type, account_name, char_name, remote_ip, zone_id, pos_x, pos_z, details) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(event_type)
        .bind(account_name)
        .bind(char_name)
        .bind(remote_ip)
        .bind(zone_id)
        .bind(pos_x)
        .bind(pos_z)
        .bind(details)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Insert a minimal audit log entry (no position, empty details).
    pub async fn insert_simple(
        &self,
        event_type: i16,
        account_name: &str,
        char_name: &str,
        remote_ip: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO game_audit_log \
             (event_type, account_name, char_name, remote_ip) \
             VALUES ($1, $2, $3, $4)",
        )
        .bind(event_type)
        .bind(account_name)
        .bind(char_name)
        .bind(remote_ip)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
