//! Forgotten Temple (Monster Challenge) event handler.
//!                `EventMainTimer.cpp`
//! ## Event Lifecycle
//! 1. **Start**: `ForgettenTempleStart()` -- activates event, opens join phase
//! 2. **Join Phase**: Players can enter zone 55 (ZONE_FORGOTTEN_TEMPLE)
//! 3. **Summon Phase**: After `summon_time` seconds, summoning begins.
//!    Stage advances automatically based on `ft_stages` time offsets.
//! 4. **Last Summon**: When no more stages exist, mark `is_last_summon`.
//! 5. **All Dead**: When `monster_count == 0` and `is_last_summon`, distribute rewards.
//! 6. **Waiting**: Wait `waiting_time` seconds, then finish.
//! 7. **Finish**: Kick all users to Moradon, reset state.
//! ## Special Monsters (Proto ID -> Skill on Death)
//! | Proto ID | Skill ID | Name              |
//! |----------|----------|-------------------|
//! | 9816     | 492059   | Forgotten Princess|
//! | 9817     | 492060   | Corrupted Priest  |
//! | 9818     | 492061   | Demon Sword Sultan|
//! | 9819     | 492060   | Scorpion King     |
//! | 9820     | 492062   | Shaitan           |

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, Ordering};

pub use crate::world::types::ZONE_FORGOTTEN_TEMPLE;

/// FT event local_id in the EVENT_REWARDS table.
pub const FT_REWARD_LOCAL_ID: i16 = 13;

/// Forgotten Temple event runtime state.
/// This struct is stored in `WorldState` behind a `tokio::sync::RwLock`.
/// The timer proc reads/writes this state every tick.
pub struct ForgettenTempleState {
    /// Whether the event is currently active.
    pub is_active: AtomicBool,
    /// Whether the join phase is open (players can enter the zone).
    pub is_join: AtomicBool,
    /// Whether monster summoning has started.
    pub is_summon: AtomicBool,
    /// Whether the initial summon check has been done.
    pub is_summon_check: AtomicBool,
    /// Whether all stages have been exhausted (no more summons).
    pub is_last_summon: AtomicBool,
    /// Whether the event is finished (rewards distributed, waiting to kick).
    pub is_finished: AtomicBool,
    /// Whether we are in the post-victory waiting phase.
    pub is_waiting: AtomicBool,
    /// Current stage number (1-based).
    pub stage: AtomicU16,
    /// Event type (matches ft_stages.event_type, typically 1).
    pub event_type: AtomicU16,
    /// Minimum player level to participate.
    pub min_level: AtomicU16,
    /// Maximum player level to participate.
    pub max_level: AtomicU16,
    /// Unix timestamp when the event started.
    pub start_time: AtomicU64,
    /// Unix timestamp when the event will expire (hard timeout).
    pub finish_time: AtomicU64,
    /// Unix timestamp for waiting phase expiry.
    pub waiting_time: AtomicU64,
    /// Unix timestamp of last summon wave.
    pub last_summon_time: AtomicU64,
    /// Number of alive FT monsters (decremented on death).
    pub monster_count: AtomicU32,
}

impl ForgettenTempleState {
    /// Create a new default (inactive) FT state.
    pub fn new() -> Self {
        Self {
            is_active: AtomicBool::new(false),
            is_join: AtomicBool::new(false),
            is_summon: AtomicBool::new(false),
            is_summon_check: AtomicBool::new(false),
            is_last_summon: AtomicBool::new(false),
            is_finished: AtomicBool::new(false),
            is_waiting: AtomicBool::new(false),
            stage: AtomicU16::new(1),
            event_type: AtomicU16::new(0),
            min_level: AtomicU16::new(0),
            max_level: AtomicU16::new(0),
            start_time: AtomicU64::new(0),
            finish_time: AtomicU64::new(0),
            waiting_time: AtomicU64::new(0),
            last_summon_time: AtomicU64::new(0),
            monster_count: AtomicU32::new(0),
        }
    }

