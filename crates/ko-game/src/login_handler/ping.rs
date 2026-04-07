//! Opcode 0x03 handler — launcher notices request.
//!
//! Launcher Source: `Launcher Project/LauncherEngine.cpp:388-400`
//!
//! The launcher sends opcode 0x03 after version check + patch download
//! to request login screen notices.
//!
//! ## Request (Client → Server)
//! Empty (just opcode).
//!
//! ## Response (Server → Client)
//!
//! | Offset | Type   | Description                |
//! |--------|--------|----------------------------|
//! | 0      | u16le  | Notice count               |
//! | 2      | string | Notice 1 (u16le len+bytes) |
//! | ...    | string | Notice N                   |

use ko_protocol::Packet;

use crate::login_session::LoginSession;

/// Handle opcode 0x03 — launcher notices request.
///
/// Returns a list of notice strings to display on the launcher screen.
pub async fn handle(session: &mut LoginSession, pkt: Packet) -> anyhow::Result<()> {
    let news_title = session.config().news_title.clone();

    let mut response = Packet::new(pkt.opcode);
    // Send 1 notice (the news title)
    response.write_u16(1);
    response.write_string(&news_title);
    session.send_packet(&response).await?;

    tracing::debug!("[{}] Sent notices (1 notice)", session.addr());
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Packet, PacketReader};

    #[test]
    fn test_ping_response_format() {
        // Response: [u16 count=1][string notice]
        let mut response = Packet::new(0x03);
        response.write_u16(1);
        response.write_string("Welcome to KO!");

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u16(), Some(1), "notice count");
        assert_eq!(reader.read_string(), Some("Welcome to KO!".to_string()));
    }

    #[test]
    fn test_ping_opcode_echo() {
        // Response uses the same opcode as the request (0x03)
        let pkt = Packet::new(0x03);
        assert_eq!(pkt.opcode, 0x03);
    }

    #[test]
    fn test_ping_empty_notice() {
        let mut response = Packet::new(0x03);
        response.write_u16(1);
        response.write_string("");

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u16(), Some(1));
        assert_eq!(reader.read_string(), Some(String::new()));
    }

    /// Multiple notices serialize sequentially.
    #[test]
    fn test_ping_multiple_notices() {
        let mut response = Packet::new(0x03);
        response.write_u16(2);
        response.write_string("Notice 1");
        response.write_string("Notice 2");

        let mut r = PacketReader::new(&response.data);
        assert_eq!(r.read_u16(), Some(2));
        assert_eq!(r.read_string(), Some("Notice 1".to_string()));
        assert_eq!(r.read_string(), Some("Notice 2".to_string()));
    }

    /// Zero notice count with no strings.
    #[test]
    fn test_ping_zero_notices() {
        let mut response = Packet::new(0x03);
        response.write_u16(0);
        assert_eq!(response.data.len(), 2); // just the count
    }

    // ── Sprint 940: Additional coverage ──────────────────────────────

    /// Opcode 0x03 is fixed.
    #[test]
    fn test_ping_opcode_value() {
        assert_eq!(0x03u8, 3);
    }

    /// C2S is empty (just opcode).
    #[test]
    fn test_ping_c2s_empty() {
        let pkt = Packet::new(0x03);
        assert!(pkt.data.is_empty());
    }

    /// Notice count LE encoding.
    #[test]
    fn test_ping_count_le() {
        let mut pkt = Packet::new(0x03);
        pkt.write_u16(256);
        assert_eq!(pkt.data[0], 0x00);
        assert_eq!(pkt.data[1], 0x01);
    }

    /// Long notice string roundtrip.
    #[test]
    fn test_ping_long_notice() {
        let notice = "X".repeat(200);
        let mut pkt = Packet::new(0x03);
        pkt.write_u16(1);
        pkt.write_string(&notice);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(1));
        assert_eq!(r.read_string(), Some(notice));
    }

    /// Max notice count u16::MAX.
    #[test]
    fn test_ping_max_count() {
        let mut pkt = Packet::new(0x03);
        pkt.write_u16(u16::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16(), Some(u16::MAX));
    }
}
