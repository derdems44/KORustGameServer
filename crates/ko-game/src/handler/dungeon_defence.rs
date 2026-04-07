//! Dungeon Defence (Full Moon Rift) instance handler.
//! ## Instance Lifecycle
//! 1. **Sign**: Party leader initiates entry via `DungeonDefenceSign()`.
//!    Validates party (all in Moradon, alive, have rift voucher), determines difficulty.
//! 2. **Room Assign**: Finds a free DD room, sets initial state, teleports party in.
//! 3. **Guardian Spawn**: 6 guardian NPCs (31737-31740) are spawned in the room.
//! 4. **Initial Timer**: 60s countdown before first monster wave.
//! 5. **Stage Loop**: `DungeonDefenceTimer()` ticks every 1s, spawning monster waves.
//!    After all monsters in a stage are killed, `ChangeDungeonDefenceStage()` advances.
//! 6. **Finish**: After the final stage boss dies, 30s finish timer starts.
//! 7. **Kick**: All players teleported back to Moradon, room reset.
//! ## Difficulty Levels
//! | Difficulty | Party Size | Stages    | Stage IDs |
//! |------------|------------|-----------|-----------|
//! | Easy (1)   | 2-3        | 5 stages  | 1-5       |
//! | Normal (2) | 4-7        | 12 stages | 6-17      |
//! | Hard (3)   | 8+         | 18 stages | 18-35     |
//! ## Rewards (per monster kill)
//! - **Full Moon Rift Jewel**: 1 for stages 1-17, 2 for stages 18-35 (killer only)
//! - **Monster Coin**: 2 for killer, 1 for each other party member
//! - **Lunar Order Token**: 1 per party member when boss dies (9931, 9936, 9951, 9959, 9971)

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicU16, AtomicU32, Ordering};

use ko_db::models::dungeon_defence::{DfMonsterRow, DfStageRow};

// ── Zone & Item Constants ──────────────────────────────────────────────

pub use crate::world::types::ZONE_DUNGEON_DEFENCE;

/// Full Moon Rift Voucher item (required to enter).
pub const DUNGEON_DEFENCE_RIFT_ITEM: u32 = 914057000;

/// Monster Coin item (reward per kill).
pub const MONSTER_COIN_ITEM: u32 = 914058000;

/// Full Moon Rift Jewel item (reward per kill, 1 or 2 depending on stage).
pub const MONSTER_RIFT_JEWEL: u32 = 914069000;

/// Lunar Order Token item (boss-kill reward for whole party).
pub const LUNAR_ORDER_TOKEN: u32 = 810977000;

/// Maximum number of DD rooms available.
pub const DD_MAX_ROOMS: u16 = 60;

/// Initial spawn timer (seconds before first wave).
pub const DD_INITIAL_SPAWN_TIME: u32 = 60;

/// Room close timer (seconds before forced eviction).
pub const DD_ROOM_CLOSE_TIME: u32 = 7200;

/// Finish timer (seconds after final stage before kick).
pub const DD_FINISH_TIME: u16 = 30;

/// WIZ_EVENT sub-opcode for dungeon defence sign.
pub const TEMPLE_EVENT_DUNGEON_SIGN: u8 = 58;

/// WIZ_EVENT sub-opcode for stage counter display.
pub const TEMPLE_EVENT_STAGE_COUNTER: u8 = 60;

// ── Guardian NPC Templates ─────────────────────────────────────────────

/// Guardian NPC spawn definitions.
/// Format: (npc_id, pos_x, pos_z)
pub const GUARDIAN_NPCS: [(u16, i16, i16); 6] = [
    (31737, 199, 200),
    (31737, 52, 124),
    (31737, 211, 38),
    (31740, 167, 212),
    (31739, 65, 134),
    (31738, 196, 54),
];

// ── Boss Monster IDs ───────────────────────────────────────────────────

/// Boss monster proto IDs that grant Lunar Order Token on death.
pub const BOSS_MONSTER_IDS: [u16; 5] = [9931, 9951, 9936, 9959, 9971];

/// All valid DD monster proto IDs (for reward eligibility).
pub const DD_MONSTER_IDS: [u16; 26] = [
    9927, 9928, 9929, 9930, 9931, 9932, 9933, 9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942,
    9947, 9948, 9949, 9950, 9951, 9955, 9956, 9957, 9958, 9959,
];

// ── Sign Result Codes ──────────────────────────────────────────────────

/// Error/result codes sent in the TEMPLE_EVENT_DUNGEON_SIGN response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DdSignResult {
    /// System error / instance generation failed.
    SystemError = 0,
    /// Generic failure (no room, already in event, etc.).
    Failed = 1,
    /// Party members not all in Moradon.
    NotInMoradon = 2,
    /// Another instance is being created.
    InstanceBusy = 3,
    /// A party member is dead.
    MemberDead = 4,
    /// Not in a party.
    NoParty = 5,
    /// Not a Full Moon Rift party.
    WrongPartyType = 6,
    /// A party member lacks the rift voucher.
    NoRiftItem = 7,
}

// ── Difficulty Enum ────────────────────────────────────────────────────

