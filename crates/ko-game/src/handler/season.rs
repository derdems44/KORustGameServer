//! WIZ_SEASON (0xD7) handler — Season event system.
//!
//! v2525 client's season event notification system. Displays item spawn
//! effects and system messages via `CUIMessageBox`.
//!
//! ## Client RE
//!
//! - Handler: `0x7BEAC0` — single function, Group A (always callable, no panel check)
//! - Dispatch: GameMain index 209 (0xD1), group 147
//! - Season system global: `0x10929FC` — non-null when season active
//! - MessageBox: `0xBB8A10` → `CUIMessageBox` (vtable `0xFBA6FC`, ctor `0xBB8000`)
//!
//! ## S2C Packet Format
//!
//! ```text
//! [u8 header]        — MUST be 1 (client returns if != 1)
//! [i32 action_type]  — determines behavior:
//!   action_type=1: Item spawn
//!     [i32 item_id]  — item/NPC lookup ID
//!     [u16 count]    — spawn count (must be >0)
//!   action_type=2..11+: System message
//!     (no additional fields)
//! ```
//!
//! ## Action Types
//!
//! | action_type | String ID | Hex    | Behavior                        |
//! |-------------|-----------|--------|---------------------------------|
//! | 1           | —         | —      | Item spawn effect (reads extra) |
//! | 2           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 3           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 4           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 5           | 10714     | 0x29DA | Plain string display            |
//! | 6           | 6653      | 0x19FD | Plain string display            |
//! | 7           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 8           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 9           | 40302     | 0x9D6E | Formatted message (%d)          |
//! | 10          | —         | —      | Static: timed item grant failure |
//! | 11          | —         | —      | Static: timed item grant failure |
//! | default     | 40302     | 0x9D6E | Formatted message (fallback)    |
//!
//! ## C2S Packets
//!
//! None — WizSeason is S2C only. No C2S send sites in the binary.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── Action type constants ─────────────────────────────────────────────

/// Item spawn action — reads additional `[i32 item_id][u16 count]`.
pub const ACTION_ITEM_SPAWN: i32 = 1;

/// Season message with format string (text_id 40302, `%d` placeholder).
pub const ACTION_MSG_FORMAT_2: i32 = 2;
/// Season message with format string.
pub const ACTION_MSG_FORMAT_3: i32 = 3;
/// Season message with format string.
pub const ACTION_MSG_FORMAT_4: i32 = 4;

/// Season notification (text_id 10714, plain string).
pub const ACTION_MSG_NOTIFY: i32 = 5;

/// Season notification (text_id 6653, plain string).
pub const ACTION_MSG_SPECIAL: i32 = 6;

/// Season message with format string.
pub const ACTION_MSG_FORMAT_7: i32 = 7;
/// Season message with format string.
pub const ACTION_MSG_FORMAT_8: i32 = 8;
/// Season message with format string.
pub const ACTION_MSG_FORMAT_9: i32 = 9;

/// Timed item grant failure (hardcoded Korean string).
pub const ACTION_TIMED_FAIL_10: i32 = 10;
/// Timed item grant failure (hardcoded Korean string).
pub const ACTION_TIMED_FAIL_11: i32 = 11;

// ── S2C Builders ──────────────────────────────────────────────────────

/// Build a season item spawn packet (action_type=1).
///
/// Client reads: `[u8=1][i32=1][i32 item_id][u16 count]`
///
/// Triggers item visual effect via `0x893520` if:
/// - Season system is active (`[0x10929FC] != 0`)
/// - Item/NPC found by `item_id`
/// - `count > 0`
///
/// - `item_id`: Item or NPC ID for visual lookup
/// - `count`: Number of items to spawn (must be >0 for client to process)
pub fn build_item_spawn(item_id: i32, count: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizSeason as u8);
    pkt.write_u8(1); // header (must be 1)
    pkt.write_i32(ACTION_ITEM_SPAWN);
    pkt.write_i32(item_id);
    pkt.write_u16(count);
    pkt
}

/// Build a season system message packet.
///
/// Client reads: `[u8=1][i32 action_type]`
///
/// Displays a `CUIMessageBox` with text determined by `action_type`:
/// - 2,3,4,7,8,9,default → text_id 40302 (format string with `%d` = action_type)
/// - 5 → text_id 10714 (plain notification)
/// - 6 → text_id 6653 (plain notification)
/// - 10,11 → hardcoded Korean string (timed item failure)
///
/// - `action_type`: Message type (2-11, or any value for fallback format)
pub fn build_message(action_type: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizSeason as u8);
    pkt.write_u8(1); // header (must be 1)
    pkt.write_i32(action_type);
    pkt
}

// ── C2S Handler ───────────────────────────────────────────────────────

