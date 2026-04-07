//! WIZ_CORPSE (0x4E) handler — corpse name tag display.
//!
//! S2C-only opcode that shows a name tag on a dead player's corpse.
//!
//! ## Client RE
//!
//! Dispatch at `0x82E170`: `case 0x4E` calls `sub_777BA0(g_pMain, pkt, ...)`.
//! `sub_777BA0` is the main user-info parser which processes a full user
//! appearance packet. The 0x4E opcode variant sends minimal user data so the
//! client can render a corpse with a name tag at the death position.
//!
//! ## S2C Packet Format
//!
//! The C++ server sends a corpse packet with the dead player's ID and name
//! so nearby players see a clickable name tag on the corpse.
//!
//! ```text
//! [u16 user_id] [SByte name]
//! ```
//!
//! ## C2S Packets
//!
//! S2C-only — client never sends this opcode.

use ko_protocol::{Opcode, Packet};
use std::sync::Arc;

use crate::world::WorldState;
use crate::zone::SessionId;

// ── S2C Builder ─────────────────────────────────────────────────────

/// Build a WIZ_CORPSE (0x4E) corpse name tag packet.
///
/// - `user_id`: The dead player's session/entity ID (u16)
/// - `name`: The dead player's character name
///
/// Wire: `[u16 user_id][SByte name]`
pub fn build_corpse(user_id: u16, name: &str) -> Packet {
    let mut pkt = Packet::new(Opcode::WizCorpse as u8);
    pkt.write_u16(user_id);
    pkt.write_sbyte_string(name);
    pkt
}

/// Broadcast a corpse name tag to the 3x3 region grid.
///
/// Called after `broadcast_death()` in `dead.rs` so nearby players see
/// the dead player's name on their corpse.
pub fn broadcast_corpse(world: &WorldState, dead_sid: SessionId) {
    let ch = match world.get_character_info(dead_sid) {
        Some(c) => c,
        None => return,
    };

    let pos = match world.get_position(dead_sid) {
        Some(p) => p,
        None => return,
    };

    let pkt = build_corpse(dead_sid, &ch.name);
    let event_room = world.get_event_room(dead_sid);
    world.broadcast_to_3x3(
        pos.zone_id,
        pos.region_x,
        pos.region_z,
        Arc::new(pkt),
        Some(dead_sid), // exclude self (dead player already sees own death)
        event_room,
    );

    tracing::trace!(
        "Broadcast corpse name tag for '{}' (sid={}) in zone {}",
        ch.name,
        dead_sid,
        pos.zone_id,
    );
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_build_corpse_opcode() {
        let pkt = build_corpse(1234, "Warrior");
        assert_eq!(pkt.opcode, Opcode::WizCorpse as u8);
    }

    #[test]
    fn test_build_corpse_opcode_value() {
        assert_eq!(Opcode::WizCorpse as u8, 0x4E);
    }

    #[test]
    fn test_build_corpse_format() {
        let pkt = build_corpse(1234, "Warrior");
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(1234)); // user_id
        assert_eq!(r.read_sbyte_string(), Some("Warrior".to_string())); // name
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_corpse_empty_name() {
        let pkt = build_corpse(0, "");
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_sbyte_string(), Some("".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_corpse_data_length() {
        let name = "TestPlayer";
        let pkt = build_corpse(100, name);
        // u16(2) + sbyte_len(1) + name_bytes(10) = 13
        assert_eq!(pkt.data.len(), 2 + 1 + name.len());
    }

    #[test]
    fn test_build_corpse_max_name() {
        let name = "ABCDEFGHIJKLMNOPQRST"; // 20 chars (KO max)
        let pkt = build_corpse(u16::MAX, name);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(u16::MAX));
        assert_eq!(r.read_sbyte_string(), Some(name.to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_corpse_roundtrip_bytes() {
        let pkt = build_corpse(500, "Hero");
        // user_id: 500 = 0x01F4 little-endian = [0xF4, 0x01]
        assert_eq!(pkt.data[0], 0xF4);
        assert_eq!(pkt.data[1], 0x01);
        // sbyte len = 4
        assert_eq!(pkt.data[2], 4);
        // "Hero" = [72, 101, 114, 111]
        assert_eq!(&pkt.data[3..7], b"Hero");
    }
}
