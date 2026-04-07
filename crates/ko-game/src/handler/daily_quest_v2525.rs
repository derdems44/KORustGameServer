//! WIZ_DAILY_QUEST (0xC7) handler — v2525 native daily quest system.
//!
//! v2525 client's native daily quest panel with 4 quest slots.
//! Distinct from the ext_hook daily_quest module.
//!
//! ## Client RE
//!
//! - Dispatch: `0x82F9FA` — panel at `[esi+0x1F0]`, delegates to `0xE13590`
//! - Panel: Group B (panel-dependent for sub=1/2, sub=0 always callable)
//! - Quest validation: `quest_id / 1000` and `quest_id % 1000`
//! - NPC validation: `npc_id > 200 && npc_id * slot_count >= 100000` → error 19610
//!
//! ## S2C Packet Format
//!
//! ```text
//! [u8 sub] — sub-opcode:
//!   sub=0: Error/Status
//!     [u8 pad=0]
//!     [i32 error_code]  — maps to text_id (see table)
//!   sub=1: Quest Slot Init
//!     [u8 quest_index]  — slot 0-3
//!     [i32 quest_id]    — composite ID (validated via div/mod 1000)
//!     [u16 npc_id]      — target NPC
//!     [u8 status]       — slot state (0=active)
//!   sub=2: Quest Completion
//!     [u8 slot_index]   — which slot completed
//!     [i32 quest_id]    — completed quest ID
//!     [u16 pad=0]       — padding
//! ```
//!
//! ## Error Codes (sub=0)
//!
//! | code    | text_id | Hex    |
//! |---------|---------|--------|
//! | -7      | 2601    | 0x0A29 |
//! | -6      | 12423   | 0x3087 |
//! | -5      | 43687   | 0xAAA7 |
//! | -4      | 10714   | 0x29DA |
//! | -3      | 19383   | 0x4BB7 |
//! | -2      | 43743   | 0xAADF |
//! | -1      | 19384   | 0x4BB8 |
//! | default | 1915    | 0x077B |
//!
//! ## String IDs
//!
//! - Completion: 43740 (0xAADC) — quest name in crimson (0xFFDC143C)
//!
//! ## C2S Packets
//!
//! Panel interactions send C2S — currently logged and ignored.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── Sub-opcode constants ──────────────────────────────────────────────

/// Error/status response.
pub const SUB_ERROR: u8 = 0;

/// Quest slot init (set quest into slot 0-3).
pub const SUB_INIT: u8 = 1;

/// Quest completion (remove from slot, show message).
pub const SUB_COMPLETE: u8 = 2;

// ── S2C Builders ──────────────────────────────────────────────────────

/// Build a quest slot init packet (sub=1).
///
/// Client sets quest into one of 4 slots. Validates quest_id via
/// div/mod 1000 and checks NPC ID overflow.
///
/// - `quest_index`: Slot 0-3
/// - `quest_id`: Composite quest ID
/// - `npc_id`: Target NPC ID
/// - `status`: Slot state (0=active)
///
/// Wire: `[u8 sub=1][u8 quest_index][i32 quest_id][u16 npc_id][u8 status]`
pub fn build_init(quest_index: u8, quest_id: i32, npc_id: u16, status: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizDailyQuest as u8);
    pkt.write_u8(SUB_INIT);
    pkt.write_u8(quest_index);
    pkt.write_i32(quest_id);
    pkt.write_u16(npc_id);
    pkt.write_u8(status);
    pkt
}

/// Build a quest completion packet (sub=2).
///
/// Client removes quest from slot and shows completion message
/// using text_id 43740 (0xAADC) in crimson (0xFFDC143C).
///
/// - `slot_index`: Which slot completed
/// - `quest_id`: Completed quest ID
///
/// Wire: `[u8 sub=2][u8 slot_index][i32 quest_id][u16 pad=0]`
pub fn build_complete(slot_index: u8, quest_id: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizDailyQuest as u8);
    pkt.write_u8(SUB_COMPLETE);
    pkt.write_u8(slot_index);
    pkt.write_i32(quest_id);
    pkt.write_u16(0); // padding
    pkt
}

/// Build an error/status packet (sub=0).
///
/// Client maps error_code to text_id and displays in crimson.
///
/// - `error_code`: Error code (see table in module docs)
///
/// Wire: `[u8 sub=0][u8 pad=0][i32 error_code]`
pub fn build_error(error_code: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizDailyQuest as u8);
    pkt.write_u8(SUB_ERROR);
    pkt.write_u8(0); // padding
    pkt.write_i32(error_code);
    pkt
}