/// Dungeon Defence difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DdDifficulty {
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl DdDifficulty {
    /// Determine difficulty from party member count.
    ///
    pub fn from_party_size(count: u16) -> Option<Self> {
        match count {
            2..=3 => Some(Self::Easy),
            4..=7 => Some(Self::Normal),
            8.. => Some(Self::Hard),
            _ => None,
        }
    }

    /// Get the starting stage ID for this difficulty.
    ///
    pub fn starting_stage(&self) -> i16 {
        match self {
            Self::Easy => 1,
            Self::Normal => 6,
            Self::Hard => 18,
        }
    }

    /// Get the final stage ID for this difficulty.
    ///
    pub fn final_stage(&self) -> i16 {
        match self {
            Self::Easy => 5,
            Self::Normal => 17,
            Self::Hard => 35,
        }
    }

    /// Get the maximum number of stages in this difficulty.
    ///
    pub fn max_stages(&self) -> u8 {
        match self {
            Self::Easy => 5,
            Self::Normal => 12,
            Self::Hard => 18,
        }
    }

    /// Get the stage offset to compute the display stage number.
    ///
    ///   Easy: stage_id directly, Normal: stage_id - 5, Hard: stage_id - 17
    pub fn stage_offset(&self) -> i16 {
        match self {
            Self::Easy => 0,
            Self::Normal => 5,
            Self::Hard => 17,
        }
    }
}

// ── Room State ─────────────────────────────────────────────────────────

/// Runtime state for a single Dungeon Defence room instance.
pub struct DdRoomInfo {
    /// Room ID (1-based).
    pub room_id: AtomicU16,
    /// Whether this room is currently in use.
    pub is_started: AtomicBool,
    /// Current difficulty level.
    pub difficulty: AtomicU16,
    /// Current stage ID.
    pub stage_id: AtomicI16,
    /// Kill count tracking (for stage advancement).
    pub kill_count: AtomicU32,
    /// Seconds until next monster spawn.
    pub spawn_time: AtomicU32,
    /// Seconds until room forced close (2 hours).
    pub room_close: AtomicU32,
    /// Seconds until players are kicked after finishing.
    pub out_time: AtomicU16,
    /// Whether the initial spawn countdown is active.
    pub monster_beginner_spawned: AtomicBool,
    /// Whether monsters have been spawned and we're waiting for kills.
    pub monster_spawned: AtomicBool,
    /// Whether the event is finished (all stages cleared).
    pub is_finished: AtomicBool,
    /// User list in this room (character names).
    pub users: parking_lot::RwLock<HashMap<usize, String>>,
}

impl DdRoomInfo {
    /// Create a new empty room.
    pub fn new(room_id: u16) -> Self {
        Self {
            room_id: AtomicU16::new(room_id),
            is_started: AtomicBool::new(false),
            difficulty: AtomicU16::new(0),
            stage_id: AtomicI16::new(0),
            kill_count: AtomicU32::new(0),
            spawn_time: AtomicU32::new(DD_INITIAL_SPAWN_TIME),
            room_close: AtomicU32::new(DD_ROOM_CLOSE_TIME),
            out_time: AtomicU16::new(0),
            monster_beginner_spawned: AtomicBool::new(false),
            monster_spawned: AtomicBool::new(false),
            is_finished: AtomicBool::new(false),
            users: parking_lot::RwLock::new(HashMap::new()),
        }
    }

    /// Reset the room to idle state.
    ///
    pub fn reset(&self) {
        self.kill_count.store(0, Ordering::Relaxed);
        self.spawn_time
            .store(DD_INITIAL_SPAWN_TIME, Ordering::Relaxed);
        self.room_close.store(DD_ROOM_CLOSE_TIME, Ordering::Relaxed);
        self.out_time.store(0, Ordering::Relaxed);
        self.is_started.store(false, Ordering::Relaxed);
        self.monster_beginner_spawned
            .store(false, Ordering::Relaxed);
        self.monster_spawned.store(false, Ordering::Relaxed);
        self.is_finished.store(false, Ordering::Relaxed);
        self.stage_id.store(0, Ordering::Relaxed);
        self.difficulty.store(0, Ordering::Relaxed);
        self.users.write().clear();
    }

    /// Atomically claim and initialize a room for a new DD run.
    ///
    /// Uses CAS on `is_started` to prevent TOCTOU races when multiple
    /// parties try to claim rooms concurrently.
    ///
    ///
    /// Returns `true` if the room was successfully claimed and initialized,
    /// `false` if it was already started by another party.
    pub fn try_claim_and_start(&self, difficulty: DdDifficulty) -> bool {
        // Atomically flip is_started from false → true
        if self
            .is_started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return false;
        }
        self.kill_count.store(0, Ordering::Relaxed);
        self.spawn_time
            .store(DD_INITIAL_SPAWN_TIME, Ordering::Relaxed);
        self.room_close.store(DD_ROOM_CLOSE_TIME, Ordering::Relaxed);
        self.out_time.store(0, Ordering::Relaxed);
        self.monster_beginner_spawned.store(true, Ordering::Relaxed);
        self.monster_spawned.store(false, Ordering::Relaxed);
        self.is_finished.store(false, Ordering::Relaxed);
        self.difficulty.store(difficulty as u16, Ordering::Relaxed);
        self.stage_id
            .store(difficulty.starting_stage(), Ordering::Relaxed);
        true
    }
}

/// Atomically find and claim a free DD room, initializing it for the given difficulty.
/// Combines the find + start into a single atomic operation to prevent
/// TOCTOU races where two parties could claim the same room.
pub fn try_claim_free_room(rooms: &[DdRoomInfo], difficulty: DdDifficulty) -> Option<u16> {
    for room in rooms {
        if room.try_claim_and_start(difficulty) {
            return Some(room.room_id.load(Ordering::Relaxed));
        }
    }
    None
}

