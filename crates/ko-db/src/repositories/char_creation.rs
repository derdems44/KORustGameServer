//! Character creation repository — loads starting equipment and stats from PostgreSQL.
//! - `CDBAgent::LoadNewCharSet()` — stored procedure `LOAD_NEW_CHAR_SET`
//! - `CDBAgent::LoadNewCharValue()` — stored procedure `LOAD_NEW_CHAR_VALUE`

use crate::models::char_creation::{CreateNewCharSetRow, CreateNewCharValueRow};
use crate::DbPool;

/// Repository for character creation data tables.
pub struct CharCreationRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> CharCreationRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all starting equipment entries (375 rows: 5 classes x 75 slots).
    ///
    pub async fn load_all_char_set(&self) -> Result<Vec<CreateNewCharSetRow>, sqlx::Error> {
        sqlx::query_as::<_, CreateNewCharSetRow>(
            "SELECT id, class_type, slot_id, item_id, item_duration, item_count, \
             item_flag, item_expire_time \
             FROM create_new_char_set ORDER BY class_type, slot_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all starting stat/level/gold entries (25 rows: 5 classes x 5 job types).
    ///
    pub async fn load_all_char_value(&self) -> Result<Vec<CreateNewCharValueRow>, sqlx::Error> {
        sqlx::query_as::<_, CreateNewCharValueRow>(
            "SELECT n_index, class_type, job_type, level, exp, strength, health, \
             dexterity, intelligence, magic_power, free_points, skill_point_free, \
             skill_point_cat1, skill_point_cat2, skill_point_cat3, skill_point_master, gold \
             FROM create_new_char_value ORDER BY class_type, job_type",
        )
        .fetch_all(self.pool)
        .await
    }
}
