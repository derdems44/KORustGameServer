//! Monster resource repository — death notice strings.
//!
//! Source: MSSQL `MONSTER_RESOURCE` (4 rows)

use crate::models::monster_resource::MonsterResource;
use crate::DbPool;

/// Repository for `monster_resource` table access.
pub struct MonsterResourceRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> MonsterResourceRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all monster resource entries.
    pub async fn load_all(&self) -> Result<Vec<MonsterResource>, sqlx::Error> {
        sqlx::query_as::<_, MonsterResource>(
            "SELECT sid, sid_name, resource, notice_zone, notice_type \
             FROM monster_resource ORDER BY sid",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Get resource entry for a specific monster SID.
    pub async fn get_by_sid(&self, sid: i16) -> Result<Option<MonsterResource>, sqlx::Error> {
        sqlx::query_as::<_, MonsterResource>(
            "SELECT sid, sid_name, resource, notice_zone, notice_type \
             FROM monster_resource WHERE sid = $1",
        )
        .bind(sid)
        .fetch_optional(self.pool)
        .await
    }
}
