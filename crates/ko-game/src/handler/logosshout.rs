//! WIZ_LOGOSSHOUT (0x7D) handler — logos shout / server-wide announcement.
//! A player with a Logos Shout item (800075000) can send a server-wide
//! colored announcement. The server validates the item, reads the message
//! and RGBA colors, then broadcasts to all players.
//! ## Client -> Server (SByte mode)
//! `[u8 sub_opcode] [u8 R] [u8 G] [u8 B] [u8 C] [sbyte_string message]`
//! ## Server -> All (success, SByte mode)
//! `[u8 2] [u8 1(success)] [u8 R] [u8 G] [u8 B] [u8 C] [sbyte_string "Name: message"] [u8 rank]`
//! ## Server -> Sender (failure)
//! `[u8 1] [u8 2(fail)]`

use std::sync::Arc;

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

/// Maximum message length for logos shout.
const MAX_MESSAGE_LEN: usize = 128;

/// Logos Shout item ID required to broadcast.
const LOGOSSHOUT_ITEM: u32 = 800_075_000;

/// Build a logos shout failure response.
fn build_fail_response() -> Packet {
    let mut pkt = Packet::new(Opcode::WizLogosshout as u8);
    pkt.write_u8(1); // type = direct to sender
    pkt.write_u8(2); // result = fail
    pkt
}

/// Build a logos shout success broadcast.
fn build_success_broadcast(r: u8, g: u8, b: u8, c: u8, message: &str, rank: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizLogosshout as u8);
    pkt.write_u8(2); // type = broadcast
    pkt.write_u8(1); // result = success
    pkt.write_u8(r);
    pkt.write_u8(g);
    pkt.write_u8(b);
    pkt.write_u8(c);
    pkt.write_sbyte_string(message);
    pkt.write_u8(rank);
    pkt
}

/// Handle WIZ_LOGOSSHOUT from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();

    let char_info = match world.get_character_info(sid) {
        Some(info) => info,
        None => return Ok(()),
    };

    // Parse packet (SByte mode in C++)
    let mut reader = PacketReader::new(&pkt.data);
    let _sub_opcode = reader.read_u8().unwrap_or(0);
    let r = reader.read_u8().unwrap_or(0);
    let g = reader.read_u8().unwrap_or(0);
    let b = reader.read_u8().unwrap_or(0);
    let c = reader.read_u8().unwrap_or(0);
    let message = reader.read_sbyte_string().unwrap_or_default();

    // Check item + validate message
    if !world.check_exist_item(sid, LOGOSSHOUT_ITEM, 1)
        || message.is_empty()
        || message.len() > MAX_MESSAGE_LEN
    {
        let fail = build_fail_response();
        session.send_packet(&fail).await?;
        debug!(
            "[{}] WIZ_LOGOSSHOUT: failed (msg_len={})",
            session.addr(),
            message.len()
        );
        return Ok(());
    }

    // Get personal rank from session handle
    let rank = world.with_session(sid, |h| h.personal_rank).unwrap_or(0);

    // Build broadcast message: "Name: message"
    let full_message = format!("{}: {}", char_info.name, message);

    // Broadcast first, then consume the item.
    let broadcast = build_success_broadcast(r, g, b, c, &full_message, rank);
    world.broadcast_to_all(Arc::new(broadcast), None);

    // Consume the item after successful broadcast
    world.rob_item(sid, LOGOSSHOUT_ITEM, 1);

    debug!(
        "[{}] WIZ_LOGOSSHOUT: '{}' broadcast to all (R={},G={},B={},C={})",
        session.addr(),
        full_message,
        r,
        g,
        b,
        c,
    );

    Ok(())
}

// ── Drop / Upgrade Notice Builders ─────────────────────────────────
//
// These are server-wide broadcasts via WIZ_LOGOSSHOUT (0x7D) sub=0x02.

