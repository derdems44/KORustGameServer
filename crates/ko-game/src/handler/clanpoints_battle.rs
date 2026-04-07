//! WIZ_CLANPOINTS_BATTLE (0x91) handler — clan notification display.
//!
//! S2C-only opcode that shows a screen notification with clan-related strings.
//!
//! ## Client RE
//!
//! Handler at `0x7C1E00` — reads `[u8 type]` (must be 1), then `[u8 sub]`.
//! Sub selects a string ID:
//! - 0 → string 25051
//! - 1 → string 25052
//! - 2, 5 → string 1511
//! - 3 → string 25053
//! - 4 → string 25054
//!
//! Formats and displays the string as a screen notification.
//!
//! ## S2C Packet Format
//!
//! ```text
//! [u8 type=1] [u8 sub]
//! ```
//!
//! ## C2S Packets
//!
//! S2C-only — client never sends this opcode.

use ko_protocol::{Opcode, Packet};

// ── String ID Constants ─────────────────────────────────────────────

/// String IDs displayed per sub-opcode value.
pub const STRING_SUB_0: u32 = 25051;
pub const STRING_SUB_1: u32 = 25052;
pub const STRING_SUB_2_5: u32 = 1511;
pub const STRING_SUB_3: u32 = 25053;
pub const STRING_SUB_4: u32 = 25054;

// ── S2C Builder ─────────────────────────────────────────────────────

/// Build a WIZ_CLANPOINTS_BATTLE (0x91) notification packet.
///
/// - `sub`: Selects which string to display (0-5)
///
/// Wire: `[u8 type=1][u8 sub]`
pub fn build_notification(sub: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizClanpointsBattle as u8);
    pkt.write_u8(1); // type must be 1
    pkt.write_u8(sub);
    pkt
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_build_notification_opcode() {
        let pkt = build_notification(0);
        assert_eq!(pkt.opcode, Opcode::WizClanpointsBattle as u8);
    }

    #[test]
    fn test_build_notification_format() {
        let pkt = build_notification(3);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type=1
        assert_eq!(r.read_u8(), Some(3)); // sub
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_notification_all_subs() {
        for sub in 0..=5 {
            let pkt = build_notification(sub);
            assert_eq!(pkt.opcode, 0x91);
            assert_eq!(pkt.data.len(), 2); // type + sub
        }
    }

    #[test]
    fn test_build_notification_data_length() {
        assert_eq!(build_notification(0).data.len(), 2);
    }

    // ── Sprint 920: Integration context tests ───────────────────────

    /// Sub=0 is used for clan disband notification (knights.rs handle_destroy).
    #[test]
    fn test_clan_disband_notification_sub_0() {
        let pkt = build_notification(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type
        assert_eq!(r.read_u8(), Some(0)); // sub=0 → string 25051
    }

    /// String ID mapping: verify documented sub→string constants.
    #[test]
    fn test_string_id_constants() {
        assert_eq!(STRING_SUB_0, 25051);
        assert_eq!(STRING_SUB_1, 25052);
        assert_eq!(STRING_SUB_2_5, 1511);
        assert_eq!(STRING_SUB_3, 25053);
        assert_eq!(STRING_SUB_4, 25054);
    }

    // ── Sprint 930: Additional coverage ──────────────────────────────

    /// Opcode value is 0x91.
    #[test]
    fn test_clanpoints_battle_opcode_value() {
        assert_eq!(Opcode::WizClanpointsBattle as u8, 0x91);
    }

    /// Type field is always 1.
    #[test]
    fn test_clanpoints_battle_type_always_one() {
        for sub in 0..=5u8 {
            let pkt = build_notification(sub);
            assert_eq!(pkt.data[0], 1, "type must be 1 for sub={sub}");
        }
    }

    /// Sub 2 and 5 share the same string ID (1511).
    #[test]
    fn test_clanpoints_battle_sub2_sub5_same_string() {
        assert_eq!(STRING_SUB_2_5, 1511);
        // Both sub=2 and sub=5 map to string 1511
        let pkt2 = build_notification(2);
        let pkt5 = build_notification(5);
        assert_eq!(pkt2.data[1], 2);
        assert_eq!(pkt5.data[1], 5);
    }

    /// Notification packet roundtrip for all valid subs.
    #[test]
    fn test_clanpoints_battle_roundtrip_all_subs() {
        for sub in 0..=5u8 {
            let pkt = build_notification(sub);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(1));
            assert_eq!(r.read_u8(), Some(sub));
            assert_eq!(r.remaining(), 0);
        }
    }

    /// S2C-only: client never sends this opcode.
    #[test]
    fn test_clanpoints_battle_s2c_only() {
        // No C2S handler exists — this is purely server-initiated.
        // Packet is always 2 bytes (type + sub).
        let pkt = build_notification(0);
        assert_eq!(pkt.data.len(), 2);
    }
}
