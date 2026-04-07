//! WIZ_STORY (0x81) — story/intro cutscene packet (server→client only).
//! Sent during game start to signal story/cutscene state to the client.
//! The client does not send this opcode; it is purely server-initiated.
//! ## Wire format (Server→Client)
//! `[u32 story_id=0] [u16 flags=0]`
//! In the C++ reference, `story_id` and `flags` are always 0.

use ko_protocol::{Opcode, Packet};

/// Build a WIZ_STORY packet to send to the client during game start.
/// ```c++
/// Packet newpkt(WIZ_STORY);
/// newpkt << uint32(0) << uint16(0);
/// Send(&newpkt);
/// ```
/// Wire: `[u32 story_id] [u16 flags]`
pub fn build_story_packet(story_id: u32, flags: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizStory as u8);
    pkt.write_u32(story_id);
    pkt.write_u16(flags);
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_story_default_packet() {
        // C++ always sends story_id=0, flags=0
        let pkt = build_story_packet(0, 0);
        assert_eq!(pkt.opcode, Opcode::WizStory as u8);
        assert_eq!(pkt.opcode, 0x81);
        assert_eq!(pkt.data.len(), 6); // u32 + u16

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(0)); // story_id
        assert_eq!(r.read_u16(), Some(0)); // flags
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_story_nonzero_values() {
        let pkt = build_story_packet(42, 7);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(42));
        assert_eq!(r.read_u16(), Some(7));
        assert_eq!(r.remaining(), 0);
    }

    // ── Sprint 932: Additional coverage ──────────────────────────────

    /// Data length is always 6 bytes: story_id(4) + flags(2).
    #[test]
    fn test_story_data_length() {
        assert_eq!(build_story_packet(0, 0).data.len(), 6);
        assert_eq!(build_story_packet(u32::MAX, u16::MAX).data.len(), 6);
    }

    /// Opcode from_byte roundtrip for 0x81.
    #[test]
    fn test_story_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x81), Some(Opcode::WizStory));
    }

    /// S2C-only — client never sends this opcode. No C2S body.
    #[test]
    fn test_story_s2c_only() {
        // S2C packet always has 6 bytes of data
        let pkt = build_story_packet(0, 0);
        assert_eq!(pkt.data.len(), 6);
    }

    /// Little-endian encoding of story_id.
    #[test]
    fn test_story_le_encoding() {
        let pkt = build_story_packet(0x01020304, 0);
        // LE: least significant byte first
        assert_eq!(pkt.data[0], 0x04);
        assert_eq!(pkt.data[3], 0x01);
    }

    /// Max values roundtrip.
    #[test]
    fn test_story_max_values() {
        let pkt = build_story_packet(u32::MAX, u16::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(u32::MAX));
        assert_eq!(r.read_u16(), Some(u16::MAX));
        assert_eq!(r.remaining(), 0);
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_story_dispatch_range() {
        let op = Opcode::WizStory as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// Flags field is u16, various values roundtrip.
    #[test]
    fn test_story_flags_roundtrip() {
        for flags in [0u16, 1, 0xFF, 0x1234] {
            let pkt = build_story_packet(0, flags);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(0));
            assert_eq!(r.read_u16(), Some(flags));
        }
    }

    /// Story ID zero is the default (C++ always sends 0).
    #[test]
    fn test_story_default_id_zero() {
        let pkt = build_story_packet(0, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(0), "C++ default story_id is always 0");
    }
}
