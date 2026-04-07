//! WIZ_LOADING_LOGIN (0x9F) handler — server queue / capacity check.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/CharacterSelectionHandler.cpp:1231-1265`
//!
//! ## Request Packet (Client → Server)
//!
//! | Offset | Type | Value | Description          |
//! |--------|------|-------|----------------------|
//! | 0      | u8   | 0x01  | Unknown flag         |
//!
//! ## Response Packet (Server → Client)
//!
//! | Offset | Type  | Value | Description              |
//! |--------|-------|-------|--------------------------|
//! | 0      | u8    | 0x01  | Success flag             |
//! | 1      | i32le | N     | Queue position (0 = ok)  |

use ko_protocol::{Opcode, Packet};

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_LOADING_LOGIN from the client.
///
/// The client sends this right after WIZ_LOGIN succeeds.
/// Server responds with queue position (0 means no queue, proceed).
/// C++ Reference: `CharacterSelectionHandler.cpp:1231-1265`
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    // Only accept after login (before game start).
    match session.state() {
        SessionState::LoggedIn | SessionState::NationSelected | SessionState::CharacterSelected => {
        }
        _ => return Ok(()),
    }
    let mut response = Packet::new(Opcode::WizLoadingLogin as u8);
    response.write_u8(1); // success flag
    response.write_i32(0); // queue position: 0 = no queue

    session.send_packet(&response).await?;

    tracing::info!("[{}] Loading login OK (no queue)", session.addr());
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_loading_login_response_format() {
        // Response: [u8 success=1][i32 queue_position=0]
        let mut response = Packet::new(Opcode::WizLoadingLogin as u8);
        response.write_u8(1);
        response.write_i32(0);

        assert_eq!(response.opcode, Opcode::WizLoadingLogin as u8);
        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u8(), Some(1), "success flag");
        assert_eq!(reader.read_i32(), Some(0), "no queue");
    }

    #[test]
    fn test_loading_login_opcode_value() {
        assert_eq!(Opcode::WizLoadingLogin as u8, 0x9F);
    }

    #[test]
    fn test_loading_login_queue_position_format() {
        // If server had a queue, position would be > 0
        let mut response = Packet::new(Opcode::WizLoadingLogin as u8);
        response.write_u8(1);
        response.write_i32(42); // queue position 42

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_i32(), Some(42));
    }

    // ── Sprint 931: Additional coverage ──────────────────────────────

    /// Response data length: success(1) + queue_pos(4) = 5.
    #[test]
    fn test_loading_login_response_data_length() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(1);
        pkt.write_i32(0);
        assert_eq!(pkt.data.len(), 5);
    }

    /// C2S has only u8 flag = 1.
    #[test]
    fn test_loading_login_c2s_format() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(0x01);
        assert_eq!(pkt.data.len(), 1);
        assert_eq!(pkt.data[0], 1);
    }

    /// Queue position 0 means "no queue, proceed".
    #[test]
    fn test_loading_login_no_queue() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(1);
        pkt.write_i32(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(0), "0 = no queue");
        assert_eq!(r.remaining(), 0);
    }

    /// Negative queue position edge case.
    #[test]
    fn test_loading_login_negative_queue() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(1);
        pkt.write_i32(-1);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8();
        assert_eq!(r.read_i32(), Some(-1));
    }

    /// Success flag is always 1.
    #[test]
    fn test_loading_login_success_flag() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(1);
        pkt.write_i32(0);
        assert_eq!(pkt.data[0], 1);
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode from_byte roundtrip for 0x9F.
    #[test]
    fn test_loading_login_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x9F), Some(Opcode::WizLoadingLogin));
    }

    /// Full response roundtrip — success(1) + queue_pos(0).
    #[test]
    fn test_loading_login_full_roundtrip() {
        let mut pkt = Packet::new(Opcode::WizLoadingLogin as u8);
        pkt.write_u8(1);
        pkt.write_i32(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(0));
        assert_eq!(r.remaining(), 0);
    }
}
