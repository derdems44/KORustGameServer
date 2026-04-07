//! Rental item repository — user_rental_item table access.
//!
//! C++ Reference: `CDBAgent::LoadRentalData()` in `DBAgent.cpp:308-355`
//!
//! Provides CRUD operations for the `user_rental_item` table which tracks
//! player-to-player item rentals (lender/borrower relationships).

use sqlx::PgPool;

use crate::models::UserRentalItemRow;

/// Repository for rental item database operations.
pub struct RentalRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RentalRepository<'a> {
    /// Create a new rental repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all rental items for a character.
    ///
    /// C++ Reference: `CDBAgent::LoadRentalData()` — loads by account_id,
    /// then filters by char_id.
    pub async fn load_by_user(&self, user_id: &str) -> Result<Vec<UserRentalItemRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRentalItemRow>(
            "SELECT user_id, account_id, rental_type, reg_type, rental_index, \
             item_index, durability, serial_number, rental_money, rental_time, \
             during_time, rental_at, registered_at \
             FROM user_rental_item WHERE user_id = $1 \
             ORDER BY rental_index",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load a specific rental item by user_id and rental_index.
    pub async fn load_one(
        &self,
        user_id: &str,
        rental_index: i32,
    ) -> Result<Option<UserRentalItemRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRentalItemRow>(
            "SELECT user_id, account_id, rental_type, reg_type, rental_index, \
             item_index, durability, serial_number, rental_money, rental_time, \
             during_time, rental_at, registered_at \
             FROM user_rental_item \
             WHERE user_id = $1 AND rental_index = $2",
        )
        .bind(user_id)
        .bind(rental_index)
        .fetch_optional(self.pool)
        .await
    }

    /// Insert a new rental item record.
    ///
    /// Called when a player registers an item for rental or borrows one.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert(
        &self,
        user_id: &str,
        account_id: &str,
        rental_type: i16,
        reg_type: i16,
        rental_index: i32,
        item_index: i32,
        durability: i16,
        serial_number: i64,
        rental_money: i32,
        rental_time: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_rental_item \
             (user_id, account_id, rental_type, reg_type, rental_index, \
              item_index, durability, serial_number, rental_money, rental_time, \
              during_time, rental_at, registered_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, NOW(), NOW()) \
             ON CONFLICT (user_id, rental_index) DO NOTHING",
        )
        .bind(user_id)
        .bind(account_id)
        .bind(rental_type)
        .bind(reg_type)
        .bind(rental_index)
        .bind(item_index)
        .bind(durability)
        .bind(serial_number)
        .bind(rental_money)
        .bind(rental_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete a rental item record (cancel/return).
    pub async fn delete(&self, user_id: &str, rental_index: i32) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("DELETE FROM user_rental_item WHERE user_id = $1 AND rental_index = $2")
                .bind(user_id)
                .bind(rental_index)
                .execute(self.pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Count rental items for a user.
    pub async fn count_by_user(&self, user_id: &str) -> Result<i64, sqlx::Error> {
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM user_rental_item WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(self.pool)
                .await?;
        Ok(row.0)
    }

    /// Update the rental_item table: set borrower and update state.
    ///
    /// When a player borrows (lends from another), update the master rental_item
    /// table to record the borrower.
    pub async fn update_rental_item_borrower(
        &self,
        rental_index: i32,
        borrower_char_id: &str,
    ) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("UPDATE rental_item SET borrower_char_id = $1 WHERE rental_index = $2")
                .bind(borrower_char_id)
                .bind(rental_index)
                .execute(self.pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Clear borrower from rental_item table (item returned).
    pub async fn clear_rental_item_borrower(&self, rental_index: i32) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("UPDATE rental_item SET borrower_char_id = '' WHERE rental_index = $1")
                .bind(rental_index)
                .execute(self.pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Insert a new entry into the rental_item catalog table.
    ///
    /// Called when a player registers an item for PvP rental.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_catalog(
        &self,
        rental_index: i32,
        item_index: i32,
        durability: i16,
        serial_number: i64,
        reg_type: i16,
        item_type: i16,
        item_class: i16,
        rental_time: i16,
        rental_money: i32,
        lender_char_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO rental_item \
             (rental_index, item_index, durability, serial_number, reg_type, \
              item_type, item_class, rental_time, rental_money, \
              lender_char_id, borrower_char_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, '') \
             ON CONFLICT (rental_index) DO NOTHING",
        )
        .bind(rental_index)
        .bind(item_index)
        .bind(durability)
        .bind(serial_number)
        .bind(reg_type)
        .bind(item_type)
        .bind(item_class)
        .bind(rental_time)
        .bind(rental_money)
        .bind(lender_char_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete an entry from the rental_item catalog table.
    ///
    /// Called when a player cancels a rental registration.
    pub async fn delete_catalog(&self, rental_index: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM rental_item WHERE rental_index = $1")
            .bind(rental_index)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
