//! Server info model — maps to `game_server_list` PostgreSQL table.

use sqlx::FromRow;

/// A game server entry as stored in the database.
#[derive(Debug, Clone, FromRow)]
pub struct ServerInfo {
    pub server_id: i16,
    pub group_id: i16,
    pub screen_type: i16,
    pub server_name: String,
    pub server_ip: String,
    pub lan_ip: String,
    pub player_cap: i16,
    pub free_player_cap: i16,
    #[sqlx(default)]
    pub karus_king: String,
    #[sqlx(default)]
    pub karus_notice: String,
    #[sqlx(default)]
    pub elmorad_king: String,
    #[sqlx(default)]
    pub elmorad_notice: String,
    /// Current online player count (updated every 120s by background task).
    ///
    #[sqlx(default)]
    pub concurrent_users: i32,
}
