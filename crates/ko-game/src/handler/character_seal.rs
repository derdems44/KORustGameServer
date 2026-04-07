//! Character Seal handler — WIZ_ITEM_UPGRADE (0x5B) sub-opcode 9.
//!
//! C++ Reference: `SealHandler.cpp` lines 206-820
//!
//! Allows a player to seal an alt character into a Cypher Ring item,
//! preview the sealed character, and later restore it to an empty slot.
//!
//! ## Sub-Opcodes (CharacterSealOpcodes)
//!
//! | Sub-Op | Name         | Description                              |
//! |--------|-------------|------------------------------------------|
//! | 1      | ShowList    | List all account characters               |
//! | 2      | UseScroll   | Seal character → Cypher Ring              |
//! | 3      | UseRing     | Restore character from Cypher Ring         |
//! | 4      | Preview     | Preview sealed character stats/items       |
//! | 5      | Echo        | Echo sub-opcode 5 back                    |
//! | 6      | AchieveList | List achievements of sealed character     |

use ko_db::repositories::account::AccountRepository;
use ko_db::repositories::character::CharacterRepository;
use ko_db::repositories::character_seal::CharacterSealRepository;
use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};
use crate::world::ITEM_FLAG_NONE;

use super::{HAVE_MAX, SLOT_MAX};

// ── Constants ──────────────────────────────────────────────────────────────

/// C++ `ITEM_CHARACTER_SEAL = 9` — WIZ_ITEM_UPGRADE sub-opcode.
pub const ITEM_CHARACTER_SEAL: u8 = 9;

/// C++ `ITEM_SEAL_SCROLL = 800111000` — scroll consumed to seal a character.
const ITEM_SEAL_SCROLL: u32 = 800111000;

use super::unique_item_info::ITEM_CYPHER_RING;

use super::INVENTORY_COSP;

// ── Sub-Opcodes ────────────────────────────────────────────────────────────

/// C++ `CharacterSealOpcodes` enum.
const SEAL_SHOW_LIST: u8 = 1;
const SEAL_USE_SCROLL: u8 = 2;
const SEAL_USE_RING: u8 = 3;
const SEAL_PREVIEW: u8 = 4;
const SEAL_ECHO: u8 = 5;
const SEAL_ACHIEVE_LIST: u8 = 6;

// ── Handler ────────────────────────────────────────────────────────────────

/// Send UseScroll error response (u16 = 0).
async fn send_scroll_error(session: &mut ClientSession) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
    pkt.write_u8(ITEM_CHARACTER_SEAL);
    pkt.write_u8(SEAL_USE_SCROLL);
    pkt.write_u16(0);
    session.send_packet(&pkt).await?;
    Ok(())
}

/// Send UseRing error response.
async fn send_ring_error(session: &mut ClientSession, code: u16) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
    pkt.write_u8(ITEM_CHARACTER_SEAL);
    pkt.write_u8(SEAL_USE_RING);
    pkt.write_u16(code);
    session.send_packet(&pkt).await?;
    Ok(())
}