// ── Timer Tick Result ──────────────────────────────────────────────────

/// Result of a single DD timer tick for a room.
#[derive(Debug, PartialEq, Eq)]
pub enum DdTickResult {
    /// No action needed.
    Idle,
    /// Spawn monsters for the current stage and send stage counter.
    SpawnStage {
        /// Stage ID to spawn monsters for.
        stage_id: i16,
        /// Difficulty for display packet.
        difficulty: u16,
    },
    /// Room time expired or finish timer expired — kick all players.
    KickAll,
}

/// Run one tick of the DD timer for a room.
/// Called every 1 second for each active room.
pub fn timer_tick(room: &DdRoomInfo) -> DdTickResult {
    if !room.is_started.load(Ordering::Relaxed) {
        return DdTickResult::Idle;
    }

    // Decrement spawn timer
    let spawn_time = room.spawn_time.load(Ordering::Relaxed);
    if spawn_time > 0 {
        room.spawn_time.store(spawn_time - 1, Ordering::Relaxed);
    }

    // Check: spawn timer expired and we need to spawn
    let spawn_time = room.spawn_time.load(Ordering::Relaxed);
    if spawn_time == 0
        && (room.monster_beginner_spawned.load(Ordering::Relaxed)
            || room.monster_spawned.load(Ordering::Relaxed))
    {
        // Clear beginner flag after first spawn
        if room.monster_beginner_spawned.load(Ordering::Relaxed) {
            room.monster_beginner_spawned
                .store(false, Ordering::Relaxed);
        }
        room.monster_spawned.store(false, Ordering::Relaxed);

        let difficulty = room.difficulty.load(Ordering::Relaxed);
        let stage_id = room.stage_id.load(Ordering::Relaxed);
        return DdTickResult::SpawnStage {
            stage_id,
            difficulty,
        };
    }

    // Decrement room close timer
    let close = room.room_close.load(Ordering::Relaxed);
    if close > 0 {
        room.room_close.store(close - 1, Ordering::Relaxed);
    }

    // Decrement out timer
    let out_time = room.out_time.load(Ordering::Relaxed);
    if out_time > 0 {
        room.out_time.store(out_time - 1, Ordering::Relaxed);
    }

    // Check: room close expired
    let close = room.room_close.load(Ordering::Relaxed);
    if close == 0 && room.is_started.load(Ordering::Relaxed) {
        room.is_started.store(false, Ordering::Relaxed);
        return DdTickResult::KickAll;
    }

    // Check: finish out timer expired
    let out_time = room.out_time.load(Ordering::Relaxed);
    if out_time == 0 && room.is_finished.load(Ordering::Relaxed) {
        room.is_finished.store(false, Ordering::Relaxed);
        return DdTickResult::KickAll;
    }

    DdTickResult::Idle
}

// ── Stage Advancement ──────────────────────────────────────────────────

/// Result of advancing to the next stage.
#[derive(Debug, PartialEq, Eq)]
pub enum StageAdvanceResult {
    /// Advanced to the next stage successfully. Spawn timer set.
    NextStage {
        /// The new stage ID.
        new_stage_id: i16,
        /// Spawn delay in seconds.
        spawn_delay: u32,
    },
    /// All stages cleared — trigger finish timer.
    Finished,
    /// Error: could not find next stage data.
    Error,
}

/// Advance to the next stage after all monsters in the current stage are killed.
pub fn advance_stage(room: &DdRoomInfo, stages: &[DfStageRow]) -> StageAdvanceResult {
    let difficulty = room.difficulty.load(Ordering::Relaxed);
    let current_stage = room.stage_id.load(Ordering::Relaxed);

    let dd_difficulty = match difficulty {
        1 => DdDifficulty::Easy,
        2 => DdDifficulty::Normal,
        3 => DdDifficulty::Hard,
        _ => return StageAdvanceResult::Error,
    };

    // Check if current stage is the final stage for this difficulty
    if current_stage == dd_difficulty.final_stage() {
        return StageAdvanceResult::Finished;
    }

    // Find the next stage ID from the stage list
    // C++ increments the stage_id and looks up in the monster list
    let next_stage = current_stage + 1;

    // Verify the stage exists in the stage table
    let stage_exists = stages
        .iter()
        .any(|s| s.difficulty == difficulty as i16 && s.stage_id == next_stage);

    if !stage_exists {
        // No more stages — finished
        return StageAdvanceResult::Finished;
    }

    // Set the new stage
    room.stage_id.store(next_stage, Ordering::Relaxed);
    room.monster_spawned.store(true, Ordering::Relaxed);

    // Determine spawn delay based on difficulty and stage
    let delay = get_spawn_delay(difficulty, next_stage);
    room.spawn_time.store(delay, Ordering::Relaxed);

    StageAdvanceResult::NextStage {
        new_stage_id: next_stage,
        spawn_delay: delay,
    }
}

