//! WIZ_EDIT_BOX (0x59) handler — PPCard (product key / serial code) system.
//! This handles product key / serial code redemption ("PPCard"). The client
//! sends a serial key split into a numeric prefix (4 digits) and string
//! suffix (16 chars). The server validates format, checks the 5-minute
//! cooldown, then looks up the key in the `ppcard_list` DB table.
//! On success the card's Knight Cash / TL balance is awarded via
//! `give_balance()`.
//! ## Client -> Server
//! `[u8 opcode(4)] [i32 key_prefix] [sbyte_string key_suffix]`
//! ## Server -> Client (result)
//! `[u8 4] [u8 result]`
//! Result codes:
//! - 1 = Success ("Item has been inserted successfully. Please check the letter with pressing [L].")
//! - 2 = Failed ("The serial is not existed or wrong. Please insert other serial after 5 minutes")

use std::time::{Duration, Instant};

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};

/// PPCard result codes
const PPCARD_SUCCESS: u8 = 1;
const PPCARD_FAILED: u8 = 2;

/// Cooldown between PPCard attempts (40 seconds).
const PPCARD_COOLDOWN: Duration = Duration::from_secs(40);

/// Build a PPCard result response.
fn build_ppcard_result(error_code: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizEditBox as u8);
    pkt.write_u8(4);
    pkt.write_u8(error_code);
    pkt
}

/// Handle WIZ_EDIT_BOX from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let opcode = reader.read_u8().unwrap_or(0);

    if opcode == 4 {
        handle_ppcard(session, &mut reader).await?;
    } else {
        debug!(
            "[{}] WIZ_EDIT_BOX: unknown sub-opcode {}",
            session.addr(),
            opcode,
        );
    }

    Ok(())
}

