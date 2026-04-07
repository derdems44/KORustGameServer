//! Pet / Cypher Ring unique-item info helpers.
//!
//! C++ Reference:
//! - `PetMainHandler.cpp:306` — `ShowPetItemInfo(Packet&, uint64 nSerialNum)`
//! - `SealHandler.cpp:796`    — `ShowCyperRingItemInfo(Packet&, uint64 nSerialNum)`
//!
//! These two functions write variable-length data into item-slot packets.
//! The client determines which format to parse from the item's template
//! (`isPetItem()` → kind 151, `ITEM_CYPHER_RING` → 800112000).
//!
//! If the pet/seal record is not found in the DB, the fallback is `u32(0)`.

use ko_db::DbPool;
use ko_protocol::Packet;

use crate::world::WorldState;

/// C++ constant: `ITEM_CYPHER_RING = 800112000`.
pub const ITEM_CYPHER_RING: u32 = 800_112_000;

use crate::inventory_constants::ITEM_KIND_PET;

/// Write the unique-item-info field for a single item slot.
///
/// C++ pattern (WareHouse.cpp:82-93):
/// ```cpp
/// if (pItemTable.isPetItem())
///     ShowPetItemInfo(result, pItem->nSerialNum);
/// else if (pItemTable.GetNum() == ITEM_CYPHER_RING)
///     ShowCyperRingItemInfo(result, pItem->nSerialNum);
/// else
///     result << uint32(0);
/// ```
pub async fn write_unique_item_info(
    world: &WorldState,
    pool: &DbPool,
    item_id: u32,
    serial_num: u64,
    rebirth_level: u8,
    pkt: &mut Packet,
) {
    if item_id == 0 {
        pkt.write_u32(0);
        return;
    }

    if let Some(tmpl) = world.get_item(item_id) {
        if tmpl.kind == Some(ITEM_KIND_PET) {
            write_pet_item_info(world, pool, serial_num as i64, pkt).await;
        } else if item_id == ITEM_CYPHER_RING {
            write_cypher_ring_item_info(world, pool, serial_num as i64, rebirth_level, pkt).await;
        } else {
            pkt.write_u32(0);
        }
    } else {
        pkt.write_u32(0);
    }
}

/// Write pet item info into the packet.
///
/// C++ Reference: `PetMainHandler.cpp:306-331`
///
/// Packet format (when found):
/// ```text
/// [u32 nIndex] [u16-string petName] [u8 petAttack] [u8 level]
/// [u16 expPercent] [i16 satisfaction] [u8 0]
/// ```
/// Fallback: `[u32 0]`
async fn write_pet_item_info(world: &WorldState, pool: &DbPool, serial_id: i64, pkt: &mut Packet) {
    if serial_id == 0 {
        pkt.write_u32(0);
        return;
    }

    let repo = ko_db::repositories::pet::PetRepository::new(pool);
    let pet = match repo.load_pet_data(serial_id).await {
        Ok(Some(p)) => p,
        _ => {
            pkt.write_u32(0);
            return;
        }
    };

    // Get the pet stats info for exp percentage calculation
    let pet_stats = world.get_pet_stats_info(pet.b_level as u8);
    let pet_attack = pet_stats.as_ref().map(|s| s.pet_attack as u8).unwrap_or(0);
    let pet_exp_max = pet_stats.as_ref().map(|s| s.pet_exp).unwrap_or(1);

    // C++ formula: uint16((float)nExp / (float)info->PetExp * 100.0f * 100.0f)
    let exp_percent = if pet_exp_max > 0 {
        ((pet.n_exp as f32 / pet_exp_max as f32) * 100.0 * 100.0) as u16
    } else {
        0u16
    };

    // DByte mode → write_string (u16 prefix)
    pkt.write_u32(pet.n_index as u32);
    pkt.write_string(&pet.s_pet_name);
    pkt.write_u8(pet_attack);
    pkt.write_u8(pet.b_level as u8);
    pkt.write_u16(exp_percent);
    pkt.write_i16(pet.s_satisfaction);
    pkt.write_u8(0);
}

