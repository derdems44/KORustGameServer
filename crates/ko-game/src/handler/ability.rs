//! WIZ_ABILITY (0xCF) handler — Hermetic Seal ability system.
//!
//! v2525 client's native "Hermetic Seal" panel (panel at `[esi+0x694]`).
//! 24-slot circular wheel, 9 upgrade levels (stars), 2-hour progress timer.
//!
//! ## Client RE
//!
//! - Panel object: `[esi+0x694]` — created on UI open, null-checked before dispatch
//! - Main handler: `0xA96910` — sub-opcode chain (sub 1-6)
//! - Class name: `CUIItemHermeticSeal` (debug string at `0xFB78F4`)
//! - Assets: `ui/Belong_icon.dxt`, `ui/HermeticSeal_icon.dxt`
//! - C2S: sub=1 (open), sub=2 (select slot), sub=3 (upgrade), sub=4 (progress tick), sub=5 (confirm)
//! - S2C: sub=1 (init), sub=2 (result), sub=3 (level up), sub=4 (error detail),
//!   sub=5 (item result), sub=6 (slot update)
//!
//! ## Constants
//!
//! - 24 slots in circular wheel (client-side item definitions from TBL)
//! - 9 levels per slot (gold stars 0xFFE2C477, grey 0xFF808080)
//! - 7200.0 seconds (2 hours) total progress time
//! - Progress tick every 2 seconds (C2S sub=4 auto-sent)
//! - Max tier: 9 (client caps at 9 even if server sends 10+)

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

// ── S2C Sub-type constants ────────────────────────────────────────────────

/// Sub 1: Full state init — panel open.
const ABILITY_SUB_INIT: u8 = 1;

/// Sub 2: Result/status notification.
const ABILITY_SUB_RESULT: u8 = 2;

/// Sub 3: Level upgrade result.
const ABILITY_SUB_LEVEL_UP: u8 = 3;

/// Sub 4: Error with detail code.
#[cfg(test)]
const ABILITY_SUB_ERROR_DETAIL: u8 = 4;

/// Sub 5: Upgrade item result.
const ABILITY_SUB_ITEM_RESULT: u8 = 5;

/// Sub 6: Slot selection update.
const ABILITY_SUB_SLOT_UPDATE: u8 = 6;

// ── Result codes ──────────────────────────────────────────────────────────

/// Sub 2 result: Close/cancel — string 0x8FD (2301).
const RESULT_CLOSE: u8 = 2;

/// Sub 2 result: Success — string 0xABB0 (43952).
const RESULT_SUCCESS: u8 = 3;

/// Sub 3 result: Level upgrade succeeded — increment star.
const LEVEL_UP_SUCCESS: u8 = 1;

/// Sub 3 result: Level upgrade failed — string 0xABBC (43964).
#[cfg(test)]
const LEVEL_UP_FAILED: u8 = 2;

/// Total progress time in seconds (2 hours).
const TOTAL_PROGRESS_TIME: f64 = 7200.0;

/// Maximum ability slots.
const MAX_SLOTS: u8 = 24;

/// Maximum level per slot (9 stars).
const MAX_LEVEL: u8 = 9;

// ── S2C Packet Builders ──────────────────────────────────────────────────

/// Build a Sub 1 (full state init) packet.
///
/// Client RE: `0xA969A4` — initializes panel with tier/slot/level/timer data.
/// First byte after sub MUST be 0x01 (verify).
///
/// Wire: `[0xCF][0x01][u8 verify=1][u8 max_tier][u8 selected_slot]
///        [u8 status][u8 upgrade_count][u8 current_level][f64 elapsed_time]`
fn build_init(
    max_tier: u8,
    selected_slot: u8,
    status: u8,
    upgrade_count: u8,
    current_level: u8,
    elapsed_time: f64,
) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_INIT);
    pkt.write_u8(1); // verify byte (must be 1)
    pkt.write_u8(max_tier.min(MAX_LEVEL));
    pkt.write_u8(selected_slot % MAX_SLOTS);
    pkt.write_u8(status);
    pkt.write_u8(upgrade_count);
    pkt.write_u8(current_level.min(MAX_LEVEL));
    // f64 as raw 8 bytes (little-endian)
    let time_bytes = elapsed_time.to_le_bytes();
    for &b in &time_bytes {
        pkt.write_u8(b);
    }
    pkt
}

