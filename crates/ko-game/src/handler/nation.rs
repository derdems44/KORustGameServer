//! WIZ_SEL_NATION (0x05) handler — nation selection.
//! ## Request (C->S)
//! | Offset | Type | Description |
//! |--------|------|-------------|
//! | 0      | u8   | Nation (1=Karus, 2=Elmorad) |
//! ## Response (S->C)
//! | Offset | Type | Description |
//! |--------|------|-------------|
//! | 0      | u8   | Result (0=fail, 1=Karus, 2=Elmorad) |

use ko_db::repositories::account::AccountRepository;
use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};
use crate::world::{NATION_ELMORAD, NATION_KARUS};

/// Handle WIZ_SEL_NATION from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::LoggedIn {
        return Ok(());
    }

    let account_id = match session.account_id() {
        Some(id) => id.to_string(),
        None => return Ok(()),
    };

    let mut reader = PacketReader::new(&pkt.data);
    let nation = match reader.read_u8() {
        Some(n) if n == NATION_KARUS || n == NATION_ELMORAD => n,
        _ => {
            let mut response = Packet::new(Opcode::WizSelNation as u8);
            response.write_u8(0); // fail
            session.send_packet(&response).await?;
            return Ok(());
        }
    };

    let repo = AccountRepository::new(session.pool());

    match repo.set_nation(&account_id, nation as i16).await {
        Ok(()) => {
            session.set_state(SessionState::NationSelected);

            let mut response = Packet::new(Opcode::WizSelNation as u8);
            response.write_u8(nation);
            session.send_packet(&response).await?;

            tracing::debug!(
                "[{}] Nation selected: {} ({})",
                session.addr(),
                if nation == NATION_KARUS {
                    "Karus"
                } else {
                    "Elmorad"
                },
                account_id
            );
        }
        Err(e) => {
            tracing::error!("[{}] DB error setting nation: {}", session.addr(), e);
            let mut response = Packet::new(Opcode::WizSelNation as u8);
            response.write_u8(0);
            session.send_packet(&response).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_nation_c2s_packet_format() {
        // C2S: [u8 nation]
        let mut pkt = Packet::new(Opcode::WizSelNation as u8);
        pkt.write_u8(1); // Karus

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(1));
    }

    #[test]
    fn test_nation_success_response() {
        // Success: [u8 nation] (echo back selected nation)
        for nation in [NATION_KARUS, NATION_ELMORAD] {
            let mut response = Packet::new(Opcode::WizSelNation as u8);
            response.write_u8(nation);

            let mut reader = PacketReader::new(&response.data);
            assert_eq!(reader.read_u8(), Some(nation));
        }
    }

    #[test]
    fn test_nation_fail_response() {
        // Fail: [u8 0]
        let mut response = Packet::new(Opcode::WizSelNation as u8);
        response.write_u8(0);

        let mut reader = PacketReader::new(&response.data);
        assert_eq!(reader.read_u8(), Some(0));
    }

    #[test]
    fn test_nation_constants() {
        assert_eq!(NATION_KARUS, 1);
        assert_eq!(NATION_ELMORAD, 2);
    }

    #[test]
    fn test_nation_invalid_values_rejected() {
        // Only 1 (Karus) and 2 (Elmorad) are valid
        for invalid in [0u8, 3, 128, 255] {
            assert!(
                invalid != NATION_KARUS && invalid != NATION_ELMORAD,
                "nation {invalid} should be rejected"
            );
        }
    }

    // ── Sprint 929: Additional coverage ──────────────────────────────

    /// C2S data length: nation(1) = 1 byte.
    #[test]
    fn test_nation_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizSelNation as u8);
        pkt.write_u8(1);
        assert_eq!(pkt.data.len(), 1);
    }

    /// Response is always 1 byte (result).
    #[test]
    fn test_nation_response_data_length() {
        let mut s = Packet::new(Opcode::WizSelNation as u8);
        s.write_u8(NATION_KARUS);
        assert_eq!(s.data.len(), 1);

        let mut f = Packet::new(Opcode::WizSelNation as u8);
        f.write_u8(0);
        assert_eq!(f.data.len(), 1);
    }

    /// Opcode value is 0x05.
    #[test]
    fn test_nation_opcode_value() {
        assert_eq!(Opcode::WizSelNation as u8, 0x05);
    }

    /// Success response echoes selected nation (1 or 2), fail returns 0.
    #[test]
    fn test_nation_response_values() {
        // Success: echoes nation
        let mut karus = Packet::new(Opcode::WizSelNation as u8);
        karus.write_u8(NATION_KARUS);
        assert_eq!(karus.data[0], 1);

        let mut elmo = Packet::new(Opcode::WizSelNation as u8);
        elmo.write_u8(NATION_ELMORAD);
        assert_eq!(elmo.data[0], 2);

        // Fail: 0
        let mut fail = Packet::new(Opcode::WizSelNation as u8);
        fail.write_u8(0);
        assert_eq!(fail.data[0], 0);
    }

    /// Nation as i16 for DB save.
    #[test]
    fn test_nation_i16_conversion() {
        assert_eq!(NATION_KARUS as i16, 1);
        assert_eq!(NATION_ELMORAD as i16, 2);
    }
}
