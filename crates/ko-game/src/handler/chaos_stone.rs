//! Chaos Stone world event handler.
//! ## System Overview
//! Chaos Stones are world-spawned destructible NPCs in zones 71 (Ronark Land),
//! 72 (Ardream), and 73 (unused/future). When destroyed, they:
//! 1. Send a zone-wide notice (`CHAOS_STONE_ENEMY_NOTICE`)
//! 2. Advance to the next rank
//! 3. Spawn summoned monsters (from `chaos_stone_summon_list`) at the stone's position
//! 4. Begin a 30-minute respawn timer for the stone itself
//! 5. Track summoned monster boss kills; when all bosses are dead, mark wave complete
//! ## Rank Progression
//! Each zone has 4 ranks. When a stone of rank N is killed, its `ChaosStoneInfo`
//! advances to rank N+1. If rank N+1 doesn't exist in `chaos_stone_spawn`, it
//! wraps back to rank 1.
//! ## Monster Family Rotation
//! After spawning monsters of family F, the family counter increments. If the new
//! family doesn't exist in `chaos_stone_summon_stage` for that zone, it wraps to 1.
//! ## Constants
//! | Name | Value | Description |
//! |------|-------|-------------|
//! | `CHAOS_STONE_RESPAWN_TIME` | 1800 (30 min) | Time before stone respawns |
//! | `CHAOS_STONE_MONSTER_RESPAWN_RADIUS` | 20 | Radius for spawned monsters |
//! | `CHAOS_STONE_MONSTER_LIVE_TIME` | 900 (15 min) | Lifetime of spawned special-stone mobs |

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicU16, AtomicU32, AtomicU8, Ordering};

use ko_db::models::chaos_stone::{
    ChaosStoneSpawnRow, ChaosStoneSummonListRow, ChaosStoneSummonStageRow, EventChaosRewardRow,
};
use ko_db::models::SpecialStoneRow;

/// Respawn timer for chaos stones (seconds). `30 * MINUTE`.
pub const CHAOS_STONE_RESPAWN_TIME: u32 = 30 * 60; // 1800s

/// Radius around stone position for spawned monsters. `CHAOS_STONE_MONSTER_RESPAWN_RADIUS`.
pub const CHAOS_STONE_MONSTER_RESPAWN_RADIUS: u16 = 20;

/// Lifetime of spawned special-stone monsters (seconds). `CHAOS_STONE_MONSTER_LIVE_TIME`.
pub const CHAOS_STONE_MONSTER_LIVE_TIME: u32 = 900;

/// Maximum number of tracked boss IDs per stone info. `sBoosID[10]`.
pub const MAX_BOSS_IDS: usize = 10;

/// Runtime state for a single chaos stone instance.
/// One `ChaosStoneInfo` is created per rank-1 spawn entry during `ChaosStoneLoad()`.
pub struct ChaosStoneInfo {
    /// Index in the info array (1-based, matches loading order).
    pub chaos_index: AtomicU8,
    /// NPC template ID of the chaos stone.
    pub chaos_id: AtomicU16,
    /// Zone ID where this stone belongs.
    pub zone_id: AtomicU16,
    /// Current rank (advances on kill, wraps around).
    pub rank: AtomicU8,
    /// Respawn countdown in seconds (decremented by timer).
    pub spawn_time: AtomicU32,
    /// Current monster family to spawn on next death.
    pub monster_family: AtomicU8,
    /// Runtime IDs of tracked boss NPCs for kill tracking.
    ///
    /// C++ stores `uint16 sBoosID[10]`; Rust uses u32 to match `NpcId` type.
    pub boss_ids: [AtomicU32; MAX_BOSS_IDS],
    /// Number of bosses remaining to kill.
    pub boss_killed_count: AtomicI16,
    /// Whether this stone has been killed (waiting for respawn).
    pub is_chaos_stone_killed: AtomicBool,
    /// Whether all summoned bosses have been killed.
    pub is_total_killed_monster: AtomicBool,
    /// Whether the respawn timer is counting down.
    pub is_on_res_timer: AtomicBool,
    /// Whether this stone spawn point is enabled.
    pub chaos_stone_on: AtomicBool,
}

impl ChaosStoneInfo {
    /// Create a new default (inactive) chaos stone info.
    pub fn new() -> Self {
        Self {
            chaos_index: AtomicU8::new(0),
            chaos_id: AtomicU16::new(0),
            zone_id: AtomicU16::new(0),
            rank: AtomicU8::new(1),
            spawn_time: AtomicU32::new(CHAOS_STONE_RESPAWN_TIME),
            monster_family: AtomicU8::new(0),
            boss_ids: std::array::from_fn(|_| AtomicU32::new(0)),
            boss_killed_count: AtomicI16::new(0),
            is_chaos_stone_killed: AtomicBool::new(false),
            is_total_killed_monster: AtomicBool::new(false),
            is_on_res_timer: AtomicBool::new(false),
            chaos_stone_on: AtomicBool::new(false),
        }
    }