/// Build a Sub 1 (init) for empty/default state — no active seal.
///
/// Sends max_tier=0, slot=0, status=1 (paused), level=0, elapsed=0.0.
fn build_init_empty() -> Packet {
    build_init(0, 0, 1, 0, 0, 0.0)
}

/// Build a Sub 2 (result/status notification) packet.
///
/// Client RE: `0xA96E88` — result dispatch:
/// 2=close, 3=success, 5=special error, other=generic error.
///
/// Wire: `[0xCF][0x02][u8 result]`
fn build_result(result: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_RESULT);
    pkt.write_u8(result);
    pkt
}

/// Build a Sub 3 (level upgrade result) packet.
///
/// Client RE: `0xA96EEE` — 1=success (increment star), 2=fail, other=error.
///
/// Wire: `[0xCF][0x03][u8 result]`
fn build_level_up_result(result: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_LEVEL_UP);
    pkt.write_u8(result);
    pkt
}

/// Build a Sub 5 (upgrade item result — close panel) packet.
///
/// Client RE: `0xA97120` — 1=success+item, 2=close, 3=success+effect.
///
/// Wire: `[0xCF][0x05][u8 result=2]`
fn build_item_result_close() -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_ITEM_RESULT);
    pkt.write_u8(2); // ITEM_RESULT_CLOSE
    pkt
}

/// Build a Sub 4 (error with detail) packet (test-only).
#[cfg(test)]
fn build_error_detail(error_code: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(4); // ABILITY_SUB_ERROR_DETAIL
    pkt.write_u8(error_code);
    pkt
}

/// Build a Sub 5 (upgrade item result) with item data (test-only).
#[cfg(test)]
fn build_item_result_success(item_id: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_ITEM_RESULT);
    pkt.write_u8(1); // ITEM_RESULT_SUCCESS
    pkt.write_i32(item_id);
    pkt.write_u16(0);
    pkt
}

/// Build a Sub 6 (slot selection update) packet.
///
/// Client RE: `0xA973D0` — updates selected slot with rotation animation.
/// First byte after sub MUST be 0x01 (verify).
///
/// Wire: `[0xCF][0x06][u8 verify=1][u8 ignored][u8 max_tier][u8 selected_slot]
///        [u8 status][u8 upgrade_count][u8 field3][u8 field0][u8 field1][u8 field2]`
fn build_slot_update(max_tier: u8, selected_slot: u8, status: u8, upgrade_count: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizAbility as u8);
    pkt.write_u8(ABILITY_SUB_SLOT_UPDATE);
    pkt.write_u8(1); // verify
    pkt.write_u8(0); // ignored
    pkt.write_u8(max_tier.min(MAX_LEVEL));
    pkt.write_u8(selected_slot % MAX_SLOTS);
    pkt.write_u8(status);
    pkt.write_u8(upgrade_count);
    pkt.write_u8(0); // field3
    pkt.write_u8(0); // field0
    pkt.write_u8(0); // field1
    pkt.write_u8(0); // field2
    pkt
}

// ── C2S Handler ──────────────────────────────────────────────────────────

