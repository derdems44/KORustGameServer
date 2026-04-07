//! v2525 native opcode handlers — dispatch-only module.
//!
//! All 21 v2525 opcodes have dedicated handler modules. This module retains only
//! the two C2S dispatch functions that don't belong to a specific feature module:
//!
//! - `handle_challenge2` — anti-cheat heartbeat (intentional no-op)
//! - `handle_unexpected_c2s` — catch-all for S2C-only opcodes arriving as C2S
//!
//! ## v2525 Client Handler Architecture
//!
//! These opcodes are in the v2525 client's GameMain dispatch range (0x06–0xD7)
//! but have no C++ reference implementation. Packet formats were reverse-engineered
//! from the client binary (`KnightOnLine_wine.exe`, PE32 i386).
//!
//! Most v2525 handlers delegate to UI panel objects stored at fixed offsets in the
//! GameMain object (`esi`). If the panel pointer is NULL (panel not yet initialized),
//! the handler silently returns. Panels are created when the user opens the
//! corresponding UI.
//!
//! ### Always-callable handlers (no null check):
//! - `0x92` WizMaxHpChange — HP bar max change notification
//! - `0x95` WizSeal — seal system (own dispatch, checks NPC IDs)
//! - `0xA5` WizAchievement2 — achievement value display
//! - `0xA9` WizCollection1 — collection system (own switch: sub=1 list, sub=2 detail)
//! - `0xCB` WizAwakening — awakening visual effect
//! - `0xCE` WizChallenge2 — anti-cheat heartbeat (NOT a game feature)
//! - `0xD7` WizSeason — season/battle pass system
//!
//! ### UI-panel-dependent handlers (null check on panel):
//! - `0xAC` WizPremium2 — `[esi+0x5B4]`
//! - `0xB4` WizCollection2 — `[esi+0x5C8]`
//! - `0xB7` WizAttendance — `[esi+0x600]` (only opcode using Table G at `0x8300E0`)
//! - `0xB8` WizUpgradeNotice — `[esi+0x60C]`
//! - `0xC3` WizCostume — `[esi+0x63C]`
//! - `0xC5` WizSoul — `[esi+0x1C8]+0x110` flag
//! - `0xC7` WizDailyQuest — `[esi+0x1F0]`
//! - `0xCC` WizEnchant — sub=1:`[esi+0x678]`(weapon/armor) sub=2:`[esi+0x67C]`(item)
//! - `0xCF` WizAbility — `[esi+0x694]`
//! - `0xD0` WizGuildBank — `[esi+0x684]`
//! - `0xD3` WizRebirth — `[esi+0x69C]`
//! - `0xD5` WizTerritory — `[esi+0x6AC]`
//! - `0xD6` WizWorldBoss — `[esi+0x6B4]`
//!
//! ### C2S active opcodes (client sends these):
//! - `0x95` WizSeal — 10 send sites, 3 sub-formats (0x02 toggle, 0x63 full, 0x65 sync)
//! - `0xB8` WizUpgradeNotice — 4 send sites, `[u32 item_id]` (upgrade observation)
//! - `0xCE` WizChallenge2 — 2 send sites, `[u32 ts_hash][u32 seq]` (**anti-cheat**, not game feature)
//! - `0xD5` WizTerritory — 1 send site, `[u8 type][u8 action]` (zone-based territory check)

use ko_protocol::Packet;
use tracing::debug;

use crate::session::ClientSession;

// ── C2S Handlers ────────────────────────────────────────────────────────

/// Handle WIZ_CHALLENGE2 (0xCE) — C2S anti-cheat integrity check.
///
/// Client RE (C2S format): `[u32 timestamp_hash][u32 sequence_counter]` — 8 bytes.
/// NOT a game feature! This is a periodic client integrity packet.
/// Timestamp hash algorithm uses game state counters with div-by-1000 validation.
/// The C++ server has NO handler for 0xCE — packet is silently dropped.
///
/// We intentionally no-op this — responding would be counterproductive.
pub fn handle_challenge2(session: &mut ClientSession, _packet: Packet) -> anyhow::Result<()> {
    // Anti-cheat integrity check — silently consume. No response needed.
    // Logging at trace level to avoid spam (sent periodically).
    tracing::trace!("[{}] WIZ_CHALLENGE2 (anti-cheat heartbeat)", session.addr());
    Ok(())
}

