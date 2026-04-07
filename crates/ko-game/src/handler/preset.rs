//! WIZ_PRESET (0xB9) handler — stat/skill preset system.
//! **v2525 CONFLICT**: Client opcode 0xB9 = WIZ_PET_STAT (pet statistics panel),
//! NOT WIZ_PRESET. The v2525 client's handler at 0xB9 dispatches sub-types 1/2
//! with inner sub-opcodes for pet stat updates and mode changes. Sending preset
//! S2C packets on 0xB9 causes the client to misinterpret data as pet stat commands,
//! corrupting the pet UI panel.
//! We accept C2S but respond with a WIZ_CHAT notice instead of 0xB9 packets.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};
#[cfg(test)]
use crate::world::WorldState;

/// Preset result: success.
#[cfg(test)]
const PRESET_SUCCESS: u8 = 1;
/// Preset result: failed.
#[cfg(test)]
const PRESET_FAILED: u8 = 3;

/// Maximum skill points per tree (C++ CeilPoint = {83, 83, 83, 23}).
#[cfg(test)]
const SKILL_CEIL: [u8; 4] = [83, 83, 83, 23];

/// Build a stat preset response.
/// Wire: `[u8 1] [u8 result] [u8 str..cha] [u16 free_points]`
#[cfg(test)]
fn build_stat_response(result: u8, stats: [u8; 5], free_points: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizPreset as u8);
    pkt.write_u8(1); // type = stat
    pkt.write_u8(result);
    if result == PRESET_SUCCESS {
        for s in &stats {
            pkt.write_u8(*s);
        }
        pkt.write_u16(free_points);
    }
    pkt
}

/// Build a skill preset response.
/// Wire: `[u8 2] [u8 result] [u8 skill0..3] [u8 free_skill_pts]`
#[cfg(test)]
fn build_skill_response(result: u8, skills: [u8; 4], free_skill_pts: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizPreset as u8);
    pkt.write_u8(2); // type = skill
    pkt.write_u8(result);
    if result == PRESET_SUCCESS {
        for s in &skills {
            pkt.write_u8(*s);
        }
        pkt.write_u8(free_skill_pts);
    }
    pkt
}

/// Handle WIZ_PRESET from the client.
/// **v2525 CONFLICT**: Client opcode 0xB9 = WIZ_PET_STAT (pet statistics),
/// NOT WIZ_PRESET. The v2525 client will never send C2S 0xB9 as a preset request
/// (the pet stat panel uses a different sub-opcode structure). If a packet does
/// arrive, respond with WIZ_CHAT instead of broken 0xB9 S2C packets.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    // v2525: 0xB9 = WIZ_PET_STAT on client side. Preset UI is unavailable.
    // Send WIZ_CHAT fallback instead of broken 0xB9 S2C packets.
    let mut reader = PacketReader::new(&pkt.data);
    let preset_type = reader.read_u8().unwrap_or(0);
    debug!(
        "[{}] WIZ_PRESET type={} — blocked (v2525 0xB9=WizPetStat conflict)",
        session.addr(),
        preset_type,
    );

    let mut chat = Packet::new(Opcode::WizChat as u8);
    chat.write_u8(7); // PUBLIC_CHAT
    chat.write_u8(20); // system_msg type
    chat.write_string("Stat/Skill Preset is not available in this client version.");
    session.send_packet(&chat).await
}

/// Original stat preset implementation (v2525 conflict blocks this — preserved for future).
#[cfg(test)]
#[allow(dead_code)]
async fn handle_stat_preset(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
    world: &WorldState,
    sid: u16,
) -> anyhow::Result<()> {
    // Read requested stat values (u8 each: str, sta, dex, int, cha)
    let mut stats = [0u8; 5];
    for stat in &mut stats {
        *stat = reader.read_u8().unwrap_or(0);
    }

    let char_info = match world.get_character_info(sid) {
        Some(info) => info,
        None => return Ok(()),
    };

    let level = char_info.level as u16;

    // Total available free points: 10 + (level-1)*3, +2*(level-60) if level>60
    let mut total_free: u16 = 10 + (level.saturating_sub(1)) * 3;
    if level > 60 {
        total_free += 2 * (level - 60);
    }

    // Base stats for race/class (simplified: 50 per stat as minimum)
    let base_min: u8 = 50;

    // Validate each stat is at least the base minimum
    for s in &stats {
        if *s < base_min {
            let fail = build_stat_response(PRESET_FAILED, [0; 5], 0);
            session.send_packet(&fail).await?;
            return Ok(());
        }
    }

    // Sum of points used above base
    let total_used: u16 = stats
        .iter()
        .map(|s| (*s as u16).saturating_sub(base_min as u16))
        .sum();

    if total_used > total_free || level > 83 {
        let fail = build_stat_response(PRESET_FAILED, [0; 5], 0);
        session.send_packet(&fail).await?;
        return Ok(());
    }

    let remaining = total_free - total_used;

    // Update character stats in world state
    world.update_character_stats(sid, |ch| {
        ch.str = stats[0];
        ch.sta = stats[1];
        ch.dex = stats[2];
        ch.intel = stats[3];
        ch.cha = stats[4];
        ch.free_points = remaining;
    });

    let response = build_stat_response(PRESET_SUCCESS, stats, remaining);
    session.send_packet(&response).await?;

    debug!(
        "[{}] WIZ_PRESET stat: STR={} STA={} DEX={} INT={} CHA={} free={}",
        session.addr(),
        stats[0],
        stats[1],
        stats[2],
        stats[3],
        stats[4],
        remaining,
    );

    Ok(())
}

