//! WIZ_SEL_CHAR (0x04) handler — character selection.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/CharacterSelectionHandler.cpp:644-869`
//!
//! ## Request (C->S)
//!
//! | Offset | Type   | Description |
//! |--------|--------|-------------|
//! | 0      | string | Account ID |
//! | N      | string | Character ID |
//! | M      | u8     | Init flag |
//!
//! ## Response (S->C) — opcode 0x04
//!
//! C++ format: `result << bResult << uint16(GetZoneID()) << GetSPosX() << GetSPosZ() << GetSPosY() << GetNation() << int16(-1);`
//! where `GetSPosX()` = `uint16(GetX() * 10)` (Unit.h:255-257)
//!
//! | Offset | Type   | Description |
//! |--------|--------|-------------|
//! | 0      | u8     | Result (0=fail, 1=ok) |
//! | 1      | u16le  | Zone ID |
//! | 3      | u16le  | Position X * 10 |
//! | 5      | u16le  | Position Z * 10 |
//! | 7      | u16le  | Position Y * 10 |
//! | 9      | u8     | Nation |
//! | 10     | i16le  | Unknown (-1) |

use ko_db::repositories::character::CharacterRepository;
use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};
use crate::world::{
    ZONE_BIFROST, ZONE_BORDER_DEFENSE_WAR, ZONE_CHAOS_DUNGEON, ZONE_DELOS, ZONE_ELMORAD,
    ZONE_JURAID_MOUNTAIN, ZONE_KARUS, ZONE_MORADON, ZONE_STONE1, ZONE_STONE2, ZONE_STONE3,
};

/// Moradon default spawn position (version 2369).
/// C++ Reference: CharacterSelectionHandler.cpp:732-735
const MORADON_DEFAULT_X: f32 = 81400.0 / 100.0; // 814.0
const MORADON_DEFAULT_Z: f32 = 43750.0 / 100.0; // 437.5

