//! Daily quest definition and user progress models.
//!
//! C++ Reference: `_DAILY_QUEST` struct in `GameDefine.h:4862`,
//!                `_DAILY_USERQUEST` struct in `GameDefine.h:4878`,
//!                `DailyQuestArray` in `LoadServerData.h:183`.

/// A row from the `daily_quests` table — server-wide quest definition.
///
/// C++ Reference: `_DAILY_QUEST` struct.
///
/// | `time_type` | Meaning        |
/// |-------------|----------------|
/// | 0           | Repeatable     |
/// | 1           | Time-gated     |
/// | 2           | Single (once)  |
///
/// | `kill_type` | Meaning     |
/// |-------------|-------------|
/// | 0           | Solo only   |
/// | 1           | Party only  |
/// | 2           | Any         |
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyQuestRow {
    /// Quest definition ID (PK), matches C++ `_DAILY_QUEST::index`.
    pub id: i16,
    /// Display name of the quest.
    pub quest_name: Option<String>,
    /// Associated quest system ID (0 = standalone daily quest).
    pub quest_id: i16,
    /// Time type: 0=repeat, 1=time, 2=single.
    pub time_type: i16,
    /// Kill type: 0=solo, 1=party, 2=any.
    pub kill_type: i16,
    /// Target monster ID slot 1 (0 = unused).
    pub mob_id_1: i32,
    /// Target monster ID slot 2 (0 = unused).
    pub mob_id_2: i32,
    /// Target monster ID slot 3 (0 = unused).
    pub mob_id_3: i32,
    /// Target monster ID slot 4 (0 = unused).
    pub mob_id_4: i32,
    /// Number of kills required to complete the quest.
    pub kill_count: i32,
    /// Reward item ID slot 1.
    pub reward_1: i32,
    /// Reward item ID slot 2.
    pub reward_2: i32,
    /// Reward item ID slot 3.
    pub reward_3: i32,
    /// Reward item ID slot 4.
    pub reward_4: i32,
    /// Reward count slot 1.
    pub count_1: i32,
    /// Reward count slot 2.
    pub count_2: i32,
    /// Reward count slot 3.
    pub count_3: i32,
    /// Reward count slot 4.
    pub count_4: i32,
    /// Required zone ID (21=Moradon, 1=Karus, 2=Elmorad, 11=Eslant, 71=BDW area).
    pub zone_id: i16,
    /// Minimum character level to accept.
    pub min_level: i16,
    /// Maximum character level to accept.
    pub max_level: i16,
    /// Replay cooldown in hours (0 = no replay).
    pub replay_time: i16,
    /// Random reward pool ID (0 = no random rewards).
    pub random_id: i16,
}

/// A row from the `user_daily_quest` table — per-character quest progress.
///
/// C++ Reference: `_DAILY_USERQUEST` struct.
///
/// | `status` | Meaning           |
/// |----------|-------------------|
/// | 0        | Time-wait         |
/// | 1        | Completed         |
/// | 2        | Ongoing           |
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDailyQuestRow {
    /// Character name (PK part 1).
    pub character_id: String,
    /// Daily quest definition ID (PK part 2).
    pub quest_id: i16,
    /// Current kill count progress.
    pub kill_count: i32,
    /// Quest status: 0=timewait, 1=comp, 2=ongoing.
    pub status: i16,
    /// Unix timestamp when the quest can be replayed (0 = no cooldown).
    pub replay_time: i32,
}

/// Daily quest time type enumeration.
///
/// C++ Reference: `enum class DailyQuesttimetype { repeat, time, single }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DailyQuestTimeType {
    /// Quest can be repeated indefinitely.
    Repeat = 0,
    /// Quest has a timed cooldown after completion.
    Time = 1,
    /// Quest can only be completed once.
    Single = 2,
}

/// Daily quest status enumeration.
///
/// C++ Reference: `enum class DailyQuestStatus { timewait, comp, ongoing }`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DailyQuestStatus {
    /// Waiting for cooldown timer to expire.
    TimeWait = 0,
    /// Quest completed (single-time quests).
    Completed = 1,
    /// Quest is active and in progress.
    Ongoing = 2,
}
