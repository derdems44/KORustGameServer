//! WIZ_SOUL (0xC5) handler — Soul system.
//!
//! v2525 client's native soul stat panel (panel at `[[esi+0x1C8]+0x110]`).
//! Displays soul categories with rank values and individual soul slots.
//!
//! ## Client RE
//!
//! - Panel: 2-layer pointer `[esi+0x1C8]+0x110` — non-zero when panel is open
//! - S2C handler: `0xE0AC00` — single packet, 2-phase sequential read
//! - Post-process: `0xE0C9D0` — populates 20 UI widget slots
//! - Icon path: `symbol_us\simple_rank_mark_{id}_{index}.dxt`
//!
//! ## S2C Packet Format (single packet, 2 phases)
//!
//! ```text
//! [u8 cat_count]
//!   for each category:
//!     [i16 cat_id]       — 0-7 valid
//!     [i16 value_0]      — sub-slot 0 rank value
//!     [i16 value_1]      — sub-slot 1 rank value
//!     [i16 value_2]      — sub-slot 2 rank value
//! [u8 slot_count]
//!   for each slot:
//!     [i16 raw_slot_id]  — 0-4 valid (client adds +50 internally)
//!     [i16 slot_value]   — slot stat value
//! ```
//!
//! ## Category String IDs (jump table at `0xE0B330`)
//!
//! | cat_id | String ID | Hex    |
//! |--------|-----------|--------|
//! | 0      | 43620     | 0xAA64 |
//! | 1      | 43621     | 0xAA65 |
//! | 2      | 43622     | 0xAA66 |
//! | 3      | (skip)    | —      | ← entry NOT stored in display vector
//! | 4      | 43624     | 0xAA68 |
//! | 5      | 43625     | 0xAA69 |
//! | 6      | 43626     | 0xAA6A |
//! | 7      | 43619     | 0xAA63 | ← default/fallback
//!
//! ## Slot String IDs (jump table at `0xE0B350`)
//!
//! | raw_slot_id | internal | String ID | Hex    |
//! |-------------|----------|-----------|--------|
//! | 0           | 50       | 43627     | 0xAA6B |
//! | 1           | 51       | 43628     | 0xAA6C |
//! | 2           | 52       | 43629     | 0xAA6D |
//! | 3           | 53       | 43630     | 0xAA6E |
//! | 4           | 54       | 43631     | 0xAA6F |
//!
//! ## C2S Packets
//!
//! - Init request: empty body (just opcode 0xC5) — sent on panel open
//! - Button click: `[i32 button_data]` from widget at `[esi+0x128]`

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── Constants ───────────────────────────────────────────────────────────

/// Maximum category ID (0-7).
#[cfg(test)]
const MAX_CAT_ID: i16 = 7;

/// Maximum raw slot ID (0-4).
#[cfg(test)]
const MAX_SLOT_ID: i16 = 4;

/// Internal slot ID offset (client adds +50 to raw_slot_id).
#[cfg(test)]
const SLOT_ID_OFFSET: i16 = 50;

// ── Data types ──────────────────────────────────────────────────────────

/// A soul category with 3 sub-slot rank values.
#[derive(Debug, Clone, Copy)]
pub struct SoulCategory {
    /// Category type ID (0-7).
    pub cat_id: i16,
    /// Rank value for sub-slot 0.
    pub value_0: i16,
    /// Rank value for sub-slot 1.
    pub value_1: i16,
    /// Rank value for sub-slot 2.
    pub value_2: i16,
}

/// A soul slot with a stat value.
#[derive(Debug, Clone, Copy)]
pub struct SoulSlot {
    /// Raw slot ID (0-4, client adds +50 internally).
    pub slot_id: i16,
    /// Slot stat value.
    pub value: i16,
}

// ── S2C Builders ────────────────────────────────────────────────────────

/// Build an empty soul data packet (no categories, no slots).
///
/// Wire: `[u8 cat_count=0][u8 slot_count=0]`
pub fn build_empty() -> Packet {
    let mut pkt = Packet::new(Opcode::WizSoul as u8);
    pkt.write_u8(0); // category count
    pkt.write_u8(0); // slot count
    pkt
}

