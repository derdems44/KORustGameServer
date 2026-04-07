//! WIZ_ITEM_REMOVE (0x3F) handler — destroy/delete an item from inventory.
//! Packet format (from client):
//! ```text
//! [u8 type] [u8 pos] [u32 item_id]
//! ```
//! type: 0=inventory bag, 1=equipment slot, 2=inventory bag (alternate), 3=COSP/magic bag
//! Response (WIZ_ITEM_REMOVE):
//! ```text
//! [u8 result] — 0=fail, 1=success
//! ```

use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};
use crate::world::{UserItemSlot, ITEM_FLAG_RENTED, ITEM_FLAG_SEALED, ZONE_CHAOS_DUNGEON};

use super::{HAVE_MAX, INVENTORY_COSP, SLOT_MAX};

/// Handle WIZ_ITEM_REMOVE from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    // Dead players cannot destroy items
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();

    // Cannot remove items while mining or fishing
    if world.is_mining(sid) || world.is_fishing(sid) {
        let mut result = Packet::new(Opcode::WizItemRemove as u8);
        result.write_u8(0);
        return session.send_packet(&result).await;
    }

    // Cannot remove items in Chaos Dungeon
    if world.get_position(sid).map(|p| p.zone_id).unwrap_or(0) == ZONE_CHAOS_DUNGEON {
        let mut result = Packet::new(Opcode::WizItemRemove as u8);
        result.write_u8(0);
        return session.send_packet(&result).await;
    }

    let mut reader = PacketReader::new(&pkt.data);

    let item_type = reader.read_u8().unwrap_or(0);
    let pos = reader.read_u8().unwrap_or(0);
    let item_id = reader.read_u32().unwrap_or(0);

    let mut result = Packet::new(Opcode::WizItemRemove as u8);

    // Validate item_id
    if world.get_item(item_id).is_none() {
        result.write_u8(0);
        return session.send_packet(&result).await;
    }

    // Calculate actual inventory index based on type
    let actual_idx = match item_type {
        0 | 2 => {
            // Inventory bag
            if pos as usize >= HAVE_MAX {
                result.write_u8(0);
                return session.send_packet(&result).await;
            }
            SLOT_MAX + pos as usize
        }
        1 => {
            // Equipment slot
            if pos as usize >= SLOT_MAX {
                result.write_u8(0);
                return session.send_packet(&result).await;
            }
            pos as usize
        }
        3 => {
            // COSP / magic bag slot
            //   `if (bPos >= SLOT_MAX) goto fail_return; bPos += INVENTORY_COSP + 8;`
            if pos as usize >= SLOT_MAX {
                result.write_u8(0);
                return session.send_packet(&result).await;
            }
            INVENTORY_COSP + 8 + pos as usize
        }
        _ => {
            result.write_u8(0);
            return session.send_packet(&result).await;
        }
    };

    // Verify item matches
    let slot = match world.get_inventory_slot(sid, actual_idx) {
        Some(s) => s,
        None => {
            result.write_u8(0);
            return session.send_packet(&result).await;
        }
    };

    if slot.item_id != item_id {
        result.write_u8(0);
        return session.send_packet(&result).await;
    }

    // Cannot remove sealed or rented items — use equality, NOT bitmask.
    if slot.flag == ITEM_FLAG_SEALED || slot.flag == ITEM_FLAG_RENTED {
        result.write_u8(0);
        return session.send_packet(&result).await;
    }

    // Clear the slot
    world.update_inventory(sid, |inv| {
        if actual_idx < inv.len() {
            inv[actual_idx] = UserItemSlot::default();
            true
        } else {
            false
        }
    });

    // Recalculate stats (weight notification is integrated into set_user_ability)
    world.set_user_ability(sid);

    // Update equipped_items in CharacterInfo if removing from equipment slot
    if item_type == 1 {
        let inventory = world.get_inventory(sid);
        world.update_character_stats(sid, |ch| {
            for s in 0..14 {
                if s < inventory.len() {
                    ch.equipped_items[s] = inventory[s].item_id;
                }
            }
        });
    }

    // FerihaLog: ItemRemoveInsertLog
    super::audit_log::log_item_remove(
        session.pool(),
        session.account_id().unwrap_or(""),
        &world.get_session_name(sid).unwrap_or_default(),
        &session.addr().to_string(),
        item_id,
    );

    result.write_u8(1); // success
    session.send_packet(&result).await
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;
    use crate::inventory_constants::INVENTORY_TOTAL;

    // ── Sprint 291: COSP item type 3 support ──────────────────────────

    #[test]
    fn test_cosp_type3_index_calculation() {
        // `if (bType == 3) { if (bPos >= SLOT_MAX) goto fail_return; bPos += INVENTORY_COSP + 8; }`
        assert_eq!(INVENTORY_COSP, 42);

        // bPos=0 → index 50 (COSP slot #8)
        assert_eq!((INVENTORY_COSP + 8), 50);
        // bPos=1 → index 51
        assert_eq!(INVENTORY_COSP + 8 + 1, 51);
        // bPos=13 → index 63 (max valid for SLOT_MAX-1)
        assert_eq!(INVENTORY_COSP + 8 + 13, 63);
        // All within INVENTORY_TOTAL
        assert!(63 < INVENTORY_TOTAL);
    }

    #[test]
    fn test_cosp_type3_bounds_check() {
        // `if (bPos >= SLOT_MAX) goto fail_return;`
        // bPos must be < SLOT_MAX (14) for type 3
        assert!(14 >= SLOT_MAX); // pos=14 should fail
        assert!(0 < SLOT_MAX); // pos=0 should pass
        assert!(13 < SLOT_MAX); // pos=13 should pass
    }

    #[test]
    fn test_item_type_variants() {
        // type 0: inventory bag → SLOT_MAX + pos
        // type 1: equipment slot → pos
        // type 2: inventory bag (alternate) → SLOT_MAX + pos
        // type 3: COSP/magic bag → INVENTORY_COSP + 8 + pos
        let pos: usize = 5;
        assert_eq!(SLOT_MAX + pos, 19); // type 0/2
        assert_eq!(pos, 5); // type 1
        assert_eq!(INVENTORY_COSP + 8 + pos, 55); // type 3
    }

    #[test]
    fn test_zone_chaos_dungeon_constant() {
        assert_eq!(ZONE_CHAOS_DUNGEON, 85);
    }

    #[test]
    fn test_c2s_item_remove_packet_format() {
        // C2S: [u8 type] [u8 pos] [u32 item_id]
        let mut pkt = Packet::new(Opcode::WizItemRemove as u8);
        pkt.write_u8(0); // type=inventory bag
        pkt.write_u8(5); // pos
        pkt.write_u32(379006001); // item_id

        assert_eq!(pkt.opcode, Opcode::WizItemRemove as u8);
        assert_eq!(pkt.data.len(), 6); // 1 + 1 + 4

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_u8(), Some(5));
        assert_eq!(r.read_u32(), Some(379006001));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_s2c_item_remove_success_response() {
        let mut pkt = Packet::new(Opcode::WizItemRemove as u8);
        pkt.write_u8(1); // success
        assert_eq!(pkt.data.len(), 1);
        assert_eq!(pkt.data[0], 1);
    }

    #[test]
    fn test_s2c_item_remove_fail_response() {
        let mut pkt = Packet::new(Opcode::WizItemRemove as u8);
        pkt.write_u8(0); // fail
        assert_eq!(pkt.data.len(), 1);
        assert_eq!(pkt.data[0], 0);
    }

    #[test]
    fn test_item_remove_opcode_value() {
        assert_eq!(Opcode::WizItemRemove as u8, 0x3F);
    }

    #[test]
    fn test_sealed_rented_flag_constants() {
        // Cannot remove sealed or rented items
        assert_eq!(ITEM_FLAG_SEALED, 4);
        assert_eq!(ITEM_FLAG_RENTED, 1);
        assert_ne!(ITEM_FLAG_SEALED, ITEM_FLAG_RENTED);
    }

    // ── Sprint 929: Additional coverage ──────────────────────────────

    /// C2S data length: type(1) + pos(1) + item_id(4) = 6.
    #[test]
    fn test_item_remove_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizItemRemove as u8);
        pkt.write_u8(0);
        pkt.write_u8(5);
        pkt.write_u32(379006001);
        assert_eq!(pkt.data.len(), 6);
    }

    /// Response is always 1 byte (result only).
    #[test]
    fn test_item_remove_response_data_length() {
        let mut s = Packet::new(Opcode::WizItemRemove as u8);
        s.write_u8(1);
        assert_eq!(s.data.len(), 1);

        let mut f = Packet::new(Opcode::WizItemRemove as u8);
        f.write_u8(0);
        assert_eq!(f.data.len(), 1);
    }

    /// INVENTORY_COSP constant = 42.
    #[test]
    fn test_item_remove_inventory_cosp_constant() {
        assert_eq!(INVENTORY_COSP, 42);
        // COSP+8 = 50 (base for type-3 items)
        assert_eq!(INVENTORY_COSP + 8, 50);
    }

    /// Type 0 and type 2 both map to SLOT_MAX + pos (inventory bag).
    #[test]
    fn test_item_remove_type0_type2_same_mapping() {
        for item_type in [0u8, 2] {
            let pos: usize = 10;
            let actual_idx = SLOT_MAX + pos;
            assert_eq!(actual_idx, 24, "type {item_type} maps to SLOT_MAX+pos");
        }
    }

    /// Full C2S roundtrip for type 3 (COSP).
    #[test]
    fn test_item_remove_type3_roundtrip() {
        let mut pkt = Packet::new(Opcode::WizItemRemove as u8);
        pkt.write_u8(3); // COSP type
        pkt.write_u8(2); // pos
        pkt.write_u32(810001000);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(3));
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u32(), Some(810001000));
        assert_eq!(r.remaining(), 0);
    }
}