    /// Reset the FT state (called on event end or manual close).
    ///
    pub fn reset(&self) {
        self.is_active.store(false, Ordering::Relaxed);
        self.is_join.store(false, Ordering::Relaxed);
        self.is_summon.store(false, Ordering::Relaxed);
        self.is_summon_check.store(false, Ordering::Relaxed);
        self.is_last_summon.store(false, Ordering::Relaxed);
        self.is_finished.store(false, Ordering::Relaxed);
        self.is_waiting.store(false, Ordering::Relaxed);
        self.stage.store(1, Ordering::Relaxed);
        self.event_type.store(0, Ordering::Relaxed);
        self.min_level.store(0, Ordering::Relaxed);
        self.max_level.store(0, Ordering::Relaxed);
        self.start_time.store(0, Ordering::Relaxed);
        self.finish_time.store(0, Ordering::Relaxed);
        self.waiting_time.store(0, Ordering::Relaxed);
        self.last_summon_time.store(0, Ordering::Relaxed);
        self.monster_count.store(0, Ordering::Relaxed);
    }
}

impl Default for ForgettenTempleState {
    fn default() -> Self {
        Self::new()
    }
}

/// FT event timer options loaded from DB.
#[derive(Debug, Clone, Default)]
pub struct FtTimerOptions {
    /// Total playing time in minutes.
    pub playing_time: u16,
    /// Seconds after event start before summoning begins.
    pub summon_time: u16,
    /// Minimum spawn interval in seconds (not used in timer logic directly).
    pub spawn_min_time: u16,
    /// Seconds to wait after victory before kicking players.
    pub waiting_time: u16,
    /// Minimum player level.
    pub min_level: u8,
    /// Maximum player level.
    pub max_level: u8,
}

/// Retrieve the current stage definition for the FT event.
/// Returns the stage row matching the current event type and stage number,
/// or `None` if no matching stage exists (signals end of stages).
pub fn get_current_stage(
    stages: &[FtStageEntry],
    event_type: u16,
    current_stage: u16,
) -> Option<&FtStageEntry> {
    stages
        .iter()
        .find(|s| s.event_type == event_type as i16 && s.stage == current_stage as i16)
}

/// Load spawn list for the current stage.
/// Returns all summon entries matching the current event type and stage.
pub fn load_stage_spawns(
    summons: &[FtSummonEntry],
    event_type: u16,
    current_stage: u16,
) -> Vec<&FtSummonEntry> {
    summons
        .iter()
        .filter(|s| s.event_type == event_type as i16 && s.stage == current_stage as i16)
        .collect()
}

/// Count total monsters that will be spawned for a given stage.
/// Sums up `sid_count` for all summon entries in the stage.
pub fn count_stage_monsters(summons: &[FtSummonEntry], event_type: u16, current_stage: u16) -> u32 {
    summons
        .iter()
        .filter(|s| s.event_type == event_type as i16 && s.stage == current_stage as i16)
        .map(|s| s.sid_count as u32)
        .sum()
}

/// Get the death-skill mapping for special FT monsters.
/// Returns `Some(skill_id)` if the proto_id is a special FT boss that casts
/// a skill on death, `None` otherwise.
pub fn get_death_skill(proto_id: u16) -> Option<u32> {
    match proto_id {
        9816 => Some(492059), // Forgotten Princess
        9817 => Some(492060), // Corrupted Priest
        9818 => Some(492061), // Demon Sword Sultan
        9819 => Some(492060), // Scorpion King
        9820 => Some(492062), // Shaitan
        _ => None,
    }
}

/// Process FT monster death.
/// Decrements the monster count. Returns the skill ID to cast if any.
pub fn on_monster_dead(state: &ForgettenTempleState, proto_id: u16, zone_id: u16) -> Option<u32> {
    if !state.is_active.load(Ordering::Relaxed) || zone_id != ZONE_FORGOTTEN_TEMPLE {
        return None;
    }

    let count = state.monster_count.load(Ordering::Relaxed);
    if count > 0 {
        state.monster_count.fetch_sub(1, Ordering::Relaxed);
    }

    get_death_skill(proto_id)
}