/// PPCard redemption handler.
/// enforces cooldown, queries DB, awards Knight Cash via `GiveBalance()`.
async fn handle_ppcard(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    // Read key parts (C++ switches to SByte mode)
    let key_prefix = reader.read_i32().unwrap_or(0);
    let key_suffix = reader.read_sbyte_string().unwrap_or_default();

    // ── Cooldown check ──────────────────────────────────────────────────
    let now = Instant::now();
    let on_cooldown = world
        .with_session(sid, |h| {
            now.duration_since(h.ppcard_cooldown) < PPCARD_COOLDOWN
        })
        .unwrap_or(false);

    if on_cooldown {
        debug!("[{}] PPCard: cooldown active, rejecting", session.addr());
        session
            .send_packet(&build_ppcard_result(PPCARD_FAILED))
            .await?;
        return Ok(());
    }

    // Set cooldown immediately (C++ sets PPCardTime = UNIXTIME + PPCARD_TIME)
    world.update_session(sid, |h| {
        h.ppcard_cooldown = now;
    });

    // ── Validate key format ─────────────────────────────────────────────
    // Prefix must be exactly 4 digits, suffix must be exactly 16 chars.
    let prefix_str = format!("{}", key_prefix);
    if prefix_str.len() != 4 || !prefix_str.chars().all(|c| c.is_ascii_digit()) {
        debug!(
            "[{}] PPCard: invalid prefix length {} (expected 4)",
            session.addr(),
            prefix_str.len(),
        );
        session
            .send_packet(&build_ppcard_result(PPCARD_FAILED))
            .await?;
        return Ok(());
    }

    if key_suffix.is_empty() || key_suffix.len() != 16 {
        debug!(
            "[{}] PPCard: invalid suffix length {} (expected 16)",
            session.addr(),
            key_suffix.len(),
        );
        session
            .send_packet(&build_ppcard_result(PPCARD_FAILED))
            .await?;
        return Ok(());
    }

    // Concatenate: 4-digit prefix + 16-char suffix = 20-char final key
    let final_key = format!("{}{}", prefix_str, key_suffix);
    if final_key.len() != 20 {
        session
            .send_packet(&build_ppcard_result(PPCARD_FAILED))
            .await?;
        return Ok(());
    }

    // ── Get account/character info for DB tracking ──────────────────────
    let (account_id, char_name) = world
        .with_session(sid, |h| {
            let acct = h.account_id.clone();
            let name = h
                .character
                .as_ref()
                .map(|c| c.name.clone())
                .unwrap_or_default();
            (acct, name)
        })
        .unwrap_or_default();

    if account_id.is_empty() || char_name.is_empty() {
        session
            .send_packet(&build_ppcard_result(PPCARD_FAILED))
            .await?;
        return Ok(());
    }

    // ── DB lookup + atomic redeem ───────────────────────────────────────
    let pool = session.pool().clone();
    let repo = ko_db::repositories::ppcard::PPCardRepository::new(&pool);

    match repo.redeem(&final_key, &account_id, &char_name).await {
        Ok(Some(card)) => {
            debug!(
                "[{}] PPCard: redeemed key={} kc={} tl={} type={}",
                session.addr(),
                final_key,
                card.knight_cash,
                card.tl_balance,
                card.cash_type,
            );

            // Award Knight Cash via give_balance (C++ calls GiveBalance(Cash, tlcount))
            crate::handler::knight_cash::give_balance(session, card.knight_cash, card.tl_balance)
                .await?;

            session
                .send_packet(&build_ppcard_result(PPCARD_SUCCESS))
                .await?;
        }
        Ok(None) => {
            // Key not found or already used
            debug!(
                "[{}] PPCard: key not found or already used: {}",
                session.addr(),
                final_key,
            );
            session
                .send_packet(&build_ppcard_result(PPCARD_FAILED))
                .await?;
        }
        Err(e) => {
            warn!("[{}] PPCard: DB error during redeem: {}", session.addr(), e,);
            session
                .send_packet(&build_ppcard_result(PPCARD_FAILED))
                .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_edit_box_opcode_value() {
        assert_eq!(Opcode::WizEditBox as u8, 0x59);
        assert_eq!(Opcode::from_byte(0x59), Some(Opcode::WizEditBox));
    }

    #[test]
    fn test_ppcard_request_format() {
        // Client -> Server: [u8 4] [i32 key_prefix] [sbyte_string key_suffix]
        let mut pkt = Packet::new(Opcode::WizEditBox as u8);
        pkt.write_u8(4); // opcode
        pkt.write_i32(1234); // key prefix
        pkt.write_sbyte_string("ABCDEFGHIJKLMNOP"); // 16-char key suffix

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(4));
        assert_eq!(r.read_i32(), Some(1234));
        assert_eq!(r.read_sbyte_string(), Some("ABCDEFGHIJKLMNOP".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_ppcard_fail_response() {
        let pkt = build_ppcard_result(PPCARD_FAILED);
        assert_eq!(pkt.opcode, 0x59);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(4)); // opcode
        assert_eq!(r.read_u8(), Some(2)); // PPCARD_FAILED
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_ppcard_success_response() {
        let pkt = build_ppcard_result(PPCARD_SUCCESS);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(4));
        assert_eq!(r.read_u8(), Some(1)); // PPCARD_SUCCESS
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_ppcard_key_format_validation() {
        // Valid: 4-digit prefix
        let prefix = 1234;
        let prefix_str = format!("{}", prefix);
        assert_eq!(prefix_str.len(), 4);
        assert!(prefix_str.chars().all(|c| c.is_ascii_digit()));

        // Invalid: 3-digit prefix
        let prefix = 123;
        let prefix_str = format!("{}", prefix);
        assert_ne!(prefix_str.len(), 4);

        // Invalid: 5-digit prefix
        let prefix = 12345;
        let prefix_str = format!("{}", prefix);
        assert_ne!(prefix_str.len(), 4);

        // Invalid: negative prefix
        let prefix = -1234;
        let prefix_str = format!("{}", prefix);
        assert!(!prefix_str.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_ppcard_suffix_validation() {
        // Valid: 16-char suffix
        let suffix = "ABCDEFGHIJKLMNOP";
        assert_eq!(suffix.len(), 16);

        // Invalid: empty
        let suffix = "";
        assert!(suffix.is_empty());

        // Invalid: too short
        let suffix = "ABCDEFGHIJ";
        assert_ne!(suffix.len(), 16);

        // Invalid: too long
        let suffix = "ABCDEFGHIJKLMNOPQ";
        assert_ne!(suffix.len(), 16);
    }

    #[test]
    fn test_ppcard_final_key_concatenation() {
        let prefix = 1234;
        let suffix = "ABCDEFGHIJKLMNOP";
        let final_key = format!("{}{}", prefix, suffix);
        assert_eq!(final_key.len(), 20);
        assert_eq!(final_key, "1234ABCDEFGHIJKLMNOP");
    }

    #[test]
    fn test_ppcard_cooldown_constant() {
        // C++ User.h:55 — #define PPCARD_TIME (40)
        assert_eq!(PPCARD_COOLDOWN, Duration::from_secs(40));
    }

    #[test]
    fn test_ppcard_result_codes() {
        assert_eq!(PPCARD_SUCCESS, 1);
        assert_eq!(PPCARD_FAILED, 2);
    }

    // ── Sprint 928: Additional coverage ──────────────────────────────

    /// Result response data length: opcode(1) + result(1) = 2.
    #[test]
    fn test_ppcard_result_data_length() {
        let pkt = build_ppcard_result(PPCARD_SUCCESS);
        assert_eq!(pkt.data.len(), 2);
        let pkt2 = build_ppcard_result(PPCARD_FAILED);
        assert_eq!(pkt2.data.len(), 2);
    }

    /// C2S data length: opcode(1) + prefix(4) + sbyte_str(1+16) = 22.
    #[test]
    fn test_ppcard_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizEditBox as u8);
        pkt.write_u8(4);
        pkt.write_i32(1234);
        pkt.write_sbyte_string("ABCDEFGHIJKLMNOP");
        assert_eq!(pkt.data.len(), 22);
    }

    /// 4-digit prefix boundary values: 1000 (min) and 9999 (max).
    #[test]
    fn test_ppcard_boundary_prefix_values() {
        let min_str = format!("{}", 1000);
        assert_eq!(min_str.len(), 4);
        assert!(min_str.chars().all(|c| c.is_ascii_digit()));

        let max_str = format!("{}", 9999);
        assert_eq!(max_str.len(), 4);
        assert!(max_str.chars().all(|c| c.is_ascii_digit()));

        // 999 is 3 digits → invalid
        let short_str = format!("{}", 999);
        assert_ne!(short_str.len(), 4);
    }

    /// Final key is always 20 chars (4 prefix + 16 suffix).
    #[test]
    fn test_ppcard_final_key_length() {
        for prefix in [1000, 5555, 9999] {
            let key = format!("{}ABCDEFGHIJKLMNOP", prefix);
            assert_eq!(key.len(), 20, "prefix={prefix}");
        }
    }

    /// Cooldown is exactly 40 seconds.
    #[test]
    fn test_ppcard_cooldown_value() {
        assert_eq!(PPCARD_COOLDOWN.as_secs(), 40);
        assert_eq!(PPCARD_COOLDOWN.subsec_nanos(), 0);
    }
}
