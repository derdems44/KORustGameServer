//! Pet repository — loads and saves pet data from PostgreSQL.
//! - `GameServer/LoadServerData.cpp` — pet info/transform loading
//! - `GameServer/DBAgent.cpp` — `CreateNewPet()`, `LoadPetData()`

use crate::models::pet::{PetImageChangeRow, PetStatsInfoRow, PetUserDataRow};
use crate::DbPool;

/// Repository for pet system table access.
pub struct PetRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> PetRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all pet stats info rows (bulk load at startup, 60 rows).
    ///
    pub async fn load_all_stats_info(&self) -> Result<Vec<PetStatsInfoRow>, sqlx::Error> {
        sqlx::query_as::<_, PetStatsInfoRow>(
            "SELECT pet_level, pet_max_hp, pet_max_sp, pet_attack, pet_defence, pet_res, pet_exp \
             FROM pet_stats_info ORDER BY pet_level",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all pet image change/transform recipes (bulk load at startup).
    ///
    pub async fn load_all_image_changes(&self) -> Result<Vec<PetImageChangeRow>, sqlx::Error> {
        sqlx::query_as::<_, PetImageChangeRow>(
            "SELECT s_index, n_req_item0, n_req_item1, n_req_item2, n_replace_item, \
             s_replace_spid, s_replace_size, str_name, s_percent \
             FROM pet_image_change ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load pet data for a specific serial ID.
    ///
    pub async fn load_pet_data(
        &self,
        serial_id: i64,
    ) -> Result<Option<PetUserDataRow>, sqlx::Error> {
        sqlx::query_as::<_, PetUserDataRow>(
            "SELECT n_serial_id, s_pet_name, b_level, s_hp, s_mp, n_index, \
             s_satisfaction, n_exp, s_pid, s_size \
             FROM pet_user_data WHERE n_serial_id = $1",
        )
        .bind(serial_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save (upsert) pet data back to the database.
    ///
    pub async fn save_pet_data(&self, pet: &PetUserDataRow) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO pet_user_data (n_serial_id, s_pet_name, b_level, s_hp, s_mp, \
             n_index, s_satisfaction, n_exp, s_pid, s_size) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
             ON CONFLICT (n_serial_id) DO UPDATE SET \
             s_pet_name = $2, b_level = $3, s_hp = $4, s_mp = $5, \
             n_index = $6, s_satisfaction = $7, n_exp = $8, s_pid = $9, s_size = $10",
        )
        .bind(pet.n_serial_id)
        .bind(&pet.s_pet_name)
        .bind(pet.b_level)
        .bind(pet.s_hp)
        .bind(pet.s_mp)
        .bind(pet.n_index)
        .bind(pet.s_satisfaction)
        .bind(pet.n_exp)
        .bind(pet.s_pid)
        .bind(pet.s_size)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Create a new pet and return the auto-assigned index.
    ///
    pub async fn create_pet(
        &self,
        serial_id: i64,
        level: i16,
        name: &str,
        max_hp: i16,
        max_mp: i16,
    ) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO pet_user_data (n_serial_id, s_pet_name, b_level, s_hp, s_mp, \
             s_satisfaction, n_exp, s_pid, s_size) \
             VALUES ($1, $2, $3, $4, $5, 9000, 0, 25500, 100) \
             RETURNING n_index",
        )
        .bind(serial_id)
        .bind(name)
        .bind(level)
        .bind(max_hp)
        .bind(max_mp)
        .fetch_one(self.pool)
        .await?;
        Ok(row.0)
    }
}
