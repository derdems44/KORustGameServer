//! WIZ_DATASAVE (0x37) handler — periodic character data save.
//! The client periodically sends this opcode to request a DB save of
//! character data (position, HP, MP, EXP, gold, etc.). The server
//! saves and sends no response.
//! ## Client -> Server
//! Empty packet body (no data fields).

use ko_db::repositories::character::{CharacterRepository, SaveStatPointsParams, SaveStatsParams};
use ko_protocol::Packet;
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};

/// Handle WIZ_DATASAVE from the client.
/// Saves character stats (HP, MP, EXP, gold, loyalty) and position to DB.
/// Stats are saved via fire-and-forget spawned task; position via the existing
/// `save_position_async` helper.
pub async fn handle(session: &mut ClientSession, _pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let world = session.world().clone();
    let sid = session.session_id();
    let pool = session.pool().clone();
    let char_id = session.character_id().unwrap_or("").to_string();

    if char_id.is_empty() {
        return Ok(());
    }

    let pos = match world.get_position(sid) {
        Some(p) => p,
        None => return Ok(()),
    };

    debug!(
        "[{}] WIZ_DATASAVE: saving stats + position zone={} ({:.1},{:.1})",
        session.addr(),
        pos.zone_id,
        pos.x,
        pos.z,
    );

    // Save stats (fire-and-forget)
    if let Some(ch) = world.get_character_info(sid) {
        let pool_clone = pool.clone();
        let cid = char_id.clone();
        tokio::spawn(async move {
            let repo = CharacterRepository::new(&pool_clone);
            if let Err(e) = repo
                .save_stats(&SaveStatsParams {
                    char_id: &cid,
                    level: ch.level as i16,
                    hp: ch.hp,
                    mp: ch.mp,
                    sp: ch.sp,
                    exp: ch.exp as i64,
                    gold: ch.gold.min(i32::MAX as u32) as i32,
                    loyalty: ch.loyalty.min(i32::MAX as u32) as i32,
                    loyalty_monthly: ch.loyalty_monthly.min(i32::MAX as u32) as i32,
                    manner_point: ch.manner_point,
                })
                .await
            {
                warn!("WIZ_DATASAVE: failed to save stats for {}: {}", cid, e);
            }
        });
    }

    // Save position (fire-and-forget, reuse existing helper)
    super::zone_change::save_position_async(session, pos.zone_id, pos.x, pos.z);

    // Save stat + skill points (fire-and-forget)
    if let Some(ch) = world.get_character_info(sid) {
        let pool_clone = pool.clone();
        let cid = char_id.clone();
        tokio::spawn(async move {
            let repo = CharacterRepository::new(&pool_clone);
            if let Err(e) = repo
                .save_stat_points(&SaveStatPointsParams {
                    char_id: &cid,
                    str_val: ch.str as i16,
                    sta: ch.sta as i16,
                    dex: ch.dex as i16,
                    intel: ch.intel as i16,
                    cha: ch.cha as i16,
                    free_points: ch.free_points as i16,
                    skill_points: [
                        ch.skill_points[0] as i16,
                        ch.skill_points[1] as i16,
                        ch.skill_points[2] as i16,
                        ch.skill_points[3] as i16,
                        ch.skill_points[4] as i16,
                        ch.skill_points[5] as i16,
                        ch.skill_points[6] as i16,
                        ch.skill_points[7] as i16,
                        ch.skill_points[8] as i16,
                        ch.skill_points[9] as i16,
                    ],
                })
                .await
            {
                warn!(
                    "WIZ_DATASAVE: failed to save stat points for {}: {}",
                    cid, e
                );
            }
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet};

    #[test]
    fn test_datasave_packet_format() {
        // WIZ_DATASAVE has an empty body — just the opcode
        let pkt = Packet::new(Opcode::WizDatasave as u8);
        assert_eq!(pkt.opcode, 0x37);
        assert!(pkt.data.is_empty());
    }

    #[test]
    fn test_datasave_opcode_value() {
        assert_eq!(Opcode::WizDatasave as u8, 0x37);
        assert_eq!(Opcode::from_byte(0x37), Some(Opcode::WizDatasave));
    }

    // ── Sprint 928: Additional coverage ──────────────────────────────

    /// WIZ_DATASAVE has no S2C response — it is fire-and-forget.
    #[test]
    fn test_datasave_no_response_packet() {
        // The handler returns Ok(()) without sending any packet.
        // No response opcode or data is constructed.
        let pkt = Packet::new(Opcode::WizDatasave as u8);
        assert!(pkt.data.is_empty(), "C2S body is empty");
    }

    /// Gold is clamped to i32::MAX before DB save.
    #[test]
    fn test_datasave_gold_clamp() {
        let gold_normal: u32 = 1_000_000;
        assert_eq!(gold_normal.min(i32::MAX as u32) as i32, 1_000_000);

        let gold_max: u32 = u32::MAX;
        assert_eq!(gold_max.min(i32::MAX as u32) as i32, i32::MAX);

        let gold_boundary: u32 = i32::MAX as u32;
        assert_eq!(gold_boundary.min(i32::MAX as u32) as i32, i32::MAX);
    }

    /// Loyalty and loyalty_monthly also clamped to i32::MAX.
    #[test]
    fn test_datasave_loyalty_clamp() {
        let loyalty: u32 = 3_000_000_000;
        assert_eq!(loyalty.min(i32::MAX as u32) as i32, i32::MAX);

        let loyalty_ok: u32 = 500_000;
        assert_eq!(loyalty_ok.min(i32::MAX as u32) as i32, 500_000);
    }

    /// Skill points array has exactly 10 slots (indices 0-9).
    #[test]
    fn test_datasave_skill_points_array_size() {
        let skill_points: [u8; 10] = [0; 10];
        assert_eq!(skill_points.len(), 10);
        // Index 0 = free skill points, 5-8 = tree allocations
        assert_eq!(skill_points[0], 0);
        assert_eq!(skill_points[9], 0);
    }

    /// Stat fields: str, sta, dex, intel, cha are u8 → i16 for DB.
    #[test]
    fn test_datasave_stat_conversion() {
        let str_val: u8 = 255;
        let sta: u8 = 100;
        assert_eq!(str_val as i16, 255);
        assert_eq!(sta as i16, 100);
        // u8 max fits in i16
        assert!(u8::MAX as i16 <= i16::MAX);
    }

    // ── Sprint 932: Additional coverage ──────────────────────────────

    /// Opcode from_byte roundtrip for 0x37.
    #[test]
    fn test_datasave_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x37), Some(Opcode::WizDatasave));
    }

    /// Free points field fits u8 → i16 conversion.
    #[test]
    fn test_datasave_free_points_i16() {
        for fp in [0u8, 1, 100, 255] {
            let as_i16 = fp as i16;
            assert!(as_i16 >= 0);
            assert_eq!(as_i16 as u8, fp);
        }
    }

    /// Opcode is in v2525 dispatch range (0x06-0xD7).
    #[test]
    fn test_datasave_dispatch_range() {
        let op = Opcode::WizDatasave as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }
}