    /// Reset the chaos stone info to default state.
    ///
    pub fn reset(&self) {
        self.chaos_index.store(0, Ordering::Relaxed);
        self.chaos_id.store(0, Ordering::Relaxed);
        self.zone_id.store(0, Ordering::Relaxed);
        self.rank.store(1, Ordering::Relaxed);
        self.spawn_time
            .store(CHAOS_STONE_RESPAWN_TIME, Ordering::Relaxed);
        self.monster_family.store(0, Ordering::Relaxed);
        for id in &self.boss_ids {
            id.store(0, Ordering::Relaxed);
        }
        self.boss_killed_count.store(0, Ordering::Relaxed);
        self.is_chaos_stone_killed.store(false, Ordering::Relaxed);
        self.is_total_killed_monster.store(false, Ordering::Relaxed);
        self.is_on_res_timer.store(false, Ordering::Relaxed);
        self.chaos_stone_on.store(false, Ordering::Relaxed);
    }
}

impl Default for ChaosStoneInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize chaos stone info entries from spawn data.
/// Creates one `ChaosStoneInfo` per rank-1 spawn entry. Returns a 1-indexed
/// map (index -> info) matching the C++ array indexing.
pub fn load_chaos_stones(spawns: &[ChaosStoneSpawnRow]) -> HashMap<u8, ChaosStoneInfo> {
    let mut result = HashMap::new();
    let mut index: u8 = 0;

    for spawn in spawns {
        if spawn.rank != 1 {
            continue;
        }

        index += 1;
        if index > 50 {
            break;
        }

        let info = ChaosStoneInfo::new();
        info.chaos_index.store(index, Ordering::Relaxed);
        info.chaos_id
            .store(spawn.chaos_id as u16, Ordering::Relaxed);
        info.rank.store(spawn.rank as u8, Ordering::Relaxed);
        info.zone_id.store(spawn.zone_id as u16, Ordering::Relaxed);
        info.spawn_time
            .store(CHAOS_STONE_RESPAWN_TIME, Ordering::Relaxed);
        info.monster_family.store(1, Ordering::Relaxed);
        info.chaos_stone_on.store(spawn.is_open, Ordering::Relaxed);

        result.insert(index, info);
    }

    result
}

/// Result of the respawn timer tick for a single chaos stone.
#[derive(Debug, PartialEq, Eq)]
pub enum ChaosStoneTimerResult {
    /// No action needed.
    Idle,
    /// The stone should respawn now. Contains (chaos_id, rank, zone_id).
    Respawn(u16, u8, u16),
}

/// Tick the respawn timer for all chaos stone infos.
/// Should be called once per second. Returns a list of stones that need respawning.
pub fn respawn_timer_tick(infos: &HashMap<u8, ChaosStoneInfo>) -> Vec<ChaosStoneTimerResult> {
    let mut results = Vec::new();

    for (_idx, info) in infos.iter() {
        if !info.chaos_stone_on.load(Ordering::Relaxed)
            || !info.is_on_res_timer.load(Ordering::Relaxed)
            || !info.is_chaos_stone_killed.load(Ordering::Relaxed)
        {
            continue;
        }

        let time = info.spawn_time.load(Ordering::Relaxed);
        if time > 0 {
            info.spawn_time.fetch_sub(1, Ordering::Relaxed);
            continue;
        }

        // Timer expired — respawn the stone
        let chaos_id = info.chaos_id.load(Ordering::Relaxed);
        let rank = info.rank.load(Ordering::Relaxed);
        let zone_id = info.zone_id.load(Ordering::Relaxed);

        info.is_chaos_stone_killed.store(false, Ordering::Relaxed);
        info.is_on_res_timer.store(false, Ordering::Relaxed);
        info.is_total_killed_monster.store(false, Ordering::Relaxed);
        info.boss_killed_count.store(0, Ordering::Relaxed);

        results.push(ChaosStoneTimerResult::Respawn(chaos_id, rank, zone_id));
    }

    results
}

/// Find the chaos stone info index for a given stone.
pub fn find_info_index(
    infos: &HashMap<u8, ChaosStoneInfo>,
    chaos_id: u16,
    rank: u8,
    zone_id: u16,
) -> Option<u8> {
    for (&idx, info) in infos.iter() {
        if info.chaos_id.load(Ordering::Relaxed) == chaos_id
            && info.rank.load(Ordering::Relaxed) == rank
            && info.zone_id.load(Ordering::Relaxed) == zone_id
        {
            return Some(idx);
        }
    }
    None
}

/// Find spawn row index for a given chaos stone proto and rank.
pub fn find_spawn_index(
    spawns: &[ChaosStoneSpawnRow],
    proto_id: u16,
    rank: u8,
    zone_id: u16,
) -> Option<usize> {
    spawns.iter().position(|s| {
        s.is_open
            && s.chaos_id as u16 == proto_id
            && s.rank as u8 == rank
            && s.zone_id as u16 == zone_id
    })
}