/// Handle WIZ_SEASON (0xD7) from the client.
///
/// This opcode is S2C-only — the client never sends it.
/// If received, log and discard.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let reader = PacketReader::new(&pkt.data);
    debug!(
        "[{}] WIZ_SEASON (S2C-only opcode, {}B ignored)",
        session.addr(),
        reader.remaining()
    );
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ── Item spawn builder ────────────────────────────────────────────

    #[test]
    fn test_build_item_spawn_opcode() {
        let pkt = build_item_spawn(100001, 5);
        assert_eq!(pkt.opcode, Opcode::WizSeason as u8);
    }

    #[test]
    fn test_build_item_spawn_format() {
        let pkt = build_item_spawn(370004000, 10);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // header
        assert_eq!(r.read_i32(), Some(ACTION_ITEM_SPAWN)); // action_type=1
        assert_eq!(r.read_i32(), Some(370004000)); // item_id
        assert_eq!(r.read_u16(), Some(10)); // count
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_item_spawn_data_length() {
        // u8 header + i32 action + i32 item_id + u16 count = 1+4+4+2 = 11
        let pkt = build_item_spawn(0, 0);
        assert_eq!(pkt.data.len(), 11);
    }

    #[test]
    fn test_build_item_spawn_max_count() {
        let pkt = build_item_spawn(i32::MAX, u16::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(1)); // action_type
        assert_eq!(r.read_i32(), Some(i32::MAX));
        assert_eq!(r.read_u16(), Some(u16::MAX));
    }

    // ── Message builder ───────────────────────────────────────────────

    #[test]
    fn test_build_message_opcode() {
        let pkt = build_message(ACTION_MSG_FORMAT_2);
        assert_eq!(pkt.opcode, Opcode::WizSeason as u8);
    }

    #[test]
    fn test_build_message_format() {
        let pkt = build_message(ACTION_MSG_NOTIFY);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // header
        assert_eq!(r.read_i32(), Some(ACTION_MSG_NOTIFY)); // action_type=5
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_message_data_length() {
        // u8 header + i32 action_type = 1+4 = 5
        let pkt = build_message(0);
        assert_eq!(pkt.data.len(), 5);
    }

    #[test]
    fn test_build_message_all_format_types() {
        for &action in &[
            ACTION_MSG_FORMAT_2,
            ACTION_MSG_FORMAT_3,
            ACTION_MSG_FORMAT_4,
            ACTION_MSG_FORMAT_7,
            ACTION_MSG_FORMAT_8,
            ACTION_MSG_FORMAT_9,
        ] {
            let pkt = build_message(action);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(1));
            assert_eq!(r.read_i32(), Some(action));
            assert_eq!(r.remaining(), 0);
        }
    }

    #[test]
    fn test_build_message_special_type() {
        let pkt = build_message(ACTION_MSG_SPECIAL);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(6)); // text_id 6653
    }

    #[test]
    fn test_build_message_timed_fail() {
        for &action in &[ACTION_TIMED_FAIL_10, ACTION_TIMED_FAIL_11] {
            let pkt = build_message(action);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(1));
            assert_eq!(r.read_i32(), Some(action));
            assert_eq!(r.remaining(), 0);
        }
    }

    // ── Header validation ─────────────────────────────────────────────

    #[test]
    fn test_header_always_one() {
        // Client checks header==1, returns if not
        assert_eq!(build_item_spawn(0, 0).data[0], 1);
        assert_eq!(build_message(2).data[0], 1);
        assert_eq!(build_message(5).data[0], 1);
        assert_eq!(build_message(10).data[0], 1);
    }

    // ── Item spawn vs message size difference ─────────────────────────

    #[test]
    fn test_item_spawn_longer_than_message() {
        let spawn = build_item_spawn(1, 1);
        let msg = build_message(2);
        // Item spawn: 11 bytes, message: 5 bytes
        assert_eq!(spawn.data.len(), 11);
        assert_eq!(msg.data.len(), 5);
        assert!(spawn.data.len() > msg.data.len());
    }

    // ── Action type constants ─────────────────────────────────────────

    #[test]
    fn test_action_type_values() {
        assert_eq!(ACTION_ITEM_SPAWN, 1);
        assert_eq!(ACTION_MSG_FORMAT_2, 2);
        assert_eq!(ACTION_MSG_FORMAT_3, 3);
        assert_eq!(ACTION_MSG_FORMAT_4, 4);
        assert_eq!(ACTION_MSG_NOTIFY, 5);
        assert_eq!(ACTION_MSG_SPECIAL, 6);
        assert_eq!(ACTION_MSG_FORMAT_7, 7);
        assert_eq!(ACTION_MSG_FORMAT_8, 8);
        assert_eq!(ACTION_MSG_FORMAT_9, 9);
        assert_eq!(ACTION_TIMED_FAIL_10, 10);
        assert_eq!(ACTION_TIMED_FAIL_11, 11);
    }

    // ── Negative / edge values ────────────────────────────────────────

    #[test]
    fn test_build_item_spawn_negative_id() {
        let pkt = build_item_spawn(-1, 1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(1)); // action_type
        assert_eq!(r.read_i32(), Some(-1)); // item_id preserved as i32
        assert_eq!(r.read_u16(), Some(1));
    }

    #[test]
    fn test_build_message_default_fallback() {
        // action_type=99 still works — client falls through to text_id 40302
        let pkt = build_message(99);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(99));
        assert_eq!(r.remaining(), 0);
    }
}