/// Original skill preset implementation (v2525 conflict blocks this — preserved for future).
#[cfg(test)]
#[allow(dead_code)]
async fn handle_skill_preset(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
    world: &WorldState,
    sid: u16,
) -> anyhow::Result<()> {
    // Read requested skill point distribution
    let mut skill_pts = [0u8; 4];
    for pt in &mut skill_pts {
        *pt = reader.read_u8().unwrap_or(0);
    }

    // Validate: each must be within ceiling
    for (i, pt) in skill_pts.iter().enumerate() {
        if *pt > SKILL_CEIL[i] {
            let fail = build_skill_response(PRESET_FAILED, [0; 4], 0);
            session.send_packet(&fail).await?;
            return Ok(());
        }
    }

    // Check that at least one value is non-zero
    if skill_pts.iter().all(|p| *p == 0) {
        let fail = build_skill_response(PRESET_FAILED, [0; 4], 0);
        session.send_packet(&fail).await?;
        return Ok(());
    }

    let char_info = match world.get_character_info(sid) {
        Some(info) => info,
        None => return Ok(()),
    };

    let level = char_info.level as u16;

    // Must be level 10+ to use skill presets
    if level < 10 {
        let fail = build_skill_response(PRESET_FAILED, [0; 4], 0);
        session.send_packet(&fail).await?;
        return Ok(());
    }

    // Max skill points = (level - 9) * 2
    let max_points: u16 = (level - 9) * 2;
    let used: u16 = skill_pts.iter().map(|p| *p as u16).sum();

    if used > max_points {
        let fail = build_skill_response(PRESET_FAILED, [0; 4], 0);
        session.send_packet(&fail).await?;
        return Ok(());
    }

    let free = (max_points - used) as u8;

    // Update character skill points in world state
    world.update_character_stats(sid, |ch| {
        ch.skill_points[0] = free;
        ch.skill_points[5] = skill_pts[0];
        ch.skill_points[6] = skill_pts[1];
        ch.skill_points[7] = skill_pts[2];
        ch.skill_points[8] = skill_pts[3];
    });

    let response = build_skill_response(PRESET_SUCCESS, skill_pts, free);
    session.send_packet(&response).await?;

    debug!(
        "[{}] WIZ_PRESET skill: [{}, {}, {}, {}] free={}",
        session.addr(),
        skill_pts[0],
        skill_pts[1],
        skill_pts[2],
        skill_pts[3],
        free,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, Packet, PacketReader};

    #[test]
    fn test_preset_opcode_value() {
        assert_eq!(Opcode::WizPreset as u8, 0xB9);
        assert_eq!(Opcode::from_byte(0xB9), Some(Opcode::WizPreset));
    }

    #[test]
    fn test_stat_preset_request_format() {
        // Client -> Server: [u8 1] [u8 str] [u8 sta] [u8 dex] [u8 int] [u8 cha]
        let mut pkt = Packet::new(Opcode::WizPreset as u8);
        pkt.write_u8(1); // type = stat
        pkt.write_u8(80); // str
        pkt.write_u8(75); // sta
        pkt.write_u8(90); // dex
        pkt.write_u8(60); // int
        pkt.write_u8(55); // cha

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(80));
        assert_eq!(r.read_u8(), Some(75));
        assert_eq!(r.read_u8(), Some(90));
        assert_eq!(r.read_u8(), Some(60));
        assert_eq!(r.read_u8(), Some(55));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_skill_preset_request_format() {
        // Client -> Server: [u8 2] [u8 s0] [u8 s1] [u8 s2] [u8 s3]
        let mut pkt = Packet::new(Opcode::WizPreset as u8);
        pkt.write_u8(2); // type = skill
        pkt.write_u8(40); // skill tree 0
        pkt.write_u8(50); // skill tree 1
        pkt.write_u8(30); // skill tree 2
        pkt.write_u8(10); // skill tree 3

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(40));
        assert_eq!(r.read_u8(), Some(50));
        assert_eq!(r.read_u8(), Some(30));
        assert_eq!(r.read_u8(), Some(10));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_stat_response_success() {
        let pkt = build_stat_response(PRESET_SUCCESS, [80, 75, 90, 60, 55], 10);
        assert_eq!(pkt.opcode, 0xB9);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type = stat
        assert_eq!(r.read_u8(), Some(1)); // result = success
        assert_eq!(r.read_u8(), Some(80)); // str
        assert_eq!(r.read_u8(), Some(75)); // sta
        assert_eq!(r.read_u8(), Some(90)); // dex
        assert_eq!(r.read_u8(), Some(60)); // int
        assert_eq!(r.read_u8(), Some(55)); // cha
        assert_eq!(r.read_u16(), Some(10)); // free_points
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_stat_response_failed() {
        let pkt = build_stat_response(PRESET_FAILED, [0; 5], 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type = stat
        assert_eq!(r.read_u8(), Some(3)); // result = failed
        assert_eq!(r.remaining(), 0); // no stats on failure
    }

    #[test]
    fn test_skill_response_success() {
        let pkt = build_skill_response(PRESET_SUCCESS, [40, 50, 30, 10], 5);
        assert_eq!(pkt.opcode, 0xB9);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type = skill
        assert_eq!(r.read_u8(), Some(1)); // result = success
        assert_eq!(r.read_u8(), Some(40));
        assert_eq!(r.read_u8(), Some(50));
        assert_eq!(r.read_u8(), Some(30));
        assert_eq!(r.read_u8(), Some(10));
        assert_eq!(r.read_u8(), Some(5)); // free_skill_pts
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_skill_response_failed() {
        let pkt = build_skill_response(PRESET_FAILED, [0; 4], 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type = skill
        assert_eq!(r.read_u8(), Some(3)); // result = failed
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_skill_ceil_constants() {
        assert_eq!(SKILL_CEIL, [83, 83, 83, 23]);
    }

    // ── Sprint 927: Additional coverage ──────────────────────────────

    /// Stat success response: type(1) + result(1) + 5 stats(5) + free_points(2) = 9.
    #[test]
    fn test_stat_response_success_data_length() {
        let pkt = build_stat_response(PRESET_SUCCESS, [60, 60, 60, 60, 60], 0);
        assert_eq!(pkt.data.len(), 9);
    }

    /// Stat fail response: type(1) + result(1) = 2 (no stats appended).
    #[test]
    fn test_stat_response_fail_data_length() {
        let pkt = build_stat_response(PRESET_FAILED, [0; 5], 0);
        assert_eq!(pkt.data.len(), 2);
    }

    /// Skill success response: type(1) + result(1) + 4 skills(4) + free(1) = 7.
    #[test]
    fn test_skill_response_success_data_length() {
        let pkt = build_skill_response(PRESET_SUCCESS, [20, 30, 20, 10], 2);
        assert_eq!(pkt.data.len(), 7);
    }

    /// Stat free points formula: 10 + (level-1)*3, +2*(level-60) if level>60.
    #[test]
    fn test_stat_free_points_formula() {
        // Level 1: 10 + 0 = 10
        let lvl1: u16 = 10 + (1u16.saturating_sub(1)) * 3;
        assert_eq!(lvl1, 10);

        // Level 60: 10 + 59*3 = 187
        let lvl60: u16 = 10 + (60u16 - 1) * 3;
        assert_eq!(lvl60, 187);

        // Level 83: 10 + 82*3 + 2*23 = 302
        let mut lvl83: u16 = 10 + (83u16 - 1) * 3;
        lvl83 += 2 * (83 - 60);
        assert_eq!(lvl83, 302);
    }

    /// Skill max points formula: (level - 9) * 2, requires level >= 10.
    #[test]
    fn test_skill_max_points_formula() {
        // Level 10: (10-9)*2 = 2
        assert_eq!((10u16 - 9) * 2, 2);
        // Level 60: (60-9)*2 = 102
        assert_eq!((60u16 - 9) * 2, 102);
        // Level 83: (83-9)*2 = 148
        assert_eq!((83u16 - 9) * 2, 148);
        // Skill ceil sum: 83+83+83+23 = 272 (max possible allocation)
        let ceil_sum: u16 = SKILL_CEIL.iter().map(|v| *v as u16).sum();
        assert_eq!(ceil_sum, 272);
    }
}