/// Build a full soul data packet with categories and slots.
///
/// Wire: `[u8 cat_count][{i16 cat_id, i16 v0, i16 v1, i16 v2} × cat_count]`
///       `[u8 slot_count][{i16 slot_id, i16 value} × slot_count]`
pub fn build_full(categories: &[SoulCategory], slots: &[SoulSlot]) -> Packet {
    let mut pkt = Packet::new(Opcode::WizSoul as u8);

    // Phase 1: Categories
    pkt.write_u8(categories.len().min(255) as u8);
    for cat in categories.iter().take(255) {
        pkt.write_i16(cat.cat_id);
        pkt.write_i16(cat.value_0);
        pkt.write_i16(cat.value_1);
        pkt.write_i16(cat.value_2);
    }

    // Phase 2: Slots
    pkt.write_u8(slots.len().min(255) as u8);
    for slot in slots.iter().take(255) {
        pkt.write_i16(slot.slot_id);
        pkt.write_i16(slot.value);
    }

    pkt
}

/// Build a soul data packet with only categories (no slots).
pub fn build_categories_only(categories: &[SoulCategory]) -> Packet {
    build_full(categories, &[])
}

/// Build a soul data packet with only slots (no categories).
pub fn build_slots_only(slots: &[SoulSlot]) -> Packet {
    build_full(&[], slots)
}

// ── C2S Handler ─────────────────────────────────────────────────────────

