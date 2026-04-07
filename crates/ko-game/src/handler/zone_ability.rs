//! WIZ_ZONEABILITY (0x5E) sub-opcode 1 — zone ability packet builder.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/User.cpp:1839-1918`
//!     — `CUser::SetZoneAbilityChange(uint16 sNewZone)`
//!
//! Tells the client the zone's rules: PvP type, cross-nation trade/talk, tariff rate.
//! Called after game start (UserInfoSystem.cpp:246) and after zone change
//! (ZoneChangeWarpHandler.cpp:287).
//!
//! ## Packet Format (sub-opcode 1)
//!
//! ```text
//! WIZ_ZONEABILITY (0x5E)
//!   u8(1)              — sub-opcode (zone ability info)
//!   u8(can_trade)      — canTradeWithOtherNation (bool as u8)
//!   u8(zone_type)      — ZoneAbilityType enum value
//!   u8(can_talk)       — canTalkToOtherNation (bool as u8)
//!   u16(tariff)        — tariff rate
//! ```
//!
//! ## Tariff Calculation
//!
//! C++ Reference: `User.cpp:1847-1910` — switch(sNewZone)
//!
//! - Territory zones (Karus, Elmorad, battles, etc.): 10 + king_system.territory_tariff
//! - Moradon zones (21-25, 48/Arena): siege_war.moradon_tariff
//! - Delos zones (30, 32, 33, 35): siege_war.delos_tariff
//! - Default: 0

use ko_protocol::{Opcode, Packet};

use crate::session::ClientSession;
use crate::world::{
    WorldState, ZONE_ARDREAM, ZONE_ARENA, ZONE_BATTLE, ZONE_BATTLE2, ZONE_BATTLE3, ZONE_BATTLE4,
    ZONE_BATTLE5, ZONE_BATTLE6, ZONE_BIFROST, ZONE_BORDER_DEFENSE_WAR, ZONE_CHAOS_DUNGEON,
    ZONE_CLAN_WAR_ARDREAM, ZONE_CLAN_WAR_RONARK, ZONE_DELOS, ZONE_DELOS_CASTELLAN,
    ZONE_DESPERATION_ABYSS, ZONE_ELMORAD, ZONE_ELMORAD2, ZONE_ELMORAD3, ZONE_ELMORAD_ESLANT,
    ZONE_ELMORAD_ESLANT2, ZONE_ELMORAD_ESLANT3, ZONE_HELL_ABYSS, ZONE_JURAID_MOUNTAIN, ZONE_KARUS,
    ZONE_KARUS2, ZONE_KARUS3, ZONE_KARUS_ESLANT, ZONE_KARUS_ESLANT2, ZONE_KARUS_ESLANT3,
    ZONE_KNIGHT_ROYALE, ZONE_KROWAZ_DOMINION, ZONE_MORADON, ZONE_MORADON2, ZONE_MORADON3,
    ZONE_MORADON4, ZONE_MORADON5, ZONE_PARTY_VS_1, ZONE_PARTY_VS_2, ZONE_PARTY_VS_3,
    ZONE_PARTY_VS_4, ZONE_RONARK_LAND, ZONE_RONARK_LAND_BASE, ZONE_SNOW_BATTLE, ZONE_STONE1,
    ZONE_STONE2, ZONE_STONE3, ZONE_UNDER_CASTLE,
};