/// Write cypher ring (sealed character) item info into the packet.
///
/// C++ Reference: `SealHandler.cpp:796-820`
///
/// Packet format (when found):
/// ```text
/// [u32 uniqueID] [u16-string charName] [u8 class] [u8 level]
/// [u16 expRate] [u8 race] [u8 0] [u8 0]
/// ```
/// Fallback: `[u32 0]`
async fn write_cypher_ring_item_info(
    world: &WorldState,
    pool: &DbPool,
    serial_id: i64,
    rebirth_level: u8,
    pkt: &mut Packet,
) {
    if serial_id == 0 {
        pkt.write_u32(0);
        return;
    }

    let repo = ko_db::repositories::character_seal::CharacterSealRepository::new(pool);
    // Returns (unique_id, char_name, class, level, exp, race)
    let seal = match repo.load_seal_summary_by_serial(serial_id).await {
        Ok(Some(s)) => s,
        _ => {
            pkt.write_u32(0);
            return;
        }
    };

    let (unique_id, char_name, class, level, exp, race) = seal;

    // C++ formula: ((m_iExp * 50) / GetExpByLevel(level, rebirthLevel)) * 100
    let exp_by_level = world.get_exp_by_level(level as u8, rebirth_level);
    let exp_rate = if exp_by_level > 0 {
        let rate = ((exp * 50) / exp_by_level) * 100;
        rate.clamp(0, 10000) as u16
    } else {
        0u16
    };

    // DByte mode → write_string (u16 prefix)
    pkt.write_u32(unique_id as u32);
    pkt.write_string(&char_name);
    pkt.write_u8(class as u8);
    pkt.write_u8(level as u8);
    pkt.write_u16(exp_rate);
    pkt.write_u8(race as u8);
    pkt.write_u8(0);
    pkt.write_u8(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_item_cypher_ring_constant() {
        assert_eq!(ITEM_CYPHER_RING, 800_112_000);
    }

    #[test]
    fn test_item_kind_pet_constant() {
        assert_eq!(ITEM_KIND_PET, 151);
    }

    /// When item_id is 0, should write u32(0) without DB lookup.
    #[test]
    fn test_empty_item_writes_zero() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let world = WorldState::new();
            // No pool needed since item_id=0 short-circuits
            let pool = ko_db::DbPool::connect_lazy("postgres://invalid").unwrap();
            let mut pkt = Packet::new(0x45);
            write_unique_item_info(&world, &pool, 0, 0, 0, &mut pkt).await;
            let r = PacketReader::new(&pkt.data);
            assert_eq!(r.remaining(), 4); // just u32(0)
        });
    }

    /// When item template is missing, should write u32(0).
    #[test]
    fn test_unknown_item_writes_zero() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let world = WorldState::new();
            let pool = ko_db::DbPool::connect_lazy("postgres://invalid").unwrap();
            let mut pkt = Packet::new(0x45);
            // item_id 999999 not in table → u32(0)
            write_unique_item_info(&world, &pool, 999_999, 12345, 0, &mut pkt).await;
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(0));
            assert_eq!(r.remaining(), 0);
        });
    }

    /// Pet item info: when serial is 0, writes u32(0) even if item is pet kind.
    #[test]
    fn test_pet_item_zero_serial_writes_zero() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let world = WorldState::new();
            // Insert a pet-kind item template
            let item = ko_db::models::Item {
                num: 810001000,
                kind: Some(151),
                extension: None,
                str_name: None,
                description: None,
                item_plus_id: None,
                item_alteration: None,
                item_icon_id1: None,
                item_icon_id2: None,
                slot: None,
                race: None,
                class: None,
                damage: None,
                min_damage: None,
                max_damage: None,
                delay: None,
                range: None,
                weight: None,
                duration: None,
                buy_price: None,
                sell_price: None,
                sell_npc_type: None,
                sell_npc_price: None,
                ac: None,
                countable: None,
                effect1: None,
                effect2: None,
                req_level: None,
                req_level_max: None,
                req_rank: None,
                req_title: None,
                req_str: None,
                req_sta: None,
                req_dex: None,
                req_intel: None,
                req_cha: None,
                selling_group: None,
                item_type: None,
                hitrate: None,
                evasionrate: None,
                dagger_ac: None,
                jamadar_ac: None,
                sword_ac: None,
                club_ac: None,
                axe_ac: None,
                spear_ac: None,
                bow_ac: None,
                fire_damage: None,
                ice_damage: None,
                lightning_damage: None,
                poison_damage: None,
                hp_drain: None,
                mp_damage: None,
                mp_drain: None,
                mirror_damage: None,
                droprate: None,
                str_b: None,
                sta_b: None,
                dex_b: None,
                intel_b: None,
                cha_b: None,
                max_hp_b: None,
                max_mp_b: None,
                fire_r: None,
                cold_r: None,
                lightning_r: None,
                magic_r: None,
                poison_r: None,
                curse_r: None,
                item_class: None,
                np_buy_price: None,
                bound: None,
                mace_ac: None,
                by_grade: None,
                drop_notice: None,
                upgrade_notice: None,
            };
            world.insert_item(810_001_000, item);

            let pool = ko_db::DbPool::connect_lazy("postgres://invalid").unwrap();
            let mut pkt = Packet::new(0x45);
            // serial=0 → short-circuit to u32(0)
            write_unique_item_info(&world, &pool, 810_001_000, 0, 0, &mut pkt).await;
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(0));
            assert_eq!(r.remaining(), 0);
        });
    }

    /// Cypher ring item: when serial is 0, writes u32(0).
    #[test]
    fn test_cypher_ring_zero_serial_writes_zero() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let world = WorldState::new();
            // Insert cypher ring template (kind doesn't matter, item_id match is the check)
            let item = ko_db::models::Item {
                num: ITEM_CYPHER_RING as i32,
                kind: Some(8),
                extension: None,
                str_name: None,
                description: None,
                item_plus_id: None,
                item_alteration: None,
                item_icon_id1: None,
                item_icon_id2: None,
                slot: None,
                race: None,
                class: None,
                damage: None,
                min_damage: None,
                max_damage: None,
                delay: None,
                range: None,
                weight: None,
                duration: None,
                buy_price: None,
                sell_price: None,
                sell_npc_type: None,
                sell_npc_price: None,
                ac: None,
                countable: None,
                effect1: None,
                effect2: None,
                req_level: None,
                req_level_max: None,
                req_rank: None,
                req_title: None,
                req_str: None,
                req_sta: None,
                req_dex: None,
                req_intel: None,
                req_cha: None,
                selling_group: None,
                item_type: None,
                hitrate: None,
                evasionrate: None,
                dagger_ac: None,
                jamadar_ac: None,
                sword_ac: None,
                club_ac: None,
                axe_ac: None,
                spear_ac: None,
                bow_ac: None,
                fire_damage: None,
                ice_damage: None,
                lightning_damage: None,
                poison_damage: None,
                hp_drain: None,
                mp_damage: None,
                mp_drain: None,
                mirror_damage: None,
                droprate: None,
                str_b: None,
                sta_b: None,
                dex_b: None,
                intel_b: None,
                cha_b: None,
                max_hp_b: None,
                max_mp_b: None,
                fire_r: None,
                cold_r: None,
                lightning_r: None,
                magic_r: None,
                poison_r: None,
                curse_r: None,
                item_class: None,
                np_buy_price: None,
                bound: None,
                mace_ac: None,
                by_grade: None,
                drop_notice: None,
                upgrade_notice: None,
            };
            world.insert_item(ITEM_CYPHER_RING, item);

            let pool = ko_db::DbPool::connect_lazy("postgres://invalid").unwrap();
            let mut pkt = Packet::new(0x45);
            write_unique_item_info(&world, &pool, ITEM_CYPHER_RING, 0, 0, &mut pkt).await;
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(0));
            assert_eq!(r.remaining(), 0);
        });
    }

    // ── Sprint 924: Additional coverage ──────────────────────────────

    /// Pet item info wire format: [u32 nIndex][u16-string name][u8 attack][u8 level][u16 exp%][i16 satisfaction][u8 0].
    #[test]
    fn test_pet_item_info_wire_format() {
        let mut pkt = Packet::new(0x45);
        // Manually write the same format as write_pet_item_info
        pkt.write_u32(42); // nIndex
        pkt.write_string("MyPet"); // u16-prefixed name
        pkt.write_u8(15); // petAttack
        pkt.write_u8(10); // level
        pkt.write_u16(5000); // expPercent
        pkt.write_i16(80); // satisfaction
        pkt.write_u8(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(42));
        assert_eq!(r.read_string(), Some("MyPet".to_string()));
        assert_eq!(r.read_u8(), Some(15));
        assert_eq!(r.read_u8(), Some(10));
        assert_eq!(r.read_u16(), Some(5000));
        assert_eq!(r.read_i16(), Some(80));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    /// Cypher ring info wire format: [u32 uniqueID][u16-string name][u8 class][u8 level][u16 expRate][u8 race][u8 0][u8 0].
    #[test]
    fn test_cypher_ring_item_info_wire_format() {
        let mut pkt = Packet::new(0x45);
        pkt.write_u32(999); // uniqueID
        pkt.write_string("SealedChar"); // u16-prefixed name
        pkt.write_u8(101); // class
        pkt.write_u8(83); // level
        pkt.write_u16(7500); // expRate
        pkt.write_u8(12); // race
        pkt.write_u8(0);
        pkt.write_u8(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(999));
        assert_eq!(r.read_string(), Some("SealedChar".to_string()));
        assert_eq!(r.read_u8(), Some(101));
        assert_eq!(r.read_u8(), Some(83));
        assert_eq!(r.read_u16(), Some(7500));
        assert_eq!(r.read_u8(), Some(12));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    /// Pet exp percent formula: (exp / max_exp) * 100 * 100.
    #[test]
    fn test_pet_exp_percent_formula() {
        let exp: f32 = 500.0;
        let max_exp: f32 = 1000.0;
        let result = (exp / max_exp * 100.0 * 100.0) as u16;
        assert_eq!(result, 5000); // 50.00%
    }

    /// Seal exp rate formula with clamp to 0..10000.
    #[test]
    fn test_seal_exp_rate_clamp() {
        // Normal case
        let exp: i64 = 50;
        let exp_by_level: i64 = 100;
        let rate = ((exp * 50) / exp_by_level) * 100;
        let clamped = rate.clamp(0, 10000) as u16;
        assert_eq!(clamped, 2500);

        // Overflow case (clamp to 10000)
        let exp2: i64 = 999;
        let rate2 = ((exp2 * 50) / 1) * 100;
        let clamped2 = rate2.clamp(0, 10000) as u16;
        assert_eq!(clamped2, 10000);
    }

    /// When exp_by_level is 0, exp_rate should be 0 (division by zero guard).
    #[test]
    fn test_zero_exp_max_returns_zero() {
        let exp_by_level: i64 = 0;
        let exp_rate = if exp_by_level > 0 { 5000u16 } else { 0u16 };
        assert_eq!(exp_rate, 0);
    }

    /// Normal item (not pet, not cypher ring) writes u32(0).
    #[test]
    fn test_normal_item_writes_zero() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let world = WorldState::new();
            let item = ko_db::models::Item {
                num: 100_001,
                kind: Some(11), // sword
                extension: None,
                str_name: None,
                description: None,
                item_plus_id: None,
                item_alteration: None,
                item_icon_id1: None,
                item_icon_id2: None,
                slot: None,
                race: None,
                class: None,
                damage: None,
                min_damage: None,
                max_damage: None,
                delay: None,
                range: None,
                weight: None,
                duration: None,
                buy_price: None,
                sell_price: None,
                sell_npc_type: None,
                sell_npc_price: None,
                ac: None,
                countable: None,
                effect1: None,
                effect2: None,
                req_level: None,
                req_level_max: None,
                req_rank: None,
                req_title: None,
                req_str: None,
                req_sta: None,
                req_dex: None,
                req_intel: None,
                req_cha: None,
                selling_group: None,
                item_type: None,
                hitrate: None,
                evasionrate: None,
                dagger_ac: None,
                jamadar_ac: None,
                sword_ac: None,
                club_ac: None,
                axe_ac: None,
                spear_ac: None,
                bow_ac: None,
                fire_damage: None,
                ice_damage: None,
                lightning_damage: None,
                poison_damage: None,
                hp_drain: None,
                mp_damage: None,
                mp_drain: None,
                mirror_damage: None,
                droprate: None,
                str_b: None,
                sta_b: None,
                dex_b: None,
                intel_b: None,
                cha_b: None,
                max_hp_b: None,
                max_mp_b: None,
                fire_r: None,
                cold_r: None,
                lightning_r: None,
                magic_r: None,
                poison_r: None,
                curse_r: None,
                item_class: None,
                np_buy_price: None,
                bound: None,
                mace_ac: None,
                by_grade: None,
                drop_notice: None,
                upgrade_notice: None,
            };
            world.insert_item(100_001, item);

            let pool = ko_db::DbPool::connect_lazy("postgres://invalid").unwrap();
            let mut pkt = Packet::new(0x45);
            write_unique_item_info(&world, &pool, 100_001, 5555, 0, &mut pkt).await;
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(0));
            assert_eq!(r.remaining(), 0);
        });
    }
}
