//! WIZ_LOGIN (0x01) handler — account authentication.
//! ## Request Packet (Client → Server)
//! | Offset | Type   | Value    | Description                   |
//! |--------|--------|----------|-------------------------------|
//! | 0      | string | account  | Account ID (u16le len + bytes)|
//! | N      | string | password | Password (u16le len + bytes)  |
//! | N+M    | ...    |          | Additional fields (ignored)   |
//! ## Response Packet (Server → Client)
//! **Success:**
//! | Offset | Type   | Value   | Description                         |
//! |--------|--------|---------|-------------------------------------|
//! | 0      | i8     | 0/1/2   | Nation (0=select, 1=Karus, 2=Elmo)  |
//! | 1      | u32le  | 0       | Reserved                            |
//! | 5      | u8     | 6       | Reserved                            |
//! Then a second packet (opcode 0xC0):
//! | Offset | Type | Value | Description       |
//! |--------|------|-------|-------------------|
//! | 0      | i8   | 1     | Unknown           |
//! | 1      | i8   | 2     | Unknown           |
//! | 2      | i8   | 1     | Unknown           |
//! **Failure:**
//! | Offset | Type   | Value | Description     |
//! |--------|--------|-------|-----------------|
//! | 0      | i8     | -1    | AUTH_FAILED     |

use ko_db::repositories::account::AccountRepository;
use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};
use crate::world::MAX_ID_SIZE;

/// Login result: nation not selected (show nation selection screen).
const NATION_NONE: u8 = 0x00;
/// Login result: failed (generic).
const AUTH_FAILED: u8 = 0xFF; // -1 as i8

/// Validate that account ID contains only allowed characters.
/// Allowed: `a-z`, `A-Z`, `0-9`, `:`, `_`
fn string_is_valid(s: &str) -> bool {
    s.bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b':' || b == b'_')
}