/// Handle `CharacterSealProcess` — dispatches to sub-handlers.
///
/// C++ Reference: `CUser::CharacterSealProcess(Packet& pkt)`
pub async fn handle(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let sub_opcode = reader.read_u8().unwrap_or(0);

    match sub_opcode {
        SEAL_SHOW_LIST => handle_show_list(session).await,
        SEAL_USE_SCROLL => handle_use_scroll(session, reader).await,
        SEAL_USE_RING => handle_use_ring(session, reader).await,
        SEAL_PREVIEW => handle_preview(session, reader).await,
        SEAL_ECHO => {
            // C++ just echoes sub-opcode 5 back
            let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
            pkt.write_u8(ITEM_CHARACTER_SEAL);
            pkt.write_u8(SEAL_ECHO);
            session.send_packet(&pkt).await?;
            Ok(())
        }
        SEAL_ACHIEVE_LIST => handle_achieve_list(session, reader).await,
        _ => {
            warn!(
                "[{}] Character Seal: unhandled sub-opcode 0x{:02X}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

// ── ShowList (sub=1) ───────────────────────────────────────────────────────

/// Show all 4 character slots for the account.
///
/// C++ Reference: `CUser::ReqCharacterSealShowList()`
///
/// Response: `[ITEM_CHARACTER_SEAL][ShowList][u8 success]([string name][u8 race][u8 face][u16 class][u8 level]) x4`
async fn handle_show_list(session: &mut ClientSession) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    // Must have seal scroll or cypher ring
    let has_item = world.update_inventory(sid, |inv| {
        for slot in inv.iter().skip(SLOT_MAX).take(HAVE_MAX) {
            if (slot.item_id == ITEM_SEAL_SCROLL || slot.item_id == ITEM_CYPHER_RING)
                && slot.count > 0
            {
                return true;
            }
        }
        false
    });

    if !has_item {
        return Ok(());
    }

    let account_id = match session.account_id() {
        Some(id) => id.to_string(),
        None => return Ok(()),
    };

    // Load all 4 character slots
    let char_repo = CharacterRepository::new(session.pool());
    let chars = char_repo.load_all_for_account(&account_id).await?;

    // Also need account_char to know slot order
    let account_repo = AccountRepository::new(session.pool());
    let ac = account_repo.get_account_chars(&account_id).await?;

    let mut result = Packet::new(Opcode::WizItemUpgrade as u8);
    result.write_u8(ITEM_CHARACTER_SEAL);
    result.write_u8(SEAL_SHOW_LIST);
    result.write_u8(1); // success flag

    // Write each of the 4 slots
    let slot_names = match ac {
        Some(ref a) => [
            a.str_char_id1.as_deref().unwrap_or(""),
            a.str_char_id2.as_deref().unwrap_or(""),
            a.str_char_id3.as_deref().unwrap_or(""),
            a.str_char_id4.as_deref().unwrap_or(""),
        ],
        None => ["", "", "", ""],
    };

    for slot_name in &slot_names {
        if slot_name.is_empty() {
            // Empty slot
            result.write_sbyte_string("");
            result.write_u8(0); // race
            result.write_u8(0); // face
            result.write_u16(0); // class
            result.write_u8(0); // level
        } else if let Some(c) = chars
            .iter()
            .find(|c| c.str_user_id.eq_ignore_ascii_case(slot_name))
        {
            result.write_sbyte_string(&c.str_user_id);
            result.write_u8(c.race as u8);
            result.write_u8(c.face as u8);
            result.write_u16(c.class as u16);
            result.write_u8(c.level as u8);
        } else {
            result.write_sbyte_string("");
            result.write_u8(0);
            result.write_u8(0);
            result.write_u16(0);
            result.write_u8(0);
        }
    }

    session.send_packet(&result).await?;
    Ok(())
}

// ── UseScroll (sub=2) ──────────────────────────────────────────────────────

/// Seal a character into a Cypher Ring.
///
/// C++ Reference: `CUser::CharacterSealUseScroll()` + `CUser::ReqCharacterSealUseScroll()`
///
/// Client: `[u32 unknown][u8 src_slot][u32 item_id][string char_name][string password]`
async fn handle_use_scroll(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    let _unknown = reader.read_u32().unwrap_or(0);
    let src_slot = reader.read_u8().unwrap_or(0) as usize;
    let item_id = reader.read_u32().unwrap_or(0);
    let target_name = reader.read_string().unwrap_or_default();
    let password = reader.read_string().unwrap_or_default();

    // Validate target name
    let current_name = world
        .get_character_info(sid)
        .map(|c| c.name.clone())
        .unwrap_or_default();

    if target_name.is_empty() || target_name.eq_ignore_ascii_case(&current_name) {
        send_scroll_error(session).await?;
        return Ok(());
    }

    // Validate password
    let vip_pass = world.get_vip_password(sid);
    if vip_pass != password {
        send_scroll_error(session).await?;
        return Ok(());
    }

    // Validate item slot
    if src_slot >= HAVE_MAX {
        send_scroll_error(session).await?;
        return Ok(());
    }

    let abs_slot = SLOT_MAX + src_slot;
    let slot_data = world.get_inventory_slot(sid, abs_slot);

    let slot_data = match slot_data {
        Some(s)
            if s.item_id == ITEM_SEAL_SCROLL
                && item_id == ITEM_SEAL_SCROLL
                && s.flag == ITEM_FLAG_NONE =>
        {
            s
        }
        _ => {
            send_scroll_error(session).await?;
            return Ok(());
        }
    };
    let slot_serial = slot_data.serial_num;

    let account_id = match session.account_id() {
        Some(id) => id.to_string(),
        None => {
            send_scroll_error(session).await?;
            return Ok(());
        }
    };

    // Check that target character belongs to this account
    let account_repo = AccountRepository::new(session.pool());
    let ac = match account_repo.get_account_chars(&account_id).await? {
        Some(a) => a,
        None => {
            send_scroll_error(session).await?;
            return Ok(());
        }
    };

    let slots = [
        ac.str_char_id1.as_deref().unwrap_or(""),
        ac.str_char_id2.as_deref().unwrap_or(""),
        ac.str_char_id3.as_deref().unwrap_or(""),
        ac.str_char_id4.as_deref().unwrap_or(""),
    ];

    let char_slot_idx = match slots
        .iter()
        .position(|s| s.eq_ignore_ascii_case(&target_name))
    {
        Some(idx) => idx,
        None => {
            send_scroll_error(session).await?;
            return Ok(());
        }
    };

    // Seal character to DB
    let seal_repo = CharacterSealRepository::new(session.pool());
    let seal_id = seal_repo
        .seal_character(&account_id, &target_name, slot_serial as i64)
        .await?;

    // Generate unique ID and create mapping
    let unique_id = seal_repo.next_unique_id().await?;
    seal_repo
        .create_mapping(unique_id, seal_id, &account_id)
        .await?;

    // Remove character from account_char slot
    let char_repo = CharacterRepository::new(session.pool());

    // Load the target char info BEFORE deleting (for response packet)
    let target_char = char_repo.load(&target_name).await?;

    // Remove the character slot from account_char and delete character data
    let account_repo2 = AccountRepository::new(session.pool());
    account_repo2
        .clear_char_slot(&account_id, char_slot_idx as u8)
        .await?;

    seal_repo.delete_character_data(&target_name).await?;

    // Transform Seal Scroll → Cypher Ring in inventory
    world.update_inventory(sid, |inv| {
        if let Some(slot) = inv.get_mut(abs_slot) {
            slot.item_id = ITEM_CYPHER_RING;
        }
        true
    });

    // Build success response
    let mut result = Packet::new(Opcode::WizItemUpgrade as u8);
    result.write_u8(ITEM_CHARACTER_SEAL);
    result.write_u8(SEAL_USE_SCROLL);
    result.write_u8(1); // success
    result.write_u8(src_slot as u8);
    result.write_u32(ITEM_CYPHER_RING);
    result.write_u32(unique_id as u32);

    if let Some(tc) = &target_char {
        result.write_sbyte_string(&tc.str_user_id);
        result.write_u8(tc.class as u8);
        result.write_u8(tc.level as u8);
        result.write_u16(0);
        result.write_u8(tc.race as u8);
        result.write_u8(0);
    } else {
        result.write_sbyte_string(&target_name);
        result.write_u8(0);
        result.write_u8(0);
        result.write_u16(0);
        result.write_u8(0);
        result.write_u8(0);
    }

    session.send_packet(&result).await?;

    debug!(
        "[{}] Character Seal: sealed '{}' into cypher ring (unique_id={})",
        session.addr(),
        target_name,
        unique_id
    );

    Ok(())
}

// ── UseRing (sub=3) ────────────────────────────────────────────────────────

/// Restore a sealed character from Cypher Ring.
///
/// C++ Reference: `CUser::CharacterSealUseRing()` + `CUser::ReqCharacterSealUseRing()`
///
/// Client: `[u32 unknown][u8 src_slot][u32 item_id][u8 target_char_slot]`
async fn handle_use_ring(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    let _unknown = reader.read_u32().unwrap_or(0);
    let src_slot = reader.read_u8().unwrap_or(0) as usize;
    let item_id = reader.read_u32().unwrap_or(0);
    let target_slot = reader.read_u8().unwrap_or(0);

    // Validate
    if src_slot >= HAVE_MAX || item_id != ITEM_CYPHER_RING || target_slot >= 4 {
        return Ok(());
    }

    let abs_slot = SLOT_MAX + src_slot;
    let slot_data = world.get_inventory_slot(sid, abs_slot);

    match &slot_data {
        Some(s) if s.item_id == ITEM_CYPHER_RING && s.flag == ITEM_FLAG_NONE => {}
        _ => {
            send_ring_error(session, 0).await?;
            return Ok(());
        }
    }

    let account_id = match session.account_id() {
        Some(id) => id.to_string(),
        None => {
            send_ring_error(session, 0).await?;
            return Ok(());
        }
    };

    // Check that target slot is empty
    let account_repo = AccountRepository::new(session.pool());
    let ac = match account_repo.get_account_chars(&account_id).await? {
        Some(a) => a,
        None => {
            send_ring_error(session, 0).await?;
            return Ok(());
        }
    };

    let slots = [
        ac.str_char_id1.as_deref().unwrap_or(""),
        ac.str_char_id2.as_deref().unwrap_or(""),
        ac.str_char_id3.as_deref().unwrap_or(""),
        ac.str_char_id4.as_deref().unwrap_or(""),
    ];

    if !slots[target_slot as usize].is_empty() {
        send_ring_error(session, 3).await?; // slot occupied
        return Ok(());
    }

    // Find the seal mapping by serial number
    let seal_repo = CharacterSealRepository::new(session.pool());

    // Look up seal data — we need to find the mapping that corresponds to this ring's serial
    let mappings = seal_repo.get_seal_list(&account_id).await?;
    let mapping = match mappings.first() {
        Some(m) => m,
        None => {
            send_ring_error(session, 0).await?;
            return Ok(());
        }
    };

    let unique_id = mapping.unique_id;

    // Check nation match
    let seal_item = seal_repo.load_seal_item_by_unique_id(unique_id).await?;
    if let Some(ref si) = seal_item {
        let current_nation = world.get_character_info(sid).map(|c| c.nation).unwrap_or(0);
        let sealed_nation = (si.class / 100) as u8;
        if sealed_nation != current_nation {
            send_ring_error(session, 0).await?;
            return Ok(());
        }
    }

    // Restore character from seal
    let restored = seal_repo
        .unseal_character(unique_id, &account_id, target_slot)
        .await?;

    if restored.is_none() {
        send_ring_error(session, 0).await?;
        return Ok(());
    }

    // Clear the Cypher Ring from inventory
    world.update_inventory(sid, |inv| {
        if let Some(slot) = inv.get_mut(abs_slot) {
            slot.item_id = 0;
            slot.durability = 0;
            slot.count = 0;
            slot.flag = 0;
            slot.serial_num = 0;
        }
        true
    });

    // Save cleared slot to DB
    let save_params = ko_db::repositories::character::SaveItemParams {
        char_id: &world.get_session_name(sid).unwrap_or_default(),
        slot_index: abs_slot as i16,
        item_id: 0,
        durability: 0,
        count: 0,
        flag: 0,
        original_flag: 0,
        serial_num: 0,
        expire_time: 0,
    };
    let char_repo = CharacterRepository::new(session.pool());
    if let Err(e) = char_repo.save_item(&save_params).await {
        tracing::warn!("Failed to save sealed item to DB: {e}");
    }

    // Success response
    let mut result = Packet::new(Opcode::WizItemUpgrade as u8);
    result.write_u8(ITEM_CHARACTER_SEAL);
    result.write_u8(SEAL_USE_RING);
    result.write_u8(1); // success
    result.write_u8(src_slot as u8);
    result.write_u32(0);

    session.send_packet(&result).await?;

    debug!(
        "[{}] Character Seal: restored character from cypher ring (slot={})",
        session.addr(),
        target_slot
    );

    Ok(())
}

// ── Preview (sub=4) ────────────────────────────────────────────────────────

/// Preview sealed character stats and items.
///
/// C++ Reference: `CUser::CharacterSealPreview()`
///
/// Client: `[u32 unique_id]`
async fn handle_preview(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let unique_id = reader.read_u32().unwrap_or(0) as i32;

    let seal_repo = CharacterSealRepository::new(session.pool());
    let seal_item = seal_repo.load_seal_item_by_unique_id(unique_id).await?;

    let mut result = Packet::new(Opcode::WizItemUpgrade as u8);
    result.write_u8(ITEM_CHARACTER_SEAL);

    match seal_item {
        Some(si) => {
            result.write_u8(SEAL_PREVIEW);
            result.write_u8(1); // success

            result.write_sbyte_string(&si.char_name);
            result.write_u8(1); // unknown constant
            result.write_u8(si.race as u8);
            result.write_u16(si.class as u16);
            result.write_u8(si.level as u8);
            result.write_u32(si.loyalty as u32);
            result.write_u8(si.strong as u8); // STR
            result.write_u8(si.sta as u8);
            result.write_u8(si.dex as u8);
            result.write_u8(si.intel as u8);
            result.write_u8(si.cha as u8);
            result.write_u32(si.gold as u32);
            result.write_u8(si.free_points as u8);
            result.write_u32(1); // skill flag
            result.write_u8(si.skill_cat1 as u8);
            result.write_u8(si.skill_cat2 as u8);
            result.write_u8(si.skill_cat3 as u8);
            result.write_u8(si.skill_master as u8);

            // Inventory items (INVENTORY_COSP = 42 slots)
            // If we have serialized inventory_data, decode it;
            // otherwise send empty slots.
            if let Some(ref data) = si.inventory_data {
                // Each slot: [u32 item_id][i16 durability][u16 count][u8 flag] = 9 bytes
                for i in 0..INVENTORY_COSP {
                    let offset = i * 9;
                    if offset + 9 <= data.len() {
                        let item_id = u32::from_le_bytes(
                            data[offset..offset + 4].try_into().unwrap_or([0; 4]),
                        );
                        let durability = i16::from_le_bytes(
                            data[offset + 4..offset + 6].try_into().unwrap_or([0; 2]),
                        );
                        let count = u16::from_le_bytes(
                            data[offset + 6..offset + 8].try_into().unwrap_or([0; 2]),
                        );
                        let flag = data[offset + 8];
                        result.write_u32(item_id);
                        result.write_i16(durability);
                        result.write_u16(count);
                        result.write_u8(flag);
                    } else {
                        result.write_u32(0);
                        result.write_i16(0);
                        result.write_u16(0);
                        result.write_u8(0);
                    }
                }
            } else {
                // No inventory data — send empty slots
                for _ in 0..INVENTORY_COSP {
                    result.write_u32(0);
                    result.write_i16(0);
                    result.write_u16(0);
                    result.write_u8(0);
                }
            }
        }
        None => {
            result.write_u8(SEAL_PREVIEW);
            result.write_u8(2); // error
        }
    }

    session.send_packet(&result).await?;
    Ok(())
}

// ── AchieveList (sub=6) ────────────────────────────────────────────────────

/// Show achievements of sealed character.
///
/// C++ Reference: `CUser::CharacterSealAchieveList()`
///
/// For now, returns an empty achievement list (the sealed character's achievements
/// are not currently persisted in the seal snapshot).
async fn handle_achieve_list(
    session: &mut ClientSession,
    _reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let mut result = Packet::new(Opcode::WizItemUpgrade as u8);
    result.write_u8(ITEM_CHARACTER_SEAL);
    result.write_u8(SEAL_ACHIEVE_LIST);
    result.write_u8(1); // success
    result.write_u32(0); // count = 0

    session.send_packet(&result).await?;
    Ok(())
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ITEM_CHARACTER_SEAL, 9);
        assert_eq!(ITEM_SEAL_SCROLL, 800111000);
        assert_eq!(ITEM_CYPHER_RING, 800112000);
        assert_eq!(INVENTORY_COSP, 42);
    }

    #[test]
    fn test_sub_opcodes() {
        assert_eq!(SEAL_SHOW_LIST, 1);
        assert_eq!(SEAL_USE_SCROLL, 2);
        assert_eq!(SEAL_USE_RING, 3);
        assert_eq!(SEAL_PREVIEW, 4);
        assert_eq!(SEAL_ECHO, 5);
        assert_eq!(SEAL_ACHIEVE_LIST, 6);
    }

    #[test]
    fn test_echo_packet_format() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_ECHO);
        assert_eq!(pkt.opcode, Opcode::WizItemUpgrade as u8);
        assert_eq!(pkt.data[0], ITEM_CHARACTER_SEAL);
        assert_eq!(pkt.data[1], SEAL_ECHO);
        assert_eq!(pkt.data.len(), 2);
    }

    #[test]
    fn test_show_list_error_packet() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_SCROLL);
        pkt.write_u16(0); // error
        assert_eq!(pkt.data.len(), 4);
        assert_eq!(pkt.data[2], 0);
        assert_eq!(pkt.data[3], 0);
    }

    #[test]
    fn test_preview_error_packet() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_PREVIEW);
        pkt.write_u8(2); // error code
        assert_eq!(pkt.data.len(), 3);
        assert_eq!(pkt.data[2], 2);
    }

    #[test]
    fn test_use_ring_success_packet() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_RING);
        pkt.write_u8(1); // success
        pkt.write_u8(5); // src_slot
        pkt.write_u32(0);
        assert_eq!(pkt.data.len(), 8); // 2 header + 1 success + 1 slot + 4 padding
        assert_eq!(pkt.data[2], 1); // success
        assert_eq!(pkt.data[3], 5); // slot
    }

    #[test]
    fn test_achieve_list_empty_packet() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_ACHIEVE_LIST);
        pkt.write_u8(1);
        pkt.write_u32(0);
        assert_eq!(pkt.data.len(), 7);
        assert_eq!(pkt.data[2], 1); // success
                                    // count = 0 (4 bytes LE)
        assert_eq!(&pkt.data[3..7], &[0, 0, 0, 0]);
    }

    #[test]
    fn test_use_scroll_success_packet() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_SCROLL);
        pkt.write_u8(1); // success
        pkt.write_u8(3); // src_slot
        pkt.write_u32(ITEM_CYPHER_RING);
        pkt.write_u32(42); // unique_id
        pkt.write_sbyte_string("TestChar");
        pkt.write_u8(1); // class
        pkt.write_u8(70); // level
        pkt.write_u16(0);
        pkt.write_u8(12); // race
        pkt.write_u8(0);
        // Verify structure
        assert_eq!(pkt.data[0], ITEM_CHARACTER_SEAL);
        assert_eq!(pkt.data[1], SEAL_USE_SCROLL);
        assert_eq!(pkt.data[2], 1); // success
        assert_eq!(pkt.data[3], 3); // slot
    }

    #[test]
    fn test_preview_inventory_data_decode() {
        // Test inventory data serialization format: 9 bytes per slot
        let mut data = Vec::new();
        // Slot 0: item_id=100, durability=50, count=1, flag=0
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(&50i16.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.push(0u8);
        assert_eq!(data.len(), 9);

        // Decode
        let item_id = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let durability = i16::from_le_bytes(data[4..6].try_into().unwrap());
        let count = u16::from_le_bytes(data[6..8].try_into().unwrap());
        let flag = data[8];
        assert_eq!(item_id, 100);
        assert_eq!(durability, 50);
        assert_eq!(count, 1);
        assert_eq!(flag, 0);
    }

    // ── Sprint 923: Additional coverage ──────────────────────────────

    /// Show list success header: [ITEM_CHARACTER_SEAL][SHOW_LIST][1=success].
    #[test]
    fn test_show_list_success_header() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_SHOW_LIST);
        pkt.write_u8(1); // success
        assert_eq!(pkt.data[0], 9);
        assert_eq!(pkt.data[1], 1);
        assert_eq!(pkt.data[2], 1);
    }

    /// Show list empty slot format: [sbyte ""][race 0][face 0][class 0][level 0].
    #[test]
    fn test_show_list_empty_slot_format() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_SHOW_LIST);
        pkt.write_u8(1);
        // 4 empty character slots
        for _ in 0..4 {
            pkt.write_sbyte_string(""); // 1 byte (len=0)
            pkt.write_u8(0); // race
            pkt.write_u8(0); // face
            pkt.write_u16(0); // class
            pkt.write_u8(0); // level
        }
        // header(3) + 4 * (1+1+1+2+1) = 3 + 24 = 27
        assert_eq!(pkt.data.len(), 27);
    }

    /// UseScroll C2S format: [u32 unknown][u8 src_slot][u32 item_id][string name][string pass].
    #[test]
    fn test_use_scroll_c2s_format() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_SCROLL);
        pkt.write_u32(0); // unknown
        pkt.write_u8(5); // src_slot
        pkt.write_u32(ITEM_SEAL_SCROLL);
        pkt.write_string("AltChar");
        pkt.write_string("mypass");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ITEM_CHARACTER_SEAL));
        assert_eq!(r.read_u8(), Some(SEAL_USE_SCROLL));
        assert_eq!(r.read_u32(), Some(0));
        assert_eq!(r.read_u8(), Some(5));
        assert_eq!(r.read_u32(), Some(ITEM_SEAL_SCROLL));
        assert_eq!(r.read_string(), Some("AltChar".to_string()));
        assert_eq!(r.read_string(), Some("mypass".to_string()));
    }

    /// UseRing C2S format: [u32 unknown][u8 src_slot][u32 item_id][u8 target_slot].
    #[test]
    fn test_use_ring_c2s_format() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_RING);
        pkt.write_u32(0);
        pkt.write_u8(2); // src_slot
        pkt.write_u32(ITEM_CYPHER_RING);
        pkt.write_u8(3); // target_slot

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ITEM_CHARACTER_SEAL));
        assert_eq!(r.read_u8(), Some(SEAL_USE_RING));
        assert_eq!(r.read_u32(), Some(0));
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u32(), Some(ITEM_CYPHER_RING));
        assert_eq!(r.read_u8(), Some(3));
        assert_eq!(r.remaining(), 0);
    }

    /// UseRing error code 3 = slot occupied.
    #[test]
    fn test_use_ring_slot_occupied_error() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        pkt.write_u8(SEAL_USE_RING);
        pkt.write_u16(3); // error: slot occupied
        assert_eq!(pkt.data.len(), 4);
        // u16(3) little-endian = [3, 0]
        assert_eq!(pkt.data[2], 3);
        assert_eq!(pkt.data[3], 0);
    }

    /// Preview C2S format: [u32 unique_id].
    #[test]
    fn test_preview_c2s_format() {
        let mut pkt = Packet::new(Opcode::WizItemUpgrade as u8);
        pkt.write_u8(ITEM_CHARACTER_SEAL);
        // sub-opcode is part of higher-level dispatch, but we can verify unique_id encoding
        pkt.write_u32(12345);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ITEM_CHARACTER_SEAL));
        assert_eq!(r.read_u32(), Some(12345));
    }
}