/// Handle WIZ_ABILITY (0xCF) from the client.
///
/// C2S sub-opcodes:
/// - sub=1: Open panel request (no payload)
/// - sub=2: Select slot `[u8 slot_index]`
/// - sub=3: Upgrade/confirm (no payload)
/// - sub=4: Progress tick `[u8 current_step]` (auto-sent every 2s)
/// - sub=5: Confirm action (no payload)
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub = reader.read_u8().unwrap_or(0);

    match sub {
        1 => handle_open(session).await,
        2 => handle_select_slot(session, &mut reader).await,
        3 => handle_upgrade(session).await,
        4 => handle_progress_tick(session, &mut reader),
        5 => handle_confirm(session).await,
        _ => {
            debug!(
                "[{}] WIZ_ABILITY unknown C2S sub={} ({}B)",
                session.addr(),
                sub,
                reader.remaining()
            );
            Ok(())
        }
    }
}

/// Handle C2S sub=1: Open panel — send current seal state from session.
async fn handle_open(session: &mut ClientSession) -> anyhow::Result<()> {
    debug!("[{}] WIZ_ABILITY sub=1 open panel", session.addr());

    let seal = session
        .world()
        .with_session(session.session_id(), |h| {
            if h.seal_loaded {
                Some((
                    h.seal_max_tier,
                    h.seal_selected_slot,
                    h.seal_status,
                    h.seal_upgrade_count,
                    h.seal_current_level,
                    h.seal_elapsed_time,
                ))
            } else {
                None
            }
        })
        .flatten();

    let pkt = if let Some((max_tier, slot, status, upgrade_count, level, elapsed)) = seal {
        build_init(max_tier, slot, status, upgrade_count, level, elapsed)
    } else {
        build_init_empty()
    };
    session.send_packet(&pkt).await
}

/// Handle C2S sub=2: Select a seal slot.
///
/// Wire: `[u8 slot_index]` (0-23).
/// Persists the selection to session and fires a DB save.
async fn handle_select_slot(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let slot = reader.read_u8().unwrap_or(0);
    if slot >= MAX_SLOTS {
        return session.send_packet(&build_result(RESULT_CLOSE)).await;
    }

    debug!(
        "[{}] WIZ_ABILITY sub=2 select slot={}",
        session.addr(),
        slot,
    );

    // Update session
    let world = session.world();
    let sid = session.session_id();
    world.update_session(sid, |h| {
        h.seal_selected_slot = slot;
    });

    // Fire-and-forget DB save
    if let Some(char_id) = session.character_id() {
        let pool = session.pool().clone();
        let name = char_id.to_string();
        tokio::spawn(async move {
            let repo = ko_db::repositories::hermetic_seal::HermeticSealRepository::new(&pool);
            if let Err(e) = repo.update_selected_slot(&name, slot as i16).await {
                tracing::warn!("Failed to save seal slot for {}: {}", name, e);
            }
        });
    }

    // Send slot update to client
    let state = world.with_session(sid, |h| {
        (h.seal_max_tier, h.seal_status, h.seal_upgrade_count)
    });
    if let Some((max_tier, status, upgrade_count)) = state {
        let pkt = build_slot_update(max_tier, slot, status, upgrade_count);
        session.send_packet(&pkt).await
    } else {
        session.send_packet(&build_result(RESULT_CLOSE)).await
    }
}