/// Get the spawn delay for a given difficulty and stage.
pub fn get_spawn_delay(difficulty: u16, stage_id: i16) -> u32 {
    match difficulty {
        1 => {
            // Easy: stages 1-5 all get 10s
            if (1..=5).contains(&stage_id) {
                10
            } else {
                20
            }
        }
        2 => {
            // Normal: stages 6-10 = 10s, 11-17 = 20s, else 15s
            if (6..=10).contains(&stage_id) {
                10
            } else if (11..=17).contains(&stage_id) {
                20
            } else {
                15
            }
        }
        3 => {
            // Hard: 18-22 = 10s, 23-29 = 20s, 30-35 = 30s, else 20s
            if (18..=22).contains(&stage_id) {
                10
            } else if (23..=29).contains(&stage_id) {
                20
            } else if (30..=35).contains(&stage_id) {
                30
            } else {
                20
            }
        }
        _ => 20,
    }
}

// ── Monster Kill Processing ────────────────────────────────────────────

/// Check if a monster proto ID is a valid DD monster.
pub fn is_dd_monster(proto_id: u16) -> bool {
    DD_MONSTER_IDS.contains(&proto_id)
}

/// Check if a monster proto ID is a DD boss (grants Lunar Order Token).
pub fn is_dd_boss(proto_id: u16) -> bool {
    BOSS_MONSTER_IDS.contains(&proto_id)
}

/// Reward calculation for a DD monster kill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DdKillReward {
    /// Number of Full Moon Rift Jewels for the killer.
    pub rift_jewel_count: u16,
    /// Number of Monster Coins for the killer.
    pub killer_coin_count: u16,
    /// Number of Monster Coins for other party members.
    pub party_coin_count: u16,
    /// Whether Lunar Order Tokens should be given to party.
    pub lunar_token: bool,
}

/// Calculate rewards for a DD monster kill.
pub fn calculate_kill_reward(proto_id: u16, current_stage: i16) -> Option<DdKillReward> {
    if !is_dd_monster(proto_id) {
        return None;
    }

    // Full Moon Rift Jewel: 1 for stages 1-17, 2 for stages 18-35
    let rift_jewel_count = if (18..=35).contains(&current_stage) {
        2
    } else {
        1
    };

    // Monster Coin: 2 for killer, 1 for party
    let killer_coin_count = 2;
    let party_coin_count = 1;

    // Lunar Order Token: only for boss kills
    let lunar_token = is_dd_boss(proto_id);

    Some(DdKillReward {
        rift_jewel_count,
        killer_coin_count,
        party_coin_count,
        lunar_token,
    })
}

// ── Stage Counter Display ──────────────────────────────────────────────

/// Build the stage counter display values for a room.
/// Returns (max_stage, display_stage) for the TEMPLE_EVENT_STAGE_COUNTER packet.
pub fn get_stage_display(difficulty: u16, stage_id: i16) -> Option<(u8, u8)> {
    let dd_diff = match difficulty {
        1 => DdDifficulty::Easy,
        2 => DdDifficulty::Normal,
        3 => DdDifficulty::Hard,
        _ => return None,
    };

    let max_stage = dd_diff.max_stages();
    let display_stage = (stage_id - dd_diff.stage_offset()) as u8;
    Some((max_stage, display_stage))
}

// ── Finish Timer ───────────────────────────────────────────────────────

/// Trigger the finish sequence for a DD room.
/// Sets the room to finished state and configures the out timer.
pub fn trigger_finish(room: &DdRoomInfo) {
    room.monster_beginner_spawned
        .store(false, Ordering::Relaxed);
    room.monster_spawned.store(false, Ordering::Relaxed);
    room.is_finished.store(true, Ordering::Relaxed);
    room.out_time.store(DD_FINISH_TIME, Ordering::Relaxed);
}

// ── Stage Lookup ───────────────────────────────────────────────────────

/// Find the current stage entry in the stage list.
pub fn select_stage(stages: &[DfStageRow], difficulty: u16, stage_id: i16) -> Option<&DfStageRow> {
    stages
        .iter()
        .find(|s| s.stage_id == stage_id && s.difficulty == difficulty as i16)
}

/// Get all monster spawns for a given stage.
pub fn get_stage_monsters(monsters: &[DfMonsterRow], stage_id: i16) -> Vec<&DfMonsterRow> {
    monsters
        .iter()
        .filter(|m| m.id == stage_id as i32)
        .collect()
}

// ── Monster Coin Cleanup ───────────────────────────────────────────────

