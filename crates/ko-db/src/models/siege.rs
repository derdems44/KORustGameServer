//! Siege warfare model -- maps to the `knights_siege_warfare` PostgreSQL table.
//! - `_KNIGHTS_SIEGE_WARFARE` struct
//! - `KnightsSiegeWarFare.h` — OdbcRecordset columns
//! - MSSQL `KNIGHTS_SIEGE_WARFARE` table (38 columns, 1 row)
//! One row per castle (castle_index=1 for Delos). Stores war schedule,
//! challenge/request clan lists, tariffs, tax revenue, and dungeon charges.

/// A single siege warfare row from the database.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsSiegeWarfareRow {
    /// Castle identifier (1 = Delos).
    pub s_castle_index: i16,
    /// Clan ID of the castle owner (master knights).
    pub s_master_knights: i16,
    /// Siege type (0=none, 1=regular).
    pub by_siege_type: i16,
    /// Scheduled war day.
    pub by_war_day: i16,
    /// Scheduled war hour.
    pub by_war_time: i16,
    /// Scheduled war minute.
    pub by_war_minute: i16,

    /// Challenge clan list slots 1-10.
    pub s_challenge_list_1: i16,
    pub s_challenge_list_2: i16,
    pub s_challenge_list_3: i16,
    pub s_challenge_list_4: i16,
    pub s_challenge_list_5: i16,
    pub s_challenge_list_6: i16,
    pub s_challenge_list_7: i16,
    pub s_challenge_list_8: i16,
    pub s_challenge_list_9: i16,
    pub s_challenge_list_10: i16,

    /// War request schedule.
    pub by_war_request_day: i16,
    pub by_war_request_time: i16,
    pub by_war_request_minute: i16,

    /// Guerrilla war schedule.
    pub by_guerrilla_war_day: i16,
    pub by_guerrilla_war_time: i16,
    pub by_guerrilla_war_minute: i16,

    /// Challenge list as string (legacy field).
    pub str_challenge_list: String,

    /// Moradon zone tariff rate (0-20).
    pub s_moradon_tariff: i16,
    /// Delos zone tariff rate (0-20).
    pub s_delos_tariff: i16,
    /// Accumulated dungeon charge revenue.
    pub n_dungeon_charge: i32,
    /// Accumulated Moradon tax revenue.
    pub n_moradon_tax: i32,
    /// Accumulated Delos tax revenue.
    pub n_delos_tax: i32,

    /// Request clan list slots 1-10.
    pub s_request_list_1: i16,
    pub s_request_list_2: i16,
    pub s_request_list_3: i16,
    pub s_request_list_4: i16,
    pub s_request_list_5: i16,
    pub s_request_list_6: i16,
    pub s_request_list_7: i16,
    pub s_request_list_8: i16,
    pub s_request_list_9: i16,
    pub s_request_list_10: i16,
}
