//! WIZ_SERVER_INDEX (0x6B) handler — server index response.
//! ## Request (C->S)
//! Empty — client sends just the opcode.
//! ## Response (S->C)
//! | Offset | Type  | Description |
//! |--------|-------|-------------|
//! | 0      | u16le | Flag (always 1) |
//! | 2      | u16le | Server number |

use ko_protocol::{Opcode, Packet};

use crate::session::ClientSession;

/// Default server number (from `m_nServerNo` config).
const DEFAULT_SERVER_NO: u16 = 1;

/// Handle WIZ_SERVER_INDEX from the client.
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    let mut response = Packet::new(Opcode::WizServerIndex as u8);
    response.write_u16(1); // flag (always 1)
    response.write_u16(DEFAULT_SERVER_NO);
    session.send_packet(&response).await?;

    tracing::debug!(
        "[{}] Sent server index: server_no={}",
        session.addr(),
        DEFAULT_SERVER_NO
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_server_index_response_format() {
        // Response: [u16 flag=1][u16 server_no]
        let mut response = Packet::new(Opcode::WizServerIndex as u8);
        response.write_u16(1);
        response.write_u16(DEFAULT_SERVER_NO);

        assert_eq!(response.opcode, Opcode::WizServerIndex as u8);
        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u16(), Some(1), "flag always 1");
        assert_eq!(reader.read_u16(), Some(1), "default server_no");
    }

    #[test]
    fn test_server_index_opcode_value() {
        assert_eq!(Opcode::WizServerIndex as u8, 0x6B);
    }

    #[test]
    fn test_server_index_default_constant() {
        assert_eq!(DEFAULT_SERVER_NO, 1);
    }

    // ── Sprint 931: Additional coverage ──────────────────────────────

    /// Response data length: flag(2) + server_no(2) = 4.
    #[test]
    fn test_server_index_response_data_length() {
        let mut pkt = Packet::new(Opcode::WizServerIndex as u8);
        pkt.write_u16(1);
        pkt.write_u16(DEFAULT_SERVER_NO);
        assert_eq!(pkt.data.len(), 4);
    }

    /// C2S is empty — client sends just the opcode.
    #[test]
    fn test_server_index_c2s_empty() {
        let pkt = Packet::new(Opcode::WizServerIndex as u8);
        assert!(pkt.data.is_empty());
    }

    /// Flag is always 1 (u16).
    #[test]
    fn test_server_index_flag_always_one() {
        let mut pkt = Packet::new(Opcode::WizServerIndex as u8);
        pkt.write_u16(1);
        pkt.write_u16(1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(1), "flag must be 1");
    }

    /// Roundtrip with different server numbers.
    #[test]
    fn test_server_index_roundtrip() {
        for no in [1u16, 2, 10, 255] {
            let mut pkt = Packet::new(Opcode::WizServerIndex as u8);
            pkt.write_u16(1);
            pkt.write_u16(no);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u16(), Some(1));
            assert_eq!(r.read_u16(), Some(no));
            assert_eq!(r.remaining(), 0);
        }
    }

    /// Opcode from_byte roundtrip.
    #[test]
    fn test_server_index_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x6B), Some(Opcode::WizServerIndex));
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_server_index_dispatch_range() {
        let op = Opcode::WizServerIndex as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// Full response roundtrip — flag(1) + server_no(1).
    #[test]
    fn test_server_index_full_roundtrip() {
        let mut pkt = Packet::new(Opcode::WizServerIndex as u8);
        pkt.write_u16(1);
        pkt.write_u16(DEFAULT_SERVER_NO);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(1));
        assert_eq!(r.read_u16(), Some(1));
        assert_eq!(r.remaining(), 0);
    }
}
