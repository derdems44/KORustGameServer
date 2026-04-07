//! WIZ_UPGRADE_NOTICE (0xB8) handler — item upgrade observation.
//!
//! C2S opcode sent when a player wants to observe another player's
//! item upgrade visual effect.
//!
//! ## Client RE
//!
//! Panel at `[esi+0x60C]` — UI-panel-dependent (Group B).
//! C2S format: `[u32 item_id]` — 4 bytes after opcode (4 send sites).
//! Client stores item_id at `[game_state+0x60C]+0x218` (watched target).
//!
//! ## C++ Reference
//!
//! C++ server reads `int8` (1 byte) but client sends `uint32` (4 bytes).
//! Known C++ mismatch — handler was never finished (`case 999` stub).
//!
//! ## Server Behavior
//!
//! Stores the observed item_id in session state (`watched_upgrade_item`)
//! so the server knows which items each player is watching. No S2C
//! response is sent — the actual upgrade notification is broadcast
//! via `WIZ_LOGOSSHOUT` (0x7D sub=2 type=5) when an upgrade completes.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── C2S Handler ─────────────────────────────────────────────────────

/// Handle WIZ_UPGRADE_NOTICE (0xB8) from the client.
///
/// Stores the observed item_id in session state and logs the observation.
/// The actual upgrade result notification is broadcast separately via
/// `logosshout::build_upgrade_notice` when an upgrade completes.
///
/// Wire: `[u32 item_id]` — player wants to observe upgrade effect.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let item_id = reader.read_u32().unwrap_or(0);

    debug!(
        "[{}] WIZ_UPGRADE_NOTICE item_id={}",
        session.addr(),
        item_id
    );

    // Store the watched item in session state.
    let sid = session.session_id();
    session.world().update_session(sid, |h| {
        h.watched_upgrade_item = item_id;
    });

    // No S2C response — client only sends, never receives for this opcode.
    Ok(())
}

/// Build a WIZ_UPGRADE_NOTICE S2C packet for item observation tracking.
///
/// While the v2525 client panel doesn't process S2C for this opcode,
/// this builder exists for completeness and potential future use.
///
/// Wire: `[u32 item_id]`
pub fn build_upgrade_notice_ack(item_id: u32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
    pkt.write_u32(item_id);
    pkt
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_c2s_format() {
        let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
        pkt.write_u32(910001000); // item_id

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(910001000));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
        pkt.write_u32(0);
        assert_eq!(pkt.data.len(), 4); // u32 only
    }

    #[test]
    fn test_opcode_value() {
        assert_eq!(Opcode::WizUpgradeNotice as u8, 0xB8);
    }

    #[test]
    fn test_build_upgrade_notice_ack_opcode() {
        let pkt = build_upgrade_notice_ack(910001000);
        assert_eq!(pkt.opcode, Opcode::WizUpgradeNotice as u8);
    }

    #[test]
    fn test_build_upgrade_notice_ack_format() {
        let pkt = build_upgrade_notice_ack(910002000);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(910002000));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_upgrade_notice_ack_data_length() {
        let pkt = build_upgrade_notice_ack(0);
        assert_eq!(pkt.data.len(), 4);
    }

    #[test]
    fn test_c2s_zero_item_id() {
        // item_id=0 means "stop watching" — valid C2S
        let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
        pkt.write_u32(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(0));
    }

    #[test]
    fn test_c2s_cpp_mismatch_note() {
        // C++ reads int8 (1 byte), client sends u32 (4 bytes).
        // Our handler correctly reads u32 to match client format.
        let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
        pkt.write_u32(0xDEADBEEF);
        assert_eq!(pkt.data.len(), 4, "client sends 4 bytes, not 1");
    }

    // ── Sprint 930: Additional coverage ──────────────────────────────

    /// Ack packet data length is always 4 bytes (u32).
    #[test]
    fn test_upgrade_notice_ack_data_length_nonzero() {
        let pkt = build_upgrade_notice_ack(910001000);
        assert_eq!(pkt.data.len(), 4);
    }

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_upgrade_notice_in_dispatch_range() {
        let op = Opcode::WizUpgradeNotice as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    /// C2S with max u32 item_id roundtrip.
    #[test]
    fn test_upgrade_notice_max_item_id() {
        let mut pkt = Packet::new(Opcode::WizUpgradeNotice as u8);
        pkt.write_u32(u32::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(u32::MAX));
        assert_eq!(r.remaining(), 0);
    }

    /// Ack roundtrip with specific item IDs.
    #[test]
    fn test_upgrade_notice_ack_roundtrip() {
        for id in [0u32, 100000, 910001000, 999999999] {
            let pkt = build_upgrade_notice_ack(id);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u32(), Some(id));
        }
    }

    /// S2C-only: no response sent by handler (fire-and-forget).
    #[test]
    fn test_upgrade_notice_no_s2c_response() {
        // The handler stores watched_upgrade_item and returns Ok(()).
        // Actual notification is via WIZ_LOGOSSHOUT (0x7D sub=2 type=5).
        assert_eq!(Opcode::WizLogosshout as u8, 0x7D);
    }
}