/// Check if a user needs Monster Coin cleanup on DD zone enter.
/// Removes all Monster Coins from the user when entering the DD zone.
pub fn should_rob_monster_coins() -> bool {
    // Always rob monster coins on DD entry
    true
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stages() -> Vec<DfStageRow> {
        vec![
            DfStageRow {
                id: 1,
                difficulty: 1,
                difficulty_name: Some("Easy".into()),
                stage_id: 1,
            },
            DfStageRow {
                id: 3,
                difficulty: 1,
                difficulty_name: Some("Easy".into()),
                stage_id: 2,
            },
            DfStageRow {
                id: 4,
                difficulty: 1,
                difficulty_name: Some("Easy".into()),
                stage_id: 3,
            },
            DfStageRow {
                id: 5,
                difficulty: 1,
                difficulty_name: Some("Easy".into()),
                stage_id: 4,
            },
            DfStageRow {
                id: 6,
                difficulty: 1,
                difficulty_name: Some("Easy".into()),
                stage_id: 5,
            },
            DfStageRow {
                id: 7,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 6,
            },
            DfStageRow {
                id: 8,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 7,
            },
            DfStageRow {
                id: 9,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 8,
            },
            DfStageRow {
                id: 10,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 9,
            },
            DfStageRow {
                id: 11,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 10,
            },
            DfStageRow {
                id: 12,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 11,
            },
            DfStageRow {
                id: 13,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 12,
            },
            DfStageRow {
                id: 14,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 13,
            },
            DfStageRow {
                id: 15,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 14,
            },
            DfStageRow {
                id: 16,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 15,
            },
            DfStageRow {
                id: 17,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 16,
            },
            DfStageRow {
                id: 18,
                difficulty: 2,
                difficulty_name: Some("Normal".into()),
                stage_id: 17,
            },
            DfStageRow {
                id: 19,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 18,
            },
            DfStageRow {
                id: 20,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 19,
            },
            DfStageRow {
                id: 21,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 20,
            },
            DfStageRow {
                id: 22,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 21,
            },
            DfStageRow {
                id: 23,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 22,
            },
            DfStageRow {
                id: 24,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 23,
            },
            DfStageRow {
                id: 25,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 24,
            },
            DfStageRow {
                id: 26,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 25,
            },
            DfStageRow {
                id: 27,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 26,
            },
            DfStageRow {
                id: 28,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 27,
            },
            DfStageRow {
                id: 29,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 28,
            },
            DfStageRow {
                id: 30,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 29,
            },
            DfStageRow {
                id: 31,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 30,
            },
            DfStageRow {
                id: 32,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 31,
            },
            DfStageRow {
                id: 33,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 32,
            },
            DfStageRow {
                id: 34,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 33,
            },
            DfStageRow {
                id: 35,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 34,
            },
            DfStageRow {
                id: 36,
                difficulty: 3,
                difficulty_name: Some("Hard".into()),
                stage_id: 35,
            },
        ]
    }

    fn make_monsters() -> Vec<DfMonsterRow> {
        vec![
            DfMonsterRow {
                id: 1,
                difficulty: Some(1),
                monster_id: 9927,
                is_monster: false,
                pos_x: 60,
                pos_z: 42,
                s_count: Some(2),
                s_direction: 0,
                s_radius_range: Some(10),
            },
            DfMonsterRow {
                id: 2,
                difficulty: Some(1),
                monster_id: 9928,
                is_monster: false,
                pos_x: 60,
                pos_z: 42,
                s_count: Some(1),
                s_direction: 0,
                s_radius_range: Some(0),
            },
            DfMonsterRow {
                id: 5,
                difficulty: Some(1),
                monster_id: 9931,
                is_monster: false,
                pos_x: 60,
                pos_z: 42,
                s_count: Some(1),
                s_direction: 0,
                s_radius_range: Some(0),
            },
        ]
    }

    // ── Difficulty Tests ──────────────────────────────────────────────

    #[test]
    fn test_difficulty_from_party_size() {
        assert_eq!(DdDifficulty::from_party_size(0), None);
        assert_eq!(DdDifficulty::from_party_size(1), None);
        assert_eq!(DdDifficulty::from_party_size(2), Some(DdDifficulty::Easy));
        assert_eq!(DdDifficulty::from_party_size(3), Some(DdDifficulty::Easy));
        assert_eq!(DdDifficulty::from_party_size(4), Some(DdDifficulty::Normal));
        assert_eq!(DdDifficulty::from_party_size(7), Some(DdDifficulty::Normal));
        assert_eq!(DdDifficulty::from_party_size(8), Some(DdDifficulty::Hard));
        assert_eq!(DdDifficulty::from_party_size(10), Some(DdDifficulty::Hard));
    }

    #[test]
    fn test_difficulty_starting_stage() {
        assert_eq!(DdDifficulty::Easy.starting_stage(), 1);
        assert_eq!(DdDifficulty::Normal.starting_stage(), 6);
        assert_eq!(DdDifficulty::Hard.starting_stage(), 18);
    }

    #[test]
    fn test_difficulty_final_stage() {
        assert_eq!(DdDifficulty::Easy.final_stage(), 5);
        assert_eq!(DdDifficulty::Normal.final_stage(), 17);
        assert_eq!(DdDifficulty::Hard.final_stage(), 35);
    }

    #[test]
    fn test_difficulty_max_stages() {
        assert_eq!(DdDifficulty::Easy.max_stages(), 5);
        assert_eq!(DdDifficulty::Normal.max_stages(), 12);
        assert_eq!(DdDifficulty::Hard.max_stages(), 18);
    }

    #[test]
    fn test_difficulty_stage_offset() {
        assert_eq!(DdDifficulty::Easy.stage_offset(), 0);
        assert_eq!(DdDifficulty::Normal.stage_offset(), 5);
        assert_eq!(DdDifficulty::Hard.stage_offset(), 17);
    }

    // ── Room State Tests ──────────────────────────────────────────────

    #[test]
    fn test_room_new() {
        let room = DdRoomInfo::new(1);
        assert_eq!(room.room_id.load(Ordering::Relaxed), 1);
        assert!(!room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 0);
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 0);
        assert_eq!(room.kill_count.load(Ordering::Relaxed), 0);
        assert_eq!(
            room.spawn_time.load(Ordering::Relaxed),
            DD_INITIAL_SPAWN_TIME
        );
        assert_eq!(room.room_close.load(Ordering::Relaxed), DD_ROOM_CLOSE_TIME);
    }

    #[test]
    fn test_room_start_easy() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);

        assert!(room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 1);
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 1);
        assert!(room.monster_beginner_spawned.load(Ordering::Relaxed));
        assert!(!room.monster_spawned.load(Ordering::Relaxed));
        assert!(!room.is_finished.load(Ordering::Relaxed));
    }

    #[test]
    fn test_room_start_normal() {
        let room = DdRoomInfo::new(2);
        room.try_claim_and_start(DdDifficulty::Normal);

        assert!(room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 2);
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 6);
    }

    #[test]
    fn test_room_start_hard() {
        let room = DdRoomInfo::new(3);
        room.try_claim_and_start(DdDifficulty::Hard);

        assert!(room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 3);
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 18);
    }

    #[test]
    fn test_room_reset() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);
        room.kill_count.store(10, Ordering::Relaxed);

        room.reset();

        assert!(!room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 0);
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 0);
        assert_eq!(room.kill_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_room_try_claim_double_fails() {
        let room = DdRoomInfo::new(1);
        assert!(room.try_claim_and_start(DdDifficulty::Easy));
        // Second claim on the same room should fail (CAS)
        assert!(!room.try_claim_and_start(DdDifficulty::Normal));
        // Room should retain original difficulty
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_room_try_claim_after_reset() {
        let room = DdRoomInfo::new(1);
        assert!(room.try_claim_and_start(DdDifficulty::Easy));
        room.reset();
        // After reset, room can be claimed again
        assert!(room.try_claim_and_start(DdDifficulty::Hard));
        assert_eq!(room.difficulty.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_try_claim_free_room() {
        let rooms: Vec<DdRoomInfo> = (1..=3).map(DdRoomInfo::new).collect();
        // First claim gets room 1
        assert_eq!(try_claim_free_room(&rooms, DdDifficulty::Easy), Some(1));
        // Second claim gets room 2 (room 1 is already started)
        assert_eq!(try_claim_free_room(&rooms, DdDifficulty::Normal), Some(2));
        // Third claim gets room 3
        assert_eq!(try_claim_free_room(&rooms, DdDifficulty::Hard), Some(3));
        // No more free rooms
        assert_eq!(try_claim_free_room(&rooms, DdDifficulty::Easy), None);
    }

    // ── Timer Tick Tests ──────────────────────────────────────────────

    #[test]
    fn test_timer_idle_when_not_started() {
        let room = DdRoomInfo::new(1);
        assert_eq!(timer_tick(&room), DdTickResult::Idle);
    }

    #[test]
    fn test_timer_decrements_spawn_time() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);

        // First tick decrements from 60 to 59
        let result = timer_tick(&room);
        assert_eq!(result, DdTickResult::Idle);
        assert_eq!(room.spawn_time.load(Ordering::Relaxed), 59);
    }

    #[test]
    fn test_timer_spawns_at_zero() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);
        room.spawn_time.store(1, Ordering::Relaxed);

        let result = timer_tick(&room);
        assert_eq!(
            result,
            DdTickResult::SpawnStage {
                stage_id: 1,
                difficulty: 1,
            }
        );
        // beginner flag cleared
        assert!(!room.monster_beginner_spawned.load(Ordering::Relaxed));
    }

    #[test]
    fn test_timer_room_close_kicks() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);
        room.monster_beginner_spawned
            .store(false, Ordering::Relaxed);
        room.spawn_time.store(999, Ordering::Relaxed);
        room.room_close.store(1, Ordering::Relaxed);

        let result = timer_tick(&room);
        assert_eq!(result, DdTickResult::KickAll);
    }

    #[test]
    fn test_timer_finish_kicks() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);
        room.monster_beginner_spawned
            .store(false, Ordering::Relaxed);
        room.spawn_time.store(999, Ordering::Relaxed);
        room.is_finished.store(true, Ordering::Relaxed);
        room.out_time.store(1, Ordering::Relaxed);
        room.room_close.store(999, Ordering::Relaxed);

        let result = timer_tick(&room);
        assert_eq!(result, DdTickResult::KickAll);
    }

    // ── Stage Advancement Tests ───────────────────────────────────────

    #[test]
    fn test_advance_stage_easy() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 2,
                spawn_delay: 10,
            }
        );
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_advance_stage_easy_final() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);
        room.stage_id.store(5, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(result, StageAdvanceResult::Finished);
    }

    #[test]
    fn test_advance_stage_normal() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Normal);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 7,
                spawn_delay: 10,
            }
        );
    }

    #[test]
    fn test_advance_stage_normal_later() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Normal);
        room.stage_id.store(11, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 12,
                spawn_delay: 20,
            }
        );
    }

    #[test]
    fn test_advance_stage_normal_final() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Normal);
        room.stage_id.store(17, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(result, StageAdvanceResult::Finished);
    }

    #[test]
    fn test_advance_stage_hard() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 19,
                spawn_delay: 10,
            }
        );
    }

    #[test]
    fn test_advance_stage_hard_mid() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);
        room.stage_id.store(25, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 26,
                spawn_delay: 20,
            }
        );
    }

    #[test]
    fn test_advance_stage_hard_late() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);
        room.stage_id.store(30, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(
            result,
            StageAdvanceResult::NextStage {
                new_stage_id: 31,
                spawn_delay: 30,
            }
        );
    }

    #[test]
    fn test_advance_stage_hard_final() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);
        room.stage_id.store(35, Ordering::Relaxed);

        let result = advance_stage(&room, &stages);
        assert_eq!(result, StageAdvanceResult::Finished);
    }

    // ── Spawn Delay Tests ─────────────────────────────────────────────

    #[test]
    fn test_spawn_delay_easy() {
        assert_eq!(get_spawn_delay(1, 1), 10);
        assert_eq!(get_spawn_delay(1, 3), 10);
        assert_eq!(get_spawn_delay(1, 5), 10);
    }

    #[test]
    fn test_spawn_delay_normal() {
        assert_eq!(get_spawn_delay(2, 6), 10);
        assert_eq!(get_spawn_delay(2, 10), 10);
        assert_eq!(get_spawn_delay(2, 11), 20);
        assert_eq!(get_spawn_delay(2, 17), 20);
    }

    #[test]
    fn test_spawn_delay_hard() {
        assert_eq!(get_spawn_delay(3, 18), 10);
        assert_eq!(get_spawn_delay(3, 22), 10);
        assert_eq!(get_spawn_delay(3, 23), 20);
        assert_eq!(get_spawn_delay(3, 29), 20);
        assert_eq!(get_spawn_delay(3, 30), 30);
        assert_eq!(get_spawn_delay(3, 35), 30);
    }

    // ── Monster ID Tests ──────────────────────────────────────────────

    #[test]
    fn test_is_dd_monster() {
        assert!(is_dd_monster(9927));
        assert!(is_dd_monster(9942));
        assert!(is_dd_monster(9947));
        assert!(is_dd_monster(9958));
        assert!(is_dd_monster(9959));
        assert!(!is_dd_monster(1234));
        assert!(!is_dd_monster(9943));
        assert!(!is_dd_monster(9960));
    }

    #[test]
    fn test_is_dd_boss() {
        assert!(is_dd_boss(9931));
        assert!(is_dd_boss(9951));
        assert!(is_dd_boss(9936));
        assert!(is_dd_boss(9959));
        assert!(is_dd_boss(9971));
        assert!(!is_dd_boss(9927));
        assert!(!is_dd_boss(1234));
    }

    // ── Kill Reward Tests ─────────────────────────────────────────────

    #[test]
    fn test_kill_reward_normal_monster_early_stage() {
        let reward = calculate_kill_reward(9927, 5);
        assert!(reward.is_some());
        let r = reward.unwrap();
        assert_eq!(r.rift_jewel_count, 1);
        assert_eq!(r.killer_coin_count, 2);
        assert_eq!(r.party_coin_count, 1);
        assert!(!r.lunar_token);
    }

    #[test]
    fn test_kill_reward_normal_monster_late_stage() {
        let reward = calculate_kill_reward(9955, 20);
        assert!(reward.is_some());
        let r = reward.unwrap();
        assert_eq!(r.rift_jewel_count, 2);
        assert_eq!(r.killer_coin_count, 2);
        assert_eq!(r.party_coin_count, 1);
        assert!(!r.lunar_token);
    }

    #[test]
    fn test_kill_reward_boss_monster() {
        let reward = calculate_kill_reward(9931, 5);
        assert!(reward.is_some());
        let r = reward.unwrap();
        assert!(r.lunar_token);
    }

    #[test]
    fn test_kill_reward_non_dd_monster() {
        let reward = calculate_kill_reward(1234, 5);
        assert!(reward.is_none());
    }

    // ── Stage Display Tests ───────────────────────────────────────────

    #[test]
    fn test_stage_display_easy() {
        let (max, display) = get_stage_display(1, 1).unwrap();
        assert_eq!(max, 5);
        assert_eq!(display, 1);

        let (max, display) = get_stage_display(1, 5).unwrap();
        assert_eq!(max, 5);
        assert_eq!(display, 5);
    }

    #[test]
    fn test_stage_display_normal() {
        let (max, display) = get_stage_display(2, 6).unwrap();
        assert_eq!(max, 12);
        assert_eq!(display, 1);

        let (max, display) = get_stage_display(2, 17).unwrap();
        assert_eq!(max, 12);
        assert_eq!(display, 12);
    }

    #[test]
    fn test_stage_display_hard() {
        let (max, display) = get_stage_display(3, 18).unwrap();
        assert_eq!(max, 18);
        assert_eq!(display, 1);

        let (max, display) = get_stage_display(3, 35).unwrap();
        assert_eq!(max, 18);
        assert_eq!(display, 18);
    }

    #[test]
    fn test_stage_display_invalid_difficulty() {
        assert!(get_stage_display(0, 1).is_none());
        assert!(get_stage_display(4, 1).is_none());
    }

    // ── Finish Timer Tests ────────────────────────────────────────────

    #[test]
    fn test_trigger_finish() {
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);

        trigger_finish(&room);

        assert!(!room.monster_beginner_spawned.load(Ordering::Relaxed));
        assert!(!room.monster_spawned.load(Ordering::Relaxed));
        assert!(room.is_finished.load(Ordering::Relaxed));
        assert_eq!(room.out_time.load(Ordering::Relaxed), DD_FINISH_TIME);
    }

    // ── Stage Lookup Tests ────────────────────────────────────────────

    #[test]
    fn test_select_stage() {
        let stages = make_stages();
        let stage = select_stage(&stages, 1, 1);
        assert!(stage.is_some());
        assert_eq!(stage.unwrap().id, 1);

        let stage = select_stage(&stages, 2, 6);
        assert!(stage.is_some());

        let stage = select_stage(&stages, 1, 99);
        assert!(stage.is_none());
    }

    #[test]
    fn test_get_stage_monsters() {
        let monsters = make_monsters();
        let spawns = get_stage_monsters(&monsters, 1);
        assert_eq!(spawns.len(), 1);
        assert_eq!(spawns[0].monster_id, 9927);

        let spawns = get_stage_monsters(&monsters, 5);
        assert_eq!(spawns.len(), 1);
        assert_eq!(spawns[0].monster_id, 9931);

        let spawns = get_stage_monsters(&monsters, 99);
        assert_eq!(spawns.len(), 0);
    }

    // ── Guardian NPC Tests ────────────────────────────────────────────

    #[test]
    fn test_guardian_npcs_defined() {
        assert_eq!(GUARDIAN_NPCS.len(), 6);
        // First 3 are NPC 31737
        assert_eq!(GUARDIAN_NPCS[0].0, 31737);
        assert_eq!(GUARDIAN_NPCS[1].0, 31737);
        assert_eq!(GUARDIAN_NPCS[2].0, 31737);
        // Others are unique
        assert_eq!(GUARDIAN_NPCS[3].0, 31740);
        assert_eq!(GUARDIAN_NPCS[4].0, 31739);
        assert_eq!(GUARDIAN_NPCS[5].0, 31738);
    }

    // ── Constants Tests ───────────────────────────────────────────────

    #[test]
    fn test_constants() {
        assert_eq!(ZONE_DUNGEON_DEFENCE, 89);
        assert_eq!(DUNGEON_DEFENCE_RIFT_ITEM, 914057000);
        assert_eq!(MONSTER_COIN_ITEM, 914058000);
        assert_eq!(MONSTER_RIFT_JEWEL, 914069000);
        assert_eq!(LUNAR_ORDER_TOKEN, 810977000);
        assert_eq!(DD_INITIAL_SPAWN_TIME, 60);
        assert_eq!(DD_ROOM_CLOSE_TIME, 7200);
        assert_eq!(DD_FINISH_TIME, 30);
        assert_eq!(TEMPLE_EVENT_DUNGEON_SIGN, 58);
        assert_eq!(TEMPLE_EVENT_STAGE_COUNTER, 60);
    }

    // ── Full Lifecycle Test ───────────────────────────────────────────

    #[test]
    fn test_full_easy_lifecycle() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Easy);

        // Room is started, stage 1
        assert!(room.is_started.load(Ordering::Relaxed));
        assert_eq!(room.stage_id.load(Ordering::Relaxed), 1);

        // Simulate ticks until spawn
        room.spawn_time.store(1, Ordering::Relaxed);
        let result = timer_tick(&room);
        assert_eq!(
            result,
            DdTickResult::SpawnStage {
                stage_id: 1,
                difficulty: 1,
            }
        );

        // Advance through all stages
        for expected_stage in 2..=5 {
            let advance = advance_stage(&room, &stages);
            if expected_stage <= 4 {
                match advance {
                    StageAdvanceResult::NextStage { new_stage_id, .. } => {
                        assert_eq!(new_stage_id, expected_stage);
                    }
                    _ => panic!("Expected NextStage for stage {}", expected_stage),
                }
            }
        }

        // Final stage: should be finished
        let advance = advance_stage(&room, &stages);
        assert_eq!(advance, StageAdvanceResult::Finished);

        // Trigger finish
        trigger_finish(&room);
        assert!(room.is_finished.load(Ordering::Relaxed));
        assert_eq!(room.out_time.load(Ordering::Relaxed), DD_FINISH_TIME);

        // Simulate finish timer expiry
        room.monster_beginner_spawned
            .store(false, Ordering::Relaxed);
        room.monster_spawned.store(false, Ordering::Relaxed);
        room.spawn_time.store(999, Ordering::Relaxed);
        room.room_close.store(999, Ordering::Relaxed);
        room.out_time.store(1, Ordering::Relaxed);
        let result = timer_tick(&room);
        assert_eq!(result, DdTickResult::KickAll);
    }

    #[test]
    fn test_full_normal_lifecycle() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Normal);

        assert_eq!(room.stage_id.load(Ordering::Relaxed), 6);

        // Advance through all 12 normal stages (6-17)
        for expected_stage in 7..=17 {
            let advance = advance_stage(&room, &stages);
            match advance {
                StageAdvanceResult::NextStage { new_stage_id, .. } => {
                    assert_eq!(new_stage_id, expected_stage);
                }
                _ => panic!("Expected NextStage for stage {}", expected_stage),
            }
        }

        // Final stage 17: should be finished
        let advance = advance_stage(&room, &stages);
        assert_eq!(advance, StageAdvanceResult::Finished);
    }

    #[test]
    fn test_full_hard_lifecycle() {
        let stages = make_stages();
        let room = DdRoomInfo::new(1);
        room.try_claim_and_start(DdDifficulty::Hard);

        assert_eq!(room.stage_id.load(Ordering::Relaxed), 18);

        // Advance through all 18 hard stages (18-35)
        for expected_stage in 19..=35 {
            let advance = advance_stage(&room, &stages);
            match advance {
                StageAdvanceResult::NextStage { new_stage_id, .. } => {
                    assert_eq!(new_stage_id, expected_stage);
                }
                _ => panic!("Expected NextStage for stage {}", expected_stage),
            }
        }

        // Final stage 35: should be finished
        let advance = advance_stage(&room, &stages);
        assert_eq!(advance, StageAdvanceResult::Finished);
    }
}
