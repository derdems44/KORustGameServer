//! Item production (crafting loot generation) system.
//! Port of `CNpc::ItemProdution()`, `CNpc::GetItemGrade()`,
//! from .
//! These functions generate random item IDs for NPC loot drops based on
//! the MAKE_ITEM, MAKE_ITEM_GRADECODE, MAKE_ITEM_LARECODE, MAKE_WEAPON,
//! and MAKE_DEFENSIVE tables.

use crate::world::WorldState;
use rand::Rng;

/// Maximum upgrade weapon classes
const MAX_UPGRADE_WEAPON: usize = 12;

#[inline]
fn compare(x: i32, min: i32, max: i32) -> bool {
    x >= min && x < max
}

/// Port of `CNpc::GetItemGrade()`.
/// Rolls a random grade (1..9) using weighted probabilities from the
/// `make_item_gradecode` table.
/// Returns 0 if the grade code is not found or no grade matched.
pub fn get_item_grade(world: &WorldState, item_grade: i16) -> i32 {
    let grade_data = match world.get_make_grade_code(item_grade) {
        Some(d) => d,
        None => return 0,
    };

    let grades = [
        grade_data.grade_1,
        grade_data.grade_2,
        grade_data.grade_3,
        grade_data.grade_4,
        grade_data.grade_5,
        grade_data.grade_6,
        grade_data.grade_7,
        grade_data.grade_8,
        grade_data.grade_9,
    ];

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..=1000);
    let mut percent: i32 = 0;

    for (i, &grade) in grades.iter().enumerate() {
        let weight = grade as i32;
        if weight == 0 {
            percent += weight;
            continue;
        }
        if i == 0 {
            if compare(random, 0, weight) {
                return (i + 1) as i32;
            }
            percent += weight;
        } else {
            if compare(random, percent, percent + weight) {
                return (i + 1) as i32;
            }
            percent += weight;
        }
    }

    0
}

/// Port of `CNpc::GetWeaponItemCodeNumber()`.
/// Rolls a random weapon/defensive class (1..12) using weighted probabilities
/// from the `make_weapon` or `make_defensive` table.
/// Returns 0 if the table entry is not found or no class matched.
pub fn get_weapon_item_code_number(world: &WorldState, npc_level: i32, is_weapon: bool) -> i32 {
    let by_level = (npc_level / 10) as i16;

    if is_weapon {
        let data = match world.get_make_weapon(by_level) {
            Some(d) => d,
            None => return 0,
        };
        let classes = [
            data.class_1,
            data.class_2,
            data.class_3,
            data.class_4,
            data.class_5,
            data.class_6,
            data.class_7,
            data.class_8,
            data.class_9,
            data.class_10,
            data.class_11,
            data.class_12,
        ];
        roll_weapon_class(&classes)
    } else {
        let data = match world.get_make_defensive(by_level) {
            Some(d) => d,
            None => return 0,
        };
        let classes = [
            data.class_1,
            data.class_2,
            data.class_3,
            data.class_4,
            data.class_5,
            data.class_6,
            data.class_7,
            0,
            0,
            0,
            0,
            0,
        ];
        roll_weapon_class(&classes)
    }
}

/// Roll a weapon/defensive class from weighted probabilities.
fn roll_weapon_class(classes: &[i16; MAX_UPGRADE_WEAPON]) -> i32 {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=1000);
    let mut percent: i32 = 0;

    for (i, &class) in classes.iter().enumerate() {
        let weight = class as i32;
        if weight == 0 {
            percent += weight;
            continue;
        }
        if compare(random, percent, percent + weight) {
            return (i + 1) as i32;
        }
        percent += weight;
    }

    0
}

/// Port of `CNpc::GetItemCodeNumber()`.
/// Rolls a random item code number based on rarity (rare/magic/general)
/// using the `make_item_larecode` table, then generates a sub-code
/// depending on the item_type (1=weapon, 2=defensive, 3=accessory).
/// Returns -1 on error (no lare code data found).
pub fn get_item_code_number(world: &WorldState, level: i32, item_type: i32) -> i32 {
    let lare_data = match world.get_make_lare_code(level as i16) {
        Some(d) => d,
        None => return -1,
    };

    let item_percents = [
        lare_data.lare_item as i32,
        lare_data.magic_item as i32,
        lare_data.general_item as i32,
    ];

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=1000);
    let mut item_type_result = 0i32;
    let mut percent = 0i32;

    for (i, &pct) in item_percents.iter().enumerate() {
        if i == 0 {
            if compare(random, 0, pct) {
                item_type_result = (i + 1) as i32;
                break;
            }
            percent += pct;
        } else {
            if compare(random, percent, percent + pct) {
                item_type_result = (i + 1) as i32;
                break;
            }
            percent += pct;
        }
    }

    match item_type_result {
        1 => {
            // Rare item
            match item_type {
                1 => rng.gen_range(16..=24),
                2 => rng.gen_range(12..=24),
                3 => rng.gen_range(0..=10),
                _ => 0,
            }
        }
        2 => {
            // Magic item
            match item_type {
                1 => rng.gen_range(6..=15),
                2 => rng.gen_range(6..=11),
                3 => rng.gen_range(0..=10),
                _ => 0,
            }
        }
        3 => {
            // General item
            match item_type {
                1 | 2 => 5,
                3 => rng.gen_range(0..=10),
                _ => 0,
            }
        }
        _ => 0,
    }
}

