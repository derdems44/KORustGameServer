//! WIZ_WEIGHT_CHANGE (0x54) handler — send current weight to client.
//! Response (WIZ_WEIGHT_CHANGE):
//! ```text
//! [u16 item_weight]
//! ```

use ko_protocol::{Opcode, Packet};

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_WEIGHT_CHANGE from the client (request for current weight).
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();

    let stats = world.get_equipped_stats(sid);
    let mut result = Packet::new(Opcode::WizWeightChange as u8);
    result.write_u32(stats.item_weight);
    session.send_packet(&result).await
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_weight_change_response_format() {
        // Response: [u32 item_weight]
        let weight: u32 = 1500;
        let mut result = Packet::new(Opcode::WizWeightChange as u8);
        result.write_u32(weight);

        assert_eq!(result.opcode, Opcode::WizWeightChange as u8);
        let mut reader = PacketReader::new(&result.data);
        assert_eq!(reader.read_u32(), Some(1500));
    }

    #[test]
    fn test_weight_change_zero_weight() {
        let mut result = Packet::new(Opcode::WizWeightChange as u8);
        result.write_u32(0);

        let mut reader = PacketReader::new(&result.data);
        assert_eq!(reader.read_u32(), Some(0), "zero weight is valid");
    }

    #[test]
    fn test_weight_change_max_weight() {
        // u32 max — extreme edge case
        let mut result = Packet::new(Opcode::WizWeightChange as u8);
        result.write_u32(u32::MAX);

        let mut reader = PacketReader::new(&result.data);
        assert_eq!(reader.read_u32(), Some(u32::MAX));
    }

    // ── Sprint 931: Additional coverage ──────────────────────────────

    /// Opcode value is 0x54.
    #[test]
    fn test_weight_change_opcode_value() {
        assert_eq!(Opcode::WizWeightChange as u8, 0x54);
    }

    /// Response data length: u32(4) = 4 bytes.
    #[test]
    fn test_weight_change_response_data_length() {
        let mut pkt = Packet::new(Opcode::WizWeightChange as u8);
        pkt.write_u32(1500);
        assert_eq!(pkt.data.len(), 4);
    }

    /// C2S is empty — client sends just the opcode.
    #[test]
    fn test_weight_change_c2s_empty() {
        let pkt = Packet::new(Opcode::WizWeightChange as u8);
        assert!(pkt.data.is_empty());
    }

    /// Roundtrip with typical weight values.
    #[test]
    fn test_weight_change_roundtrip() {
        for w in [0u32, 100, 1500, 5000, 65535] {
            let mut pkt = Packet::new(Opcode::WizWeightChange as u8);
            pkt.write_u32(w);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(w));
            assert_eq!(r.remaining(), 0);
        }
    }

    /// Weight uses u32 (not u16 as doc comment suggests).
    #[test]
    fn test_weight_change_u32_not_u16() {
        // Handler writes u32 (4 bytes), not u16 (2 bytes)
        let mut pkt = Packet::new(Opcode::WizWeightChange as u8);
        pkt.write_u32(70000); // > u16::MAX
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(70000));
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode from_byte roundtrip for 0x54.
    #[test]
    fn test_weight_change_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x54), Some(Opcode::WizWeightChange));
    }

    /// Weight LE encoding verification.
    #[test]
    fn test_weight_change_le_encoding() {
        let mut pkt = Packet::new(Opcode::WizWeightChange as u8);
        pkt.write_u32(0x01020304);
        // LE: least significant byte first
        assert_eq!(pkt.data[0], 0x04);
        assert_eq!(pkt.data[3], 0x01);
    }
}