/// Result of a single timer tick.
#[derive(Debug, PartialEq, Eq)]
pub enum FtTickResult {
    /// No action needed.
    Idle,
    /// The summon phase just started. Announce to all players.
    SummonPhaseStarted,
    /// A new wave of monsters should be spawned for the given stage.
    SpawnStage(u16),
    /// All stages exhausted, no more summons.
    LastSummonReached,
    /// All monsters dead after last summon. Distribute rewards.
    Victory,
    /// Event finished (timeout or waiting phase over). Kick players.
    Finish,
}

/// Run one tick of the FT timer.
/// This should be called periodically (e.g. every second) while the event is active.
/// It returns a `FtTickResult` indicating what action the caller should take.
pub fn timer_tick(
    state: &ForgettenTempleState,
    options: &FtTimerOptions,
    stages: &[FtStageEntry],
    now: u64,
) -> FtTickResult {
    if !state.is_active.load(Ordering::Relaxed) {
        return FtTickResult::Idle;
    }

    // Check: finished + waiting phase expired -> kick
    if state.is_finished.load(Ordering::Relaxed) && state.is_waiting.load(Ordering::Relaxed) {
        let wt = state.waiting_time.load(Ordering::Relaxed);
        if now > wt {
            return FtTickResult::Finish;
        }
        return FtTickResult::Idle;
    }

    // Check: already finished but not yet in waiting expiry
    if state.is_finished.load(Ordering::Relaxed) {
        return FtTickResult::Idle;
    }

    // Check: hard timeout
    let ft = state.finish_time.load(Ordering::Relaxed);
    if ft > 0 && now > ft {
        return FtTickResult::Finish;
    }

    let st = state.start_time.load(Ordering::Relaxed);

    // Check: summon phase start
    if !state.is_summon_check.load(Ordering::Relaxed) {
        let summon_start = st + options.summon_time as u64;
        if now > summon_start {
            state.is_summon_check.store(true, Ordering::Relaxed);
            state.stage.store(1, Ordering::Relaxed);
            state.last_summon_time.store(now + 30, Ordering::Relaxed);
            state.is_join.store(false, Ordering::Relaxed);
            state.is_summon.store(true, Ordering::Relaxed);
            return FtTickResult::SummonPhaseStarted;
        }
        return FtTickResult::Idle;
    }

    // Check: summon waves
    if state.is_summon.load(Ordering::Relaxed) && !state.is_last_summon.load(Ordering::Relaxed) {
        let current_stage = state.stage.load(Ordering::Relaxed);
        let et = state.event_type.load(Ordering::Relaxed);

        if let Some(stage_def) = get_current_stage(stages, et, current_stage) {
            let trigger_time = st + stage_def.time_offset as u64 + options.summon_time as u64;
            if now > trigger_time {
                let spawns_exist = stages
                    .iter()
                    .any(|s| s.event_type == et as i16 && s.stage == current_stage as i16);

                if spawns_exist {
                    let result_stage = current_stage;
                    state.stage.fetch_add(1, Ordering::Relaxed);
                    state.last_summon_time.store(now, Ordering::Relaxed);
                    return FtTickResult::SpawnStage(result_stage);
                }

                // No spawns for this stage -> last summon
                state.is_last_summon.store(true, Ordering::Relaxed);
                state.is_summon.store(false, Ordering::Relaxed);
                return FtTickResult::LastSummonReached;
            }
        } else {
            // No stage definition found -> all stages done
            state.is_last_summon.store(true, Ordering::Relaxed);
            state.is_summon.store(false, Ordering::Relaxed);
            return FtTickResult::LastSummonReached;
        }
    }

    // Check: all monsters dead after last summon -> victory
    if state.is_last_summon.load(Ordering::Relaxed)
        && !state.is_summon.load(Ordering::Relaxed)
        && state.monster_count.load(Ordering::Relaxed) == 0
    {
        state
            .waiting_time
            .store(now + options.waiting_time as u64, Ordering::Relaxed);
        state.is_waiting.store(true, Ordering::Relaxed);
        state.is_finished.store(true, Ordering::Relaxed);
        return FtTickResult::Victory;
    }

    FtTickResult::Idle
}