/// Handle WIZ_SEL_CHAR from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::LoggedIn && session.state() != SessionState::NationSelected
    {
        return Ok(());
    }

    let session_account = match session.account_id() {
        Some(id) => id.to_string(),
        None => return Ok(()),
    };

    let mut reader = PacketReader::new(&pkt.data);
    let account_id = reader.read_string().unwrap_or_default();
    let char_id = reader.read_string().unwrap_or_default();
    let _init_flag = reader.read_u8().unwrap_or(0);

    // v2600: sub=2 path sends empty account_id. Use session account.
    let account_id = if account_id.is_empty() {
        session_account.clone()
    } else {
        account_id
    };

    // Verify account matches session
    if account_id != session_account || char_id.is_empty() {
        let mut response = Packet::new(Opcode::WizSelChar as u8);
        response.write_u8(0); // fail
        session.send_packet(&response).await?;
        return Ok(());
    }

    let char_repo = CharacterRepository::new(session.pool());

    match char_repo.load(&char_id).await {
        Ok(Some(ch)) => {
            // C++ CharacterSelectionHandler.cpp:659-663 — check if character already in-game
            {
                let world = session.world();
                if let Some(existing_sid) = world.find_session_by_name(&char_id) {
                    let existing_account = world
                        .with_session(existing_sid, |h| h.account_id.clone())
                        .unwrap_or_default();
                    if existing_account == session_account {
                        // Same account, different socket → kick the old session
                        tracing::info!(
                            "[{}] Kicking duplicate session for character {} (same account)",
                            session.addr(),
                            char_id
                        );
                        let world_clone = world.clone();
                        world_clone.kick_session_for_duplicate(existing_sid).await;
                    } else {
                        // Different account → disconnect this connection
                        tracing::warn!(
                            "[{}] Character {} already in-game under different account, disconnecting",
                            session.addr(),
                            char_id
                        );
                        let mut response = Packet::new(Opcode::WizSelChar as u8);
                        response.write_u8(0);
                        session.send_packet(&response).await?;
                        return Ok(());
                    }
                }
            }

            session.set_state(SessionState::CharacterSelected);
            session.set_character_id(char_id.clone());

            // Convert DB integer positions to float (stored as pos * 100)
            let mut zone_id = ch.zone as u16;
            let mut pos_x = ch.px as f32 / 100.0;
            let mut pos_z = ch.pz as f32 / 100.0;
            let pos_y = ch.py as f32 / 100.0;
            let nation = ch.nation as u8;

            // ── Zone redirect checks ─────────────────────────────────────
            // C++ Reference: CharacterSelectionHandler.cpp:709-745
            // If the player is in a restricted zone, redirect to Moradon.
            let world = session.world();
            let redirect = should_redirect_to_moradon(world, zone_id, nation, ch.knights as u16);
            if redirect {
                tracing::info!(
                    "[{}] Redirecting {} from zone {} to Moradon (restricted zone)",
                    session.addr(),
                    char_id,
                    zone_id
                );
                zone_id = ZONE_MORADON;
                // Use nation-specific start_position for Moradon
                if let Some(sp) = world.get_start_position(ZONE_MORADON) {
                    let (base_x, base_z) = if nation == 1 {
                        (sp.karus_x as f32, sp.karus_z as f32)
                    } else {
                        (sp.elmorad_x as f32, sp.elmorad_z as f32)
                    };
                    if base_x != 0.0 || base_z != 0.0 {
                        pos_x = base_x;
                        pos_z = base_z;
                    } else {
                        pos_x = MORADON_DEFAULT_X;
                        pos_z = MORADON_DEFAULT_Z;
                    }
                } else {
                    pos_x = MORADON_DEFAULT_X;
                    pos_z = MORADON_DEFAULT_Z;
                }
            }

            // C++ format: result << bResult << uint16(zone) << GetSPosX() << GetSPosZ() << GetSPosY() << GetNation() << int16(-1)
            // GetSPosX() = uint16(GetX() * 10) — positions are u16 scaled by *10
            let mut response = Packet::new(Opcode::WizSelChar as u8);
            response.write_u8(1); // success
            response.write_u16(zone_id);
            response.write_u16((pos_x * 10.0) as u16);
            response.write_u16((pos_z * 10.0) as u16);
            response.write_u16((pos_y * 10.0) as u16);
            response.write_u8(nation);
            response.write_i16(-1); // unknown
            session.send_packet(&response).await?;

            // v2600 PCAP verified: original server sends HP_CHANGE + MP_CHANGE
            // immediately after selchar success (seq 9-10 in sniffer).
            // Format: [0x17][max_hp:i16][hp:i16][attacker:u32=0xFFFFFFFF]
            //         [0x18][mp:u16][max_mp:u16]
            {
                let hp = ch.hp;
                let mp = ch.mp;
                let hp_pkt = crate::systems::regen::build_hp_change_packet(hp, hp);
                session.send_packet(&hp_pkt).await?;

                let mut mp_pkt = Packet::new(Opcode::WizMspChange as u8);
                mp_pkt.write_u16(mp as u16);
                mp_pkt.write_u16(mp as u16);
                session.send_packet(&mp_pkt).await?;
            }

            // PCAP verified: original server sends WIZ_RENTAL + WIZ_SERVER_INDEX
            // BEFORE MyInfo (sniffer seq 14-15). These may initialize client inventory state.
            // WIZ_RENTAL: 73 02 03 E2 FF
            {
                let mut rental_pkt = Packet::new(Opcode::WizRental as u8);
                rental_pkt.write_u8(0x02);
                rental_pkt.write_u8(0x03);
                rental_pkt.write_u16(0xFFE2u16); // sniffer: E2 FF
                session.send_packet(&rental_pkt).await?;
            }
            // WIZ_SERVER_INDEX: 6B 01 00 01 00
            {
                let mut idx_pkt = Packet::new(Opcode::WizServerIndex as u8);
                idx_pkt.write_u16(1); // 01 00
                idx_pkt.write_u16(1); // 01 00
                session.send_packet(&idx_pkt).await?;
            }

            tracing::info!(
                "[{}] Character selected: {} (zone={}, pos={:.1},{:.1},{:.1})",
                session.addr(),
                char_id,
                zone_id,
                pos_x,
                pos_z,
                pos_y
            );
        }
        Ok(None) => {
            tracing::debug!("[{}] Character not found: {}", session.addr(), char_id);
            let mut fail = Packet::new(Opcode::WizSelChar as u8);
            fail.write_u8(0); // result = fail
            session.send_packet(&fail).await?;
        }
        Err(e) => {
            tracing::error!("[{}] DB error loading character: {}", session.addr(), e);
            let mut fail = Packet::new(Opcode::WizSelChar as u8);
            fail.write_u8(0); // result = fail
            session.send_packet(&fail).await?;
        }
    }

    Ok(())
}

