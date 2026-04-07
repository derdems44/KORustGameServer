//! Server list repository — loads game server entries from PostgreSQL.
//!
//! C++ Reference: `KOOriginalGameServer/LoginServer/DBProcess.cpp` (LoadServerList)

use crate::models::ServerInfo;
use crate::DbPool;

/// Repository for `game_server_list` table access.
pub struct ServerListRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ServerListRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all game server entries from the database.
    ///
    /// C++ Reference: `DBProcess::LoadServerList()` — `SELECT * FROM GAME_SERVER_LIST`
    pub async fn load_all(&self) -> Result<Vec<ServerInfo>, sqlx::Error> {
        sqlx::query_as::<_, ServerInfo>(
            "SELECT server_id, group_id, screen_type, server_name, server_ip, lan_ip, \
             player_cap, free_player_cap, \
             COALESCE(karus_king, '') as karus_king, \
             COALESCE(karus_notice, '') as karus_notice, \
             COALESCE(elmorad_king, '') as elmorad_king, \
             COALESCE(elmorad_notice, '') as elmorad_notice, \
             concurrent_users \
             FROM game_server_list ORDER BY server_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Update the concurrent user count for a specific server.
    ///
    /// C++ Reference: `CDBAgent::UpdateConCurrentUserCount()` —
    /// `UPDATE CONCURRENT SET zone1_count = ? WHERE serverid = ?`
    pub async fn update_concurrent_users(
        &self,
        server_id: i16,
        count: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE game_server_list SET concurrent_users = $1 WHERE server_id = $2")
            .bind(count)
            .bind(server_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