/// Process chaos stone death — advances rank, starts respawn timer, returns info index.
/// Returns the chaos_index of the info entry that was updated, for use with
/// `death_respawn_monsters()`.
pub fn on_chaos_stone_death(
    infos: &HashMap<u8, ChaosStoneInfo>,
    spawns: &[ChaosStoneSpawnRow],
    proto_id: u16,
    zone_id: u16,
) -> Option<u8> {
    let mut result_index = None;

    for (&idx, info) in infos.iter() {
        if !info.chaos_stone_on.load(Ordering::Relaxed)
            || info.zone_id.load(Ordering::Relaxed) != zone_id
            || info.chaos_id.load(Ordering::Relaxed) != proto_id
        {
            continue;
        }

        // Advance rank
        let new_rank = info.rank.load(Ordering::Relaxed).wrapping_add(1);
        info.rank.store(new_rank, Ordering::Relaxed);
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);

        // Try to find spawn for new rank
        if let Some(spawn_idx) = find_spawn_index(spawns, proto_id, new_rank, zone_id) {
            info.spawn_time
                .store(CHAOS_STONE_RESPAWN_TIME, Ordering::Relaxed);
            info.rank
                .store(spawns[spawn_idx].rank as u8, Ordering::Relaxed);
        } else {
            // Wrap back to rank 1
            if let Some(spawn_idx) = find_spawn_index(spawns, proto_id, 1, zone_id) {
                info.spawn_time
                    .store(CHAOS_STONE_RESPAWN_TIME, Ordering::Relaxed);
                info.rank
                    .store(spawns[spawn_idx].rank as u8, Ordering::Relaxed);
            } else {
                info.spawn_time
                    .store(CHAOS_STONE_RESPAWN_TIME, Ordering::Relaxed);
                info.rank.store(1, Ordering::Relaxed);
            }
        }

        info.is_total_killed_monster.store(false, Ordering::Relaxed);
        info.boss_killed_count.store(0, Ordering::Relaxed);
        info.is_on_res_timer.store(true, Ordering::Relaxed);
        result_index = Some(idx);
    }

    result_index
}

/// Register spawned boss NPC runtime IDs for kill tracking.
/// NPCs are processed, their `GetID()` runtime IDs are stored in `sBoosID[0-9]`
/// and `sBoosKilledCount` is incremented.
/// Must be called after `spawn_event_npc()` returns the spawned NPC IDs.
pub fn register_spawned_bosses(
    infos: &HashMap<u8, ChaosStoneInfo>,
    chaos_index: u8,
    npc_ids: &[u32],
) {
    let info = match infos.get(&chaos_index) {
        Some(i) => i,
        None => return,
    };

    let mut slot = 0;
    for &npc_id in npc_ids {
        if npc_id == 0 || slot >= MAX_BOSS_IDS {
            break;
        }
        // Find next empty slot (C++ if-else chain for sBoosID[0..9])
        while slot < MAX_BOSS_IDS && info.boss_ids[slot].load(Ordering::Relaxed) != 0 {
            slot += 1;
        }
        if slot < MAX_BOSS_IDS {
            info.boss_ids[slot].store(npc_id, Ordering::Relaxed);
            info.boss_killed_count.fetch_add(1, Ordering::Relaxed);
            slot += 1;
        }
    }
}

/// Get summoned monster IDs for a chaos stone death event.
/// Returns the list of NPC template IDs to spawn, filtered by zone and monster family.
/// Also advances the monster family counter for next time.
pub fn death_respawn_monsters(
    infos: &HashMap<u8, ChaosStoneInfo>,
    summon_list: &[ChaosStoneSummonListRow],
    stages: &[ChaosStoneSummonStageRow],
    chaos_index: u8,
) -> Vec<i16> {
    let info = match infos.get(&chaos_index) {
        Some(i) => i,
        None => return Vec::new(),
    };

    let zone_id = info.zone_id.load(Ordering::Relaxed);
    let family = info.monster_family.load(Ordering::Relaxed);

    let monsters: Vec<i16> = summon_list
        .iter()
        .filter(|s| s.zone_id as u16 == zone_id && s.monster_spawn_family as u8 == family)
        .map(|s| s.sid)
        .collect();

    // Advance monster family
    let next_family = family.wrapping_add(1);
    let family_exists = stages
        .iter()
        .any(|s| s.zone_id as u16 == zone_id && s.index_family as u8 == next_family);

    if family_exists {
        info.monster_family.store(next_family, Ordering::Relaxed);
    } else {
        // Wrap to family 1
        info.monster_family.store(1, Ordering::Relaxed);
    }

    monsters
}

/// Check if a given family exists for a zone in the stage list.
pub fn family_exists_for_zone(
    stages: &[ChaosStoneSummonStageRow],
    zone_id: u16,
    family: u8,
) -> bool {
    stages
        .iter()
        .any(|s| s.zone_id as u16 == zone_id && s.index_family as u8 == family)
}

/// Process a summoned monster boss death — decrements boss kill count.
/// Returns `true` if all bosses for that stone are now dead (wave complete).
pub fn on_boss_killed(infos: &HashMap<u8, ChaosStoneInfo>, npc_id: u32, zone_id: u16) -> bool {
    for info in infos.values() {
        if info.zone_id.load(Ordering::Relaxed) != zone_id
            || !info.is_chaos_stone_killed.load(Ordering::Relaxed)
            || !info.chaos_stone_on.load(Ordering::Relaxed)
            || info.is_total_killed_monster.load(Ordering::Relaxed)
        {
            continue;
        }

        for i in 0..MAX_BOSS_IDS {
            if info.boss_ids[i].load(Ordering::Relaxed) == npc_id {
                info.boss_killed_count.fetch_sub(1, Ordering::Relaxed);
                info.boss_ids[i].store(0, Ordering::Relaxed);

                if info.boss_killed_count.load(Ordering::Relaxed) <= 0 {
                    info.is_total_killed_monster.store(true, Ordering::Relaxed);
                    return true;
                }
                return false;
            }
        }
    }
    false
}

