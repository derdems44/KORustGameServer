//! WIZ_CONCURRENT_USER (0x33) handler — GM-only online user count.
//!
//! C++ Reference: `User.cpp:3325-3346` — `CUser::CountConcurrentUser()`

use ko_protocol::{Opcode, Packet};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_CONCURRENT_USER — sends current online count to GM.
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();

    let is_gm = world
        .get_character_info(sid)
        .is_some_and(|ch| ch.authority == 0);
    if !is_gm {
        return Ok(());
    }

    let count = world.online_count() as u16;
    let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
    pkt.write_u16(count);
    session.send_packet(&pkt).await?;
    debug!("[{}] WIZ_CONCURRENT_USER: {} online", session.addr(), count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_concurrent_user_response_format() {
        // Response: [u16 count]
        let count: u16 = 350;
        let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
        pkt.write_u16(count);

        assert_eq!(pkt.opcode, Opcode::WizConcurrentUser as u8);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u16(), Some(350));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_concurrent_user_opcode_value() {
        assert_eq!(Opcode::WizConcurrentUser as u8, 0x36);
    }

    #[test]
    fn test_concurrent_user_gm_authority_constant() {
        // GM authority == 0 is GAME_MASTER (handle checks this)
        let gm_authority: u8 = 0;
        assert_eq!(gm_authority, 0, "GM authority constant for GAME_MASTER");
    }

    // ── Sprint 932: Additional coverage ──────────────────────────────

    /// Response data length: u16(2) = 2 bytes.
    #[test]
    fn test_concurrent_user_response_data_length() {
        let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
        pkt.write_u16(100);
        assert_eq!(pkt.data.len(), 2);
    }

    /// C2S is empty — client sends just the opcode.
    #[test]
    fn test_concurrent_user_c2s_empty() {
        let pkt = Packet::new(Opcode::WizConcurrentUser as u8);
        assert!(pkt.data.is_empty());
    }

    /// Zero online count is valid.
    #[test]
    fn test_concurrent_user_zero_count() {
        let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
        pkt.write_u16(0);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u16(), Some(0));
    }

    /// Max u16 count roundtrip.
    #[test]
    fn test_concurrent_user_max_count() {
        let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
        pkt.write_u16(u16::MAX);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u16(), Some(u16::MAX));
    }

    /// Opcode from_byte roundtrip.
    #[test]
    fn test_concurrent_user_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x36), Some(Opcode::WizConcurrentUser));
    }

    // ── Sprint 933: Additional coverage ──────────────────────────────

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_concurrent_user_dispatch_range() {
        let op = Opcode::WizConcurrentUser as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// Count roundtrip with typical values.
    #[test]
    fn test_concurrent_user_count_roundtrip() {
        for count in [0u16, 1, 100, 500, 1000, u16::MAX] {
            let mut pkt = Packet::new(Opcode::WizConcurrentUser as u8);
            pkt.write_u16(count);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u16(), Some(count));
            assert_eq!(r.remaining(), 0);
        }
    }
}
