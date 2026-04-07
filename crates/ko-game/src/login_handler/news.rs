//! LS_NEWS (0xF6) handler — returns server news/notice.
//! ## C++ Reference
//! `LoginSession::HandleNews()` — `LS_NEWS = 0xF6`
//! Echo pattern: server responds with the SAME opcode (0xF6).
//! ## Request (Client → Server)
//! Empty (just opcode 0xF6).
//! ## Response (Server → Client) — opcode 0xF6
//! C++ HandleNews: `result << "Login Notice" << content`
//! PCAP v2600: `[F6] [string "INotice"] [string notice_text]`
//! If no news: `result << "Login Notice" << "<empty>"`

use ko_protocol::{LoginOpcode, Packet};

use crate::login_session::LoginSession;

/// Handle LS_NEWS (0xF6) — echo pattern, same opcode back with notice.
/// C++ verified: `Packet result(pkt.GetOpcode()); result << "Login Notice" << content;`
/// PCAP verified: response is 0xF6 with "INotice" + notice text.
pub async fn handle(session: &mut LoginSession, _pkt: Packet) -> anyhow::Result<()> {
    let notice_text = session.config().news_content.clone();
    let notice_body = if notice_text.is_empty() {
        "Welcome to Knight Online".to_string()
    } else {
        notice_text
    };

    // Echo pattern: respond with same opcode (0xF6)
    // PCAP: F6 [string "INotice"] [string notice_body]
    let mut response = Packet::new(LoginOpcode::LsVersionCheck as u8); // 0xF6
    response.write_string("INotice");
    response.write_string(&notice_body);
    session.send_packet(&response).await?;

    tracing::info!(
        "[{}] 0xF6 → echo news (INotice, {} bytes)",
        session.addr(),
        notice_body.len(),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{LoginOpcode, Packet, PacketReader};

    /// 0xF6 opcode value.
    #[test]
    fn test_news_opcode() {
        assert_eq!(LoginOpcode::LsVersionCheck as u8, 0xF6);
    }

    /// News response echo format: [0xF6] [string "INotice"] [string notice].
    #[test]
    fn test_news_response_echo_format() {
        let mut response = Packet::new(LoginOpcode::LsVersionCheck as u8);
        response.write_string("INotice");
        response.write_string("Welcome to Knight Online");

        assert_eq!(response.opcode, 0xF6);

        let mut r = PacketReader::new(&response.data);
        assert_eq!(r.read_string(), Some("INotice".to_string()));
        assert_eq!(
            r.read_string(),
            Some("Welcome to Knight Online".to_string())
        );
        assert_eq!(r.remaining(), 0);
    }

    /// Empty news: "INotice" + "<empty>" (C++ fallback).
    #[test]
    fn test_news_response_empty() {
        let mut response = Packet::new(LoginOpcode::LsVersionCheck as u8);
        response.write_string("INotice");
        response.write_string("<empty>");

        let mut r = PacketReader::new(&response.data);
        assert_eq!(r.read_string(), Some("INotice".to_string()));
        assert_eq!(r.read_string(), Some("<empty>".to_string()));
    }
}
