//! WIZ_VIRTUAL_SERVER (0x4C) handler — server/channel change request.
//! In the C++ reference implementation, `ServerChangeOk()` immediately returns
//! without processing. This feature is unused in single-server setups and the
//! handler is a no-op.
//! ## Client -> Server
//! `[u16 warp_id]` (unused — function returns immediately)

use ko_protocol::Packet;
use tracing::debug;

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_VIRTUAL_SERVER from the client.
/// This is a no-op. The C++ reference immediately returns without processing.
pub fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    debug!(
        "[{}] WIZ_VIRTUAL_SERVER: no-op (single server)",
        session.addr()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::Opcode;

    #[test]
    fn test_virtual_server_opcode_value() {
        assert_eq!(Opcode::WizVirtualServer as u8, 0x4C);
        assert_eq!(Opcode::from_byte(0x4C), Some(Opcode::WizVirtualServer));
    }

    #[test]
    fn test_virtual_server_packet_format() {
        // Client -> Server: [u16 warp_id] (but C++ ignores it)
        use ko_protocol::{Packet, PacketReader};
        let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
        pkt.write_u16(1);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(1));
        assert_eq!(r.remaining(), 0);
    }

    // ── Sprint 932: Additional coverage ──────────────────────────────

    /// C2S data length: warp_id(2) = 2 bytes.
    #[test]
    fn test_virtual_server_c2s_data_length() {
        use ko_protocol::Packet;
        let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
        pkt.write_u16(1);
        assert_eq!(pkt.data.len(), 2);
    }

    /// No-op handler — no S2C response is sent.
    #[test]
    fn test_virtual_server_no_s2c_response() {
        // The handler is a no-op; it doesn't construct any response packet.
        use ko_protocol::Packet;
        let pkt = Packet::new(Opcode::WizVirtualServer as u8);
        assert_eq!(pkt.opcode, 0x4C);
    }

    /// Various warp_id values roundtrip.
    #[test]
    fn test_virtual_server_warp_id_roundtrip() {
        use ko_protocol::{Packet, PacketReader};
        for id in [0u16, 1, 100, 255, u16::MAX] {
            let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
            pkt.write_u16(id);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u16(), Some(id));
            assert_eq!(r.remaining(), 0);
        }
    }

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_virtual_server_dispatch_range() {
        let op = Opcode::WizVirtualServer as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// Zero warp_id is valid.
    #[test]
    fn test_virtual_server_zero_warp_id() {
        use ko_protocol::{Packet, PacketReader};
        let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
        pkt.write_u16(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(0));
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode from_byte roundtrip for 0x4C.
    #[test]
    fn test_virtual_server_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x4C), Some(Opcode::WizVirtualServer));
    }

    /// LE encoding of warp_id.
    #[test]
    fn test_virtual_server_le_encoding() {
        use ko_protocol::Packet;
        let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
        pkt.write_u16(0x0102);
        assert_eq!(pkt.data[0], 0x02);
        assert_eq!(pkt.data[1], 0x01);
    }

    /// Max warp_id roundtrip.
    #[test]
    fn test_virtual_server_max_warp_id() {
        use ko_protocol::{Packet, PacketReader};
        let mut pkt = Packet::new(Opcode::WizVirtualServer as u8);
        pkt.write_u16(u16::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(u16::MAX));
    }
}