/// Handle C2S sub=3: Upgrade request.
///
/// No payload. Player clicks upgrade button.
/// Upgrade requires completed timer (7200s elapsed) and level < 9.
async fn handle_upgrade(session: &mut ClientSession) -> anyhow::Result<()> {
    debug!("[{}] WIZ_ABILITY sub=3 upgrade request", session.addr());

    let world = session.world();
    let sid = session.session_id();

    let state = world
        .with_session(sid, |h| {
            if h.seal_loaded {
                Some((h.seal_current_level, h.seal_elapsed_time, h.seal_max_tier))
            } else {
                None
            }
        })
        .flatten();

    let Some((current_level, elapsed, _max_tier)) = state else {
        return session.send_packet(&build_item_result_close()).await;
    };

    // Timer must be completed (≥7200s) and level < 9
    if elapsed < TOTAL_PROGRESS_TIME || current_level >= MAX_LEVEL {
        let pkt = build_result(RESULT_CLOSE);
        return session.send_packet(&pkt).await;
    }

    // Perform upgrade: increment level, reset timer, increment upgrade count
    world.update_session(sid, |h| {
        h.seal_current_level = (h.seal_current_level + 1).min(MAX_LEVEL);
        h.seal_elapsed_time = 0.0;
        h.seal_upgrade_count = h.seal_upgrade_count.saturating_add(1);
        if h.seal_current_level > h.seal_max_tier {
            h.seal_max_tier = h.seal_current_level;
        }
    });

    // Fire-and-forget DB save
    if let Some(char_id) = session.character_id() {
        let pool = session.pool().clone();
        let name = char_id.to_string();
        let new_state = world.with_session(sid, |h| {
            (
                h.seal_max_tier,
                h.seal_selected_slot,
                h.seal_status,
                h.seal_upgrade_count,
                h.seal_current_level,
                h.seal_elapsed_time,
            )
        });
        if let Some((mt, ss, st, uc, cl, et)) = new_state {
            tokio::spawn(async move {
                let repo = ko_db::repositories::hermetic_seal::HermeticSealRepository::new(&pool);
                if let Err(e) = repo
                    .save(
                        &name, mt as i16, ss as i16, st as i16, uc as i16, cl as i16, et as f32,
                    )
                    .await
                {
                    tracing::warn!("Failed to save seal upgrade for {}: {}", name, e);
                }
            });
        }
    }

    debug!(
        "[sid={}] WIZ_ABILITY upgrade: level now {}",
        sid,
        current_level + 1
    );

    let pkt = build_level_up_result(LEVEL_UP_SUCCESS);
    session.send_packet(&pkt).await
}

/// Handle C2S sub=4: Progress tick (auto-sent every 2s).
///
/// Wire: `[u8 current_step]`.
/// This is a heartbeat from the client while the panel is open.
/// Updates session elapsed_time to track progress persistence.
fn handle_progress_tick(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let step = reader.read_u8().unwrap_or(0);

    // Update elapsed time in session (2s per tick, capped at total).
    let world = session.world();
    let sid = session.session_id();
    world.update_session(sid, |h| {
        if h.seal_loaded && h.seal_status == 0 {
            // Active state — increment elapsed time by 2s per tick
            h.seal_elapsed_time = (h.seal_elapsed_time + 2.0).min(TOTAL_PROGRESS_TIME);
        }
    });

    debug!(
        "[{}] WIZ_ABILITY sub=4 progress tick step={}",
        session.addr(),
        step,
    );

    Ok(())
}