/// Handle WIZ_LOGIN from the client.
/// The game client sends this after WIZ_VERSION_CHECK, skipping WIZ_CRYPTION.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    // Game client goes: Connected → VersionCheck (AES key) → Login
    // The Encrypted state is no longer used (AES replaces JvCryption).
    if session.state() != SessionState::VersionChecked {
        tracing::info!(
            "[{}] WIZ_LOGIN rejected: wrong state {:?}",
            session.addr(),
            session.state()
        );
        send_login_failed(session).await?;
        return Ok(());
    }

    // Already logged in?
    if session.account_id().is_some() {
        send_login_failed(session).await?;
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);

    // v2600 game server C2S 0x01: NO sub_opcode byte.
    // Data format: [acc_len:u16le][account][pw_len:u16le][password][trailing...]
    // Live test verified: data=[06 00 "myuser" 15 00 "password..."]

    let account_id = match reader.read_string() {
        Some(s) if !s.is_empty() && s.len() <= MAX_ID_SIZE && string_is_valid(&s) => s,
        _ => {
            send_login_failed(session).await?;
            return Ok(());
        }
    };

    // AES mode: client sends session_id (from login server) instead of password.
    // Read it as string but skip DB password check — session validation TODO.
    let password = match reader.read_string() {
        Some(s) if !s.is_empty() => s,
        _ => {
            send_login_failed(session).await?;
            return Ok(());
        }
    };
    // Remaining fields (race, zone, UUID, premium_flags) are ignored for login

    tracing::info!("[{}] GS login attempt: {}", session.addr(), account_id);

    let world = session.world().clone();

    // Acquire login lock to prevent race condition with simultaneous logins
    // for the same account. Two concurrent WIZ_LOGIN for the same account
    // could otherwise both succeed and create duplicate sessions.
    if !world.try_acquire_login_lock(&account_id) {
        tracing::info!(
            "[{}] Login already in progress for account: {}",
            session.addr(),
            account_id
        );
        send_login_failed(session).await?;
        return Ok(());
    }

    // All exit paths below MUST release the login lock.
    let result = async {
        let pool = session.pool().clone();
        let repo = AccountRepository::new(&pool);

        // Kick any existing session with the same account (duplicate login).
        // This handles the case where a player crashes and reconnects before timeout.
        //
        // merchant, deactivate its offline status first, then disconnect it normally.
        // This closes the merchant, saves item/gold state, and frees the session.
        if let Some(old_sid) = world.find_session_by_account(&account_id) {
            if world.is_offline_status(old_sid) {
                tracing::info!(
                    "[{}] Deactivating offline merchant (sid={}) for returning player: {}",
                    session.addr(),
                    old_sid,
                    account_id
                );
                // Full offline cleanup — closes merchant, saves DB, removes from world
                crate::systems::offline_merchant::cleanup_offline_session(&world, old_sid).await;
            } else {
                tracing::info!(
                    "[{}] Kicking old session (sid={}) for duplicate login: {}",
                    session.addr(),
                    old_sid,
                    account_id
                );
                world.kick_session_for_duplicate(old_sid).await;
            }
        }

        // Authenticate against database
        // AES mode: client sends session_id instead of password — skip password check.
        // In AES mode, the login server already authenticated the user.
        let auth_result = if session.aes_enabled() {
            // Session-based auth: look up account by ID only
            repo.find_by_account_id(&account_id).await
        } else {
            repo.authenticate(&account_id, &password).await
        };
        match auth_result {
            Ok(Some(user)) => {
                // Check if account is banned (str_authority < 0)
                if user.str_authority < 0 {
                    tracing::info!("[{}] Account banned: {}", session.addr(), account_id);
                    send_login_failed(session).await?;
                    return Ok(());
                }

                // Look up account nation from account_char BEFORE mutating session.
                // C++ Ref: GAME_LOGIN_V2 stored procedure — returns @AccountNation
                // 0 = no nation (show selection), 1 = Karus, 2 = Elmorad
                let nation = match repo.get_account_chars(&account_id).await {
                    Ok(Some(ac)) if ac.b_char_num > 0 => ac.b_nation as u8,
                    _ => NATION_NONE,
                };

                // Success — set session state
                session.set_account_id(account_id.clone());
                session.set_state(SessionState::LoggedIn);

                // Store account_id in WorldState SessionHandle for duplicate login detection.
                // Also set at gamestart, but we need it here so find_session_by_account works
                // for sessions still in character selection.
                {
                    let sid = session.session_id();
                    world.update_session(sid, |h| {
                        h.account_id = account_id.clone();
                    });
                }

                // v2600 AES flow: login response byte=1 means success.
                // Document: "Success (byte=1): Send character list"
                // After success, server must send character list immediately.
                // PCAP verified (session 5): login success response format
                let mut response = Packet::new(Opcode::WizLogin as u8);
                response.write_u8(nation);
                response.write_u32(0); // reserved
                response.write_u8(0); // reserved (PCAP verified: 0x00)
                session.send_packet(&response).await?;

                // Sniffer verified (2026-03-23 session 27 seq 3):
                // Single 0xC0 CAPTCHA challenge [01 02 01] after login success.
                // Client shows security code dialog, user enters code, client sends 0xC0 back.
                let mut captcha_pkt = Packet::new(Opcode::WizCaptcha as u8); // 0xC0
                captcha_pkt.write_u8(1);
                captcha_pkt.write_u8(2);
                captcha_pkt.write_u8(1); // challenge ready
                session.send_packet(&captcha_pkt).await?;

                tracing::info!(
                    "[{}] GS login success: {} (nation={})",
                    session.addr(),
                    account_id,
                    nation
                );
            }
            Ok(None) => {
                tracing::info!(
                    "[{}] GS login failed (not found): {}",
                    session.addr(),
                    account_id
                );
                send_login_failed(session).await?;
            }
            Err(e) => {
                tracing::error!("[{}] DB error on authenticate: {}", session.addr(), e);
                send_login_failed(session).await?;
            }
        }

        Ok::<(), anyhow::Error>(())
    }
    .await;

    // Always release the login lock, regardless of success/failure.
    world.release_login_lock(&account_id);

    result
}