/// Determine if a player should be redirected to Moradon on login.
///
/// C++ Reference: `CharacterSelectionHandler.cpp:709-722`
///
/// Returns `true` if the player's saved zone is restricted and they must
/// be teleported to Moradon instead.
pub(super) fn should_redirect_to_moradon(
    world: &crate::world::WorldState,
    zone_id: u16,
    nation: u8,
    knights_id: u16,
) -> bool {
    let battle_state = world.get_battle_state();

    // 1. Karus player in Elmorad zone when Elmorad not open
    // C++ Reference: CharacterSelectionHandler.cpp:710
    if zone_id == ZONE_ELMORAD && !battle_state.elmorad_open_flag && nation == 1 {
        return true;
    }

    // 2. Elmorad player in Karus zone when Karus not open
    // C++ Reference: CharacterSelectionHandler.cpp:711
    if zone_id == ZONE_KARUS && !battle_state.karus_open_flag && nation == 2 {
        return true;
    }

    // 3. War zone but war not open
    // C++ Reference: CharacterSelectionHandler.cpp:712
    if let Some(zone) = world.get_zone(zone_id) {
        if zone.is_war_zone() && !battle_state.is_war_open() {
            return true;
        }
        // 4. War zone, war open, but player's nation lost
        // C++ Reference: CharacterSelectionHandler.cpp:713
        if zone.is_war_zone()
            && battle_state.is_war_open()
            && battle_state.victory != 0
            && battle_state.victory != nation
        {
            return true;
        }
    }

    // 5. Stone zones — always redirect to Moradon
    // C++ Reference: CharacterSelectionHandler.cpp:717
    if zone_id == ZONE_STONE1 || zone_id == ZONE_STONE2 || zone_id == ZONE_STONE3 {
        return true;
    }

    // 6. Temple event zones — redirect (BDW, Chaos, Juraid)
    // C++ Reference: CharacterSelectionHandler.cpp:714 — isInTotalTempleEventZone()
    if zone_id == ZONE_BORDER_DEFENSE_WAR
        || zone_id == ZONE_CHAOS_DUNGEON
        || zone_id == ZONE_JURAID_MOUNTAIN
    {
        return true;
    }

    // 7. Delos (siege) + siege warfare open + no clan
    // C++ Reference: CharacterSelectionHandler.cpp:715
    if zone_id == ZONE_DELOS {
        let csw_open = world
            .csw_event()
            .try_read()
            .map(|csw| csw.is_active())
            .unwrap_or(false);
        if csw_open && knights_id == 0 {
            return true;
        }
    }

    // 8. Special event zones (Zindan) when event not opened
    // C++ Reference: CharacterSelectionHandler.cpp:716
    if crate::handler::attack::is_in_special_event_zone(zone_id) && !world.is_zindan_event_opened()
    {
        return true;
    }

    // 9. Cinderella zone — redirect
    // C++ Reference: CharacterSelectionHandler.cpp:721
    if world.is_cinderella_active() && world.cinderella_zone_id() == zone_id {
        return true;
    }

    // 10. Bifrost — redirect based on event state and player nation
    // C++ Reference: CharacterSelectionHandler.cpp:720 — BeefEventLogin() check
    if zone_id == ZONE_BIFROST && super::bifrost::should_redirect_from_bifrost(world, nation) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    /// Zone redirect: Stone zones always redirect to Moradon.
    #[test]
    fn test_stone_zones_redirect() {
        let world = crate::world::WorldState::new();
        assert!(should_redirect_to_moradon(&world, ZONE_STONE1, 1, 0));
        assert!(should_redirect_to_moradon(&world, ZONE_STONE2, 2, 0));
        assert!(should_redirect_to_moradon(&world, ZONE_STONE3, 1, 100));
    }

    /// Zone redirect: Temple event zones always redirect to Moradon.
    #[test]
    fn test_temple_event_zones_redirect() {
        let world = crate::world::WorldState::new();
        assert!(should_redirect_to_moradon(
            &world,
            ZONE_BORDER_DEFENSE_WAR,
            1,
            0
        ));
        assert!(should_redirect_to_moradon(&world, 85, 1, 0)); // Chaos Dungeon
        assert!(should_redirect_to_moradon(
            &world,
            ZONE_JURAID_MOUNTAIN,
            2,
            0
        ));
    }

    /// Zone redirect: Bifrost always redirects.
    #[test]
    fn test_bifrost_redirects() {
        let world = crate::world::WorldState::new();
        assert!(should_redirect_to_moradon(&world, ZONE_BIFROST, 1, 0));
    }

    /// Zone redirect: Normal Moradon does NOT redirect.
    #[test]
    fn test_moradon_no_redirect() {
        let world = crate::world::WorldState::new();
        assert!(!should_redirect_to_moradon(&world, ZONE_MORADON, 1, 0));
        assert!(!should_redirect_to_moradon(&world, ZONE_MORADON, 2, 100));
    }

    /// Zone redirect: Karus zone redirects Elmorad player when Karus not open.
    #[test]
    fn test_enemy_zone_redirect() {
        let world = crate::world::WorldState::new();
        // Default: Karus zone not open, Elmorad player (nation=2) → redirect
        assert!(should_redirect_to_moradon(&world, ZONE_KARUS, 2, 0));
        // Default: Elmorad zone not open, Karus player (nation=1) → redirect
        assert!(should_redirect_to_moradon(&world, ZONE_ELMORAD, 1, 0));
        // Same nation → no redirect (Karus in Karus, Elmorad in Elmorad)
        assert!(!should_redirect_to_moradon(&world, ZONE_KARUS, 1, 0));
        assert!(!should_redirect_to_moradon(&world, ZONE_ELMORAD, 2, 0));
    }

    /// SelChar response packet format matches C++ wire format (u16 positions).
    #[test]
    fn test_selchar_response_packet_format() {
        // C++: result << bResult << uint16(zone) << GetSPosX() << GetSPosZ() << GetSPosY() << GetNation() << int16(-1)
        // GetSPosX() = uint16(float * 10)
        let mut pkt = Packet::new(Opcode::WizSelChar as u8);
        pkt.write_u8(1); // success
        pkt.write_u16(21); // zone = Moradon
        pkt.write_u16(8140); // x = 814.0 * 10
        pkt.write_u16(4375); // z = 437.5 * 10
        pkt.write_u16(47); // y = 4.7 * 10
        pkt.write_u8(1); // nation = Karus
        pkt.write_i16(-1); // unknown

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u16(), Some(21));
        assert_eq!(r.read_u16(), Some(8140)); // x * 10
        assert_eq!(r.read_u16(), Some(4375)); // z * 10
        assert_eq!(r.read_u16(), Some(47)); // y * 10
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i16(), Some(-1));
        // Total: 1 + 2 + 2 + 2 + 2 + 1 + 2 = 12 bytes
        assert_eq!(pkt.data.len(), 12);
    }

    // ── Sprint 922: Additional coverage ─────────────────────────────

    #[test]
    fn test_selchar_fail_response_format() {
        let mut pkt = Packet::new(Opcode::WizSelChar as u8);
        pkt.write_u8(0); // fail

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0));
        // Fail response has only 1 byte
        assert_eq!(pkt.data.len(), 1);
    }

    #[test]
    fn test_selchar_response_data_length() {
        // Success: result(1) + zone(2) + x(2) + z(2) + y(2) + nation(1) + unknown(2) = 12
        let mut pkt = Packet::new(Opcode::WizSelChar as u8);
        pkt.write_u8(1);
        pkt.write_u16(21);
        pkt.write_u16(0); // x * 10
        pkt.write_u16(0); // z * 10
        pkt.write_u16(0); // y * 10
        pkt.write_u8(1);
        pkt.write_i16(-1);
        assert_eq!(pkt.data.len(), 12);
    }

    #[test]
    fn test_moradon_default_coords() {
        // C++ CharacterSelectionHandler.cpp:732-735
        assert!((MORADON_DEFAULT_X - 814.0).abs() < 0.1);
        assert!((MORADON_DEFAULT_Z - 437.5).abs() < 0.1);
    }

    #[test]
    fn test_position_scale_factor() {
        // DB stores pos * 100, then divides to float, then sends as (float * 10) u16
        // 81400 / 100.0 = 814.0, 814.0 * 10 = 8140
        let db_pos: i32 = 81400;
        let float_pos = db_pos as f32 / 100.0;
        let wire_pos = (float_pos * 10.0) as u16;
        assert_eq!(wire_pos, 8140);
    }

    #[test]
    fn test_delos_redirect_no_clan() {
        let world = crate::world::WorldState::new();
        // Delos with no clan + CSW not active → no redirect (CSW check)
        // When CSW is NOT active, even clanless players stay
        assert!(!should_redirect_to_moradon(&world, ZONE_DELOS, 1, 0));
    }

    #[test]
    fn test_same_nation_zone_no_redirect() {
        let world = crate::world::WorldState::new();
        // Karus player in Karus zone → no redirect
        assert!(!should_redirect_to_moradon(&world, ZONE_KARUS, 1, 0));
        // Elmorad player in Elmorad zone → no redirect
        assert!(!should_redirect_to_moradon(&world, ZONE_ELMORAD, 2, 0));
    }
}
