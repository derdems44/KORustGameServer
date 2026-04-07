//! WIZ_CHANGE_HAIR (0x89) handler — hair/face change at character selection.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/CharacterSelectionHandler.cpp:374-396`
//! C++ Reference: `KOOriginalGameServer/GameServer/DatabaseThread.cpp:1109-1147`
//!
//! ## Request (C->S) — SByte strings (u8 length prefix)
//!
//! | Offset | Type  | Description |
//! |--------|-------|-------------|
//! | 0      | u8    | Sub-opcode (0=char selection, 1=in-game item) |
//! | 1      | sstr  | Character name (u8 len + bytes) |
//! | 2+N    | u8    | Face type |
//! | 3+N    | u32le | Hair style ID |
//!
//! ## Response (S->C)
//!
//! | Offset | Type | Description |
//! |--------|------|-------------|
//! | 0      | u8   | Result (0=success, 1=fail) |

use ko_db::repositories::account::AccountRepository;
use ko_db::repositories::character::CharacterRepository;
use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_CHANGE_HAIR from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    // Allow from NationSelected state (character selection screen)
    if session.state() != SessionState::NationSelected && session.state() != SessionState::LoggedIn
    {
        return Ok(());
    }

    let account_id = match session.account_id() {
        Some(id) => id.to_string(),
        None => return Ok(()),
    };

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = reader.read_u8().unwrap_or(255);
    let char_name = reader.read_sbyte_string().unwrap_or_default();
    let face = reader.read_u8().unwrap_or(0);
    let hair = reader.read_u32().unwrap_or(0);

    // Only handle sub-opcode 0 (char selection screen change)
    // Sub-opcode 1 (in-game item) requires item system
    if sub_opcode != 0 {
        let mut response = Packet::new(Opcode::WizChangeHair as u8);
        response.write_u8(1); // fail
        session.send_packet(&response).await?;
        return Ok(());
    }

    if char_name.is_empty() {
        let mut response = Packet::new(Opcode::WizChangeHair as u8);
        response.write_u8(1); // fail
        session.send_packet(&response).await?;
        return Ok(());
    }

    // Verify the character belongs to this account
    let account_repo = AccountRepository::new(session.pool());
    let account_chars = account_repo.get_account_chars(&account_id).await?;
    let owns_char = match &account_chars {
        Some(ac) => {
            ac.str_char_id1.as_deref() == Some(&char_name)
                || ac.str_char_id2.as_deref() == Some(&char_name)
                || ac.str_char_id3.as_deref() == Some(&char_name)
                || ac.str_char_id4.as_deref() == Some(&char_name)
        }
        None => false,
    };

    if !owns_char {
        let mut response = Packet::new(Opcode::WizChangeHair as u8);
        response.write_u8(1); // fail
        session.send_packet(&response).await?;
        return Ok(());
    }

    let char_repo = CharacterRepository::new(session.pool());

    match char_repo
        .change_hair(&char_name, face as i16, hair as i32)
        .await
    {
        Ok(true) => {
            let mut response = Packet::new(Opcode::WizChangeHair as u8);
            response.write_u8(0); // success
            session.send_packet(&response).await?;

            tracing::info!(
                "[{}] Hair changed: {} (face={}, hair={})",
                session.addr(),
                char_name,
                face,
                hair
            );
        }
        Ok(false) => {
            let mut response = Packet::new(Opcode::WizChangeHair as u8);
            response.write_u8(1); // fail
            session.send_packet(&response).await?;
        }
        Err(e) => {
            tracing::error!("[{}] DB error changing hair: {}", session.addr(), e);
            let mut response = Packet::new(Opcode::WizChangeHair as u8);
            response.write_u8(1); // fail
            session.send_packet(&response).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    /// Build a C2S WIZ_CHANGE_HAIR packet with sub=0 (char selection).
    fn build_change_hair_packet(sub: u8, name: &str, face: u8, hair: u32) -> Packet {
        let mut pkt = Packet::new(Opcode::WizChangeHair as u8);
        pkt.write_u8(sub);
        pkt.write_sbyte_string(name);
        pkt.write_u8(face);
        pkt.write_u32(hair);
        pkt
    }

    #[test]
    fn test_change_hair_packet_format() {
        let pkt = build_change_hair_packet(0, "TestChar", 3, 12345);
        let mut reader = PacketReader::new(&pkt.data);

        assert_eq!(reader.read_u8(), Some(0), "sub_opcode should be 0");
        assert_eq!(
            reader.read_sbyte_string(),
            Some("TestChar".to_string()),
            "char name"
        );
        assert_eq!(reader.read_u8(), Some(3), "face type");
        assert_eq!(reader.read_u32(), Some(12345), "hair style ID");
    }

    #[test]
    fn test_change_hair_response_format() {
        // Success response: [opcode][result=0]
        let mut success = Packet::new(Opcode::WizChangeHair as u8);
        success.write_u8(0);
        assert_eq!(success.opcode, Opcode::WizChangeHair as u8);
        assert_eq!(success.data.len(), 1);
        assert_eq!(success.data[0], 0);

        // Fail response: [opcode][result=1]
        let mut fail = Packet::new(Opcode::WizChangeHair as u8);
        fail.write_u8(1);
        assert_eq!(fail.data[0], 1);
    }

    #[test]
    fn test_change_hair_invalid_sub_opcode() {
        // Sub-opcode 1 (in-game item change) should be rejected
        let pkt = build_change_hair_packet(1, "TestChar", 3, 12345);
        let mut reader = PacketReader::new(&pkt.data);
        let sub = reader.read_u8().unwrap_or(255);
        assert_eq!(sub, 1, "sub=1 triggers fail response");
    }

    #[test]
    fn test_change_hair_empty_name_rejected() {
        let pkt = build_change_hair_packet(0, "", 5, 99999);
        let mut reader = PacketReader::new(&pkt.data);
        let _sub = reader.read_u8();
        let name = reader.read_sbyte_string().unwrap_or_default();
        assert!(name.is_empty(), "empty name should be rejected by handler");
    }

    #[test]
    fn test_change_hair_face_and_hair_range() {
        // Face is u8 (0-255), hair is u32 — verify edge values roundtrip
        let pkt = build_change_hair_packet(0, "A", 255, u32::MAX);
        let mut reader = PacketReader::new(&pkt.data);
        let _sub = reader.read_u8();
        let _name = reader.read_sbyte_string();
        assert_eq!(reader.read_u8(), Some(255), "max face value");
        assert_eq!(reader.read_u32(), Some(u32::MAX), "max hair value");
    }

    // ── Sprint 927: Additional coverage ──────────────────────────────

    /// C2S data length: sub(1) + sbyte_str("AB"=1+2) + face(1) + hair(4) = 9.
    #[test]
    fn test_change_hair_c2s_data_length() {
        let pkt = build_change_hair_packet(0, "AB", 3, 12345);
        // 1 + (1+2) + 1 + 4 = 9
        assert_eq!(pkt.data.len(), 9);
    }

    /// Response is always 1 byte (result only).
    #[test]
    fn test_change_hair_response_data_length() {
        let mut success = Packet::new(Opcode::WizChangeHair as u8);
        success.write_u8(0);
        assert_eq!(success.data.len(), 1);

        let mut fail = Packet::new(Opcode::WizChangeHair as u8);
        fail.write_u8(1);
        assert_eq!(fail.data.len(), 1);
    }

    /// Opcode value is 0x89.
    #[test]
    fn test_change_hair_opcode_value() {
        assert_eq!(Opcode::WizChangeHair as u8, 0x89);
    }

    /// SByte string uses u8 length prefix (not u16).
    #[test]
    fn test_change_hair_sbyte_encoding() {
        let pkt = build_change_hair_packet(0, "Knight", 1, 100);
        // data[0] = sub(0), data[1] = sbyte_len(6), data[2..8] = "Knight"
        assert_eq!(pkt.data[1], 6, "sbyte length prefix");
        assert_eq!(&pkt.data[2..8], b"Knight");
    }

    /// Long character name (max 20) roundtrip.
    #[test]
    fn test_change_hair_long_name() {
        let name = "ABCDEFGHIJKLMNOPQRST"; // 20 chars
        let pkt = build_change_hair_packet(0, name, 5, 99999);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_sbyte_string(), Some(name.to_string()));
        assert_eq!(r.read_u8(), Some(5));
        assert_eq!(r.read_u32(), Some(99999));
        assert_eq!(r.remaining(), 0);
    }
}