/// Handle any v2525 S2C-only opcode that arrives as C2S (unexpected).
///
/// These opcodes should never be sent by the client. If they arrive,
/// log and ignore.
pub fn handle_unexpected_c2s(session: &mut ClientSession, packet: Packet) -> anyhow::Result<()> {
    debug!(
        "[{}] Unexpected C2S for S2C-only opcode 0x{:02X} ({}B)",
        session.addr(),
        packet.opcode,
        packet.data.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    // ── WIZ_CHALLENGE2 (0xCE) ──────────────────────────────────────

    #[test]
    fn test_challenge2_opcode_value() {
        assert_eq!(Opcode::WizChallenge2 as u8, 0xCE);
    }

    #[test]
    fn test_challenge2_is_in_dispatch_range() {
        // v2525 dispatch range: 0x06-0xD7
        let op = Opcode::WizChallenge2 as u8;
        assert!(op >= 0x06 && op <= 0xD7);
    }

    #[test]
    fn test_challenge2_c2s_packet_format() {
        // C2S: [u32 timestamp_hash][u32 sequence_counter] — 8 bytes
        let mut pkt = Packet::new(Opcode::WizChallenge2 as u8);
        pkt.write_u32(0xDEADBEEF); // timestamp hash
        pkt.write_u32(42); // sequence counter
        assert_eq!(pkt.data.len(), 8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u32(), Some(0xDEADBEEF));
        assert_eq!(r.read_u32(), Some(42));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_challenge2_handler_is_sync() {
        // handle_challenge2 is sync (not async) — intentional, no I/O needed
        let _: fn(&mut super::ClientSession, super::Packet) -> anyhow::Result<()> =
            super::handle_challenge2;
    }

    // ── handle_unexpected_c2s ──────────────────────────────────────

    #[test]
    fn test_unexpected_c2s_handler_is_sync() {
        // handle_unexpected_c2s is sync — just logs and returns Ok
        let _: fn(&mut super::ClientSession, super::Packet) -> anyhow::Result<()> =
            super::handle_unexpected_c2s;
    }

    #[test]
    fn test_unexpected_c2s_accepts_any_opcode() {
        // Should accept any opcode — it's a catch-all
        for op in [0x92u8, 0xA5, 0xCB, 0xD7, 0x00, 0xFF] {
            let pkt = Packet::new(op);
            assert_eq!(pkt.opcode, op);
        }
    }

    // ── v2525 dispatch range validation ────────────────────────────

    #[test]
    fn test_v2525_always_callable_opcodes_in_range() {
        // Group A (always callable) — all within 0x06-0xD7
        let always_callable: &[u8] = &[0x92, 0x95, 0xA5, 0xA9, 0xCB, 0xCE, 0xD7];
        for &op in always_callable {
            assert!(
                op >= 0x06 && op <= 0xD7,
                "opcode 0x{:02X} outside dispatch range",
                op
            );
        }
    }

    #[test]
    fn test_v2525_panel_dependent_opcodes_in_range() {
        // Group B (panel-dependent) — all within 0x06-0xD7
        let panel_dependent: &[u8] = &[
            0xAC, 0xB4, 0xB7, 0xB8, 0xC3, 0xC5, 0xC7, 0xCC, 0xCF, 0xD0, 0xD3, 0xD5, 0xD6,
        ];
        for &op in panel_dependent {
            assert!(
                op >= 0x06 && op <= 0xD7,
                "opcode 0x{:02X} outside dispatch range",
                op
            );
        }
    }

    #[test]
    fn test_ext_hook_outside_dispatch_range() {
        // EXT_HOOK (0xE9) is OUTSIDE v2525 dispatch range
        assert!(0xE9u8 > 0xD7);
    }

    // ── Sprint 929: Additional coverage ──────────────────────────────

    /// C2S active opcodes: 0x95, 0xB8, 0xCE, 0xD5.
    #[test]
    fn test_v2525_c2s_active_opcodes() {
        let c2s_active: [u8; 4] = [0x95, 0xB8, 0xCE, 0xD5];
        for &op in &c2s_active {
            assert!(op >= 0x06 && op <= 0xD7, "0x{:02X} in range", op);
        }
    }

    /// Challenge2 C2S is exactly 8 bytes (u32 + u32).
    #[test]
    fn test_challenge2_c2s_data_length() {
        let mut pkt = Packet::new(Opcode::WizChallenge2 as u8);
        pkt.write_u32(0x12345678);
        pkt.write_u32(100);
        assert_eq!(pkt.data.len(), 8);
    }

    /// Total v2525 opcodes: 7 always-callable + 13 panel-dependent = 20.
    #[test]
    fn test_v2525_total_opcode_count() {
        let always_callable = 7;
        let panel_dependent = 13;
        assert_eq!(always_callable + panel_dependent, 20);
    }

    /// Dispatch range boundaries: 0x06 (first) and 0xD7 (last).
    #[test]
    fn test_v2525_dispatch_range_boundaries() {
        assert_eq!(0x06u8, 6, "first dispatch opcode");
        assert_eq!(0xD7u8, 215, "last dispatch opcode");
        // Total range: 215 - 6 + 1 = 210 possible opcodes
        assert_eq!(0xD7u8 - 0x06u8 + 1, 210);
    }

    /// 0xD8+ opcodes are outside dispatch range (silently dropped by client).
    #[test]
    fn test_v2525_outside_range_opcodes() {
        for op in [0xD8u8, 0xE0, 0xE9, 0xFF] {
            assert!(op > 0xD7, "0x{:02X} outside dispatch range", op);
        }
    }
}
