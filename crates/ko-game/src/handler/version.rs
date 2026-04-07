//! Game server version check handler.
//!
//! ## Wireshark-Verified Wire Format (ko-original.pcap frame 1535)
//!
//! Response: `[pvp:u8] [version:u16] [key_len:u8=16] [key:16bytes] [trailer:u8=0]`
//! = 21 bytes data, wire payload = 22 bytes (0x16).
//!
//! After sending the key, the server enables AES-128-CBC encryption.
//! All subsequent packets use AES (flag byte 0x01 in payload).
//!
//! ## Client-Side Flow (v2600 VA 0x708741)
//!
//! 1. sub_833C10(key, key_len) → stores AES key, sets has_aes_key=1
//! 2. sub_833A30(0,0) → cipher reset/init, joe_encrypt_on=0
//! 3. MOV [session+0x140], 1 → per-session encryption flag
//! 4. Callback at [0x011746A4] → sets joe_encrypt_on=1 (patched in patcher)

use ko_protocol::{Opcode, Packet};

use crate::session::{ClientSession, SessionState};

/// Fallback version if server_settings is not loaded yet.
/// v2600 client expects 2599 (0x0A27).
pub const DEFAULT_SERVER_VERSION: u16 = 2599;

/// Resolve the game version from DB (server_settings.game_version).
fn resolve_version(session: &ClientSession) -> u16 {
    session
        .world()
        .get_server_settings()
        .map(|s| s.game_version as u16)
        .unwrap_or(DEFAULT_SERVER_VERSION)
}

/// Handle WIZ_VERSION_CHECK from the client.
///
/// Wireshark-verified format: `[pvp:u8] [version:u16] [key_len:u8=16] [key:16bytes] [trailer:u8=0]`
/// After sending key, AES-128-CBC is enabled for all subsequent packets.
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::Connected {
        return Ok(());
    }

    let version = resolve_version(session);

    // Generate AES key and send in version response
    let key = ko_protocol::aes_crypt::AesCryption::generate_key();

    let mut response = Packet::new(Opcode::WizVersionCheck as u8);
    response.write_u8(0);        // pvp_flag
    response.write_u16(version); // version
    response.write_u8(16);       // key_len = 16 bytes
    response.write_bytes(&key);  // AES-128 key
    response.write_u8(0);        // trailer

    // Send version response (plaintext — AES not enabled yet)
    session.send_packet(&response).await?;

    // Enable AES for subsequent packets
    session.aes_mut().set_key(key);
    session.aes_mut().enable();

    session.set_state(SessionState::VersionChecked);

    tracing::info!(
        "[{}] Version check: version={} (0x{:04X}), AES enabled (key={:02X?})",
        session.addr(),
        version,
        version,
        &key,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::DEFAULT_SERVER_VERSION;
    use ko_protocol::aes_crypt::AesCryption;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_version_check_opcode() {
        assert_eq!(Opcode::WizVersionCheck as u8, 0x2B);
    }

    #[test]
    fn test_default_server_version() {
        assert_eq!(DEFAULT_SERVER_VERSION, 2599);
    }

    /// Wireshark-verified format: pvp(1) + version(2) + key_len(1) + key(16) + trailer(1) = 21.
    #[test]
    fn test_version_response_with_aes_key() {
        let key = [0x41u8; 16];
        let mut pkt = Packet::new(Opcode::WizVersionCheck as u8);
        pkt.write_u8(0);
        pkt.write_u16(DEFAULT_SERVER_VERSION);
        pkt.write_u8(16);
        pkt.write_bytes(&key);
        pkt.write_u8(0);

        assert_eq!(pkt.data.len(), 21);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_u16(), Some(2599));
        assert_eq!(r.read_u8(), Some(16));
        let mut read_key = [0u8; 16];
        for byte in &mut read_key {
            *byte = r.read_u8().expect("key byte");
        }
        assert_eq!(read_key, key);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_aes_key_generation() {
        let key = AesCryption::generate_key();
        assert_eq!(key.len(), 16);
        for &b in &key {
            assert!((0x21..=0x7E).contains(&b));
        }
    }
}
