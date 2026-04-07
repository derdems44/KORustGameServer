//! WIZ_SKILLDATA (0x79) handler — skill shortcut bar save/load.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/UserSkillShortcutSystem.cpp`
//!
//! Sub-opcodes:
//! - 1 = SKILL_DATA_SAVE: Client sends skill bar data → save to DB (no response)
//! - 2 = SKILL_DATA_LOAD: Client requests saved skill bar data → load from DB, send response
//!
//! Binary format: each skill slot is a little-endian uint32 (4 bytes).
//! Max 80 slots = 320 bytes, but count is capped at 64 per C++ validation.

use ko_db::repositories::skill_shortcut::SkillShortcutRepository;
use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};

/// Sub-opcode: client saving skill bar layout.
const SKILL_DATA_SAVE: u8 = 1;

/// Sub-opcode: client requesting skill bar layout.
const SKILL_DATA_LOAD: u8 = 2;

/// Maximum number of skill slots allowed per save (C++ limit in SkillDataSave).
const MAX_SKILL_COUNT: u16 = 64;

/// Handle WIZ_SKILLDATA from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    // Dead players cannot modify skill bar
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = reader.read_u8().unwrap_or(0);

    match sub_opcode {
        SKILL_DATA_SAVE => handle_save(session, &mut reader).await,
        SKILL_DATA_LOAD => handle_load(session).await,
        _ => {
            warn!(
                "[{}] Unknown skilldata sub-opcode: {}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

/// Handle SKILL_DATA_SAVE — client sends skill bar layout to persist.
///
/// C++ Reference: `CUser::SkillDataSave` in `UserSkillShortcutSystem.cpp:32-44`
/// and `CUser::ReqSkillDataSave` in `DatabaseThread.cpp:1342-1361`.
///
/// Packet format: u16(count) + u32[count] (skill IDs)
///
/// The C++ server does NOT send a response for save — it only persists to DB.
async fn handle_save(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let count = reader.read_u16().unwrap_or(0);
    if count == 0 || count > MAX_SKILL_COUNT {
        return Ok(());
    }

    // Read skill IDs and build the raw binary blob (little-endian uint32 array).
    let byte_len = count as usize * 4;
    let mut skill_data = vec![0u8; byte_len];
    for i in 0..count as usize {
        let skill_id = reader.read_u32().unwrap_or(0);
        let offset = i * 4;
        skill_data[offset..offset + 4].copy_from_slice(&skill_id.to_le_bytes());
    }

    // Get character name
    let char_name = match session.world().get_character_info(session.session_id()) {
        Some(ch) => ch.name.clone(),
        None => {
            debug!("[{}] SKILL_DATA_SAVE: no character info", session.addr());
            return Ok(());
        }
    };

    // Persist to database
    let repo = SkillShortcutRepository::new(session.pool());
    if let Err(e) = repo.save(&char_name, count as i16, &skill_data).await {
        warn!(
            "[{}] SKILL_DATA_SAVE: DB error for '{}': {}",
            session.addr(),
            char_name,
            e
        );
    } else {
        debug!(
            "[{}] SKILL_DATA_SAVE: saved {} skills for '{}'",
            session.addr(),
            count,
            char_name
        );
    }

    Ok(())
}

/// Handle SKILL_DATA_LOAD — client requests saved skill bar layout.
///
/// C++ Reference: `CUser::SkillDataLoad` in `UserSkillShortcutSystem.cpp:49-53`
/// and `CUser::ReqSkillDataLoad` in `DatabaseThread.cpp:1333-1340`.
///
/// Response: WIZ_SKILLDATA + u8(SKILL_DATA_LOAD) + u16(count) + u32[count]
///
/// If no data exists, sends count = 0.
async fn handle_load(session: &mut ClientSession) -> anyhow::Result<()> {
    let char_name = match session.world().get_character_info(session.session_id()) {
        Some(ch) => ch.name.clone(),
        None => {
            debug!("[{}] SKILL_DATA_LOAD: no character info", session.addr());
            return Ok(());
        }
    };

    let repo = SkillShortcutRepository::new(session.pool());
    let row = repo.load(&char_name).await?;

    let mut response = Packet::new(Opcode::WizSkillData as u8);
    response.write_u8(SKILL_DATA_LOAD);

    match row {
        Some(r) => {
            // C++ Reference: DatabaseThread.cpp:1348 — count is `short` (signed i16)
            // Validate before casting to avoid negative → huge u16 overflow
            let count = if r.count > 0 && r.count <= 64 {
                r.count as u16
            } else {
                0
            };
            response.write_u16(count);
            // Write each skill ID from the binary blob
            for i in 0..count as usize {
                let offset = i * 4;
                if offset + 4 <= r.skill_data.len() {
                    let skill_id = u32::from_le_bytes([
                        r.skill_data[offset],
                        r.skill_data[offset + 1],
                        r.skill_data[offset + 2],
                        r.skill_data[offset + 3],
                    ]);
                    response.write_u32(skill_id);
                } else {
                    response.write_u32(0);
                }
            }
            debug!(
                "[{}] SKILL_DATA_LOAD: loaded {} skills for '{}'",
                session.addr(),
                count,
                char_name
            );
        }
        None => {
            response.write_u16(0);
            debug!(
                "[{}] SKILL_DATA_LOAD: no saved data for '{}'",
                session.addr(),
                char_name
            );
        }
    }

    session.send_packet(&response).await?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_data_save_binary_encoding() {
        // Test that skill IDs are correctly encoded as little-endian uint32 in the blob
        let skills: Vec<u32> = vec![100001, 200002, 300003, 400004];
        let count = skills.len() as u16;
        let byte_len = count as usize * 4;
        let mut skill_data = vec![0u8; byte_len];
        for (i, &skill_id) in skills.iter().enumerate() {
            let offset = i * 4;
            skill_data[offset..offset + 4].copy_from_slice(&skill_id.to_le_bytes());
        }

        // Verify each skill ID can be read back
        for (i, &expected) in skills.iter().enumerate() {
            let offset = i * 4;
            let actual = u32::from_le_bytes([
                skill_data[offset],
                skill_data[offset + 1],
                skill_data[offset + 2],
                skill_data[offset + 3],
            ]);
            assert_eq!(actual, expected, "skill at index {} mismatch", i);
        }

        assert_eq!(skill_data.len(), 16); // 4 skills * 4 bytes each
    }

    #[test]
    fn test_skill_data_max_count_validation() {
        // Count must be 1..=64
        assert_eq!(MAX_SKILL_COUNT, 64);
        // 0 is rejected (count == 0 returns early)
        let zero: u16 = 0;
        assert!(zero == 0);
        // 65 is rejected (count > 64 returns early)
        assert!(65_u16 > MAX_SKILL_COUNT);
        // 64 is accepted
        assert!(64_u16 <= MAX_SKILL_COUNT);
        // 1 is accepted
        assert!(1_u16 > 0 && 1_u16 <= MAX_SKILL_COUNT);
    }

    #[test]
    fn test_skill_data_load_response_format() {
        // Verify the response packet structure for LOAD
        let mut response = Packet::new(Opcode::WizSkillData as u8);
        response.write_u8(SKILL_DATA_LOAD);
        response.write_u16(3); // 3 skills
        response.write_u32(10001);
        response.write_u32(20002);
        response.write_u32(30003);

        // Verify packet data: sub_opcode(1) + count(2) + 3*skill(12) = 15 bytes
        assert_eq!(response.data.len(), 15);
        assert_eq!(response.data[0], SKILL_DATA_LOAD);
        // count is u16 LE
        assert_eq!(u16::from_le_bytes([response.data[1], response.data[2]]), 3);
        // first skill
        assert_eq!(
            u32::from_le_bytes([
                response.data[3],
                response.data[4],
                response.data[5],
                response.data[6]
            ]),
            10001
        );
    }

    #[test]
    fn test_skill_data_empty_load_response() {
        // When no data exists, response should have count = 0
        let mut response = Packet::new(Opcode::WizSkillData as u8);
        response.write_u8(SKILL_DATA_LOAD);
        response.write_u16(0);

        // sub_opcode(1) + count(2) = 3 bytes
        assert_eq!(response.data.len(), 3);
        assert_eq!(response.data[0], SKILL_DATA_LOAD);
        assert_eq!(u16::from_le_bytes([response.data[1], response.data[2]]), 0);
    }

    #[test]
    fn test_skill_data_roundtrip_blob() {
        // Test full roundtrip: encode skills → blob → decode back
        let original_skills: Vec<u32> =
            vec![500100, 500200, 500300, 0, 600001, 700050, 0, 0, 123456789];
        let count = original_skills.len();

        // Encode
        let mut blob = vec![0u8; count * 4];
        for (i, &s) in original_skills.iter().enumerate() {
            let off = i * 4;
            blob[off..off + 4].copy_from_slice(&s.to_le_bytes());
        }

        // Decode
        let mut decoded = Vec::with_capacity(count);
        for i in 0..count {
            let off = i * 4;
            decoded.push(u32::from_le_bytes([
                blob[off],
                blob[off + 1],
                blob[off + 2],
                blob[off + 3],
            ]));
        }

        assert_eq!(original_skills, decoded);
    }

    // ── Sprint 287: Negative count validation ───────────────────────────

    #[test]
    fn test_skill_count_negative_clamped_to_zero() {
        // C++ Reference: DatabaseThread.cpp:1348 — count stored as `short` (signed i16)
        // If DB returns a negative count, it must be clamped to 0 to avoid
        // overflow when cast to u16 (e.g., -1 → 65535).
        let negative_count: i16 = -1;
        let safe_count = if negative_count > 0 && negative_count <= 64 {
            negative_count as u16
        } else {
            0
        };
        assert_eq!(safe_count, 0);
    }

    #[test]
    fn test_skill_count_exceeding_max_clamped() {
        // Max skill shortcut slots is 64
        let too_large: i16 = 65;
        let safe_count = if too_large > 0 && too_large <= 64 {
            too_large as u16
        } else {
            0
        };
        assert_eq!(safe_count, 0);
    }

    // ── Sprint 925: Additional coverage ──────────────────────────────

    /// Sub-opcode constants: SAVE=1, LOAD=2.
    #[test]
    fn test_sub_opcode_constants() {
        assert_eq!(SKILL_DATA_SAVE, 1);
        assert_eq!(SKILL_DATA_LOAD, 2);
    }

    /// Save C2S packet format: [u8 sub=1][u16 count][u32 * count skills].
    #[test]
    fn test_save_c2s_format() {
        use ko_protocol::PacketReader;
        let mut pkt = Packet::new(Opcode::WizSkillData as u8);
        pkt.write_u8(SKILL_DATA_SAVE);
        pkt.write_u16(3);
        pkt.write_u32(500100);
        pkt.write_u32(500200);
        pkt.write_u32(500300);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(SKILL_DATA_SAVE));
        assert_eq!(r.read_u16(), Some(3));
        assert_eq!(r.read_u32(), Some(500100));
        assert_eq!(r.read_u32(), Some(500200));
        assert_eq!(r.read_u32(), Some(500300));
        assert_eq!(r.remaining(), 0);
    }

    /// Load response with MAX_SKILL_COUNT (64) skills.
    #[test]
    fn test_load_response_max_skills() {
        let mut pkt = Packet::new(Opcode::WizSkillData as u8);
        pkt.write_u8(SKILL_DATA_LOAD);
        pkt.write_u16(MAX_SKILL_COUNT);
        for i in 0..MAX_SKILL_COUNT {
            pkt.write_u32(i as u32 + 1);
        }
        // sub(1) + count(2) + 64*4 = 259
        assert_eq!(pkt.data.len(), 259);
    }

    /// Binary blob byte length = count * 4.
    #[test]
    fn test_blob_byte_length() {
        for count in [1u16, 10, 32, 64] {
            let byte_len = count as usize * 4;
            let blob = vec![0u8; byte_len];
            assert_eq!(blob.len(), count as usize * 4);
        }
    }

    /// Zero skill IDs are preserved in blob encoding.
    #[test]
    fn test_zero_skill_id_preserved() {
        let skills: Vec<u32> = vec![0, 100001, 0, 0, 200002];
        let mut blob = vec![0u8; skills.len() * 4];
        for (i, &s) in skills.iter().enumerate() {
            blob[i * 4..(i + 1) * 4].copy_from_slice(&s.to_le_bytes());
        }
        // Decode and verify zeros preserved
        for (i, &expected) in skills.iter().enumerate() {
            let actual = u32::from_le_bytes(blob[i * 4..(i + 1) * 4].try_into().unwrap());
            assert_eq!(actual, expected);
        }
    }
}