/// Start the Forgotten Temple event.
pub fn start_event(
    state: &ForgettenTempleState,
    options: &FtTimerOptions,
    event_type: u16,
    now: u64,
) -> bool {
    if state.is_active.load(Ordering::Relaxed) {
        return false;
    }

    if options.playing_time == 0 || options.summon_time == 0 {
        return false;
    }

    state.reset();
    state.is_active.store(true, Ordering::Relaxed);
    state.is_join.store(true, Ordering::Relaxed);
    state
        .min_level
        .store(options.min_level as u16, Ordering::Relaxed);
    state
        .max_level
        .store(options.max_level as u16, Ordering::Relaxed);
    state.stage.store(1, Ordering::Relaxed);
    state.event_type.store(event_type, Ordering::Relaxed);
    state.start_time.store(now, Ordering::Relaxed);
    state
        .finish_time
        .store(now + (options.playing_time as u64 * 60), Ordering::Relaxed);

    true
}

/// Stop the Forgotten Temple event and reset state.
pub fn finish_event(state: &ForgettenTempleState) {
    state.is_active.store(false, Ordering::Relaxed);
    // Caller is responsible for kicking users and removing NPCs.
    state.reset();
}

/// Check if a player's level is within the FT participation range.
pub fn can_player_join(state: &ForgettenTempleState, player_level: u16) -> bool {
    if !state.is_active.load(Ordering::Relaxed) || !state.is_join.load(Ordering::Relaxed) {
        return false;
    }
    let min = state.min_level.load(Ordering::Relaxed);
    let max = state.max_level.load(Ordering::Relaxed);
    player_level >= min && player_level <= max
}

/// In-memory stage entry (loaded from DB at startup).
#[derive(Debug, Clone)]
pub struct FtStageEntry {
    /// Primary key index.
    pub n_index: i32,
    /// Event type.
    pub event_type: i16,
    /// Stage number.
    pub stage: i16,
    /// Time offset in seconds from summon start.
    pub time_offset: i16,
}

/// In-memory summon entry (loaded from DB at startup).
#[derive(Debug, Clone)]
pub struct FtSummonEntry {
    /// Row index.
    pub b_index: i32,
    /// Event type.
    pub event_type: i16,
    /// Stage number.
    pub stage: i16,
    /// NPC template ID.
    pub sid_id: i16,
    /// Spawn count.
    pub sid_count: i16,
    /// X position.
    pub pos_x: i16,
    /// Z position.
    pub pos_z: i16,
    /// Spawn range.
    pub spawn_range: i16,
    /// Monster name.
    pub summon_name: String,
}

/// Collect reward item tuples from a reward row.
/// Returns up to 3 (item_id, count, expiration) tuples for non-zero items.
pub fn collect_reward_items(reward: &EventRewardEntry) -> Vec<(i32, i32, i32)> {
    let mut items = Vec::new();
    if reward.itemid1 != 0 {
        items.push((reward.itemid1, reward.itemcount1, reward.itemexpiration1));
    }
    if reward.itemid2 != 0 {
        items.push((reward.itemid2, reward.itemcount2, reward.itemexpiration2));
    }
    if reward.itemid3 != 0 {
        items.push((reward.itemid3, reward.itemcount3, reward.itemexpiration3));
    }
    items
}

/// In-memory event reward entry.
#[derive(Debug, Clone)]
pub struct EventRewardEntry {
    /// Event identifier (13 for FT).
    pub local_id: i16,
    /// Whether this reward is enabled.
    pub status: bool,
    /// Item rewards (up to 3).
    pub itemid1: i32,
    pub itemcount1: i32,
    pub itemexpiration1: i32,
    pub itemid2: i32,
    pub itemcount2: i32,
    pub itemexpiration2: i32,
    pub itemid3: i32,
    pub itemcount3: i32,
    pub itemexpiration3: i32,
    /// Experience reward.
    pub experience: i32,
    /// Loyalty reward.
    pub loyalty: i32,
    /// Cash reward.
    pub cash: i32,
    /// Noah (gold) reward.
    pub noah: i32,
}