/// Handle WIZ_SOUL (0xC5) from the client.
///
/// C2S packets:
/// - Empty body: init request (panel opened, client wants soul data)
/// - `[i32 button_data]`: button click interaction
///
/// Sends soul data from session (loaded at gamestart from DB).
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let reader = PacketReader::new(&pkt.data);

    if reader.remaining() == 0 {
        // Init request — send soul data from session
        debug!("[{}] WIZ_SOUL init request", session.addr());
        let world = session.world();
        let sid = session.session_id();
        let soul_data =
            world.with_session(sid, |h| (h.soul_categories, h.soul_slots, h.soul_loaded));
        let pkt = match soul_data {
            Some((cats, slots, true)) => {
                let categories: Vec<SoulCategory> = cats
                    .iter()
                    .filter(|c| c[1] != 0 || c[2] != 0 || c[3] != 0)
                    .map(|c| SoulCategory {
                        cat_id: c[0],
                        value_0: c[1],
                        value_1: c[2],
                        value_2: c[3],
                    })
                    .collect();
                let soul_slots: Vec<SoulSlot> = slots
                    .iter()
                    .filter(|s| s[1] != 0)
                    .map(|s| SoulSlot {
                        slot_id: s[0],
                        value: s[1],
                    })
                    .collect();
                if categories.is_empty() && soul_slots.is_empty() {
                    build_empty()
                } else {
                    build_full(&categories, &soul_slots)
                }
            }
            _ => build_empty(),
        };
        session.send_packet(&pkt).await?;
    } else {
        // Button click or unknown
        debug!(
            "[{}] WIZ_SOUL interaction ({}B payload)",
            session.addr(),
            reader.remaining()
        );
    }

    Ok(())
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    #[test]
    fn test_build_empty_format() {
        let pkt = build_empty();
        assert_eq!(pkt.opcode, Opcode::WizSoul as u8);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0)); // cat_count
        assert_eq!(r.read_u8(), Some(0)); // slot_count
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_empty_length() {
        assert_eq!(build_empty().data.len(), 2);
    }

    #[test]
    fn test_build_full_one_cat_one_slot() {
        let cats = [SoulCategory {
            cat_id: 0,
            value_0: 10,
            value_1: 20,
            value_2: 30,
        }];
        let slots = [SoulSlot {
            slot_id: 0,
            value: 100,
        }];
        let pkt = build_full(&cats, &slots);

        let mut r = PacketReader::new(&pkt.data);
        // Phase 1
        assert_eq!(r.read_u8(), Some(1)); // 1 category
        assert_eq!(r.read_i16(), Some(0)); // cat_id
        assert_eq!(r.read_i16(), Some(10));
        assert_eq!(r.read_i16(), Some(20));
        assert_eq!(r.read_i16(), Some(30));
        // Phase 2
        assert_eq!(r.read_u8(), Some(1)); // 1 slot
        assert_eq!(r.read_i16(), Some(0)); // slot_id
        assert_eq!(r.read_i16(), Some(100));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_full_length() {
        let cats = [
            SoulCategory {
                cat_id: 0,
                value_0: 1,
                value_1: 2,
                value_2: 3,
            },
            SoulCategory {
                cat_id: 1,
                value_0: 4,
                value_1: 5,
                value_2: 6,
            },
        ];
        let slots = [
            SoulSlot {
                slot_id: 0,
                value: 10,
            },
            SoulSlot {
                slot_id: 1,
                value: 20,
            },
            SoulSlot {
                slot_id: 2,
                value: 30,
            },
        ];
        let pkt = build_full(&cats, &slots);
        // u8 + 2*(i16*4) + u8 + 3*(i16*2) = 1 + 16 + 1 + 12 = 30
        assert_eq!(pkt.data.len(), 30);
    }

    #[test]
    fn test_build_categories_only() {
        let cats = [SoulCategory {
            cat_id: 5,
            value_0: 1,
            value_1: 2,
            value_2: 3,
        }];
        let pkt = build_categories_only(&cats);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // 1 cat
        assert_eq!(r.read_i16(), Some(5));
        assert_eq!(r.read_i16(), Some(1));
        assert_eq!(r.read_i16(), Some(2));
        assert_eq!(r.read_i16(), Some(3));
        assert_eq!(r.read_u8(), Some(0)); // 0 slots
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_slots_only() {
        let slots = [SoulSlot {
            slot_id: 4,
            value: 999,
        }];
        let pkt = build_slots_only(&slots);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0)); // 0 cats
        assert_eq!(r.read_u8(), Some(1)); // 1 slot
        assert_eq!(r.read_i16(), Some(4));
        assert_eq!(r.read_i16(), Some(999));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_full_multiple_categories() {
        let cats: Vec<SoulCategory> = (0..=MAX_CAT_ID)
            .map(|id| SoulCategory {
                cat_id: id,
                value_0: id * 10,
                value_1: id * 20,
                value_2: id * 30,
            })
            .collect();
        let pkt = build_full(&cats, &[]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(8)); // 8 categories (0-7)
        for id in 0..=MAX_CAT_ID {
            assert_eq!(r.read_i16(), Some(id));
            assert_eq!(r.read_i16(), Some(id * 10));
            assert_eq!(r.read_i16(), Some(id * 20));
            assert_eq!(r.read_i16(), Some(id * 30));
        }
        assert_eq!(r.read_u8(), Some(0)); // 0 slots
    }

    #[test]
    fn test_build_full_all_slots() {
        let slots: Vec<SoulSlot> = (0..=MAX_SLOT_ID)
            .map(|id| SoulSlot {
                slot_id: id,
                value: (id + 1) * 100,
            })
            .collect();
        let pkt = build_full(&[], &slots);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0)); // 0 cats
        assert_eq!(r.read_u8(), Some(5)); // 5 slots (0-4)
        for id in 0..=MAX_SLOT_ID {
            assert_eq!(r.read_i16(), Some(id));
            assert_eq!(r.read_i16(), Some((id + 1) * 100));
        }
    }

    #[test]
    fn test_negative_values() {
        let cats = [SoulCategory {
            cat_id: 0,
            value_0: -1,
            value_1: -100,
            value_2: i16::MIN,
        }];
        let slots = [SoulSlot {
            slot_id: 0,
            value: -50,
        }];
        let pkt = build_full(&cats, &slots);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // cat_count
        assert_eq!(r.read_i16(), Some(0));
        assert_eq!(r.read_i16(), Some(-1));
        assert_eq!(r.read_i16(), Some(-100));
        assert_eq!(r.read_i16(), Some(i16::MIN));
        r.read_u8(); // slot_count
        assert_eq!(r.read_i16(), Some(0));
        assert_eq!(r.read_i16(), Some(-50));
    }

    #[test]
    fn test_slot_id_offset_constant() {
        assert_eq!(SLOT_ID_OFFSET, 50);
    }

    #[test]
    fn test_cat_id_range() {
        assert_eq!(MAX_CAT_ID, 7);
    }

    #[test]
    fn test_slot_id_range() {
        assert_eq!(MAX_SLOT_ID, 4);
    }

    #[test]
    fn test_empty_matches_v2525_builder() {
        // Verify our build_empty matches the original v2525 build_soul_empty_packet
        let pkt = build_empty();
        assert_eq!(pkt.data.len(), 2);
        assert_eq!(pkt.data[0], 0); // cat_count
        assert_eq!(pkt.data[1], 0); // slot_count
    }
}