/// Get the spawn data for respawning a chaos stone.
/// Returns the matching spawn row for respawning the stone NPC.
pub fn get_respawn_data(
    spawns: &[ChaosStoneSpawnRow],
    chaos_id: u16,
    rank: u8,
    zone_id: u16,
) -> Option<&ChaosStoneSpawnRow> {
    spawns.iter().find(|s| {
        s.chaos_id as u16 == chaos_id
            && s.rank as u8 == rank
            && s.zone_id as u16 == zone_id
            && s.is_open
    })
}

/// Collect reward items from an `EventChaosRewardRow`.
/// Returns up to 5 (item_id, count, expiration) tuples for non-zero items.
pub fn collect_reward_items(reward: &EventChaosRewardRow) -> Vec<(i32, i32, i32)> {
    let mut items = Vec::new();
    if reward.item_id1 != 0 {
        items.push((reward.item_id1, reward.item_count1, reward.item_expiration1));
    }
    if reward.item_id2 != 0 {
        items.push((reward.item_id2, reward.item_count2, reward.item_expiration2));
    }
    if reward.item_id3 != 0 {
        items.push((reward.item_id3, reward.item_count3, reward.item_expiration3));
    }
    if reward.item_id4 != 0 {
        items.push((reward.item_id4, reward.item_count4, reward.item_expiration4));
    }
    if reward.item_id5 != 0 {
        items.push((reward.item_id5, reward.item_count5, reward.item_expiration5));
    }
    items
}

/// Get reward for a specific rank tier.
pub fn get_reward_by_rank(
    rewards: &[EventChaosRewardRow],
    rank: i16,
) -> Option<&EventChaosRewardRow> {
    rewards.iter().find(|r| r.rank_id == rank)
}

/// NPC type constant for special stones. `NPC_MONSTER_SPECIAL = 221`.
pub const NPC_MONSTER_SPECIAL: u8 = 221;