/// Handle C2S sub=5: Confirm action in dialog.
///
/// No payload. Player confirms in the upgrade confirmation dialog.
/// Activates the seal timer (sets status to active).
async fn handle_confirm(session: &mut ClientSession) -> anyhow::Result<()> {
    debug!("[{}] WIZ_ABILITY sub=5 confirm action", session.addr());

    let world = session.world();
    let sid = session.session_id();

    // Activate timer: set status to 0 (active)
    world.update_session(sid, |h| {
        if h.seal_loaded {
            h.seal_status = 0; // active
        }
    });

    let pkt = build_result(RESULT_SUCCESS);
    session.send_packet(&pkt).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::{Opcode, PacketReader};

    #[test]
    fn test_ability_opcode_value() {
        assert_eq!(Opcode::WizAbility as u8, 0xCF);
        assert_eq!(Opcode::from_byte(0xCF), Some(Opcode::WizAbility));
    }

    // ── Sub 1: Init ───────────────────────────────────────────────────

    #[test]
    fn test_build_init() {
        let pkt = build_init(3, 7, 0, 5, 4, 1800.0);
        assert_eq!(pkt.opcode, 0xCF);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_INIT)); // sub=1
        assert_eq!(r.read_u8(), Some(1)); // verify
        assert_eq!(r.read_u8(), Some(3)); // max_tier
        assert_eq!(r.read_u8(), Some(7)); // selected_slot
        assert_eq!(r.read_u8(), Some(0)); // status (active)
        assert_eq!(r.read_u8(), Some(5)); // upgrade_count
        assert_eq!(r.read_u8(), Some(4)); // current_level
                                          // f64 elapsed = 1800.0
        let mut f64_bytes = [0u8; 8];
        for b in &mut f64_bytes {
            *b = r.read_u8().unwrap();
        }
        let elapsed = f64::from_le_bytes(f64_bytes);
        assert!((elapsed - 1800.0).abs() < f64::EPSILON);
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_init_data_length() {
        // u8 sub + u8 verify + u8×5 + f64(8) = 1+1+5+8 = 15
        let pkt = build_init(0, 0, 0, 0, 0, 0.0);
        assert_eq!(pkt.data.len(), 15);
    }

    #[test]
    fn test_build_init_empty() {
        let pkt = build_init_empty();
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_INIT));
        assert_eq!(r.read_u8(), Some(1)); // verify
        assert_eq!(r.read_u8(), Some(0)); // max_tier
        assert_eq!(r.read_u8(), Some(0)); // slot
        assert_eq!(r.read_u8(), Some(1)); // status = paused
        assert_eq!(r.read_u8(), Some(0)); // upgrade_count
        assert_eq!(r.read_u8(), Some(0)); // level
    }

    #[test]
    fn test_build_init_tier_cap() {
        // max_tier capped to 9
        let pkt = build_init(15, 0, 0, 0, 0, 0.0);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub
        r.read_u8(); // verify
        assert_eq!(r.read_u8(), Some(9)); // capped
    }

    #[test]
    fn test_build_init_slot_wrap() {
        // selected_slot wraps at 24
        let pkt = build_init(0, 25, 0, 0, 0, 0.0);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub
        r.read_u8(); // verify
        r.read_u8(); // tier
        assert_eq!(r.read_u8(), Some(1)); // 25 % 24 = 1
    }

    #[test]
    fn test_build_init_level_cap() {
        // current_level capped to 9
        let pkt = build_init(0, 0, 0, 0, 12, 0.0);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // sub
        r.read_u8(); // verify
        r.read_u8(); // tier
        r.read_u8(); // slot
        r.read_u8(); // status
        r.read_u8(); // upgrade_count
        assert_eq!(r.read_u8(), Some(9)); // capped
    }

    // ── Sub 2: Result ─────────────────────────────────────────────────

    #[test]
    fn test_build_result_close() {
        let pkt = build_result(RESULT_CLOSE);
        assert_eq!(pkt.opcode, 0xCF);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_RESULT));
        assert_eq!(r.read_u8(), Some(2)); // close
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_result_success() {
        let pkt = build_result(RESULT_SUCCESS);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_RESULT));
        assert_eq!(r.read_u8(), Some(3)); // success
    }

    // ── Sub 3: Level up ───────────────────────────────────────────────

    #[test]
    fn test_build_level_up_success() {
        let pkt = build_level_up_result(LEVEL_UP_SUCCESS);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_LEVEL_UP));
        assert_eq!(r.read_u8(), Some(1)); // success
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_level_up_failed() {
        let pkt = build_level_up_result(LEVEL_UP_FAILED);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_LEVEL_UP));
        assert_eq!(r.read_u8(), Some(2)); // failed
    }

    // ── Sub 4: Error detail ───────────────────────────────────────────

    #[test]
    fn test_build_error_detail() {
        let pkt = build_error_detail(42);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_ERROR_DETAIL));
        assert_eq!(r.read_u8(), Some(42));
        assert_eq!(r.remaining(), 0);
    }

    // ── Sub 5: Item result ────────────────────────────────────────────

    #[test]
    fn test_build_item_result_close() {
        let pkt = build_item_result_close();
        assert_eq!(pkt.opcode, 0xCF);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_ITEM_RESULT));
        assert_eq!(r.read_u8(), Some(2)); // close
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_item_result_success() {
        let pkt = build_item_result_success(910252000);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_ITEM_RESULT));
        assert_eq!(r.read_u8(), Some(1)); // success
        assert_eq!(r.read_i32(), Some(910252000)); // item_id
        assert_eq!(r.read_u16(), Some(0)); // unused
        assert_eq!(r.remaining(), 0);
    }

    // ── Sub 6: Slot update ────────────────────────────────────────────

    #[test]
    fn test_build_slot_update() {
        let pkt = build_slot_update(5, 12, 0, 3);
        assert_eq!(pkt.opcode, 0xCF);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(ABILITY_SUB_SLOT_UPDATE));
        assert_eq!(r.read_u8(), Some(1)); // verify
        assert_eq!(r.read_u8(), Some(0)); // ignored
        assert_eq!(r.read_u8(), Some(5)); // max_tier
        assert_eq!(r.read_u8(), Some(12)); // selected_slot
        assert_eq!(r.read_u8(), Some(0)); // status
        assert_eq!(r.read_u8(), Some(3)); // upgrade_count
        assert_eq!(r.read_u8(), Some(0)); // field3
        assert_eq!(r.read_u8(), Some(0)); // field0
        assert_eq!(r.read_u8(), Some(0)); // field1
        assert_eq!(r.read_u8(), Some(0)); // field2
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_slot_update_data_length() {
        // u8 sub + u8 verify + u8 ignored + u8×4 params + u8×4 fields = 11
        let pkt = build_slot_update(0, 0, 0, 0);
        assert_eq!(pkt.data.len(), 11);
    }

    // ── C2S format tests ─────────────────────────────────────────────

    #[test]
    fn test_c2s_open() {
        let mut pkt = Packet::new(Opcode::WizAbility as u8);
        pkt.write_u8(1);
        assert_eq!(pkt.data.len(), 1);
    }

    #[test]
    fn test_c2s_select_slot() {
        let mut pkt = Packet::new(Opcode::WizAbility as u8);
        pkt.write_u8(2);
        pkt.write_u8(15); // slot_index

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(15));
    }

    #[test]
    fn test_c2s_upgrade() {
        let mut pkt = Packet::new(Opcode::WizAbility as u8);
        pkt.write_u8(3);
        assert_eq!(pkt.data.len(), 1);
    }

    #[test]
    fn test_c2s_progress_tick() {
        let mut pkt = Packet::new(Opcode::WizAbility as u8);
        pkt.write_u8(4);
        pkt.write_u8(7); // current_step

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(4));
        assert_eq!(r.read_u8(), Some(7));
    }

    #[test]
    fn test_c2s_confirm() {
        let mut pkt = Packet::new(Opcode::WizAbility as u8);
        pkt.write_u8(5);
        assert_eq!(pkt.data.len(), 1);
    }

    // ── Constants ─────────────────────────────────────────────────────

    #[test]
    fn test_sub_type_constants() {
        assert_eq!(ABILITY_SUB_INIT, 1);
        assert_eq!(ABILITY_SUB_RESULT, 2);
        assert_eq!(ABILITY_SUB_LEVEL_UP, 3);
        assert_eq!(ABILITY_SUB_ERROR_DETAIL, 4);
        assert_eq!(ABILITY_SUB_ITEM_RESULT, 5);
        assert_eq!(ABILITY_SUB_SLOT_UPDATE, 6);
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_SLOTS, 24);
        assert_eq!(MAX_LEVEL, 9);
        assert!((TOTAL_PROGRESS_TIME - 7200.0).abs() < f64::EPSILON);
    }
}
