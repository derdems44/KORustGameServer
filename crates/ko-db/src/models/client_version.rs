//! Client version / patch tracking model.
//!
//! Source: `client_version` table (version, history_version, filename).
//! Used by the login server to determine which patches a client needs.

/// A patch entry from the client_version table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClientVersionRow {
    pub version: i16,
    pub history_version: i16,
    pub filename: String,
}
