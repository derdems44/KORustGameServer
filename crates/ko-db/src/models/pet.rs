//! Pet system models — maps to PostgreSQL pet tables.
//!
//! C++ Reference:
//! - `shared/globals.h` — `PET_INFO`, `PET_DATA`, `PET_TRANSFORM` structs
//! - `shared/database/PetStatsInfo.h` — DB loader for PET_STATS_INFO
//! - `shared/database/PetDataInfo.h` — DB loader for PET_USER_DATA
//! - `shared/database/PetTransformSet.h` — DB loader for PET_IMAGE_CHANGE

/// Per-level pet stats from the `pet_stats_info` table.
///
/// C++ equivalent: `PET_INFO` (globals.h:598)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PetStatsInfoRow {
    /// Pet level (1-60).
    pub pet_level: i16,
    /// Maximum HP at this level.
    pub pet_max_hp: i16,
    /// Maximum SP/MP at this level.
    pub pet_max_sp: i16,
    /// Attack power at this level.
    pub pet_attack: i16,
    /// Defence at this level.
    pub pet_defence: i16,
    /// Resistance at this level.
    pub pet_res: i16,
    /// Experience required to reach next level.
    pub pet_exp: i32,
}

/// Per-character pet data from the `pet_user_data` table.
///
/// C++ equivalent: `PET_DATA` (globals.h:621)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PetUserDataRow {
    /// Unique serial ID (matches the pet item's serial number).
    pub n_serial_id: i64,
    /// Pet name (max 15 characters).
    pub s_pet_name: String,
    /// Current pet level (1-60).
    pub b_level: i16,
    /// Current HP.
    pub s_hp: i16,
    /// Current MP.
    pub s_mp: i16,
    /// Unique pet index (auto-incrementing).
    pub n_index: i32,
    /// Satisfaction value (0-10000, pet dies at 0).
    pub s_satisfaction: i16,
    /// Current experience points.
    pub n_exp: i32,
    /// Pet visual model ID.
    pub s_pid: i16,
    /// Pet visual size.
    pub s_size: i16,
}

/// Pet image transform recipe from the `pet_image_change` table.
///
/// C++ equivalent: `PET_TRANSFORM` (globals.h:609)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PetImageChangeRow {
    /// Recipe index (primary key).
    pub s_index: i32,
    /// Required item 0 (main ingredient).
    pub n_req_item0: i32,
    /// Required item 1 (optional).
    pub n_req_item1: i32,
    /// Required item 2 (optional).
    pub n_req_item2: i32,
    /// Resulting replacement item.
    pub n_replace_item: i32,
    /// Replacement NPC sprite PID.
    pub s_replace_spid: i16,
    /// Replacement NPC sprite size.
    pub s_replace_size: i16,
    /// Name of the pet form.
    pub str_name: String,
    /// Success percent weight (for weighted random selection).
    pub s_percent: i16,
}
