//! WIZ_ROTATE (0x09) handler — character rotation.
//! ## Request (C->S)
//! | Offset | Type   | Description |
//! |--------|--------|-------------|
//! | 0      | i16le  | Direction angle |
//! ## Broadcast to nearby players
//! `[u32 socket_id] [i16 direction]`

use ko_protocol::{Opcode, Packet, PacketReader};
use std::sync::Arc;

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_ROTATE from the client.
pub fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();

    if world.is_player_dead(sid) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let direction = reader.read_u16().unwrap_or(0) as i16;

    // Persist direction so write_user_info can include it in broadcasts
    world.update_session(sid, |h| {
        h.direction = direction;
    });

    // Build broadcast packet: [socket_id][direction]
    let mut bcast = Packet::new(Opcode::WizRotate as u8);
    bcast.write_u32(sid as u32);
    bcast.write_i16(direction);

    // GM invisible broadcast suppression
    let is_gm = world
        .get_character_info(sid)
        .map(|ch| ch.authority == 0)
        .unwrap_or(false);
    if is_gm && world.get_abnormal_type(sid) == 0 {
        return Ok(());
    }

    if let Some((pos, event_room)) = world.with_session(sid, |h| (h.position, h.event_room)) {
        world.broadcast_to_3x3(
            pos.zone_id,
            pos.region_x,
            pos.region_z,
            Arc::new(bcast),
            Some(sid),
            event_room,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_rotate_c2s_packet_format() {
        // C2S: [i16 direction]
        let mut pkt = Packet::new(Opcode::WizRotate as u8);
        pkt.write_i16(180);

        let mut reader = PacketReader::new(&pkt.data);
        let dir = reader.read_u16().unwrap_or(0) as i16;
        assert_eq!(dir, 180);
    }

    #[test]
    fn test_rotate_broadcast_packet_format() {
        // Broadcast: [u32 socket_id][i16 direction]
        let sid: u16 = 42;
        let direction: i16 = -90;

        let mut bcast = Packet::new(Opcode::WizRotate as u8);
        bcast.write_u32(sid as u32);
        bcast.write_i16(direction);

        assert_eq!(bcast.opcode, Opcode::WizRotate as u8);
        let mut reader = PacketReader::new(&bcast.data);
        assert_eq!(reader.read_u32(), Some(42), "socket_id");
        assert_eq!(reader.read_u16().map(|v| v as i16), Some(-90), "direction");
    }

    #[test]
    fn test_rotate_direction_wrapping() {
        // Direction is i16, wraps at -32768 to 32767
        for dir in [0i16, 90, 180, -180, -1, i16::MAX, i16::MIN] {
            let mut pkt = Packet::new(Opcode::WizRotate as u8);
            pkt.write_i16(dir);
            let mut reader = PacketReader::new(&pkt.data);
            let read_dir = reader.read_u16().unwrap_or(0) as i16;
            assert_eq!(read_dir, dir, "direction {dir} should roundtrip");
        }
    }

    // ── Sprint 929: Additional coverage ──────────────────────────────

    /// C2S data length: direction(2) = 2 bytes.
    #[test]
    fn test_rotate_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizRotate as u8);
        pkt.write_i16(180);
        assert_eq!(pkt.data.len(), 2);
    }

    /// Broadcast data length: socket_id(4) + direction(2) = 6 bytes.
    #[test]
    fn test_rotate_broadcast_data_length() {
        let mut pkt = Packet::new(Opcode::WizRotate as u8);
        pkt.write_u32(42);
        pkt.write_i16(90);
        assert_eq!(pkt.data.len(), 6);
    }

    /// Opcode value is 0x09.
    #[test]
    fn test_rotate_opcode_value() {
        assert_eq!(Opcode::WizRotate as u8, 0x09);
    }

    /// Broadcast includes socket_id as u32 (not u16).
    #[test]
    fn test_rotate_broadcast_sid_u32() {
        let sid: u16 = 500;
        let mut pkt = Packet::new(Opcode::WizRotate as u8);
        pkt.write_u32(sid as u32);
        pkt.write_i16(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(500));
        assert_eq!(r.read_u16().map(|v| v as i16), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    /// Direction 0 is valid (facing north/default).
    #[test]
    fn test_rotate_zero_direction() {
        let mut pkt = Packet::new(Opcode::WizRotate as u8);
        pkt.write_i16(0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u16().map(|v| v as i16), Some(0));
    }

    // ── Sprint 932: Additional coverage ──────────────────────────────

    /// Opcode from_byte roundtrip for 0x09.
    #[test]
    fn test_rotate_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x09), Some(Opcode::WizRotate));
    }

    /// Negative direction values roundtrip correctly.
    #[test]
    fn test_rotate_negative_directions() {
        for dir in [-1i16, -45, -90, -180, -270] {
            let mut pkt = Packet::new(Opcode::WizRotate as u8);
            pkt.write_i16(dir);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u16().map(|v| v as i16), Some(dir));
        }
    }
}