/// Build a server-wide "rare item drop" notice.
/// ```text
/// Packet newpkt(WIZ_LOGOSSHOUT, uint8(0x02));
/// newpkt.SByte();
/// newpkt << uint8(0x04) << pReceiver->GetName() << pTable.m_iNum << GetLoyaltySymbolRank();
/// g_pMain->Send_All(&newpkt);
/// ```
/// Format: `[0x7D] [0x02 sub] [0x04 type] [sbyte name] [u32 item_num] [u8 rank]`
pub fn build_drop_notice(receiver_name: &str, item_num: u32, rank: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizLogosshout as u8);
    pkt.write_u8(0x02); // sub
    pkt.write_u8(0x04); // type = drop notice
    pkt.write_sbyte_string(receiver_name);
    pkt.write_u32(item_num);
    pkt.write_u8(rank);
    pkt
}

/// Build a server-wide "item upgrade" notice.
/// ```text
/// Packet result(WIZ_LOGOSSHOUT, uint8(0x02));
/// result.SByte();
/// result << uint8(0x05) << uint8(UpgradeResult) << GetName() << pItem.m_iNum << GetLoyaltySymbolRank();
/// g_pMain->Send_All(&result);
/// ```
/// Format: `[0x7D] [0x02 sub] [0x05 type] [u8 result] [sbyte name] [u32 item_num] [u8 rank]`
/// `upgrade_result`: 0 = failed, 1 = succeeded (`UpgradeFailed`/`UpgradeSucceeded`)
pub fn build_upgrade_notice(
    upgrade_result: u8,
    player_name: &str,
    item_num: u32,
    rank: u8,
) -> Packet {
    let mut pkt = Packet::new(Opcode::WizLogosshout as u8);
    pkt.write_u8(0x02); // sub
    pkt.write_u8(0x05); // type = upgrade notice
    pkt.write_u8(upgrade_result);
    pkt.write_sbyte_string(player_name);
    pkt.write_u32(item_num);
    pkt.write_u8(rank);
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_logosshout_opcode_value() {
        assert_eq!(Opcode::WizLogosshout as u8, 0x7D);
        assert_eq!(Opcode::from_byte(0x7D), Some(Opcode::WizLogosshout));
    }

    #[test]
    fn test_logosshout_client_packet_format() {
        // Client -> Server: [u8 sub] [u8 R] [u8 G] [u8 B] [u8 C] [sbyte_str message]
        let mut pkt = Packet::new(Opcode::WizLogosshout as u8);
        pkt.write_u8(1); // sub_opcode
        pkt.write_u8(255); // R
        pkt.write_u8(128); // G
        pkt.write_u8(0); // B
        pkt.write_u8(0); // C
        pkt.write_sbyte_string("Hello World!");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(255));
        assert_eq!(r.read_u8(), Some(128));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_sbyte_string(), Some("Hello World!".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_logosshout_fail_response() {
        let pkt = build_fail_response();
        assert_eq!(pkt.opcode, 0x7D);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type = direct
        assert_eq!(r.read_u8(), Some(2)); // result = fail
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_logosshout_success_broadcast() {
        let pkt = build_success_broadcast(255, 0, 0, 0, "TestPlayer: Hello!", 3);
        assert_eq!(pkt.opcode, 0x7D);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type = broadcast
        assert_eq!(r.read_u8(), Some(1)); // result = success
        assert_eq!(r.read_u8(), Some(255)); // R
        assert_eq!(r.read_u8(), Some(0)); // G
        assert_eq!(r.read_u8(), Some(0)); // B
        assert_eq!(r.read_u8(), Some(0)); // C
        assert_eq!(
            r.read_sbyte_string(),
            Some("TestPlayer: Hello!".to_string())
        );
        assert_eq!(r.read_u8(), Some(3)); // rank
        assert_eq!(r.remaining(), 0);
    }

    // ── Sprint 701: Drop/Upgrade Notice tests ──────────────────────────

    #[test]
    fn test_drop_notice_packet_format() {
        // C++ BundleSystem.cpp:258-263:
        // [0x7D] [0x02] [0x04] [sbyte name] [u32 item_num] [u8 rank]
        let pkt = build_drop_notice("TestPlayer", 910001000, 5);
        assert_eq!(pkt.opcode, 0x7D);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02)); // sub
        assert_eq!(r.read_u8(), Some(0x04)); // type = drop
        assert_eq!(r.read_sbyte_string(), Some("TestPlayer".to_string()));
        assert_eq!(r.read_u32(), Some(910001000));
        assert_eq!(r.read_u8(), Some(5)); // rank
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_upgrade_notice_success_packet_format() {
        // C++ ItemUpgradeSystem.cpp:699-702:
        // [0x7D] [0x02] [0x05] [u8 result=1] [sbyte name] [u32 item_num] [u8 rank]
        let pkt = build_upgrade_notice(1, "UpgradePlayer", 910002000, 2);
        assert_eq!(pkt.opcode, 0x7D);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02)); // sub
        assert_eq!(r.read_u8(), Some(0x05)); // type = upgrade
        assert_eq!(r.read_u8(), Some(1)); // result = succeeded
        assert_eq!(r.read_sbyte_string(), Some("UpgradePlayer".to_string()));
        assert_eq!(r.read_u32(), Some(910002000));
        assert_eq!(r.read_u8(), Some(2)); // rank
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_upgrade_notice_failed_packet_format() {
        let pkt = build_upgrade_notice(0, "FailPlayer", 910001000, 0);
        assert_eq!(pkt.opcode, 0x7D);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02)); // sub
        assert_eq!(r.read_u8(), Some(0x05)); // type = upgrade
        assert_eq!(r.read_u8(), Some(0)); // result = failed
        assert_eq!(r.read_sbyte_string(), Some("FailPlayer".to_string()));
        assert_eq!(r.read_u32(), Some(910001000));
        assert_eq!(r.read_u8(), Some(0)); // rank
        assert_eq!(r.remaining(), 0);
    }

    // ── Sprint 928: Additional coverage ──────────────────────────────

    /// Fail response data length: type(1) + result(1) = 2.
    #[test]
    fn test_logosshout_fail_data_length() {
        let pkt = build_fail_response();
        assert_eq!(pkt.data.len(), 2);
    }

    /// Success broadcast data length: type(1) + result(1) + RGBA(4) + sbyte_str + rank(1).
    #[test]
    fn test_logosshout_success_broadcast_data_length() {
        let msg = "Player: Hello!";
        let pkt = build_success_broadcast(255, 0, 0, 0, msg, 3);
        // 1 + 1 + 4 + (1 + msg.len()) + 1 = 8 + 1 + 14 = 22
        assert_eq!(pkt.data.len(), 1 + 1 + 4 + 1 + msg.len() + 1);
    }

    /// LOGOSSHOUT_ITEM constant = 800_075_000.
    #[test]
    fn test_logosshout_item_constant() {
        assert_eq!(LOGOSSHOUT_ITEM, 800_075_000);
    }

    /// MAX_MESSAGE_LEN = 128.
    #[test]
    fn test_logosshout_max_message_length() {
        assert_eq!(MAX_MESSAGE_LEN, 128);
        // Message of exactly 128 chars should be accepted
        let msg = "A".repeat(128);
        assert!(msg.len() <= MAX_MESSAGE_LEN);
        // 129 chars should be rejected
        let long_msg = "A".repeat(129);
        assert!(long_msg.len() > MAX_MESSAGE_LEN);
    }

    /// Drop notice data length: sub(1) + type(1) + sbyte_str + item(4) + rank(1).
    #[test]
    fn test_logosshout_drop_notice_data_length() {
        let name = "TestPlayer";
        let pkt = build_drop_notice(name, 910001000, 5);
        // 1 + 1 + (1 + 10) + 4 + 1 = 18
        assert_eq!(pkt.data.len(), 1 + 1 + 1 + name.len() + 4 + 1);
    }

    #[test]
    fn test_drop_notice_empty_name() {
        let pkt = build_drop_notice("", 100000000, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02));
        assert_eq!(r.read_u8(), Some(0x04));
        assert_eq!(r.read_sbyte_string(), Some(String::new()));
        assert_eq!(r.read_u32(), Some(100000000));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }
}
