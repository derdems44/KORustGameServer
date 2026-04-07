//! Scheduled task models — maps to `send_message` and `automatic_command` PostgreSQL tables.
//!
//! Source: MSSQL `SEND_MESSAGE`, `AUTOMATIC_COMMAND` tables — timed server actions.

/// A row from the `send_message` table — defines scheduled chat messages
/// broadcast by the server at specific times.
///
/// Used for automated announcements, event notifications, and
/// periodic server messages.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SendMessage {
    /// Primary key.
    pub id: i32,
    /// Message text to broadcast (optional, NULL means disabled).
    pub message: Option<String>,
    /// Sender name displayed in chat.
    pub sender: String,
    /// Chat channel type (maps to WIZ_CHAT sub-types).
    pub chat_type: i16,
    /// Send type / trigger mode.
    pub send_type: i16,
    /// Scheduled send time as HHMM (e.g. 1430 = 14:30).
    pub send_hour_minute: i16,
}

/// A row from the `automatic_command` table — defines scheduled server
/// commands that execute automatically at specified times.
///
/// Used for periodic server maintenance tasks, event triggers,
/// and automated GM commands.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AutomaticCommand {
    /// Primary key index.
    pub idx: i32,
    /// Whether this scheduled command is active.
    pub status: bool,
    /// The command string to execute.
    pub command: String,
    /// Hour of day to execute (0-23).
    pub hour: i32,
    /// Minute of hour to execute (0-59).
    pub minute: i32,
    /// Day of week (0=Sunday, 1=Monday, ..., 6=Saturday; -1=every day).
    pub day_of_week: i32,
    /// Human-readable description of what this command does.
    pub description: String,
}
