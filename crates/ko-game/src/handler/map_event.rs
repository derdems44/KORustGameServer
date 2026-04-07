//! WIZ_MAP_EVENT (0x53) handler — zone war event status query.
//! When a player enters certain battle zones during wartime, the client
//! requests the current event state (monument points, kill counts).
//! ## Request (C->S)
//! | Offset | Type | Description |
//! |--------|------|-------------|
//! | 0      | u8   | Event type  |
//! ## Response (S->C)
//! | Offset | Type  | Description |
//! |--------|-------|-------------|
//! | 0      | u8    | Event type (echo) |
//! | 1      | i16le | Karus counter (monument points or dead count) |
//! | 3      | i16le | Elmorad counter (monument points or dead count) |
//! Counter fields are only included when war is active and the player
//! is in the appropriate battle zone.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};
use crate::world::types::{ZONE_BATTLE4, ZONE_BATTLE6};

/// Handle WIZ_MAP_EVENT from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let event_type = reader.read_u8().unwrap_or(0);

    let world = session.world().clone();
    let sid = session.session_id();

    let zone_id = world.get_position(sid).map(|p| p.zone_id).unwrap_or(0);

    let mut result = Packet::new(Opcode::WizMapEvent as u8);

    // Only send counters when war is active and player is in the right zone.
    //
    // C++ checks `g_pMain->isWarOpen()` which returns true when `m_byBattleOpen >= NATION_BATTLE`.
    // The battle system tracks monument points (ZONE_BATTLE4) and kill counts (ZONE_BATTLE6).
    let war_open = world.is_war_open();

    if war_open && zone_id == ZONE_BATTLE4 {
        // Monument capture zone — send monument points
        let (karus_monument, elmorad_monument) = world.get_battle_monument_points();
        result.write_u8(event_type);
        result.write_i16(karus_monument);
        result.write_i16(elmorad_monument);
    } else if war_open && zone_id == ZONE_BATTLE6 {
        // Kill count zone — send dead counts
        let (karus_dead, elmorad_dead) = world.get_battle_dead_counts();
        result.write_u8(event_type);
        result.write_i16(karus_dead);
        result.write_i16(elmorad_dead);
    }
    // If war is not open or wrong zone, send empty packet (matching C++ behavior)

    session.send_packet(&result).await?;

    debug!(
        "[{}] WIZ_MAP_EVENT: type={}, zone={}",
        session.addr(),
        event_type,
        zone_id,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::war::{battle_zone_open, BATTLEZONE_OPEN, NATION_BATTLE};
    use crate::world::types::ZONE_BATTLE_BASE;
    use crate::world::WorldState;

    #[test]
    fn test_zone_constants() {
        assert_eq!(ZONE_BATTLE_BASE, 60);
        assert_eq!(ZONE_BATTLE4, 64);
        assert_eq!(ZONE_BATTLE6, 66);
    }

    #[test]
    fn test_request_packet_format() {
        let mut pkt = Packet::new(Opcode::WizMapEvent as u8);
        pkt.write_u8(0); // event type

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_response_with_counters() {
        // Simulate a response packet with monument counters
        let mut pkt = Packet::new(Opcode::WizMapEvent as u8);
        pkt.write_u8(0); // event type
        pkt.write_i16(150); // karus monument
        pkt.write_i16(200); // elmorad monument

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_i16(), Some(150));
        assert_eq!(r.read_i16(), Some(200));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_empty_response_no_war() {
        // When war is not active, response is just the opcode header (no data)
        let pkt = Packet::new(Opcode::WizMapEvent as u8);
        assert_eq!(pkt.data.len(), 0);
    }

    #[test]
    fn test_response_with_kill_counts() {
        // Simulate ZONE_BATTLE6 response with kill counters
        let mut pkt = Packet::new(Opcode::WizMapEvent as u8);
        pkt.write_u8(1); // event type
        pkt.write_i16(42); // karus dead
        pkt.write_i16(37); // elmorad dead

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i16(), Some(42));
        assert_eq!(r.read_i16(), Some(37));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_battle_zone_ids() {
        // ZONE_BATTLE4 = monument capture zone (Nereid's Island)
        // ZONE_BATTLE6 = kill count zone (Oreads)
        assert_eq!(ZONE_BATTLE4, 64);
        assert_eq!(ZONE_BATTLE6, 66);
        // Verify they are contiguous from ZONE_BATTLE_BASE
        assert_eq!(ZONE_BATTLE4, ZONE_BATTLE_BASE + 4);
        assert_eq!(ZONE_BATTLE6, ZONE_BATTLE_BASE + 6);
    }

    #[test]
    fn test_world_monument_points_during_war_in_zone_battle4() {
        let world = WorldState::new();

        // Open war on zone 4 (ZONE_BATTLE4) with monument points
        world.update_battle_state(|state| {
            state.battle_time = 3600;
            battle_zone_open(state, BATTLEZONE_OPEN, 4, 1000);
            state.karus_monument_point = 150;
            state.elmorad_monument_point = 200;
        });

        assert!(world.is_war_open());
        let (k, e) = world.get_battle_monument_points();
        assert_eq!(k, 150);
        assert_eq!(e, 200);
    }

    #[test]
    fn test_world_dead_counts_during_war_in_zone_battle6() {
        let world = WorldState::new();

        // Open war on zone 6 (ZONE_BATTLE6) with kill counters
        world.update_battle_state(|state| {
            state.battle_time = 3600;
            battle_zone_open(state, BATTLEZONE_OPEN, 6, 1000);
            state.karus_dead = 42;
            state.elmorad_dead = 37;
        });

        assert!(world.is_war_open());
        let (k, e) = world.get_battle_dead_counts();
        assert_eq!(k, 42);
        assert_eq!(e, 37);
    }

    #[test]
    fn test_world_counters_zero_when_no_war() {
        let world = WorldState::new();

        // No war open — counters should be zero
        assert!(!world.is_war_open());
        let (km, em) = world.get_battle_monument_points();
        assert_eq!(km, 0);
        assert_eq!(em, 0);
        let (kd, ed) = world.get_battle_dead_counts();
        assert_eq!(kd, 0);
        assert_eq!(ed, 0);
    }

    #[test]
    fn test_world_is_war_open_uses_battle_state() {
        let world = WorldState::new();
        assert!(!world.is_war_open());

        // Set battle_open directly to NATION_BATTLE
        world.update_battle_state(|state| {
            state.battle_open = NATION_BATTLE;
        });
        assert!(world.is_war_open());
    }
}
