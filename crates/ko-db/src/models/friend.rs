//! Friend list model — maps to `friend_list` table.
//!
//! C++ Reference: `FRIEND_LIST` table in MSSQL.

/// A single friend entry in the friend list.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FriendRow {
    /// Character name of the friend list owner.
    pub user_id: String,
    /// Character name of the friend.
    pub friend_name: String,
}
