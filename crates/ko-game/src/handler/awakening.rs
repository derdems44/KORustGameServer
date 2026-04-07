//! WIZ_AWAKENING (0xCB) handler — Awakening visual effect system.
//!
//! v2525 client's awakening effect display. Triggers visual effects
//! on entities based on effect type and scale parameters.
//!
//! ## Client RE
//!
//! - Handler: `0x82FAF7` — reads `[f32][u8][i32]`
//! - Group A (always callable, no panel null-check)
//! - Effect function: `0x893520` — visual effect spawn
//! - Only processes when `effect_type == 1`
//!
//! ## S2C Packet Format
//!
//! ```text
//! [f32 effect_scale]  — visual scale multiplier
//! [u8  effect_type]   — must be 1 for processing
//! [i32 effect_id]     — effect definition ID
//! ```
//!
//! ## C2S Packets
//!
//! None — WizAwakening is S2C only. No C2S send sites in the binary.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── Effect type constants ─────────────────────────────────────────────

/// Visual effect trigger (only type processed by client).
pub const EFFECT_TYPE_VISUAL: u8 = 1;

// ── S2C Builders ──────────────────────────────────────────────────────

/// Build an awakening visual effect packet.
///
/// Client reads: `[f32 effect_scale][u8 effect_type][i32 effect_id]`
///
/// Only processed when `effect_type == 1`. Calls visual effect spawn
/// function `0x893520` with (effect_id, effect_scale).
///
/// - `effect_scale`: Visual scale multiplier (f32)
/// - `effect_type`: Must be 1 for client to process
/// - `effect_id`: Effect definition lookup ID
pub fn build_effect(effect_scale: f32, effect_type: u8, effect_id: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAwakening as u8);
    pkt.write_f32(effect_scale);
    pkt.write_u8(effect_type);
    pkt.write_i32(effect_id);
    pkt
}

/// Build an awakening visual effect with default type (1).
///
/// Convenience wrapper that sets `effect_type = 1`.
pub fn build_visual_effect(effect_scale: f32, effect_id: i32) -> Packet {
    build_effect(effect_scale, EFFECT_TYPE_VISUAL, effect_id)
}

// ── C2S Handler ───────────────────────────────────────────────────────

/// Handle WIZ_AWAKENING (0xCB) from the client.
///
/// This opcode is S2C-only — the client never sends it.
/// If received, log and discard.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let reader = PacketReader::new(&pkt.data);
    debug!(
        "[{}] WIZ_AWAKENING (S2C-only opcode, {}B ignored)",
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

    #[test]
    fn test_build_effect_opcode() {
        let pkt = build_effect(1.0, 1, 100);
        assert_eq!(pkt.opcode, Opcode::WizAwakening as u8);
    }

    #[test]
    fn test_build_effect_format() {
        let pkt = build_effect(2.5, EFFECT_TYPE_VISUAL, 42);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(2.5)); // effect_scale
        assert_eq!(r.read_u8(), Some(1)); // effect_type
        assert_eq!(r.read_i32(), Some(42)); // effect_id
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_effect_data_length() {
        // f32 + u8 + i32 = 4+1+4 = 9
        let pkt = build_effect(0.0, 0, 0);
        assert_eq!(pkt.data.len(), 9);
    }

    #[test]
    fn test_build_visual_effect_convenience() {
        let pkt = build_visual_effect(1.5, 999);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.5));
        assert_eq!(r.read_u8(), Some(EFFECT_TYPE_VISUAL)); // auto type=1
        assert_eq!(r.read_i32(), Some(999));
    }

    #[test]
    fn test_build_effect_zero_scale() {
        let pkt = build_effect(0.0, 1, 0);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(0.0));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(0));
    }

    #[test]
    fn test_build_effect_negative_id() {
        let pkt = build_effect(1.0, 1, -1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.0));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_i32(), Some(-1));
    }

    #[test]
    fn test_build_effect_max_values() {
        let pkt = build_effect(f32::MAX, u8::MAX, i32::MAX);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(f32::MAX));
        assert_eq!(r.read_u8(), Some(u8::MAX));
        assert_eq!(r.read_i32(), Some(i32::MAX));
    }

    #[test]
    fn test_effect_type_constant() {
        assert_eq!(EFFECT_TYPE_VISUAL, 1);
    }

    #[test]
    fn test_build_effect_non_visual_type() {
        // Type != 1 is valid packet format but client won't process it
        let pkt = build_effect(1.0, 0, 100);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.0));
        assert_eq!(r.read_u8(), Some(0)); // type 0 — client ignores
        assert_eq!(r.read_i32(), Some(100));
    }

    // ── Upgrade integration tests ───────────────────────────────────

    #[test]
    fn test_upgrade_effect_scale_by_grade() {
        // Low grade (0-4) → scale 1.0
        let pkt = build_visual_effect(1.0, 389010000);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.0));

        // Mid grade (5-7) → scale 1.5
        let pkt = build_visual_effect(1.5, 389010005);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.5));

        // High grade (8+) → scale 2.0
        let pkt = build_visual_effect(2.0, 389010008);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(2.0));
    }

    #[test]
    fn test_upgrade_effect_uses_new_item_id() {
        // On success, effect_id = new_item_id (upgraded item)
        let new_item_id: i32 = 389010007; // +7 weapon
        let pkt = build_visual_effect(1.5, new_item_id);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_f32(), Some(1.5));
        assert_eq!(r.read_u8(), Some(EFFECT_TYPE_VISUAL));
        assert_eq!(r.read_i32(), Some(389010007));
    }
}