// ── C2S Handler ───────────────────────────────────────────────────────

/// Handle WIZ_DAILY_QUEST (0xC7) from the client.
///
/// Panel interactions from the v2525 native daily quest UI.
/// Currently logged and ignored (no daily quest v2525 DB yet).
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub = reader.read_u8().unwrap_or(0);
    debug!(
        "[{}] WIZ_DAILY_QUEST(v2525) sub={} ({}B remaining)",
        session.addr(),
        sub,
        reader.remaining()
    );
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ── Init builder (sub=1) ──────────────────────────────────────────

    #[test]
    fn test_build_init_opcode() {
        let pkt = build_init(0, 0, 0, 0);
        assert_eq!(pkt.opcode, Opcode::WizDailyQuest as u8);
    }

    #[test]
    fn test_build_init_format() {
        let pkt = build_init(2, 1001, 500, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(SUB_INIT)); // sub=1
        assert_eq!(r.read_u8(), Some(2)); // quest_index
        assert_eq!(r.read_i32(), Some(1001)); // quest_id
        assert_eq!(r.read_u16(), Some(500)); // npc_id
        assert_eq!(r.read_u8(), Some(0)); // status
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_init_data_length() {
        // u8 sub + u8 index + i32 quest_id + u16 npc_id + u8 status = 1+1+4+2+1 = 9
        let pkt = build_init(0, 0, 0, 0);
        assert_eq!(pkt.data.len(), 9);
    }

    #[test]
    fn test_build_init_all_slots() {
        for slot in 0..4u8 {
            let pkt = build_init(slot, 1000 + slot as i32, 100, 0);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(SUB_INIT));
            assert_eq!(r.read_u8(), Some(slot));
        }
    }

    // ── Complete builder (sub=2) ──────────────────────────────────────

    #[test]
    fn test_build_complete_opcode() {
        let pkt = build_complete(0, 0);
        assert_eq!(pkt.opcode, Opcode::WizDailyQuest as u8);
    }

    #[test]
    fn test_build_complete_format() {
        let pkt = build_complete(1, 2001);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(SUB_COMPLETE)); // sub=2
        assert_eq!(r.read_u8(), Some(1)); // slot_index
        assert_eq!(r.read_i32(), Some(2001)); // quest_id
        assert_eq!(r.read_u16(), Some(0)); // padding
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_complete_data_length() {
        // u8 sub + u8 slot + i32 quest_id + u16 pad = 1+1+4+2 = 8
        let pkt = build_complete(0, 0);
        assert_eq!(pkt.data.len(), 8);
    }

    // ── Error builder (sub=0) ─────────────────────────────────────────

    #[test]
    fn test_build_error_opcode() {
        let pkt = build_error(0);
        assert_eq!(pkt.opcode, Opcode::WizDailyQuest as u8);
    }

    #[test]
    fn test_build_error_format() {
        let pkt = build_error(-3);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(SUB_ERROR)); // sub=0
        assert_eq!(r.read_u8(), Some(0)); // padding
        assert_eq!(r.read_i32(), Some(-3)); // error_code
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_error_data_length() {
        // u8 sub + u8 pad + i32 error = 1+1+4 = 6
        let pkt = build_error(0);
        assert_eq!(pkt.data.len(), 6);
    }

    #[test]
    fn test_build_error_all_codes() {
        for &code in &[-7, -6, -5, -4, -3, -2, -1, 0] {
            let pkt = build_error(code);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(SUB_ERROR));
            assert_eq!(r.read_u8(), Some(0));
            assert_eq!(r.read_i32(), Some(code));
        }
    }

    // ── Sub-opcode constants ──────────────────────────────────────────

    #[test]
    fn test_sub_opcode_values() {
        assert_eq!(SUB_ERROR, 0);
        assert_eq!(SUB_INIT, 1);
        assert_eq!(SUB_COMPLETE, 2);
    }

    // ── All builders same opcode ──────────────────────────────────────

    #[test]
    fn test_all_builders_same_opcode() {
        assert_eq!(build_init(0, 0, 0, 0).opcode, Opcode::WizDailyQuest as u8);
        assert_eq!(build_complete(0, 0).opcode, Opcode::WizDailyQuest as u8);
        assert_eq!(build_error(0).opcode, Opcode::WizDailyQuest as u8);
    }
}
