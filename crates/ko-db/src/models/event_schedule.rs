//! Event scheduling model structs — maps to PostgreSQL event_schedule_* tables.
//!
//! C++ Reference: `GameDefine.h` — `EVENT_OPENTIMELIST`, `EventType`, `EventLocalID`

use sqlx::FromRow;

/// Core schedule entry for an event (main list).
///
/// C++ Reference: `EVENT_OPENTIMELIST` struct in `GameDefine.h`
#[derive(Debug, Clone, FromRow)]
pub struct EventScheduleMainRow {
    pub eventid: i16,
    pub event_type: i16,
    pub zoneid: i16,
    pub name: String,
    pub status: i16,
    pub hour1: i16,
    pub minute1: i16,
    pub hour2: i16,
    pub minute2: i16,
    pub hour3: i16,
    pub minute3: i16,
    pub hour4: i16,
    pub minute4: i16,
    pub hour5: i16,
    pub minute5: i16,
    pub min_level: i16,
    pub max_level: i16,
    pub req_loyalty: i32,
    pub req_money: i32,
}

/// Day-of-week enablement for an event schedule.
///
/// C++ Reference: Day flags in `EVENT_OPENTIMELIST::iday[7]`
#[derive(Debug, Clone, FromRow)]
pub struct EventScheduleDayRow {
    pub eventid: i16,
    pub sunday: i16,
    pub monday: i16,
    pub tuesday: i16,
    pub wednesday: i16,
    pub thursday: i16,
    pub friday: i16,
    pub saturday: i16,
}

/// Virtual room event timing options.
///
/// C++ Reference: `CGameServerDlg::pEventTimeOpt.pvroomop[]`
#[derive(Debug, Clone, FromRow)]
pub struct EventOptVroomRow {
    pub zoneid: i16,
    pub name: String,
    pub sign: i32,
    pub play: i32,
    pub attackopen: i32,
    pub attackclose: i32,
    pub finish: i32,
}

/// Room-based event detailed play timer.
///
/// C++ Reference: `eventroomscheduleplaytimerset.h`
#[derive(Debug, Clone, FromRow)]
pub struct EventRoomPlayTimerRow {
    pub event_local_id: i16,
    pub event_zone_id: i16,
    pub event_name: String,
    pub event_sign_time: i32,
    pub event_play_time: i32,
    pub event_attack_open: i32,
    pub event_attack_close: i32,
    pub event_finish_time: i32,
}

/// Event rewards for winners/losers.
///
/// C++ Reference: `CGameServerDlg::m_EventRewardArray`
#[derive(Debug, Clone, FromRow)]
pub struct EventRewardRow {
    pub s_index: i32,
    pub status: bool,
    pub local_id: i16,
    pub is_winner: bool,
    pub description: String,
    pub item_id1: i32,
    pub item_count1: i32,
    pub item_expiration1: i32,
    pub item_id2: i32,
    pub item_count2: i32,
    pub item_expiration2: i32,
    pub item_id3: i32,
    pub item_count3: i32,
    pub item_expiration3: i32,
    pub experience: i64,
    pub loyalty: i32,
    pub cash: i32,
    pub noah: i32,
}

/// Event trigger mapping (NPC type/ID → trigger number).
///
/// C++ Reference: `CGameServerDlg::m_EventTriggerArray`
#[derive(Debug, Clone, FromRow)]
pub struct EventTriggerRow {
    pub n_index: i32,
    pub b_npc_type: i16,
    pub s_npc_id: i16,
    pub n_trigger_num: i32,
}

/// Timer display entry for the client UI.
///
/// C++ Reference: `_EVENT_TIMER_SHOW_LIST` in `GameDefine.h`
#[derive(Debug, Clone, FromRow)]
pub struct EventTimerShowRow {
    pub id: i32,
    pub name: String,
    pub status: bool,
    pub hour: i32,
    pub minute: i32,
    pub days: String,
}

/// Forgotten Temple specific timing options.
///
/// C++ Reference: `CGameServerDlg::pForgettenTemple.ptimeopt`
#[derive(Debug, Clone, FromRow)]
pub struct EventOptFtRow {
    pub playing_time: i32,
    pub summon_time: i32,
    pub spawn_min_time: i32,
    pub waiting_time: i32,
    pub min_level: i32,
    pub max_level: i32,
}

/// A row from the `event_start_schedule` table — defines a schedulable
/// event with day-of-week activation pattern.
///
/// Source: MSSQL `EVENT_START_SCHEDULE` table — event master list with day filters.
#[derive(Debug, Clone, FromRow)]
pub struct EventStartScheduleRow {
    /// Unique local ID for this event schedule entry.
    pub event_local_id: i32,
    /// Event type identifier (maps to EventType enum).
    pub event_type: i16,
    /// Zone ID where this event takes place.
    pub event_zone_id: i16,
    /// Human-readable event name.
    pub event_name: String,
    /// Comma-separated day-of-week flags (e.g. "1,1,1,1,1,0,0").
    pub start_days: String,
    /// Event status (0=disabled, 1=enabled).
    pub event_status: i16,
}

/// A row from the `event_start_time_slot` table — defines a specific
/// time slot for an event schedule entry.
///
/// Source: MSSQL `EVENT_START_TIME_SLOT` table — multiple time slots per event.
#[derive(Debug, Clone, FromRow)]
pub struct EventStartTimeSlotRow {
    /// References `event_start_schedule.event_local_id`.
    pub event_local_id: i32,
    /// Slot ordering index within the event.
    pub slot_index: i16,
    /// Hour to start the event (0-23).
    pub start_hour: i32,
    /// Minute to start the event (0-59).
    pub start_minute: i32,
    /// Duration this time slot is active (in minutes).
    pub time_active: i16,
}
