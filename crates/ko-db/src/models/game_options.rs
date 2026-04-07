//! Game options model — maps to the `game_options` PostgreSQL table.
//!
//! Source: MSSQL `GAME_OPTIONS` table — server-wide toggle/limit settings.

/// A row from the `game_options` table — global server options.
///
/// Controls maintenance mode, login restrictions, OTP, auto-register,
/// and user capacity limits.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameOptions {
    /// Primary key.
    pub id: i32,
    /// Whether the server is in maintenance mode (blocks non-GM logins).
    pub maintenance_mode: bool,
    /// Whether character-select login is enabled.
    pub char_select_login: bool,
    /// Whether OTP (one-time password) verification is required.
    pub open_otp: bool,
    /// Whether automatic account registration on first login is enabled.
    pub auto_register: bool,
    /// Free (non-premium) concurrent user limit.
    pub free_limit: i16,
    /// Total concurrent user limit across all sessions.
    pub total_user_limit: i16,
    /// Server IP address string (sent to client for reconnect/redirect).
    pub server_ip: String,
}
