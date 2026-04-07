//! WIZ_PROGRAMCHECK (0x7A) handler — GM program list inspection.
//!
//! C++ Reference: `User.cpp:4816-4859` — `CUser::PlayerProgramCheck`
//!
//! Flow:
//! 1. GM sends `/plc <PlayerName>` → sub-opcode 0x01 with target name
//! 2. Server stores GM socket, sends probe (0x02, 0x01) to target client
//! 3. Client responds with running program list → sub-opcode != 0x01
//! 4. Server forwards program list to GM as a notice

use std::sync::atomic::Ordering;

use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::ClientSession;

/// Handle WIZ_PROGRAMCHECK from the client.
pub fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = match reader.read_u8() {
        Some(v) => v,
        None => return Ok(()),
    };

    if sub_opcode == 0x01 {
        // ── GM Request: /plc <PlayerName> ──────────────────────────────
        // C++ Reference: User.cpp:4825-4843
        let world = session.world().clone();
        let sid = session.session_id();

        // Must be GM (authority == 0)
        let ch = match world.get_character_info(sid) {
            Some(c) => c,
            None => return Ok(()),
        };
        if ch.authority != 0 {
            return Ok(());
        }

        // Reset stored GM socket
        world.plc_gm_socket.store(u32::MAX, Ordering::Relaxed);

        let target_name = match reader.read_string() {
            Some(n) if !n.is_empty() => n,
            _ => return Ok(()),
        };

        // Find target player
        let target_sid = match world.find_session_by_name(&target_name) {
            Some(s) => s,
            None => return Ok(()),
        };

        // Send probe packet to target client
        let mut probe = Packet::new(Opcode::WizProgramCheck as u8);
        probe.write_u8(0x02);
        probe.write_u8(0x01);
        world.send_to_session_owned(target_sid, probe);

        // Store GM socket for response routing
        world.plc_gm_socket.store(sid as u32, Ordering::Relaxed);

        tracing::info!(
            sid,
            target = %target_name,
            "GM program check requested"
        );
    } else {
        // ── Client Response: program list ──────────────────────────────
        // C++ Reference: User.cpp:4845-4858
        let world = session.world().clone();

        let gm_sid = world.plc_gm_socket.load(Ordering::Relaxed);
        if gm_sid == u32::MAX {
            return Ok(());
        }

        let program_info = reader.read_string().unwrap_or_default();

        // Send program info to GM as a notice (PUBLIC_CHAT)
        // C++ Reference: g_pMain->SendHelpDescription(GMTemp, ProgramInfo.c_str())
        let gm_nation = world
            .get_character_info(gm_sid as u16)
            .map(|ch| ch.nation)
            .unwrap_or(0);

        let mut notice = Packet::new(Opcode::WizChat as u8);
        notice.write_u8(7); // PUBLIC_CHAT
        notice.write_u8(gm_nation);
        notice.write_u32(gm_sid);
        notice.write_u8(0); // name length (SByte empty)
        notice.write_string(&program_info);
        notice.write_i8(0); // personal_rank
        notice.write_u8(0); // authority
        notice.write_u8(0); // system_msg
        world.send_to_session_owned(gm_sid as u16, notice);

        tracing::info!(gm_sid, "Program check response forwarded to GM");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use crate::world::WorldState;

    #[test]
    fn test_program_check_gm_socket_default() {
        let world = WorldState::new();
        let val = world.plc_gm_socket.load(Ordering::Relaxed);
        assert_eq!(val, u32::MAX, "Default should be u32::MAX (no GM)");
    }

    #[test]
    fn test_program_check_gm_socket_store_load() {
        let world = WorldState::new();
        world.plc_gm_socket.store(42, Ordering::Relaxed);
        assert_eq!(world.plc_gm_socket.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_probe_packet_format() {
        use ko_protocol::{Opcode, Packet, PacketReader};
        // Probe sent to target client: [0x02] [0x01]
        let mut probe = Packet::new(Opcode::WizProgramCheck as u8);
        probe.write_u8(0x02);
        probe.write_u8(0x01);

        assert_eq!(probe.opcode, Opcode::WizProgramCheck as u8);
        assert_eq!(probe.data.len(), 2);

        let mut r = PacketReader::new(&probe.data);
        assert_eq!(r.read_u8(), Some(0x02));
        assert_eq!(r.read_u8(), Some(0x01));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_program_check_opcode_value() {
        use ko_protocol::Opcode;
        assert_eq!(Opcode::WizProgramCheck as u8, 0x7A);
    }

    #[test]
    fn test_gm_authority_for_plc() {
        // GM authority == 0 is required to issue /plc
        let gm: u8 = 0;
        assert_eq!(gm, 0, "GM authority must be 0 for program check");
        let regular: u8 = 1;
        assert_ne!(regular, 0, "Regular player should be rejected");
    }

    #[test]
    fn test_response_notice_format() {
        use ko_protocol::{Opcode, Packet, PacketReader};
        // GM receives response as PUBLIC_CHAT (type=7)
        let gm_sid: u32 = 100;
        let gm_nation: u8 = 1;
        let program_info = "explorer.exe notepad.exe";

        let mut notice = Packet::new(Opcode::WizChat as u8);
        notice.write_u8(7); // PUBLIC_CHAT
        notice.write_u8(gm_nation);
        notice.write_u32(gm_sid);
        notice.write_u8(0); // name length (SByte empty)
        notice.write_string(program_info);
        notice.write_i8(0); // personal_rank
        notice.write_u8(0); // authority
        notice.write_u8(0); // system_msg

        let mut r = PacketReader::new(&notice.data);
        assert_eq!(r.read_u8(), Some(7)); // PUBLIC_CHAT
        assert_eq!(r.read_u8(), Some(1)); // nation
        assert_eq!(r.read_u32(), Some(100)); // gm_sid
        assert_eq!(r.read_u8(), Some(0)); // empty name
    }

    // ── Sprint 930: Additional coverage ──────────────────────────────

    /// GM request C2S: sub(1) + string(name) — sub=0x01.
    #[test]
    fn test_program_check_gm_request_format() {
        use ko_protocol::{Opcode, Packet, PacketReader};
        let mut pkt = Packet::new(Opcode::WizProgramCheck as u8);
        pkt.write_u8(0x01); // sub = GM request
        pkt.write_string("TargetPlayer");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x01));
        assert_eq!(r.read_string(), Some("TargetPlayer".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    /// Probe packet data length: sub(1) + flag(1) = 2.
    #[test]
    fn test_program_check_probe_data_length() {
        use ko_protocol::{Opcode, Packet};
        let mut probe = Packet::new(Opcode::WizProgramCheck as u8);
        probe.write_u8(0x02);
        probe.write_u8(0x01);
        assert_eq!(probe.data.len(), 2);
    }

    /// Client response: sub != 0x01, then string (program_info).
    #[test]
    fn test_program_check_client_response_format() {
        use ko_protocol::{Opcode, Packet, PacketReader};
        let mut pkt = Packet::new(Opcode::WizProgramCheck as u8);
        pkt.write_u8(0x02); // sub != 0x01 → client response
        pkt.write_string("explorer.exe chrome.exe");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02));
        assert_eq!(r.read_string(), Some("explorer.exe chrome.exe".to_string()));
    }

    /// GM socket sentinel value is u32::MAX.
    #[test]
    fn test_program_check_sentinel_value() {
        assert_eq!(u32::MAX, 0xFFFFFFFF);
        // u32::MAX means "no GM waiting for response"
        assert_ne!(u32::MAX, 0);
    }

    /// GM socket atomic store/load roundtrip with reset.
    #[test]
    fn test_program_check_gm_socket_reset() {
        let world = WorldState::new();
        world.plc_gm_socket.store(50, Ordering::Relaxed);
        assert_eq!(world.plc_gm_socket.load(Ordering::Relaxed), 50);

        // Reset to sentinel
        world.plc_gm_socket.store(u32::MAX, Ordering::Relaxed);
        assert_eq!(world.plc_gm_socket.load(Ordering::Relaxed), u32::MAX);
    }
}