/// Get FT-specific rewards from the full event rewards list.
pub fn get_ft_rewards(rewards: &[EventRewardEntry]) -> Vec<&EventRewardEntry> {
    rewards
        .iter()
        .filter(|r| r.local_id == FT_REWARD_LOCAL_ID && r.status)
        .collect()
}

/// Build a lookup of stages grouped by (event_type, stage) for quick access.
pub fn build_stage_map(stages: &[FtStageEntry]) -> HashMap<(i16, i16), &FtStageEntry> {
    let mut map = HashMap::new();
    for s in stages {
        map.entry((s.event_type, s.stage)).or_insert(s);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_options() -> FtTimerOptions {
        FtTimerOptions {
            playing_time: 30,
            summon_time: 300,
            spawn_min_time: 10,
            waiting_time: 20,
            min_level: 60,
            max_level: 83,
        }
    }

    fn make_stages() -> Vec<FtStageEntry> {
        vec![
            FtStageEntry {
                n_index: 1,
                event_type: 1,
                stage: 1,
                time_offset: 0,
            },
            FtStageEntry {
                n_index: 2,
                event_type: 1,
                stage: 2,
                time_offset: 5,
            },
            FtStageEntry {
                n_index: 3,
                event_type: 1,
                stage: 3,
                time_offset: 12,
            },
        ]
    }

    fn make_summons() -> Vec<FtSummonEntry> {
        vec![
            FtSummonEntry {
                b_index: 1,
                event_type: 1,
                stage: 1,
                sid_id: 2903,
                sid_count: 5,
                pos_x: 108,
                pos_z: 156,
                spawn_range: 10,
                summon_name: "Pooka".to_string(),
            },
            FtSummonEntry {
                b_index: 2,
                event_type: 1,
                stage: 1,
                sid_id: 2903,
                sid_count: 5,
                pos_x: 98,
                pos_z: 122,
                spawn_range: 10,
                summon_name: "Pooka".to_string(),
            },
            FtSummonEntry {
                b_index: 12,
                event_type: 1,
                stage: 2,
                sid_id: 9816,
                sid_count: 1,
                pos_x: 128,
                pos_z: 127,
                spawn_range: 20,
                summon_name: "Forgotten Princess".to_string(),
            },
        ]
    }

    #[test]
    fn test_state_new_is_inactive() {
        let state = ForgettenTempleState::new();
        assert!(!state.is_active.load(Ordering::Relaxed));
        assert!(!state.is_join.load(Ordering::Relaxed));
        assert_eq!(state.stage.load(Ordering::Relaxed), 1);
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_state_reset() {
        let state = ForgettenTempleState::new();
        state.is_active.store(true, Ordering::Relaxed);
        state.stage.store(10, Ordering::Relaxed);
        state.monster_count.store(50, Ordering::Relaxed);

        state.reset();

        assert!(!state.is_active.load(Ordering::Relaxed));
        assert_eq!(state.stage.load(Ordering::Relaxed), 1);
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_start_event_success() {
        let state = ForgettenTempleState::new();
        let opts = make_options();

        let result = start_event(&state, &opts, 1, 1000);

        assert!(result);
        assert!(state.is_active.load(Ordering::Relaxed));
        assert!(state.is_join.load(Ordering::Relaxed));
        assert_eq!(state.event_type.load(Ordering::Relaxed), 1);
        assert_eq!(state.start_time.load(Ordering::Relaxed), 1000);
        assert_eq!(state.finish_time.load(Ordering::Relaxed), 1000 + 30 * 60);
        assert_eq!(state.min_level.load(Ordering::Relaxed), 60);
        assert_eq!(state.max_level.load(Ordering::Relaxed), 83);
    }

    #[test]
    fn test_start_event_already_active() {
        let state = ForgettenTempleState::new();
        let opts = make_options();

        start_event(&state, &opts, 1, 1000);
        let result = start_event(&state, &opts, 1, 2000);

        assert!(!result);
    }

    #[test]
    fn test_start_event_zero_playing_time() {
        let state = ForgettenTempleState::new();
        let mut opts = make_options();
        opts.playing_time = 0;

        assert!(!start_event(&state, &opts, 1, 1000));
    }

    #[test]
    fn test_finish_event() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        start_event(&state, &opts, 1, 1000);

        finish_event(&state);

        assert!(!state.is_active.load(Ordering::Relaxed));
        assert_eq!(state.stage.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_can_player_join() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        start_event(&state, &opts, 1, 1000);

        assert!(can_player_join(&state, 60));
        assert!(can_player_join(&state, 70));
        assert!(can_player_join(&state, 83));
        assert!(!can_player_join(&state, 59));
        assert!(!can_player_join(&state, 84));
    }

    #[test]
    fn test_can_player_join_not_active() {
        let state = ForgettenTempleState::new();
        assert!(!can_player_join(&state, 70));
    }

    #[test]
    fn test_can_player_join_not_in_join_phase() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        start_event(&state, &opts, 1, 1000);
        state.is_join.store(false, Ordering::Relaxed);

        assert!(!can_player_join(&state, 70));
    }

    #[test]
    fn test_get_current_stage() {
        let stages = make_stages();
        let result = get_current_stage(&stages, 1, 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().time_offset, 0);

        let result = get_current_stage(&stages, 1, 2);
        assert!(result.is_some());
        assert_eq!(result.unwrap().time_offset, 5);

        let result = get_current_stage(&stages, 1, 99);
        assert!(result.is_none());
    }

    #[test]
    fn test_load_stage_spawns() {
        let summons = make_summons();
        let spawns = load_stage_spawns(&summons, 1, 1);
        assert_eq!(spawns.len(), 2);

        let spawns = load_stage_spawns(&summons, 1, 2);
        assert_eq!(spawns.len(), 1);
        assert_eq!(spawns[0].sid_id, 9816);

        let spawns = load_stage_spawns(&summons, 1, 99);
        assert_eq!(spawns.len(), 0);
    }

    #[test]
    fn test_count_stage_monsters() {
        let summons = make_summons();
        assert_eq!(count_stage_monsters(&summons, 1, 1), 10); // 5 + 5
        assert_eq!(count_stage_monsters(&summons, 1, 2), 1); // 1 princess
        assert_eq!(count_stage_monsters(&summons, 1, 99), 0);
    }

    #[test]
    fn test_death_skill_mapping() {
        assert_eq!(get_death_skill(9816), Some(492059));
        assert_eq!(get_death_skill(9817), Some(492060));
        assert_eq!(get_death_skill(9818), Some(492061));
        assert_eq!(get_death_skill(9819), Some(492060));
        assert_eq!(get_death_skill(9820), Some(492062));
        assert_eq!(get_death_skill(1234), None);
    }

    #[test]
    fn test_on_monster_dead_decrements() {
        let state = ForgettenTempleState::new();
        state.is_active.store(true, Ordering::Relaxed);
        state.monster_count.store(5, Ordering::Relaxed);

        let skill = on_monster_dead(&state, 1234, ZONE_FORGOTTEN_TEMPLE);
        assert_eq!(skill, None);
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn test_on_monster_dead_special_skill() {
        let state = ForgettenTempleState::new();
        state.is_active.store(true, Ordering::Relaxed);
        state.monster_count.store(3, Ordering::Relaxed);

        let skill = on_monster_dead(&state, 9816, ZONE_FORGOTTEN_TEMPLE);
        assert_eq!(skill, Some(492059));
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_on_monster_dead_not_active() {
        let state = ForgettenTempleState::new();
        state.monster_count.store(5, Ordering::Relaxed);

        let skill = on_monster_dead(&state, 1234, ZONE_FORGOTTEN_TEMPLE);
        assert_eq!(skill, None);
        // Count not decremented when event is not active
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_on_monster_dead_wrong_zone() {
        let state = ForgettenTempleState::new();
        state.is_active.store(true, Ordering::Relaxed);
        state.monster_count.store(5, Ordering::Relaxed);

        let skill = on_monster_dead(&state, 1234, 21); // Moradon, not FT zone
        assert_eq!(skill, None);
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn test_on_monster_dead_zero_count() {
        let state = ForgettenTempleState::new();
        state.is_active.store(true, Ordering::Relaxed);
        state.monster_count.store(0, Ordering::Relaxed);

        let skill = on_monster_dead(&state, 1234, ZONE_FORGOTTEN_TEMPLE);
        assert_eq!(skill, None);
        assert_eq!(state.monster_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_timer_idle_when_not_active() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        assert_eq!(timer_tick(&state, &opts, &stages, 5000), FtTickResult::Idle);
    }

    #[test]
    fn test_timer_summon_phase_start() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);

        // Before summon time: idle
        assert_eq!(timer_tick(&state, &opts, &stages, 1200), FtTickResult::Idle);

        // After summon time (1000 + 300 = 1300): summon starts
        assert_eq!(
            timer_tick(&state, &opts, &stages, 1301),
            FtTickResult::SummonPhaseStarted
        );
        assert!(state.is_summon_check.load(Ordering::Relaxed));
        assert!(state.is_summon.load(Ordering::Relaxed));
        assert!(!state.is_join.load(Ordering::Relaxed));
    }

    #[test]
    fn test_timer_spawn_stage() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);

        // Trigger summon phase
        timer_tick(&state, &opts, &stages, 1301);

        // Stage 1: time_offset=0, so trigger at start_time + summon_time + 0 = 1300
        // Already past 1301, so stage 1 should spawn
        let result = timer_tick(&state, &opts, &stages, 1302);
        assert_eq!(result, FtTickResult::SpawnStage(1));
        assert_eq!(state.stage.load(Ordering::Relaxed), 2);

        // Stage 2: time_offset=5, trigger at 1000 + 300 + 5 = 1305
        assert_eq!(timer_tick(&state, &opts, &stages, 1304), FtTickResult::Idle);
        let result = timer_tick(&state, &opts, &stages, 1306);
        assert_eq!(result, FtTickResult::SpawnStage(2));
        assert_eq!(state.stage.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_timer_last_summon_no_more_stages() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages(); // stages 1,2,3

        start_event(&state, &opts, 1, 1000);
        timer_tick(&state, &opts, &stages, 1301); // summon start

        // Advance through all stages
        timer_tick(&state, &opts, &stages, 1301); // stage 1
        timer_tick(&state, &opts, &stages, 1306); // stage 2
        timer_tick(&state, &opts, &stages, 1313); // stage 3

        // Stage 4 does not exist -> LastSummonReached
        let result = timer_tick(&state, &opts, &stages, 1400);
        assert_eq!(result, FtTickResult::LastSummonReached);
        assert!(state.is_last_summon.load(Ordering::Relaxed));
        assert!(!state.is_summon.load(Ordering::Relaxed));
    }

    #[test]
    fn test_timer_victory() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);

        // Force to last summon state
        state.is_summon_check.store(true, Ordering::Relaxed);
        state.is_last_summon.store(true, Ordering::Relaxed);
        state.is_summon.store(false, Ordering::Relaxed);
        state.monster_count.store(0, Ordering::Relaxed);

        let result = timer_tick(&state, &opts, &stages, 1500);
        assert_eq!(result, FtTickResult::Victory);
        assert!(state.is_finished.load(Ordering::Relaxed));
        assert!(state.is_waiting.load(Ordering::Relaxed));
        assert_eq!(state.waiting_time.load(Ordering::Relaxed), 1500 + 20);
    }

    #[test]
    fn test_timer_finish_after_waiting() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);

        // Set up finished + waiting
        state.is_summon_check.store(true, Ordering::Relaxed);
        state.is_finished.store(true, Ordering::Relaxed);
        state.is_waiting.store(true, Ordering::Relaxed);
        state.waiting_time.store(1520, Ordering::Relaxed);

        // Before waiting expires
        assert_eq!(timer_tick(&state, &opts, &stages, 1519), FtTickResult::Idle);

        // After waiting expires
        assert_eq!(
            timer_tick(&state, &opts, &stages, 1521),
            FtTickResult::Finish
        );
    }

    #[test]
    fn test_timer_hard_timeout() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);
        state.is_summon_check.store(true, Ordering::Relaxed);

        // finish_time = 1000 + 30*60 = 2800
        let result = timer_tick(&state, &opts, &stages, 2801);
        assert_eq!(result, FtTickResult::Finish);
    }

    #[test]
    fn test_reward_items_collection() {
        let reward = EventRewardEntry {
            local_id: 13,
            status: true,
            itemid1: 100,
            itemcount1: 5,
            itemexpiration1: 0,
            itemid2: 200,
            itemcount2: 3,
            itemexpiration2: 3600,
            itemid3: 0,
            itemcount3: 0,
            itemexpiration3: 0,
            experience: 1000,
            loyalty: 50,
            cash: 0,
            noah: 500,
        };

        let items = collect_reward_items(&reward);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], (100, 5, 0));
        assert_eq!(items[1], (200, 3, 3600));
    }

    #[test]
    fn test_get_ft_rewards_filter() {
        let rewards = vec![
            EventRewardEntry {
                local_id: 9,
                status: true,
                itemid1: 0,
                itemcount1: 0,
                itemexpiration1: 0,
                itemid2: 0,
                itemcount2: 0,
                itemexpiration2: 0,
                itemid3: 0,
                itemcount3: 0,
                itemexpiration3: 0,
                experience: 100,
                loyalty: 0,
                cash: 0,
                noah: 0,
            },
            EventRewardEntry {
                local_id: 13,
                status: true,
                itemid1: 500,
                itemcount1: 1,
                itemexpiration1: 0,
                itemid2: 0,
                itemcount2: 0,
                itemexpiration2: 0,
                itemid3: 0,
                itemcount3: 0,
                itemexpiration3: 0,
                experience: 200,
                loyalty: 10,
                cash: 0,
                noah: 100,
            },
            EventRewardEntry {
                local_id: 13,
                status: false,
                itemid1: 0,
                itemcount1: 0,
                itemexpiration1: 0,
                itemid2: 0,
                itemcount2: 0,
                itemexpiration2: 0,
                itemid3: 0,
                itemcount3: 0,
                itemexpiration3: 0,
                experience: 0,
                loyalty: 0,
                cash: 0,
                noah: 0,
            },
        ];

        let ft_rewards = get_ft_rewards(&rewards);
        assert_eq!(ft_rewards.len(), 1);
        assert_eq!(ft_rewards[0].local_id, 13);
        assert_eq!(ft_rewards[0].experience, 200);
    }

    #[test]
    fn test_build_stage_map() {
        let stages = make_stages();
        let map = build_stage_map(&stages);

        assert_eq!(map.len(), 3);
        assert!(map.contains_key(&(1, 1)));
        assert!(map.contains_key(&(1, 2)));
        assert!(map.contains_key(&(1, 3)));
        assert!(!map.contains_key(&(1, 4)));
    }

    #[test]
    fn test_timer_monsters_alive_blocks_victory() {
        let state = ForgettenTempleState::new();
        let opts = make_options();
        let stages = make_stages();

        start_event(&state, &opts, 1, 1000);
        state.is_summon_check.store(true, Ordering::Relaxed);
        state.is_last_summon.store(true, Ordering::Relaxed);
        state.is_summon.store(false, Ordering::Relaxed);
        state.monster_count.store(5, Ordering::Relaxed); // monsters still alive

        assert_eq!(timer_tick(&state, &opts, &stages, 1500), FtTickResult::Idle);

        // Kill all monsters
        state.monster_count.store(0, Ordering::Relaxed);
        assert_eq!(
            timer_tick(&state, &opts, &stages, 1501),
            FtTickResult::Victory
        );
    }

    #[test]
    fn test_default_trait() {
        let state = ForgettenTempleState::default();
        assert!(!state.is_active.load(Ordering::Relaxed));
    }

    #[test]
    fn test_ft_timer_options_default() {
        let opts = FtTimerOptions::default();
        assert_eq!(opts.playing_time, 0);
        assert_eq!(opts.summon_time, 0);
    }
}
