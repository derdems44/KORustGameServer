//! User rental item model — maps to the `user_rental_item` PostgreSQL table.
//!
//! Source: MSSQL `USER_RENTAL_ITEM` table — time-limited item rental tracking.

use chrono::{DateTime, Utc};

/// A row from the `user_rental_item` table — tracks items rented by users
/// with time-limited ownership.
///
/// Each row records a single rented item, its rental terms, and
/// expiration timestamps.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRentalItemRow {
    /// Character ID of the renting user (PK part).
    pub user_id: String,
    /// Account ID that owns the character.
    pub account_id: Option<String>,
    /// Rental type (defines rental category/rules).
    pub rental_type: Option<i16>,
    /// Registration type (how the rental was initiated).
    pub reg_type: Option<i16>,
    /// Rental list index.
    pub rental_index: Option<i32>,
    /// Item template ID.
    pub item_index: Option<i32>,
    /// Item durability at time of rental.
    pub durability: Option<i16>,
    /// Unique serial number for the item instance.
    pub serial_number: Option<i64>,
    /// Rental cost in gold.
    pub rental_money: Option<i32>,
    /// Rental duration period (in rental time units).
    pub rental_time: Option<i16>,
    /// Elapsed time since rental start (in rental time units).
    pub during_time: Option<i16>,
    /// Timestamp when the rental was created.
    pub rental_at: Option<DateTime<Utc>>,
    /// Timestamp when the rental was registered/confirmed.
    pub registered_at: Option<DateTime<Utc>>,
}
