//! Friend list repository — friend_list table access.
//! in `DBAgent.cpp:1333-1419`.

use sqlx::PgPool;

use crate::models::FriendRow;

/// Maximum number of friends per player.
pub const MAX_FRIEND_COUNT: usize = 24;

/// Repository for friend list database operations.
pub struct FriendRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> FriendRepository<'a> {
    /// Create a new friend repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all friends for a character.
    ///
    pub async fn load_friends(&self, user_id: &str) -> Result<Vec<FriendRow>, sqlx::Error> {
        sqlx::query_as::<_, FriendRow>(
            "SELECT user_id, friend_name FROM friend_list WHERE user_id = $1 ORDER BY added_at",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await
    }

    /// Add a friend to the list. Returns true if inserted, false if already exists.
    ///
    pub async fn add_friend(&self, user_id: &str, friend_name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO friend_list (user_id, friend_name) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(friend_name)
        .execute(self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Remove a friend from the list. Returns true if removed.
    ///
    pub async fn remove_friend(
        &self,
        user_id: &str,
        friend_name: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM friend_list WHERE user_id = $1 AND friend_name = $2")
            .bind(user_id)
            .bind(friend_name)
            .execute(self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Count friends for a character.
    pub async fn count_friends(&self, user_id: &str) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM friend_list WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(self.pool)
            .await?;

        Ok(row.0)
    }

    /// Check if a character name exists in the userdata table.
    pub async fn character_exists(&self, char_name: &str) -> Result<bool, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM userdata WHERE str_user_id = $1")
            .bind(char_name)
            .fetch_one(self.pool)
            .await?;

        Ok(row.0 > 0)
    }
}
