//! Repository for client_version table — patch list queries.

use crate::models::client_version::ClientVersionRow;
use crate::DbPool;

pub struct ClientVersionRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ClientVersionRepository<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Get all patches with version > client_version, ordered ascending.
    pub async fn get_patches_after(
        &self,
        client_version: i16,
    ) -> Result<Vec<ClientVersionRow>, sqlx::Error> {
        sqlx::query_as::<_, ClientVersionRow>(
            "SELECT version, history_version, filename
             FROM client_version
             WHERE version > $1
             ORDER BY version ASC",
        )
        .bind(client_version)
        .fetch_all(self.pool)
        .await
    }
}
