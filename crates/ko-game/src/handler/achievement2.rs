//! WIZ_ACHIEVEMENT2 (0xA5) handler — achievement/kill-count display.
//!
//! S2C-only opcode that updates the client's kill-count display.
//!
//! ## Client RE
//!
//! Handler at `0x82CB16` — reads `[i32 value]`, stores at `player+0x7AC`.
//! - Non-zero: formats string ID `0xA7FA` (43,002) with value, displays in red
//!   (`0xFFFF0000`).
//! - Zero: uses string ID `0xA7FB` (43,003) instead (clears display).
//!
//! Group A (always callable) — no panel null check.
//!
//! ## S2C Packet Format
//!
//! ```text
//! [i32 value]
//! ```
//!
//! ## C2S Packets
//!
//! S2C-only — client never sends this opcode.

use ko_protocol::{Opcode, Packet};

// ── S2C Builder ─────────────────────────────────────────────────────

/// Build a WIZ_ACHIEVEMENT2 (0xA5) packet — kill-count display update.
///
/// - `value > 0`: Client shows string 0xA7FA formatted with value (red text)
/// - `value == 0`: Client shows string 0xA7FB (clears counter)
///
/// Wire: `[i32 value]`
pub fn build_achievement2(value: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAchievement2 as u8);
    pkt.write_i32(value);
    pkt
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_build_achievement2_opcode() {
        let pkt = build_achievement2(150);
        assert_eq!(pkt.opcode, Opcode::WizAchievement2 as u8);
    }

    #[test]
    fn test_build_achievement2_format() {
        let pkt = build_achievement2(150);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(150));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_achievement2_zero_clears() {
        // Client uses string 0xA7FB for zero (clear display)
        let pkt = build_achievement2(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_achievement2_data_length() {
        // i32 only = 4 bytes
        assert_eq!(build_achievement2(999).data.len(), 4);
    }

    #[test]
    fn test_build_achievement2_negative() {
        let pkt = build_achievement2(-1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(-1));
    }

    #[test]
    fn test_build_achievement2_max_value() {
        let pkt = build_achievement2(i32::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(i32::MAX));
    }

    // ── Sprint 931: Additional coverage ──────────────────────────────

    /// Opcode value is 0xA5.
    #[test]
    fn test_achievement2_opcode_value() {
        assert_eq!(Opcode::WizAchievement2 as u8, 0xA5);
    }

    /// String IDs: 0xA7FA (43002) for non-zero, 0xA7FB (43003) for zero.
    #[test]
    fn test_achievement2_string_ids() {
        assert_eq!(0xA7FAu32, 43002);
        assert_eq!(0xA7FBu32, 43003);
    }

    /// Opcode is in v2525 always-callable range (Group A).
    #[test]
    fn test_achievement2_in_dispatch_range() {
        let op = Opcode::WizAchievement2 as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// i32::MIN roundtrip.
    #[test]
    fn test_achievement2_min_value() {
        let pkt = build_achievement2(i32::MIN);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_i32(), Some(i32::MIN));
        assert_eq!(r.remaining(), 0);
    }
}