/// Process a special stone death — randomly selects and spawns a monster.
/// When a special stone NPC (type 221) is killed:
/// 1. Collects all matching entries from `k_special_stone` (same proto_id + zone)
/// 2. Randomly picks one entry
/// 3. Returns the (summon_npc, summon_count) to spawn at the stone's position
/// Returns `None` if no matching entries exist.
pub fn on_special_stone_death(
    stones: &[SpecialStoneRow],
    proto_id: u16,
    zone_id: u16,
) -> Option<(u16, u16)> {
    // Collect matching entries (C++ filters by MainNpcID + ZoneID + status)
    let matches: Vec<&SpecialStoneRow> = stones
        .iter()
        .filter(|s| s.main_npc as u16 == proto_id && s.zone_id as u16 == zone_id && s.status == 1)
        .collect();

    if matches.is_empty() {
        return None;
    }

    // C++ myrand(1, size) — 1-based random index
    let idx = if matches.len() == 1 {
        0
    } else {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as usize;
        seed % matches.len()
    };

    let selected = matches[idx];
    Some((selected.summon_npc as u16, selected.summon_count as u16))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_spawns() -> Vec<ChaosStoneSpawnRow> {
        vec![
            ChaosStoneSpawnRow {
                s_index: 1,
                zone_id: 71,
                is_open: true,
                rank: 1,
                chaos_id: 8945,
                count: 1,
                spawn_x: 1017,
                spawn_z: 948,
                spawn_time: 1,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 2,
                zone_id: 71,
                is_open: true,
                rank: 2,
                chaos_id: 8945,
                count: 1,
                spawn_x: 971,
                spawn_z: 1007,
                spawn_time: 3,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 3,
                zone_id: 71,
                is_open: true,
                rank: 3,
                chaos_id: 8945,
                count: 1,
                spawn_x: 1012,
                spawn_z: 1055,
                spawn_time: 5,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 4,
                zone_id: 71,
                is_open: true,
                rank: 4,
                chaos_id: 8945,
                count: 1,
                spawn_x: 1046,
                spawn_z: 1006,
                spawn_time: 7,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 5,
                zone_id: 72,
                is_open: true,
                rank: 1,
                chaos_id: 8946,
                count: 1,
                spawn_x: 524,
                spawn_z: 542,
                spawn_time: 1,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 6,
                zone_id: 72,
                is_open: true,
                rank: 2,
                chaos_id: 8946,
                count: 1,
                spawn_x: 520,
                spawn_z: 511,
                spawn_time: 1,
                direction: 0,
                radius_range: 0,
            },
            ChaosStoneSpawnRow {
                s_index: 9,
                zone_id: 73,
                is_open: false,
                rank: 1,
                chaos_id: 8947,
                count: 1,
                spawn_x: 0,
                spawn_z: 0,
                spawn_time: 30,
                direction: 0,
                radius_range: 0,
            },
        ]
    }

    fn make_summon_list() -> Vec<ChaosStoneSummonListRow> {
        vec![
            ChaosStoneSummonListRow {
                n_index: 1,
                zone_id: 71,
                sid: 8254,
                monster_spawn_family: 1,
            },
            ChaosStoneSummonListRow {
                n_index: 2,
                zone_id: 71,
                sid: 8915,
                monster_spawn_family: 1,
            },
            ChaosStoneSummonListRow {
                n_index: 3,
                zone_id: 71,
                sid: 8907,
                monster_spawn_family: 1,
            },
            ChaosStoneSummonListRow {
                n_index: 7,
                zone_id: 71,
                sid: 8253,
                monster_spawn_family: 2,
            },
            ChaosStoneSummonListRow {
                n_index: 8,
                zone_id: 71,
                sid: 8260,
                monster_spawn_family: 2,
            },
            ChaosStoneSummonListRow {
                n_index: 13,
                zone_id: 71,
                sid: 8255,
                monster_spawn_family: 3,
            },
        ]
    }

    fn make_stages() -> Vec<ChaosStoneSummonStageRow> {
        vec![
            ChaosStoneSummonStageRow {
                n_index: 1,
                zone_id: 71,
                index_family: 1,
            },
            ChaosStoneSummonStageRow {
                n_index: 2,
                zone_id: 71,
                index_family: 2,
            },
            ChaosStoneSummonStageRow {
                n_index: 3,
                zone_id: 71,
                index_family: 3,
            },
            ChaosStoneSummonStageRow {
                n_index: 4,
                zone_id: 72,
                index_family: 1,
            },
            ChaosStoneSummonStageRow {
                n_index: 5,
                zone_id: 72,
                index_family: 2,
            },
        ]
    }

    fn make_rewards() -> Vec<EventChaosRewardRow> {
        vec![
            EventChaosRewardRow {
                rank_id: 1,
                item_id1: 900017000,
                item_count1: 3,
                item_expiration1: 0,
                item_id2: 389196000,
                item_count2: 50,
                item_expiration2: 0,
                item_id3: 389197000,
                item_count3: 50,
                item_expiration3: 0,
                item_id4: 389198000,
                item_count4: 50,
                item_expiration4: 0,
                item_id5: 0,
                item_count5: 0,
                item_expiration5: 0,
                experience: 250000000,
                loyalty: 2000,
                cash: 150,
                noah: 10000000,
            },
            EventChaosRewardRow {
                rank_id: 2,
                item_id1: 900017000,
                item_count1: 2,
                item_expiration1: 0,
                item_id2: 389196000,
                item_count2: 25,
                item_expiration2: 0,
                item_id3: 0,
                item_count3: 0,
                item_expiration3: 0,
                item_id4: 0,
                item_count4: 0,
                item_expiration4: 0,
                item_id5: 0,
                item_count5: 0,
                item_expiration5: 0,
                experience: 200000000,
                loyalty: 1500,
                cash: 100,
                noah: 10000000,
            },
            EventChaosRewardRow {
                rank_id: 12,
                item_id1: 389301000,
                item_count1: 1,
                item_expiration1: 0,
                item_id2: 0,
                item_count2: 0,
                item_expiration2: 0,
                item_id3: 0,
                item_count3: 0,
                item_expiration3: 0,
                item_id4: 0,
                item_count4: 0,
                item_expiration4: 0,
                item_id5: 0,
                item_count5: 0,
                item_expiration5: 0,
                experience: 25000000,
                loyalty: 100,
                cash: 0,
                noah: 10000000,
            },
        ]
    }

    // --- load_chaos_stones tests ---

    #[test]
    fn test_load_chaos_stones_creates_entries_for_rank1() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        // Only rank=1 entries: zone 71 (index 0), zone 72 (index 4), zone 73 (index 6)
        assert_eq!(infos.len(), 3);

        // Zone 71 stone
        let info1 = infos.get(&1).unwrap();
        assert_eq!(info1.chaos_id.load(Ordering::Relaxed), 8945);
        assert_eq!(info1.zone_id.load(Ordering::Relaxed), 71);
        assert!(info1.chaos_stone_on.load(Ordering::Relaxed)); // is_open=1

        // Zone 72 stone
        let info2 = infos.get(&2).unwrap();
        assert_eq!(info2.chaos_id.load(Ordering::Relaxed), 8946);
        assert_eq!(info2.zone_id.load(Ordering::Relaxed), 72);
        assert!(info2.chaos_stone_on.load(Ordering::Relaxed));

        // Zone 73 stone (is_open=0)
        let info3 = infos.get(&3).unwrap();
        assert_eq!(info3.chaos_id.load(Ordering::Relaxed), 8947);
        assert!(!info3.chaos_stone_on.load(Ordering::Relaxed));
    }

    #[test]
    fn test_load_chaos_stones_skips_non_rank1() {
        let spawns = vec![ChaosStoneSpawnRow {
            s_index: 2,
            zone_id: 71,
            is_open: true,
            rank: 2,
            chaos_id: 8945,
            count: 1,
            spawn_x: 971,
            spawn_z: 1007,
            spawn_time: 3,
            direction: 0,
            radius_range: 0,
        }];
        let infos = load_chaos_stones(&spawns);
        assert_eq!(infos.len(), 0);
    }

    #[test]
    fn test_load_chaos_stones_max_50() {
        let spawns: Vec<ChaosStoneSpawnRow> = (0..60)
            .map(|i| ChaosStoneSpawnRow {
                s_index: i,
                zone_id: 71,
                is_open: true,
                rank: 1,
                chaos_id: (8900 + i),
                count: 1,
                spawn_x: 100,
                spawn_z: 100,
                spawn_time: 1,
                direction: 0,
                radius_range: 0,
            })
            .collect();
        let infos = load_chaos_stones(&spawns);
        assert_eq!(infos.len(), 50);
    }

    // --- ChaosStoneInfo tests ---

    #[test]
    fn test_chaos_stone_info_new_defaults() {
        let info = ChaosStoneInfo::new();
        assert_eq!(info.chaos_index.load(Ordering::Relaxed), 0);
        assert_eq!(info.rank.load(Ordering::Relaxed), 1);
        assert_eq!(
            info.spawn_time.load(Ordering::Relaxed),
            CHAOS_STONE_RESPAWN_TIME
        );
        assert!(!info.is_chaos_stone_killed.load(Ordering::Relaxed));
        assert!(!info.chaos_stone_on.load(Ordering::Relaxed));
    }

    #[test]
    fn test_chaos_stone_info_reset() {
        let info = ChaosStoneInfo::new();
        info.chaos_index.store(5, Ordering::Relaxed);
        info.rank.store(3, Ordering::Relaxed);
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.chaos_stone_on.store(true, Ordering::Relaxed);
        info.boss_ids[0].store(100, Ordering::Relaxed);

        info.reset();

        assert_eq!(info.chaos_index.load(Ordering::Relaxed), 0);
        assert_eq!(info.rank.load(Ordering::Relaxed), 1);
        assert!(!info.is_chaos_stone_killed.load(Ordering::Relaxed));
        assert!(!info.chaos_stone_on.load(Ordering::Relaxed));
        assert_eq!(info.boss_ids[0].load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_chaos_stone_info_default() {
        let info = ChaosStoneInfo::default();
        assert_eq!(info.rank.load(Ordering::Relaxed), 1);
    }

    // --- find_info_index tests ---

    #[test]
    fn test_find_info_index_found() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let idx = find_info_index(&infos, 8945, 1, 71);
        assert!(idx.is_some());
        assert_eq!(idx.unwrap(), 1);
    }

    #[test]
    fn test_find_info_index_not_found() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let idx = find_info_index(&infos, 9999, 1, 71);
        assert!(idx.is_none());
    }

    // --- find_spawn_index tests ---

    #[test]
    fn test_find_spawn_index_found() {
        let spawns = make_spawns();
        let idx = find_spawn_index(&spawns, 8945, 2, 71);
        assert!(idx.is_some());
        assert_eq!(spawns[idx.unwrap()].s_index, 2);
    }

    #[test]
    fn test_find_spawn_index_closed() {
        let spawns = make_spawns();
        // Zone 73 is closed (is_open=0)
        let idx = find_spawn_index(&spawns, 8947, 1, 73);
        assert!(idx.is_none());
    }

    #[test]
    fn test_find_spawn_index_not_found() {
        let spawns = make_spawns();
        let idx = find_spawn_index(&spawns, 8945, 99, 71);
        assert!(idx.is_none());
    }

    // --- on_chaos_stone_death tests ---

    #[test]
    fn test_on_chaos_stone_death_advances_rank() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let result = on_chaos_stone_death(&infos, &spawns, 8945, 71);
        assert!(result.is_some());

        let info = infos.get(&1).unwrap();
        // After death: rank should advance to 2
        assert_eq!(info.rank.load(Ordering::Relaxed), 2);
        assert!(info.is_chaos_stone_killed.load(Ordering::Relaxed));
        assert!(info.is_on_res_timer.load(Ordering::Relaxed));
    }

    #[test]
    fn test_on_chaos_stone_death_wraps_rank() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        // Set rank to 4 (max for zone 71)
        let info = infos.get(&1).unwrap();
        info.rank.store(4, Ordering::Relaxed);

        let result = on_chaos_stone_death(&infos, &spawns, 8945, 71);
        assert!(result.is_some());

        // After death from rank 4: next is 5 which doesn't exist, wraps to 1
        assert_eq!(info.rank.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_on_chaos_stone_death_disabled_stone() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        // Zone 73 stone is disabled
        let result = on_chaos_stone_death(&infos, &spawns, 8947, 73);
        assert!(result.is_none());
    }

    #[test]
    fn test_on_chaos_stone_death_wrong_zone() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let result = on_chaos_stone_death(&infos, &spawns, 8945, 72);
        assert!(result.is_none());
    }

    // --- death_respawn_monsters tests ---

    #[test]
    fn test_death_respawn_monsters_returns_family_monsters() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);
        let summon_list = make_summon_list();
        let stages = make_stages();

        // Info 1 = zone 71, family starts at 1
        let monsters = death_respawn_monsters(&infos, &summon_list, &stages, 1);
        assert_eq!(monsters.len(), 3); // 8254, 8915, 8907
        assert!(monsters.contains(&8254));
        assert!(monsters.contains(&8915));
        assert!(monsters.contains(&8907));

        // After call, family should advance to 2
        let info = infos.get(&1).unwrap();
        assert_eq!(info.monster_family.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_death_respawn_monsters_family_wraps() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);
        let summon_list = make_summon_list();
        let stages = make_stages();

        // Set family to 3 (last family for zone 71)
        let info = infos.get(&1).unwrap();
        info.monster_family.store(3, Ordering::Relaxed);

        let monsters = death_respawn_monsters(&infos, &summon_list, &stages, 1);
        assert_eq!(monsters.len(), 1); // 8255 (family 3 has 1 entry in our test data)

        // Family should wrap back to 1 (family 4 doesn't exist)
        assert_eq!(info.monster_family.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_death_respawn_monsters_invalid_index() {
        let infos = HashMap::new();
        let summon_list = make_summon_list();
        let stages = make_stages();

        let monsters = death_respawn_monsters(&infos, &summon_list, &stages, 99);
        assert!(monsters.is_empty());
    }

    // --- family_exists_for_zone tests ---

    #[test]
    fn test_family_exists_for_zone_found() {
        let stages = make_stages();
        assert!(family_exists_for_zone(&stages, 71, 1));
        assert!(family_exists_for_zone(&stages, 71, 2));
        assert!(family_exists_for_zone(&stages, 71, 3));
        assert!(family_exists_for_zone(&stages, 72, 1));
    }

    #[test]
    fn test_family_exists_for_zone_not_found() {
        let stages = make_stages();
        assert!(!family_exists_for_zone(&stages, 71, 4));
        assert!(!family_exists_for_zone(&stages, 73, 1));
        assert!(!family_exists_for_zone(&stages, 99, 1));
    }

    // --- on_boss_killed tests ---

    #[test]
    fn test_on_boss_killed_decrements() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.boss_ids[0].store(100, Ordering::Relaxed);
        info.boss_ids[1].store(101, Ordering::Relaxed);
        info.boss_killed_count.store(2, Ordering::Relaxed);

        let all_dead = on_boss_killed(&infos, 100, 71);
        assert!(!all_dead);
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 1);
        assert_eq!(info.boss_ids[0].load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_on_boss_killed_last_boss() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.boss_ids[0].store(100, Ordering::Relaxed);
        info.boss_killed_count.store(1, Ordering::Relaxed);

        let all_dead = on_boss_killed(&infos, 100, 71);
        assert!(all_dead);
        assert!(info.is_total_killed_monster.load(Ordering::Relaxed));
    }

    #[test]
    fn test_on_boss_killed_no_match() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.boss_ids[0].store(100, Ordering::Relaxed);
        info.boss_killed_count.store(1, Ordering::Relaxed);

        // NPC ID 999 doesn't match any boss
        let all_dead = on_boss_killed(&infos, 999, 71);
        assert!(!all_dead);
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_on_boss_killed_stone_not_killed() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        // Stone not killed yet — should skip
        info.boss_ids[0].store(100, Ordering::Relaxed);
        info.boss_killed_count.store(1, Ordering::Relaxed);

        let all_dead = on_boss_killed(&infos, 100, 71);
        assert!(!all_dead);
    }

    // --- respawn_timer_tick tests ---

    #[test]
    fn test_respawn_timer_tick_decrements() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.is_on_res_timer.store(true, Ordering::Relaxed);
        info.spawn_time.store(5, Ordering::Relaxed);

        let results = respawn_timer_tick(&infos);
        assert!(results.is_empty()); // Not yet zero
        assert_eq!(info.spawn_time.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn test_respawn_timer_tick_triggers_respawn() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.is_on_res_timer.store(true, Ordering::Relaxed);
        info.spawn_time.store(0, Ordering::Relaxed);

        let results = respawn_timer_tick(&infos);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], ChaosStoneTimerResult::Respawn(8945, 1, 71));

        // After respawn: flags cleared
        assert!(!info.is_chaos_stone_killed.load(Ordering::Relaxed));
        assert!(!info.is_on_res_timer.load(Ordering::Relaxed));
    }

    #[test]
    fn test_respawn_timer_tick_skips_disabled() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        // Zone 73 stone is disabled
        let info = infos.get(&3).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);
        info.is_on_res_timer.store(true, Ordering::Relaxed);
        info.spawn_time.store(0, Ordering::Relaxed);

        let results = respawn_timer_tick(&infos);
        // Should not include zone 73 stone
        for r in &results {
            if let ChaosStoneTimerResult::Respawn(_, _, zone) = r {
                assert_ne!(*zone, 73);
            }
        }
    }

    #[test]
    fn test_respawn_timer_tick_idle_when_not_killed() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let results = respawn_timer_tick(&infos);
        assert!(results.is_empty());
    }

    // --- get_respawn_data tests ---

    #[test]
    fn test_get_respawn_data_found() {
        let spawns = make_spawns();
        let data = get_respawn_data(&spawns, 8945, 1, 71);
        assert!(data.is_some());
        assert_eq!(data.unwrap().spawn_x, 1017);
    }

    #[test]
    fn test_get_respawn_data_not_found() {
        let spawns = make_spawns();
        let data = get_respawn_data(&spawns, 9999, 1, 71);
        assert!(data.is_none());
    }

    #[test]
    fn test_get_respawn_data_closed_spawn() {
        let spawns = make_spawns();
        let data = get_respawn_data(&spawns, 8947, 1, 73);
        assert!(data.is_none()); // is_open=0
    }

    // --- collect_reward_items tests ---

    #[test]
    fn test_collect_reward_items_full() {
        let rewards = make_rewards();
        let items = collect_reward_items(&rewards[0]);
        assert_eq!(items.len(), 4); // 4 non-zero items
        assert_eq!(items[0], (900017000, 3, 0));
        assert_eq!(items[1], (389196000, 50, 0));
        assert_eq!(items[2], (389197000, 50, 0));
        assert_eq!(items[3], (389198000, 50, 0));
    }

    #[test]
    fn test_collect_reward_items_partial() {
        let rewards = make_rewards();
        let items = collect_reward_items(&rewards[1]);
        assert_eq!(items.len(), 2); // Only 2 non-zero items
    }

    #[test]
    fn test_collect_reward_items_single() {
        let rewards = make_rewards();
        let items = collect_reward_items(&rewards[2]);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], (389301000, 1, 0));
    }

    // --- get_reward_by_rank tests ---

    #[test]
    fn test_get_reward_by_rank_found() {
        let rewards = make_rewards();
        let r = get_reward_by_rank(&rewards, 1);
        assert!(r.is_some());
        assert_eq!(r.unwrap().experience, 250000000);
    }

    #[test]
    fn test_get_reward_by_rank_not_found() {
        let rewards = make_rewards();
        let r = get_reward_by_rank(&rewards, 99);
        assert!(r.is_none());
    }

    // --- register_spawned_bosses tests ---

    #[test]
    fn test_register_spawned_bosses_stores_ids() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);

        let npc_ids = vec![10042, 10043, 10044];
        register_spawned_bosses(&infos, 1, &npc_ids);

        assert_eq!(info.boss_ids[0].load(Ordering::Relaxed), 10042);
        assert_eq!(info.boss_ids[1].load(Ordering::Relaxed), 10043);
        assert_eq!(info.boss_ids[2].load(Ordering::Relaxed), 10044);
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_register_spawned_bosses_max_10() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let npc_ids: Vec<u32> = (10001..=10015).collect();
        register_spawned_bosses(&infos, 1, &npc_ids);

        let info = infos.get(&1).unwrap();
        // Only first 10 should be stored
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 10);
        assert_eq!(info.boss_ids[9].load(Ordering::Relaxed), 10010);
    }

    #[test]
    fn test_register_spawned_bosses_invalid_index() {
        let infos = HashMap::new();
        // Should not panic
        register_spawned_bosses(&infos, 99, &[10001, 10002]);
    }

    #[test]
    fn test_register_and_kill_full_flow() {
        let spawns = make_spawns();
        let infos = load_chaos_stones(&spawns);

        let info = infos.get(&1).unwrap();
        info.is_chaos_stone_killed.store(true, Ordering::Relaxed);

        // Simulate spawning 3 bosses
        register_spawned_bosses(&infos, 1, &[10042, 10043, 10044]);
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 3);

        // Kill first boss (runtime ID)
        assert!(!on_boss_killed(&infos, 10042, 71));
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 2);

        // Kill second boss
        assert!(!on_boss_killed(&infos, 10043, 71));
        assert_eq!(info.boss_killed_count.load(Ordering::Relaxed), 1);

        // Kill last boss — wave complete
        assert!(on_boss_killed(&infos, 10044, 71));
        assert!(info.is_total_killed_monster.load(Ordering::Relaxed));
    }

    // --- on_special_stone_death tests ---

    fn make_special_stones() -> Vec<SpecialStoneRow> {
        vec![
            SpecialStoneRow {
                n_index: 1,
                zone_id: 71,
                main_npc: 8999,
                monster_name: "Werewolf".into(),
                summon_npc: 8100,
                summon_count: 2,
                status: 1,
            },
            SpecialStoneRow {
                n_index: 2,
                zone_id: 71,
                main_npc: 8999,
                monster_name: "Skeleton".into(),
                summon_npc: 8101,
                summon_count: 3,
                status: 1,
            },
            SpecialStoneRow {
                n_index: 3,
                zone_id: 72,
                main_npc: 8999,
                monster_name: "Orc".into(),
                summon_npc: 8102,
                summon_count: 1,
                status: 1,
            },
            SpecialStoneRow {
                n_index: 4,
                zone_id: 71,
                main_npc: 9000,
                monster_name: "Boss".into(),
                summon_npc: 8200,
                summon_count: 1,
                status: 0, // disabled
            },
        ]
    }

    #[test]
    fn test_special_stone_death_returns_match() {
        let stones = make_special_stones();
        let result = on_special_stone_death(&stones, 8999, 71);
        assert!(result.is_some());
        let (npc, count) = result.unwrap();
        // Should be one of 8100 or 8101 (random)
        assert!(npc == 8100 || npc == 8101);
        assert!(count == 2 || count == 3);
    }

    #[test]
    fn test_special_stone_death_wrong_zone() {
        let stones = make_special_stones();
        let result = on_special_stone_death(&stones, 8999, 73);
        assert!(result.is_none());
    }

    #[test]
    fn test_special_stone_death_wrong_proto() {
        let stones = make_special_stones();
        let result = on_special_stone_death(&stones, 1234, 71);
        assert!(result.is_none());
    }

    #[test]
    fn test_special_stone_death_disabled_entry() {
        let stones = make_special_stones();
        // main_npc 9000 exists only in disabled entry (status=0)
        let result = on_special_stone_death(&stones, 9000, 71);
        assert!(result.is_none());
    }

    #[test]
    fn test_special_stone_death_single_match() {
        let stones = make_special_stones();
        // zone 72 has exactly 1 match for 8999
        let result = on_special_stone_death(&stones, 8999, 72);
        assert!(result.is_some());
        let (npc, count) = result.unwrap();
        assert_eq!(npc, 8102);
        assert_eq!(count, 1);
    }

    // --- Constants tests ---

    #[test]
    fn test_constants() {
        assert_eq!(CHAOS_STONE_RESPAWN_TIME, 1800);
        assert_eq!(CHAOS_STONE_MONSTER_RESPAWN_RADIUS, 20);
        assert_eq!(CHAOS_STONE_MONSTER_LIVE_TIME, 900);
        assert_eq!(MAX_BOSS_IDS, 10);
    }
}
