//! Event scheduling repository — loads event schedule, timing, rewards, and triggers.
//!
//! C++ Reference: `CGameServerDlg::LoadEventTimeList()`, `LoadEventScheduleData()`

use crate::models::event_schedule::{
    EventOptFtRow, EventOptVroomRow, EventRewardRow, EventRoomPlayTimerRow, EventScheduleDayRow,
    EventScheduleMainRow, EventTimerShowRow, EventTriggerRow,
};
use crate::DbPool;

/// Repository for event scheduling data.
pub struct EventScheduleRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> EventScheduleRepository<'a> {
    /// Create a new event schedule repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all event schedule main list entries.
    ///
    /// C++ Reference: `CEventScheduleMainListSet`
    pub async fn load_main_list(&self) -> Result<Vec<EventScheduleMainRow>, sqlx::Error> {
        sqlx::query_as::<_, EventScheduleMainRow>(
            "SELECT eventid, event_type, zoneid, name, status, \
             hour1, minute1, hour2, minute2, hour3, minute3, \
             hour4, minute4, hour5, minute5, \
             min_level, max_level, req_loyalty, req_money \
             FROM event_schedule_main_list ORDER BY eventid",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all event schedule day-of-week entries.
    ///
    /// C++ Reference: `CEventScheduleDayListSet`
    pub async fn load_day_list(&self) -> Result<Vec<EventScheduleDayRow>, sqlx::Error> {
        sqlx::query_as::<_, EventScheduleDayRow>(
            "SELECT eventid, sunday, monday, tuesday, wednesday, thursday, friday, saturday \
             FROM event_schedule_day_list ORDER BY eventid",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load virtual room event timing options.
    ///
    /// C++ Reference: `CGameServerDlg::pEventTimeOpt.pvroomop[]`
    pub async fn load_vroom_opts(&self) -> Result<Vec<EventOptVroomRow>, sqlx::Error> {
        sqlx::query_as::<_, EventOptVroomRow>(
            "SELECT zoneid, name, sign, play, attackopen, attackclose, finish \
             FROM event_opt_vroom ORDER BY zoneid",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load room-based event play timers.
    ///
    /// C++ Reference: `eventroomscheduleplaytimerset.h`
    pub async fn load_room_play_timers(&self) -> Result<Vec<EventRoomPlayTimerRow>, sqlx::Error> {
        sqlx::query_as::<_, EventRoomPlayTimerRow>(
            "SELECT event_local_id, event_zone_id, event_name, \
             event_sign_time, event_play_time, event_attack_open, \
             event_attack_close, event_finish_time \
             FROM event_room_play_timer ORDER BY event_local_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load event rewards (winner/loser prizes).
    ///
    /// C++ Reference: `CGameServerDlg::m_EventRewardArray`
    pub async fn load_rewards(&self) -> Result<Vec<EventRewardRow>, sqlx::Error> {
        sqlx::query_as::<_, EventRewardRow>(
            "SELECT s_index, status, local_id, is_winner, description, \
             item_id1, item_count1, item_expiration1, \
             item_id2, item_count2, item_expiration2, \
             item_id3, item_count3, item_expiration3, \
             experience, loyalty, cash, noah \
             FROM event_rewards WHERE status = true ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load event trigger mappings.
    ///
    /// C++ Reference: `CGameServerDlg::m_EventTriggerArray`
    pub async fn load_triggers(&self) -> Result<Vec<EventTriggerRow>, sqlx::Error> {
        sqlx::query_as::<_, EventTriggerRow>(
            "SELECT n_index, b_npc_type, s_npc_id, n_trigger_num \
             FROM event_trigger ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load timer display entries for client UI.
    ///
    /// C++ Reference: `CGameServerDlg::m_EventTimerShowListArray`
    pub async fn load_timer_show_list(&self) -> Result<Vec<EventTimerShowRow>, sqlx::Error> {
        sqlx::query_as::<_, EventTimerShowRow>(
            "SELECT id, name, status, hour, minute, days \
             FROM event_timer_show_list WHERE status = true ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load Forgotten Temple timing options.
    ///
    /// C++ Reference: `CGameServerDlg::pForgettenTemple.ptimeopt`
    pub async fn load_ft_opts(&self) -> Result<Option<EventOptFtRow>, sqlx::Error> {
        sqlx::query_as::<_, EventOptFtRow>(
            "SELECT playing_time, summon_time, spawn_min_time, waiting_time, \
             min_level, max_level \
             FROM event_opt_ft LIMIT 1",
        )
        .fetch_optional(self.pool)
        .await
    }
}