/// Send WIZ_ZONEABILITY sub-opcode 1 to the client.
///
/// Informs the client about the zone's PvP rules, cross-nation trade/talk, and tariff rate.
///
/// C++ Reference: `User.cpp:1839-1918` — `CUser::SetZoneAbilityChange(uint16 sNewZone)`
pub async fn send_zone_ability(session: &mut ClientSession, zone_id: u16) -> anyhow::Result<()> {
    let world = session.world().clone();

    let zone = match world.get_zone(zone_id) {
        Some(z) => z,
        None => return Ok(()),
    };

    // Get the player's nation for king system tariff lookup
    let nation = world
        .get_character_info(session.session_id())
        .map(|ch| ch.nation)
        .unwrap_or(0);

    let tariff = compute_tariff(&world, zone_id, nation);

    // C++ Reference: User.cpp:1912-1916
    // Packet result(WIZ_ZONEABILITY, uint8(1));
    // result << pMap->canTradeWithOtherNation()
    //        << pMap->GetZoneType()
    //        << pMap->canTalkToOtherNation()
    //        << uint16(pMap->GetTariff());
    let mut pkt = Packet::new(Opcode::WizZoneability as u8);
    pkt.write_u8(1); // sub-opcode: zone ability info
    pkt.write_u8(zone.can_trade_other_nation() as u8);
    pkt.write_u8(zone.zone_type() as u8);
    pkt.write_u8(zone.can_talk_other_nation() as u8);
    pkt.write_u16(tariff);

    session.send_packet(&pkt).await?;

    tracing::debug!(
        "[{}] Sent zone ability: zone={}, type={:?}, trade={}, talk={}, tariff={}",
        session.addr(),
        zone_id,
        zone.zone_type(),
        zone.can_trade_other_nation(),
        zone.can_talk_other_nation(),
        tariff,
    );

    Ok(())
}

/// Send WIZ_ZONEABILITY sub-opcode 2 (SetZoneFlag) to the client.
///
/// v2600 sniff verified: sent after zone transitions to update the client's
/// dynamic zone war/flag status. Format: `[u8(2)] [u16 zone_flag]`.
///
/// The zone_flag value encodes the current war/event status of the zone.
pub async fn send_zone_flag(session: &mut ClientSession, zone_id: u16) -> anyhow::Result<()> {
    let world = session.world().clone();

    let zone_flag = world
        .get_zone(zone_id)
        .map(|z| z.zone_info.as_ref().map(|zi| zi.status as u16).unwrap_or(2))
        .unwrap_or(2); // default flag=2 (sniff verified)

    let mut pkt = Packet::new(Opcode::WizZoneability as u8);
    pkt.write_u8(2); // sub-opcode: set zone flag
    pkt.write_u16(zone_flag);
    session.send_packet(&pkt).await
}

/// Build a WIZ_ZONEABILITY sub-opcode 1 packet without sending it.
///
/// Useful for testing the packet format.
pub fn build_zone_ability_packet(
    can_trade: bool,
    zone_type: u8,
    can_talk: bool,
    tariff: u16,
) -> Packet {
    let mut pkt = Packet::new(Opcode::WizZoneability as u8);
    pkt.write_u8(1);
    pkt.write_u8(can_trade as u8);
    pkt.write_u8(zone_type);
    pkt.write_u8(can_talk as u8);
    pkt.write_u16(tariff);
    pkt
}