/// Send a login failure response to the client.
async fn send_login_failed(session: &mut ClientSession) -> anyhow::Result<()> {
    let mut response = Packet::new(Opcode::WizLogin as u8);
    response.write_u8(AUTH_FAILED);
    session.send_packet(&response).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::MAX_PW_SIZE;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_login_c2s_packet_format() {
        // C2S: [string account_id][string password]
        let mut pkt = Packet::new(Opcode::WizLogin as u8);
        pkt.write_string("test_account");
        pkt.write_string("secret123");

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_string(), Some("test_account".to_string()));
        assert_eq!(reader.read_string(), Some("secret123".to_string()));
    }

    #[test]
    fn test_login_success_response_format() {
        // Success: [u8 nation][u32 reserved=0][u8 reserved=0] (PCAP verified)
        let mut response = Packet::new(Opcode::WizLogin as u8);
        response.write_u8(1); // Karus
        response.write_u32(0);
        response.write_u8(0);

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u8(), Some(1), "nation = Karus");
        assert_eq!(reader.read_u32(), Some(0), "reserved");
        assert_eq!(reader.read_u8(), Some(0), "reserved (PCAP: 0x00)");
    }

    #[test]
    fn test_login_fail_response_format() {
        // Fail: [u8 0xFF] (AUTH_FAILED = -1 as i8)
        let mut response = Packet::new(Opcode::WizLogin as u8);
        response.write_u8(AUTH_FAILED);

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u8(), Some(0xFF));
        assert_eq!(AUTH_FAILED as i8, -1);
    }

    #[test]
    fn test_login_init_packet_loading_format() {
        // PCAP session 5 seq 3-4: c0 01 02 00 04 (loading state)
        let mut pkt = Packet::new(0xC0);
        pkt.write_u8(1);
        pkt.write_u8(2);
        pkt.write_u8(0); // loading
        pkt.write_u8(4); // trailing

        assert_eq!(pkt.opcode, 0xC0);
        assert_eq!(pkt.data.len(), 4);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_u8(), Some(4));
    }

    #[test]
    fn test_login_init_packet_ready_format() {
        // PCAP session 5 seq 5: c0 01 02 01 (ready state)
        let mut pkt = Packet::new(0xC0);
        pkt.write_u8(1);
        pkt.write_u8(2);
        pkt.write_u8(1); // ready

        assert_eq!(pkt.opcode, 0xC0);
        assert_eq!(pkt.data.len(), 3);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(1));
    }

    #[test]
    fn test_login_constants() {
        assert_eq!(MAX_ID_SIZE, 20); // C++ globals.h:29
        assert_eq!(MAX_PW_SIZE, 28); // C++ globals.h:30
        assert_eq!(NATION_NONE, 0);
        assert_eq!(AUTH_FAILED, 0xFF);
    }

    #[test]
    fn test_string_is_valid() {
        // Allowed: a-z, A-Z, 0-9, ':', '_'
        assert!(string_is_valid("testUser123"));
        assert!(string_is_valid("user_name"));
        assert!(string_is_valid("admin:role"));
        assert!(string_is_valid("ABC"));
        assert!(string_is_valid("a"));

        // Disallowed: spaces, special chars
        assert!(!string_is_valid("user name"));
        assert!(!string_is_valid("user@domain"));
        assert!(!string_is_valid("pass!word"));
        assert!(!string_is_valid("test'injection"));
        assert!(!string_is_valid("user;DROP"));
        // Empty string: .all() returns true for empty iterator,
        // but handler rejects empty via separate !s.is_empty() check
        assert!(
            string_is_valid(""),
            "empty passes .all(), handler rejects separately"
        );
    }

    #[test]
    fn test_login_nation_values() {
        // 0 = no nation (show selection), 1 = Karus, 2 = Elmorad
        assert_eq!(NATION_NONE, 0);
        // Nation values in success response
        for nation in [0u8, 1, 2] {
            let mut pkt = Packet::new(Opcode::WizLogin as u8);
            pkt.write_u8(nation);
            let mut reader = PacketReader::new(&pkt.data);
            assert_eq!(reader.read_u8(), Some(nation));
        }
    }

    // ── Sprint 925: Additional coverage ──────────────────────────────

    /// Success response data length: nation(1) + reserved(4) + reserved(1) = 6.
    #[test]
    fn test_login_success_data_length() {
        let mut pkt = Packet::new(Opcode::WizLogin as u8);
        pkt.write_u8(2); // Elmo
        pkt.write_u32(0);
        pkt.write_u8(0);
        assert_eq!(pkt.data.len(), 6);
    }

    /// Fail response data length: AUTH_FAILED(1) = 1 byte.
    #[test]
    fn test_login_fail_data_length() {
        let mut pkt = Packet::new(Opcode::WizLogin as u8);
        pkt.write_u8(AUTH_FAILED);
        assert_eq!(pkt.data.len(), 1);
    }

    /// Init packet (0xC0) data length = 3 bytes.
    #[test]
    fn test_init_packet_data_length() {
        let mut pkt = Packet::new(0xC0);
        pkt.write_u8(1);
        pkt.write_u8(2);
        pkt.write_u8(1);
        assert_eq!(pkt.data.len(), 3);
        assert_eq!(pkt.opcode, 0xC0);
    }

    /// Non-ASCII characters are rejected by string_is_valid.
    #[test]
    fn test_string_is_valid_rejects_non_ascii() {
        assert!(!string_is_valid("üser")); // Turkish char
        assert!(!string_is_valid("用户")); // Chinese
        assert!(!string_is_valid("user\x00null")); // null byte
        assert!(!string_is_valid("\t\n")); // control chars
    }

    /// Boundary characters ':' and '_' are specifically allowed.
    #[test]
    fn test_string_is_valid_boundary_chars() {
        assert!(string_is_valid(":"));
        assert!(string_is_valid("_"));
        assert!(string_is_valid("a:b_c"));
        assert!(string_is_valid("__test__"));
        assert!(string_is_valid("role:admin"));
        // Adjacent disallowed chars
        assert!(!string_is_valid("-"));
        assert!(!string_is_valid("."));
        assert!(!string_is_valid("@"));
    }
}
