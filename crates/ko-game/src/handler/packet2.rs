//! WIZ_PACKET2 (0x5D) handler — player state flag update.
//!
//! S2C-only opcode that sets a per-player visual/transform state flag.
//!
//! ## Client RE
//!
//! Handler at `0x82E1E8` — reads `[i32 target_id][u8 value]`.
//! Looks up target_id in the global player map, sets `player+0xB69 = value`.
//! The `+0xB69` field is part of the visual/transform state block
//! (`+0xB40`..`+0xB98`). Zeroed on player init/reset alongside position
//! data (`+0x7D8/7DC/7E0`).
//!
//! ## S2C Packet Format
//!
//! ```text
//! [i32 target_id] [u8 value]
//! ```
//!
//! ## C2S Packets
//!
//! S2C-only — client never sends this opcode.

use ko_protocol::{Opcode, Packet};

// ── S2C Builder ─────────────────────────────────────────────────────

/// Build a WIZ_PACKET2 (0x5D) state flag update packet.
///
/// - `target_id`: Player to update in the global player map
/// - `value`: Flag value to store at `player+0xB69` (0 = clear)
///
/// Wire: `[i32 target_id][u8 value]`
pub fn build_state_flag(target_id: i32, value: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizPacket2 as u8);
    pkt.write_i32(target_id);
    pkt.write_u8(value);
    pkt
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_build_state_flag_opcode() {
        let pkt = build_state_flag(12345, 1);
        assert_eq!(pkt.opcode, Opcode::WizPacket2 as u8);
    }

    #[test]
    fn test_build_state_flag_format() {
        let pkt = build_state_flag(12345, 1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(12345)); // target_id
        assert_eq!(r.read_u8(), Some(1)); // value
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_state_flag_zero_clears() {
        let pkt = build_state_flag(999, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(999));
        assert_eq!(r.read_u8(), Some(0)); // clears +0xB69
    }

    #[test]
    fn test_build_state_flag_data_length() {
        // i32(4) + u8(1) = 5 bytes
        assert_eq!(build_state_flag(0, 0).data.len(), 5);
    }

    // ── Sprint 920: Integration context tests ───────────────────────

    /// Stealth apply: value=1 (INVIS_DISPEL_ON_MOVE) or value=2 (INVIS_DISPEL_ON_ATTACK).
    #[test]
    fn test_stealth_apply_state_flag() {
        // INVIS_DISPEL_ON_MOVE
        let pkt = build_state_flag(42, 1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(42));
        assert_eq!(r.read_u8(), Some(1));

        // INVIS_DISPEL_ON_ATTACK
        let pkt = build_state_flag(42, 2);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(42));
        assert_eq!(r.read_u8(), Some(2));
    }

    /// Stealth remove: value=0 clears the visual state.
    #[test]
    fn test_stealth_remove_clears_flag() {
        let pkt = build_state_flag(99, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(99));
        assert_eq!(r.read_u8(), Some(0));
    }

    /// Negative target_id is valid (NPC targets use negative IDs).
    #[test]
    fn test_negative_target_id() {
        let pkt = build_state_flag(-500, 1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(-500));
        assert_eq!(r.read_u8(), Some(1));
    }

    // ── Sprint 930: Additional coverage ──────────────────────────────

    /// Opcode value is 0x5D.
    #[test]
    fn test_packet2_opcode_value() {
        assert_eq!(Opcode::WizPacket2 as u8, 0x5D);
    }

    /// Data length is always 5 bytes: i32(4) + u8(1).
    #[test]
    fn test_packet2_data_length_consistent() {
        for (id, val) in [(0, 0u8), (i32::MAX, 255), (i32::MIN, 1), (42, 0)] {
            assert_eq!(build_state_flag(id, val).data.len(), 5);
        }
    }

    /// Value 255 is maximum u8 flag value.
    #[test]
    fn test_packet2_max_value() {
        let pkt = build_state_flag(1, u8::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(1));
        assert_eq!(r.read_u8(), Some(255));
    }

    /// Target_id i32::MIN roundtrip.
    #[test]
    fn test_packet2_min_target_id() {
        let pkt = build_state_flag(i32::MIN, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(i32::MIN));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    /// S2C-only: client never sends this opcode.
    #[test]
    fn test_packet2_s2c_only() {
        // No C2S handler — purely server-initiated state update.
        let pkt = build_state_flag(42, 1);
        assert_eq!(pkt.opcode, 0x5D);
        assert_eq!(pkt.data.len(), 5);
    }
}
