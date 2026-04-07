//! WIZ_WEBPAGE (0x6F) handler -- Open URL in client browser.
//! This is an S2C-only packet that tells the client to open a URL.
//! The client calls `ShellExecuteA` with the provided URL.
//! Typically used by GMs to direct players to event pages or announcements.
//! ## Server -> Client
//! ```text
//! [sbyte_string url]
//! ```
//! ## Client behavior (IDA sub_938730)
//! When the client receives opcode 0x6F, it triggers the internal browser
//! or system browser with the URL payload.

use ko_protocol::{Opcode, Packet};
use tracing::debug;

use crate::session::{ClientSession, SessionState};
use crate::world::WorldState;
use crate::zone::SessionId;

/// Maximum URL length (safety limit).
const MAX_URL_LENGTH: usize = 255;

/// Handle WIZ_WEBPAGE (0x6F) -- server sends URL to client.
/// This is primarily S2C, but the client dispatch does route 0x6F
/// to a handler. If the client sends this opcode, we ignore it
/// (only GMs can trigger URL sends via the `send_webpage` builder).
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }
    // C2S: no action needed -- this is S2C only.
    debug!(
        "[{}] WIZ_WEBPAGE: received C2S (ignored, S2C-only)",
        session.addr()
    );
    Ok(())
}

/// Build WIZ_WEBPAGE S2C packet to open a URL on the client.
/// Format: `[sbyte_string url]`
/// Used by GM commands to direct players to web pages.
pub fn build_webpage_packet(url: &str) -> Option<Packet> {
    if url.is_empty() || url.len() > MAX_URL_LENGTH {
        return None;
    }
    let mut pkt = Packet::new(Opcode::WizWebpage as u8);
    pkt.write_sbyte_string(url);
    Some(pkt)
}

/// Send a webpage URL to a specific session.
/// Called from GM command handlers to open a URL in the target player's client.
pub fn send_webpage_to_session(world: &WorldState, target_sid: SessionId, url: &str) {
    if let Some(pkt) = build_webpage_packet(url) {
        world.send_to_session(target_sid, &pkt);
    }
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, PacketReader};

    use super::*;

    #[test]
    fn test_webpage_opcode_value() {
        assert_eq!(Opcode::WizWebpage as u8, 0x6F);
    }

    #[test]
    fn test_webpage_packet_format() {
        let pkt = build_webpage_packet("http://example.com").unwrap();
        assert_eq!(pkt.opcode, Opcode::WizWebpage as u8);
        let mut reader = PacketReader::new(&pkt.data);
        let url = reader.read_sbyte_string();
        assert_eq!(url.as_deref(), Some("http://example.com"));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_webpage_empty_url_rejected() {
        assert!(build_webpage_packet("").is_none());
    }

    #[test]
    fn test_webpage_too_long_url_rejected() {
        let long_url = "h".repeat(256);
        assert!(build_webpage_packet(&long_url).is_none());
    }

    #[test]
    fn test_webpage_max_length_url_accepted() {
        let url = "h".repeat(255);
        assert!(build_webpage_packet(&url).is_some());
    }

    #[test]
    fn test_webpage_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x6F), Some(Opcode::WizWebpage));
    }

    #[test]
    fn test_webpage_packet_data_length() {
        let url = "http://test.com";
        let pkt = build_webpage_packet(url).unwrap();
        // sbyte string: 1 byte length + url bytes
        assert_eq!(pkt.data.len(), 1 + url.len());
    }

    #[test]
    fn test_webpage_various_urls() {
        let urls = [
            "http://example.com",
            "https://game.example.com/event",
            "http://localhost:8080/page?id=1&type=event",
        ];
        for url in urls {
            let pkt = build_webpage_packet(url).unwrap();
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_sbyte_string().as_deref(), Some(url));
            assert_eq!(r.remaining(), 0);
        }
    }
}
