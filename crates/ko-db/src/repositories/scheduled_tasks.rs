//! Scheduled tasks repository — broadcast messages and automatic commands.
//!
//! Source: MSSQL `SEND_MESSAGES` + `AUTOMATIC_COMMAND`

use crate::models::scheduled_tasks::{AutomaticCommand, SendMessage};
use crate::DbPool;

/// Repository for scheduled task table access.
pub struct ScheduledTasksRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ScheduledTasksRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all broadcast messages.
    pub async fn load_messages(&self) -> Result<Vec<SendMessage>, sqlx::Error> {
        sqlx::query_as::<_, SendMessage>(
            "SELECT id, message, sender, chat_type, send_type, send_hour_minute \
             FROM send_messages ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all active automatic commands.
    pub async fn load_active_commands(&self) -> Result<Vec<AutomaticCommand>, sqlx::Error> {
        sqlx::query_as::<_, AutomaticCommand>(
            "SELECT idx, status, command, hour, minute, day_of_week, description \
             FROM automatic_command WHERE status = true ORDER BY idx",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all automatic commands (including inactive).
    pub async fn load_all_commands(&self) -> Result<Vec<AutomaticCommand>, sqlx::Error> {
        sqlx::query_as::<_, AutomaticCommand>(
            "SELECT idx, status, command, hour, minute, day_of_week, description \
             FROM automatic_command ORDER BY idx",
        )
        .fetch_all(self.pool)
        .await
    }
}