/// Port of `CNpc::ItemProdution()`.
/// Generates a random item ID for NPC loot drops.
/// Uses the MAKE_ITEM_GRADECODE, MAKE_WEAPON, MAKE_DEFENSIVE,
/// and MAKE_ITEM_LARECODE tables.
/// `item_number` is the grade code index from the monster's drop table.
/// `npc_level` is the NPC's level.
/// `max_damaged_nation` is the nation of the player who dealt the most damage (1=Karus, 2=Elmorad).
/// Returns 0 if generation fails.
pub fn item_production(
    world: &WorldState,
    item_number: i32,
    npc_level: i32,
    max_damaged_nation: u8,
) -> u32 {
    let item_grade = get_item_grade(world, item_number as i16);
    if item_grade == 0 {
        return 0;
    }
    let item_level = npc_level / 5;

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..=10000);

    if compare(random, 1, 4001) {
        // Weapon (40% chance)
        let i_default: i64 = 100_000_000;
        let random2 = rng.gen_range(1..=10000);
        let i_rand2: i64 = match () {
            _ if compare(random2, 1, 701) => 10_000_000,
            _ if compare(random2, 701, 1401) => 20_000_000,
            _ if compare(random2, 1401, 2101) => 30_000_000,
            _ if compare(random2, 2101, 2801) => 40_000_000,
            _ if compare(random2, 2801, 3501) => 50_000_000,
            _ if compare(random2, 3501, 5501) => 60_000_000,
            _ if compare(random2, 5501, 6501) => 70_000_000,
            _ if compare(random2, 6501, 8501) => 80_000_000,
            _ => 90_000_000,
        };

        let temp1 = get_weapon_item_code_number(world, npc_level, true);
        if temp1 == 0 {
            return 0;
        }
        let i_item_code: i64 = temp1 as i64 * 100_000;

        let i_rand3: i64 = if rng.gen_range(1..=10000) < 5000 {
            10_000
        } else {
            50_000
        };

        let i_rand4: i64 = if rng.gen_range(1..=10000) < 5000 {
            0
        } else {
            5_000_000
        };

        let code_num = get_item_code_number(world, item_level, 1);
        if code_num == -1 {
            return 0;
        }
        let i_rand5: i64 = code_num as i64 * 10;

        (i_default + i_item_code + i_rand2 + i_rand3 + i_rand4 + i_rand5 + item_grade as i64) as u32
    } else if compare(random, 4001, 8001) {
        // Defensive (40% chance)
        let i_default: i64 = 200_000_000;

        let temp1 = get_weapon_item_code_number(world, npc_level, false);
        if temp1 == 0 {
            return 0;
        }
        let i_item_code: i64 = temp1 as i64 * 1_000_000;

        let (i_rand2, i_rand3): (i64, i64) = if max_damaged_nation == 1 {
            // Karus
            let r = rng.gen_range(0..=10000);
            if compare(r, 0, 2000) {
                (0, 10_000)
            } else if compare(r, 2000, 4000) {
                (40_000_000, 20_000)
            } else if compare(r, 4000, 6000) {
                (60_000_000, 30_000)
            } else {
                let sub = rng.gen_range(0..=10000);
                if compare(sub, 0, 5000) {
                    (80_000_000, 20_000)
                } else {
                    (80_000_000, 40_000)
                }
            }
        } else {
            // Elmorad
            let r = rng.gen_range(0..=10000);
            if compare(r, 0, 3300) {
                let key = rng.gen_range(0..=10000);
                let r3 = if compare(key, 0, 3333) {
                    110_000
                } else if compare(key, 3333, 6666) {
                    120_000
                } else {
                    130_000
                };
                (0, r3)
            } else if compare(r, 3300, 5600) {
                let key = rng.gen_range(0..=10000);
                let r3 = if compare(key, 0, 5000) {
                    120_000
                } else {
                    130_000
                };
                (40_000_000, r3)
            } else if compare(r, 5600, 7800) {
                let key = rng.gen_range(0..=10000);
                let r3 = if compare(key, 0, 5000) {
                    120_000
                } else {
                    130_000
                };
                (60_000_000, r3)
            } else {
                let key = rng.gen_range(0..=10000);
                let r3 = if compare(key, 0, 5000) {
                    120_000
                } else {
                    130_000
                };
                (80_000_000, r3)
            }
        };

        let temp2 = rng.gen_range(0..=10000);
        let i_rand4: i64 = match () {
            _ if compare(temp2, 0, 2000) => 1_000,
            _ if compare(temp2, 2000, 4000) => 2_000,
            _ if compare(temp2, 4000, 6000) => 3_000,
            _ if compare(temp2, 6000, 8000) => 4_000,
            _ => 5_000,
        };

        let code_num = get_item_code_number(world, item_level, 2);
        if code_num == -1 {
            return 0;
        }
        let i_rand5: i64 = code_num as i64 * 10;

        (i_default + i_rand2 + i_item_code + i_rand3 + i_rand4 + i_rand5 + item_grade as i64) as u32
    } else {
        // Accessory (20% chance)
        let i_default: i64 = 300_000_000;
        let r = rng.gen_range(0..=10000);
        let i_rand2: i64 = match () {
            _ if compare(r, 0, 2500) => 10_000_000,
            _ if compare(r, 2500, 5000) => 20_000_000,
            _ if compare(r, 5000, 7500) => 30_000_000,
            _ => 40_000_000,
        };

        let i_rand3: i64 = if rng.gen_range(1..=10000) < 5000 {
            110_000
        } else {
            150_000
        };

        let code_num = get_item_code_number(world, item_level, 3);
        if code_num == -1 {
            return 0;
        }
        let i_rand4: i64 = code_num as i64 * 10;

        (i_default + i_rand2 + i_rand3 + i_rand4 + item_grade as i64) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::WorldState;
    use ko_db::models::{
        MakeDefensiveRow, MakeItemGradeCodeRow, MakeItemLareCodeRow, MakeItemRow, MakeWeaponRow,
    };

    /// Helper: build a minimal WorldState with just the tables we need.
    fn test_world() -> WorldState {
        let world = WorldState::new();
        // Insert grade code for index 5 (weighted distribution)
        world.insert_test_make_grade_code(MakeItemGradeCodeRow {
            item_index: 5,
            grade_1: 0,
            grade_2: 100,
            grade_3: 200,
            grade_4: 400,
            grade_5: 300,
            grade_6: 0,
            grade_7: 0,
            grade_8: 0,
            grade_9: 0,
        });

        // Insert lare code for level 6 (level 30 / 5)
        world.insert_test_make_lare_code(MakeItemLareCodeRow {
            level_grade: 6,
            lare_item: 250,
            magic_item: 250,
            general_item: 500,
        });

        // Insert weapon data for by_level 3 (level 30 / 10)
        world.insert_test_make_weapon(MakeWeaponRow {
            by_level: 3,
            class_1: 100,
            class_2: 100,
            class_3: 100,
            class_4: 100,
            class_5: 100,
            class_6: 100,
            class_7: 100,
            class_8: 100,
            class_9: 100,
            class_10: 100,
            class_11: 0,
            class_12: 0,
        });

        // Insert defensive data for by_level 3
        world.insert_test_make_defensive(MakeDefensiveRow {
            by_level: 3,
            class_1: 200,
            class_2: 200,
            class_3: 200,
            class_4: 200,
            class_5: 100,
            class_6: 50,
            class_7: 50,
        });

        // Insert a make_item entry
        world.insert_test_make_item(MakeItemRow {
            s_index: 1,
            item_code: 410,
            item_level: 15,
        });

        world
    }

    #[test]
    fn test_compare() {
        assert!(compare(5, 1, 10));
        assert!(compare(1, 1, 10));
        assert!(!compare(10, 1, 10));
        assert!(!compare(0, 1, 10));
    }

    #[test]
    fn test_get_item_grade_missing_data() {
        let world = WorldState::new();
        assert_eq!(get_item_grade(&world, 99), 0);
    }

    #[test]
    fn test_get_item_grade_returns_valid_grade() {
        let world = test_world();
        let mut found_nonzero = false;
        // Run many times; grade 0 is valid (1/1000 edge case per C++ behavior)
        for _ in 0..200 {
            let grade = get_item_grade(&world, 5);
            // Grade should be 0 (edge), or 2-5 (grades with non-zero weights)
            assert!(
                grade == 0 || (2..=5).contains(&grade),
                "Expected grade 0 or 2-5, got {grade}"
            );
            if grade > 0 {
                found_nonzero = true;
            }
        }
        assert!(found_nonzero, "Should produce at least one non-zero grade");
    }

    #[test]
    fn test_get_weapon_item_code_number_missing_data() {
        let world = WorldState::new();
        assert_eq!(get_weapon_item_code_number(&world, 30, true), 0);
        assert_eq!(get_weapon_item_code_number(&world, 30, false), 0);
    }

    #[test]
    fn test_get_weapon_item_code_number_weapon() {
        let world = test_world();
        let mut found_nonzero = false;
        for _ in 0..200 {
            let code = get_weapon_item_code_number(&world, 30, true);
            // Classes 1-10 have weight 100 each; 0 is edge case (random=1000)
            assert!((0..=10).contains(&code), "Expected 0-10, got {code}");
            if code > 0 {
                found_nonzero = true;
            }
        }
        assert!(found_nonzero, "Should produce at least one non-zero class");
    }

    #[test]
    fn test_get_weapon_item_code_number_defensive() {
        let world = test_world();
        let mut found_nonzero = false;
        for _ in 0..200 {
            let code = get_weapon_item_code_number(&world, 30, false);
            // Defensive classes 1-7; 0 is edge case (random=1000)
            assert!((0..=7).contains(&code), "Expected 0-7, got {code}");
            if code > 0 {
                found_nonzero = true;
            }
        }
        assert!(found_nonzero, "Should produce at least one non-zero class");
    }

    #[test]
    fn test_get_item_code_number_missing_data() {
        let world = WorldState::new();
        assert_eq!(get_item_code_number(&world, 6, 1), -1);
    }

    #[test]
    fn test_get_item_code_number_weapon_type() {
        let world = test_world();
        for _ in 0..100 {
            let code = get_item_code_number(&world, 6, 1);
            // Weapon type: rare=16..24, magic=6..15, general=5
            assert!((0..=24).contains(&code), "Expected 0-24, got {code}");
        }
    }

    #[test]
    fn test_get_item_code_number_defensive_type() {
        let world = test_world();
        for _ in 0..100 {
            let code = get_item_code_number(&world, 6, 2);
            // Defensive type: rare=12..24, magic=6..11, general=5
            assert!((0..=24).contains(&code), "Expected 0-24, got {code}");
        }
    }

    #[test]
    fn test_get_item_code_number_accessory_type() {
        let world = test_world();
        for _ in 0..100 {
            let code = get_item_code_number(&world, 6, 3);
            // Accessory type: all use 0..10
            assert!((0..=10).contains(&code), "Expected 0-10, got {code}");
        }
    }

    #[test]
    fn test_item_production_missing_grade() {
        let world = WorldState::new();
        assert_eq!(item_production(&world, 99, 30, 1), 0);
    }

    #[test]
    fn test_item_production_generates_valid_item() {
        let world = test_world();
        let mut generated_count = 0;
        for _ in 0..100 {
            let item_id = item_production(&world, 5, 30, 1);
            if item_id > 0 {
                generated_count += 1;
                // Item ID should be >= 100_000_000 (weapon/defensive/accessory)
                assert!(item_id >= 100_000_000, "Expected >= 100M, got {item_id}");
            }
        }
        // Should generate at least some valid items
        assert!(generated_count > 0, "No items were generated");
    }

    #[test]
    fn test_item_production_karus_nation() {
        let world = test_world();
        let mut found_items = false;
        for _ in 0..50 {
            let item_id = item_production(&world, 5, 30, 1);
            if item_id > 0 {
                found_items = true;
            }
        }
        assert!(found_items, "Karus nation should produce items");
    }

    #[test]
    fn test_item_production_elmorad_nation() {
        let world = test_world();
        let mut found_items = false;
        for _ in 0..50 {
            let item_id = item_production(&world, 5, 30, 2);
            if item_id > 0 {
                found_items = true;
            }
        }
        assert!(found_items, "Elmorad nation should produce items");
    }

    #[test]
    fn test_make_item_lookup() {
        let world = test_world();
        let item = world.get_make_item(1).unwrap();
        assert_eq!(item.item_code, 410);
        assert_eq!(item.item_level, 15);
        assert!(world.get_make_item(9999).is_none());
    }

    #[test]
    fn test_grade_all_weights_zero_returns_zero() {
        let world = WorldState::new();
        world.insert_test_make_grade_code(MakeItemGradeCodeRow {
            item_index: 1,
            grade_1: 0,
            grade_2: 0,
            grade_3: 0,
            grade_4: 0,
            grade_5: 0,
            grade_6: 0,
            grade_7: 0,
            grade_8: 0,
            grade_9: 0,
        });
        assert_eq!(get_item_grade(&world, 1), 0);
    }

    #[test]
    fn test_weapon_class_all_zero_returns_zero() {
        let world = WorldState::new();
        world.insert_test_make_weapon(MakeWeaponRow {
            by_level: 0,
            class_1: 0,
            class_2: 0,
            class_3: 0,
            class_4: 0,
            class_5: 0,
            class_6: 0,
            class_7: 0,
            class_8: 0,
            class_9: 0,
            class_10: 0,
            class_11: 0,
            class_12: 0,
        });
        assert_eq!(get_weapon_item_code_number(&world, 0, true), 0);
    }
}