/// Compute the tariff rate for a given zone.
///
/// C++ Reference: `User.cpp:1847-1910` — switch(sNewZone) tariff assignment
///
/// Three groups:
/// 1. Territory zones: `10 + king_system.territory_tariff`
/// 2. Moradon zones: `siege_war.moradon_tariff`
/// 3. Delos zones: `siege_war.delos_tariff`
/// 4. Default: `0` (unhandled zone)
fn compute_tariff(world: &WorldState, zone_id: u16, nation: u8) -> u16 {
    match zone_id {
        // Territory zones — king tariff
        // C++ Reference: User.cpp:1849-1891
        ZONE_KARUS
        | ZONE_KARUS2
        | ZONE_KARUS3
        | ZONE_ELMORAD
        | ZONE_ELMORAD2
        | ZONE_ELMORAD3
        | ZONE_KARUS_ESLANT
        | ZONE_KARUS_ESLANT2
        | ZONE_KARUS_ESLANT3
        | ZONE_ELMORAD_ESLANT
        | ZONE_ELMORAD_ESLANT2
        | ZONE_ELMORAD_ESLANT3
        | ZONE_BIFROST
        | ZONE_BATTLE
        | ZONE_BATTLE2
        | ZONE_BATTLE3
        | ZONE_BATTLE4
        | ZONE_BATTLE5
        | ZONE_BATTLE6
        | ZONE_SNOW_BATTLE
        | ZONE_RONARK_LAND
        | ZONE_ARDREAM
        | ZONE_RONARK_LAND_BASE
        | ZONE_KROWAZ_DOMINION
        | ZONE_STONE1
        | ZONE_STONE2
        | ZONE_STONE3
        | ZONE_BORDER_DEFENSE_WAR
        | ZONE_UNDER_CASTLE
        | ZONE_JURAID_MOUNTAIN
        | ZONE_PARTY_VS_1
        | ZONE_PARTY_VS_2
        | ZONE_PARTY_VS_3
        | ZONE_PARTY_VS_4
        | ZONE_CLAN_WAR_ARDREAM
        | ZONE_CLAN_WAR_RONARK
        | ZONE_KNIGHT_ROYALE
        | ZONE_CHAOS_DUNGEON => {
            // C++: if (pKingSystem != nullptr)
            //          pMap->SetTariff(10 + pKingSystem->m_nTerritoryTariff);
            //      else pMap->SetTariff(10);
            let king_tariff = world
                .get_king_system(nation)
                .map(|ks| ks.territory_tariff as u16)
                .unwrap_or(0);
            10 + king_tariff
        }

        // Moradon zones — siege moradon tariff
        // C++ Reference: User.cpp:1892-1898
        ZONE_MORADON | ZONE_MORADON2 | ZONE_MORADON3 | ZONE_MORADON4 | ZONE_MORADON5
        | ZONE_ARENA => {
            // C++: pMap->SetTariff((uint8)g_pMain->pSiegeWar.sMoradonTariff);
            // C++ truncates u16 → u8 via (uint8) cast, m_byTariff is uint8 (Map.h:126)
            let sw = world.siege_war().try_read();
            match sw {
                Ok(sw) => (sw.moradon_tariff as u8) as u16,
                Err(_) => 10, // fallback default
            }
        }

        // Delos zones — siege delos tariff
        // C++ Reference: User.cpp:1900-1905
        ZONE_DELOS | ZONE_DESPERATION_ABYSS | ZONE_HELL_ABYSS | ZONE_DELOS_CASTELLAN => {
            // C++: pMap->SetTariff((uint8)g_pMain->pSiegeWar.sDellosTariff);
            // C++ truncates u16 → u8 via (uint8) cast, m_byTariff is uint8 (Map.h:126)
            let sw = world.siege_war().try_read();
            match sw {
                Ok(sw) => (sw.delos_tariff as u8) as u16,
                Err(_) => 10, // fallback default
            }
        }

        // Unhandled zone — C++ logs a TRACE and sets no tariff
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone::ZoneAbilityType;
    use ko_protocol::PacketReader;

    #[test]
    fn test_zone_ability_packet_format_neutral() {
        let pkt = build_zone_ability_packet(true, ZoneAbilityType::Neutral as u8, true, 10);
        assert_eq!(pkt.opcode, Opcode::WizZoneability as u8);
        assert_eq!(pkt.data.len(), 6);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub-opcode
        assert_eq!(r.read_u8(), Some(1)); // can_trade = true
        assert_eq!(r.read_u8(), Some(0)); // zone_type = Neutral(0)
        assert_eq!(r.read_u8(), Some(1)); // can_talk = true
        assert_eq!(r.read_u16(), Some(10)); // tariff
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_zone_ability_packet_format_pvp() {
        let pkt = build_zone_ability_packet(false, ZoneAbilityType::PvP as u8, false, 15);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub-opcode
        assert_eq!(r.read_u8(), Some(0)); // can_trade = false
        assert_eq!(r.read_u8(), Some(1)); // zone_type = PvP(1)
        assert_eq!(r.read_u8(), Some(0)); // can_talk = false
        assert_eq!(r.read_u16(), Some(15)); // tariff
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_zone_ability_packet_format_siege() {
        let pkt = build_zone_ability_packet(false, ZoneAbilityType::SiegeDisabled as u8, false, 20);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub-opcode
        assert_eq!(r.read_u8(), Some(0)); // can_trade = false
        assert_eq!(r.read_u8(), Some(6)); // zone_type = SiegeDisabled(6)
        assert_eq!(r.read_u8(), Some(0)); // can_talk = false
        assert_eq!(r.read_u16(), Some(20)); // tariff
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_zone_ability_packet_format_caitharos() {
        let pkt = build_zone_ability_packet(true, ZoneAbilityType::CaitharosArena as u8, true, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // sub-opcode
        assert_eq!(r.read_u8(), Some(1)); // can_trade = true
        assert_eq!(r.read_u8(), Some(7)); // zone_type = CaitharosArena(7)
        assert_eq!(r.read_u8(), Some(1)); // can_talk = true
        assert_eq!(r.read_u16(), Some(0)); // tariff
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_zone_ability_packet_all_zone_types() {
        let types = [
            (ZoneAbilityType::Neutral, 0u8),
            (ZoneAbilityType::PvP, 1),
            (ZoneAbilityType::Spectator, 2),
            (ZoneAbilityType::Siege1, 3),
            (ZoneAbilityType::Siege2, 4),
            (ZoneAbilityType::Siege3, 5),
            (ZoneAbilityType::SiegeDisabled, 6),
            (ZoneAbilityType::CaitharosArena, 7),
            (ZoneAbilityType::PvpNeutralNpcs, 8),
            (ZoneAbilityType::PvpStoneNpcs, 9),
        ];

        for (zt, expected_val) in &types {
            let pkt = build_zone_ability_packet(false, *zt as u8, false, 0);
            let mut r = PacketReader::new(&pkt.data);
            r.read_u8(); // sub-opcode
            r.read_u8(); // can_trade
            assert_eq!(r.read_u8(), Some(*expected_val), "ZoneAbilityType {:?}", zt);
        }
    }

    #[test]
    fn test_zone_ability_packet_opcode() {
        let pkt = build_zone_ability_packet(false, 0, false, 0);
        assert_eq!(pkt.opcode, 0x5E);
    }

    #[test]
    fn test_zone_ability_packet_byte_length() {
        // sub-opcode(1) + can_trade(1) + zone_type(1) + can_talk(1) + tariff(2) = 6
        let pkt = build_zone_ability_packet(false, 0, false, 0);
        assert_eq!(pkt.data.len(), 6);
    }

    #[test]
    fn test_zone_ability_tariff_max_u16() {
        let pkt = build_zone_ability_packet(true, 1, true, u16::MAX);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub-opcode
        r.read_u8(); // can_trade
        r.read_u8(); // zone_type
        r.read_u8(); // can_talk
        assert_eq!(r.read_u16(), Some(u16::MAX));
    }

    #[test]
    fn test_zone_ability_packet_roundtrip() {
        // Build a packet, read it back, verify every byte
        let can_trade = true;
        let zone_type = ZoneAbilityType::PvP as u8;
        let can_talk = false;
        let tariff: u16 = 12;

        let pkt = build_zone_ability_packet(can_trade, zone_type, can_talk, tariff);

        // Verify raw bytes
        assert_eq!(pkt.data[0], 1); // sub-opcode
        assert_eq!(pkt.data[1], 1); // can_trade = true
        assert_eq!(pkt.data[2], 1); // PvP
        assert_eq!(pkt.data[3], 0); // can_talk = false
                                    // tariff is little-endian u16
        assert_eq!(pkt.data[4], 12); // low byte
        assert_eq!(pkt.data[5], 0); // high byte
    }

    #[test]
    fn test_zone_ability_tariff_little_endian() {
        // Verify tariff 0x0102 is stored as [0x02, 0x01] (little-endian)
        let pkt = build_zone_ability_packet(false, 0, false, 0x0102);
        assert_eq!(pkt.data[4], 0x02); // low byte
        assert_eq!(pkt.data[5], 0x01); // high byte
    }

    #[test]
    fn test_compute_tariff_karus_zone_no_king() {
        let world = WorldState::new();
        // No king system loaded — should use fallback (10 + 0 = 10)
        let tariff = compute_tariff(&world, ZONE_KARUS, 1);
        assert_eq!(tariff, 10);
    }

    #[test]
    fn test_compute_tariff_elmorad_zone_no_king() {
        let world = WorldState::new();
        let tariff = compute_tariff(&world, ZONE_ELMORAD, 2);
        assert_eq!(tariff, 10);
    }

    #[test]
    fn test_compute_tariff_moradon_zone() {
        let world = WorldState::new();
        // SiegeWarfare default has moradon_tariff = 10
        let tariff = compute_tariff(&world, ZONE_MORADON, 1);
        assert_eq!(tariff, 10);
    }

    #[test]
    fn test_compute_tariff_delos_zone() {
        let world = WorldState::new();
        // SiegeWarfare default has delos_tariff = 10
        let tariff = compute_tariff(&world, ZONE_DELOS, 1);
        assert_eq!(tariff, 10);
    }

    #[test]
    fn test_compute_tariff_unhandled_zone() {
        let world = WorldState::new();
        // Zone 999 is not in any group — should return 0
        let tariff = compute_tariff(&world, 999, 1);
        assert_eq!(tariff, 0);
    }

    #[test]
    fn test_compute_tariff_battle_zones() {
        let world = WorldState::new();
        // All battle zones should use territory tariff
        for zone_id in [
            ZONE_BATTLE,
            ZONE_BATTLE2,
            ZONE_BATTLE3,
            ZONE_BATTLE4,
            ZONE_BATTLE5,
            ZONE_BATTLE6,
        ] {
            let tariff = compute_tariff(&world, zone_id, 1);
            assert_eq!(tariff, 10, "Zone {} should have tariff 10", zone_id);
        }
    }

    #[test]
    fn test_compute_tariff_eslant_zones() {
        let world = WorldState::new();
        for zone_id in [
            ZONE_KARUS_ESLANT,
            ZONE_KARUS_ESLANT2,
            ZONE_KARUS_ESLANT3,
            ZONE_ELMORAD_ESLANT,
            ZONE_ELMORAD_ESLANT2,
            ZONE_ELMORAD_ESLANT3,
        ] {
            let tariff = compute_tariff(&world, zone_id, 1);
            assert_eq!(tariff, 10, "Zone {} should have tariff 10", zone_id);
        }
    }

    #[test]
    fn test_compute_tariff_moradon_variants() {
        let world = WorldState::new();
        for zone_id in [
            ZONE_MORADON,
            ZONE_MORADON2,
            ZONE_MORADON3,
            ZONE_MORADON4,
            ZONE_MORADON5,
            ZONE_ARENA,
        ] {
            let tariff = compute_tariff(&world, zone_id, 1);
            assert_eq!(tariff, 10, "Zone {} should have moradon tariff 10", zone_id);
        }
    }

    #[test]
    fn test_compute_tariff_delos_variants() {
        let world = WorldState::new();
        for zone_id in [
            ZONE_DELOS,
            ZONE_DESPERATION_ABYSS,
            ZONE_HELL_ABYSS,
            ZONE_DELOS_CASTELLAN,
        ] {
            let tariff = compute_tariff(&world, zone_id, 1);
            assert_eq!(tariff, 10, "Zone {} should have delos tariff 10", zone_id);
        }
    }

    #[test]
    fn test_compute_tariff_pvp_zones() {
        let world = WorldState::new();
        for zone_id in [
            ZONE_RONARK_LAND,
            ZONE_ARDREAM,
            ZONE_RONARK_LAND_BASE,
            ZONE_KROWAZ_DOMINION,
            ZONE_SNOW_BATTLE,
        ] {
            let tariff = compute_tariff(&world, zone_id, 2);
            assert_eq!(tariff, 10, "Zone {} should have tariff 10", zone_id);
        }
    }

    #[test]
    fn test_compute_tariff_special_zones() {
        let world = WorldState::new();
        for zone_id in [
            ZONE_STONE1,
            ZONE_STONE2,
            ZONE_STONE3,
            ZONE_BORDER_DEFENSE_WAR,
            ZONE_UNDER_CASTLE,
            ZONE_JURAID_MOUNTAIN,
            ZONE_PARTY_VS_1,
            ZONE_PARTY_VS_2,
            ZONE_PARTY_VS_3,
            ZONE_PARTY_VS_4,
            ZONE_CLAN_WAR_ARDREAM,
            ZONE_CLAN_WAR_RONARK,
            ZONE_KNIGHT_ROYALE,
            ZONE_CHAOS_DUNGEON,
        ] {
            let tariff = compute_tariff(&world, zone_id, 1);
            assert_eq!(tariff, 10, "Zone {} should have tariff 10", zone_id);
        }
    }

    /// Verify that Moradon/Delos tariff is truncated from u16 to u8 before being
    /// sent in the packet, matching C++ behavior where `SetTariff((uint8)sMoradonTariff)`.
    ///
    /// C++ Reference: `User.cpp:1898` — `pMap->SetTariff((uint8)g_pMain->pSiegeWar.sMoradonTariff)`
    /// C++ Reference: `Map.h:126` — `uint8 m_byTariff`
    #[test]
    fn test_compute_tariff_moradon_u8_truncation() {
        let world = WorldState::new();
        // Set moradon_tariff to 300 (0x012C) — should truncate to 0x2C = 44
        {
            let mut sw = world.siege_war().try_write().unwrap();
            sw.moradon_tariff = 300;
        }
        let tariff = compute_tariff(&world, ZONE_MORADON, 1);
        assert_eq!(tariff, 44, "300 as u8 = 44 (0x2C)");
    }

    #[test]
    fn test_compute_tariff_delos_u8_truncation() {
        let world = WorldState::new();
        // Set delos_tariff to 512 (0x0200) — should truncate to 0x00 = 0
        {
            let mut sw = world.siege_war().try_write().unwrap();
            sw.delos_tariff = 512;
        }
        let tariff = compute_tariff(&world, ZONE_DELOS, 1);
        assert_eq!(tariff, 0, "512 as u8 = 0 (0x00)");
    }

    #[test]
    fn test_compute_tariff_moradon_small_value_no_change() {
        let world = WorldState::new();
        // Small value (fits in u8) should be unchanged
        {
            let mut sw = world.siege_war().try_write().unwrap();
            sw.moradon_tariff = 25;
        }
        let tariff = compute_tariff(&world, ZONE_MORADON, 1);
        assert_eq!(tariff, 25, "25 fits in u8, no truncation");
    }

    #[test]
    fn test_compute_tariff_delos_max_u8() {
        let world = WorldState::new();
        {
            let mut sw = world.siege_war().try_write().unwrap();
            sw.delos_tariff = 255;
        }
        let tariff = compute_tariff(&world, ZONE_DELOS, 1);
        assert_eq!(tariff, 255, "255 is max u8, no truncation");
    }

    #[test]
    fn test_compute_tariff_moradon_256_wraps_to_zero() {
        let world = WorldState::new();
        {
            let mut sw = world.siege_war().try_write().unwrap();
            sw.moradon_tariff = 256;
        }
        let tariff = compute_tariff(&world, ZONE_MORADON, 1);
        assert_eq!(tariff, 0, "256 as u8 = 0 (overflow wrap)");
    }

    // ── Sprint 309: ZONE_KNIGHT_ROYALE constant fix ──────────────────

    #[test]
    fn test_zone_knight_royale_matches_cpp_define() {
        // C++ Reference: Define.h:214 — `#define ZONE_KNIGHT_ROYALE 76`
        // Previously was incorrectly set to 88 in this file.
        assert_eq!(ZONE_KNIGHT_ROYALE, 76);
    }
}
