//! WIZ_EXP_SEAL (0x9A) handler — experience seal toggle & potion system.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/UserLevelExperienceSystem.cpp:319-441`
//!
//! ## Overview
//!
//! The EXP seal system allows players to accumulate XP into a "sealed pool"
//! rather than gaining it normally. The sealed XP can then be converted into
//! tradeable items (sealed jars). The seal auto-disables once 1 billion XP
//! is accumulated.
//!
//! ## Sub-opcodes
//!
//! - 1 = Enable seal (SealExp_ON)
//! - 2 = Disable seal (SealExp_OFF)
//! - 3 = Sealed potion (convert sealed XP to item)
//! - 4 = Server→client sealed XP update (not from client)
//!
//! ## Packet Formats
//!
//! ### Client → Server
//!
//! Sub-opcode 1/2 (toggle): `[u8 sub_opcode]`
//! Sub-opcode 3 (potion): `[u8 sub_opcode=3] [u32 jar_item_id] [u8 inv_pos] [u64 sealed_exp]`
//!
//! ### Server → Client
//!
//! Status response: `[u8 sub_opcode(1=on,2=off)] [u8 result=1]`
//! Potion response: `[u8 0x03] [u32 jar_item_id] [u8 pos] [u32 sealed_exp_lo] [u32 sealed_exp_hi]`
//! XP update:       `[u8 0x04] [u32 sealed_exp] [u32 0]`

use ko_protocol::{Opcode, Packet, PacketReader};

use crate::session::{ClientSession, SessionState};
use crate::world::WorldState;
use crate::zone::SessionId;

/// Maximum sealed XP cap (1 billion).
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:418,426` — `1000000000`
const MAX_SEALED_EXP: u32 = 1_000_000_000;

/// Item ID for the empty sealed jar (consumed to fill).
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:364` — `#define SEALED_JAR 810354000`
const SEALED_JAR: u32 = 810_354_000;

/// Item ID for 100M sealed XP jar.
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:367` — `#define SEALED_JAR_100M 810357000`
const SEALED_JAR_100M: u32 = 810_357_000;

/// Item ID for 500M sealed XP jar.
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:365` — `#define SEALED_JAR_500M 810355000`
const SEALED_JAR_500M: u32 = 810_355_000;

/// Item ID for 1B sealed XP jar.
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:366` — `#define SEALED_JAR_1B 810356000`
const SEALED_JAR_1B: u32 = 810_356_000;

use super::SLOT_MAX;

/// Handle WIZ_EXP_SEAL from the client.
///
/// C++ Reference: `CUser::ExpSealHandler()` in `UserLevelExperienceSystem.cpp:320-332`
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = reader.read_u8().unwrap_or(0);

    match sub_opcode {
        1 => {
            // SealExp_ON
            exp_seal_update_status(session, true).await?;
        }
        2 => {
            // SealExp_OFF
            exp_seal_update_status(session, false).await?;
        }
        3 => {
            // Sealed potion — convert sealed XP into an item
            exp_seal_sealed_potion(session, &mut reader).await?;
        }
        _ => {
            tracing::warn!(
                "[{}] Unknown exp_seal sub-opcode: {}",
                session.addr(),
                sub_opcode
            );
        }
    }

    Ok(())
}

/// Toggle the EXP seal on or off.
///
/// C++ Reference: `CUser::ExpSealUpdateStatus()` in `UserLevelExperienceSystem.cpp:336-343`
///
/// Packet response: `[u8 sub_opcode(1=on, 2=off)] [u8 result=1]`
async fn exp_seal_update_status(session: &mut ClientSession, enable: bool) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    world.update_character_stats(sid, |ch| {
        ch.exp_seal_status = enable;
    });

    let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
    pkt.write_u8(if enable { 1 } else { 2 });
    pkt.write_u8(1); // success
    session.send_packet(&pkt).await?;

    tracing::debug!(
        "[{}] ExpSeal status changed: {}",
        session.addr(),
        if enable { "ON" } else { "OFF" }
    );

    Ok(())
}

/// Convert sealed XP into a tradeable sealed jar item.
///
/// C++ Reference: `CUser::ExpSealSealedPotion()` in `UserLevelExperienceSystem.cpp:369-412`
///
/// Client sends: `[u32 jar_item_id] [u8 inv_pos] [u64 sealed_exp]`
/// Server responds: `[u8 0x03] [u32 result_jar_id] [u8 pos] [u32 sealed_exp_lo] [u32 0]`
///
/// The player must have a SEALED_JAR item at the given inventory position.
/// The sealed XP amount must be 100M, 500M, or 1B, and must not exceed
/// the accumulated sealed_exp. The jar is consumed and replaced with the
/// appropriate sealed XP jar item.
async fn exp_seal_sealed_potion(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let jar_item_id = reader.read_u32().unwrap_or(0);
    let inv_pos = reader.read_u8().unwrap_or(0);
    let sealed_exp = reader.read_u64().unwrap_or(0);

    let world = session.world().clone();
    let sid = session.session_id();

    // Must be the SEALED_JAR item
    // C++ Reference: line 378 — `if (JarItemId != SEALED_JAR) goto fail_return;`
    if jar_item_id != SEALED_JAR {
        send_potion_fail(session).await?;
        return Ok(());
    }

    // Validate the item is actually in the inventory at SLOT_MAX + pos
    // C++ Reference: line 381-384
    let abs_pos = SLOT_MAX + inv_pos as usize;
    let inv = world.get_inventory(sid);
    let inv_valid = inv
        .get(abs_pos)
        .map(|slot| slot.item_id == SEALED_JAR)
        .unwrap_or(false);

    if !inv_valid {
        send_potion_fail(session).await?;
        return Ok(());
    }

    // Check that sealed_exp does not exceed accumulated total
    // C++ Reference: line 386 — `if (SealedExp > m_iSealedExp) goto fail_return;`
    let current_sealed = world
        .get_character_info(sid)
        .map(|ch| ch.sealed_exp)
        .unwrap_or(0);
    if sealed_exp > current_sealed as u64 {
        send_potion_fail(session).await?;
        return Ok(());
    }

    // Determine result jar item ID based on sealed exp amount
    // C++ Reference: line 392-399
    let result_item_id = match sealed_exp {
        100_000_000 => SEALED_JAR_100M,
        500_000_000 => SEALED_JAR_500M,
        1_000_000_000 => SEALED_JAR_1B,
        _ => {
            send_potion_fail(session).await?;
            return Ok(());
        }
    };

    // Check the result item exists in the item table
    // C++ Reference: line 401-403
    if world.get_item(result_item_id).is_none() {
        send_potion_fail(session).await?;
        return Ok(());
    }

    // Remove the empty jar from inventory
    // C++ Reference: line 389-390 — `memset(pItem, 0x00, sizeof(_ITEM_DATA));`
    world.update_inventory(sid, |inv| {
        if let Some(slot) = inv.get_mut(abs_pos) {
            *slot = crate::world::UserItemSlot::default();
        }
        true
    });

    // Give the filled jar
    // C++ Reference: line 405 — `GiveItem("Seal Experience Item", JarItemId, 1, true);`
    let gave = world.give_item(sid, result_item_id, 1);
    if !gave {
        tracing::warn!(
            "[sid={}] ExpSeal: failed to give sealed jar {}",
            sid,
            result_item_id
        );
        send_potion_fail(session).await?;
        return Ok(());
    }

    // Deduct sealed exp
    // C++ Reference: line 406 — `m_iSealedExp -= (uint32) SealedExp;`
    world.update_character_stats(sid, |ch| {
        ch.sealed_exp = ch.sealed_exp.saturating_sub(sealed_exp as u32);
    });

    // Send sealed XP update
    // C++ Reference: line 407 — `ExpSealChangeExp(0);`
    send_sealed_exp_update(&world, sid);

    // Build success response
    // C++ Reference: line 409 — `result << JarItemId << pos << uint32(SealedExp) << uint32(0);`
    // Note: pos here is the position where the new item was placed.
    // We find it by scanning inventory for the result item.
    let inv = world.get_inventory(sid);
    let new_pos = inv
        .iter()
        .enumerate()
        .skip(SLOT_MAX)
        .find(|(_, slot)| slot.item_id == result_item_id)
        .map(|(i, _)| (i - SLOT_MAX) as u8)
        .unwrap_or(0);

    let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
    pkt.write_u8(0x03);
    pkt.write_u32(result_item_id);
    pkt.write_u8(new_pos);
    pkt.write_u32(sealed_exp as u32);
    pkt.write_u32(0);
    session.send_packet(&pkt).await?;

    tracing::debug!(
        "[{}] ExpSeal potion: converted {} sealed XP into item {}",
        session.addr(),
        sealed_exp,
        result_item_id
    );

    Ok(())
}

/// Send a failed potion response (empty sub-opcode 3 packet).
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:410-411` — when any validation
/// fails, the function falls through to `fail_return:` and sends `result` which
/// only has the sub-opcode 0x03 and no data appended.
async fn send_potion_fail(session: &mut ClientSession) -> anyhow::Result<()> {
    let pkt = Packet::new(Opcode::WizExpSeal as u8);
    // Packet with just the opcode, no data — matching C++ fail_return behavior
    // Actually C++ initializes with `Packet result(WIZ_EXP_SEAL, uint8(0x03));`
    // so the fail packet contains the sub-opcode 0x03
    let mut pkt = pkt;
    pkt.write_u8(0x03);
    session.send_packet(&pkt).await?;
    Ok(())
}

/// Accumulate XP into the sealed pool, or overflow to real XP if at cap.
///
/// C++ Reference: `CUser::ExpSealChangeExp()` in `UserLevelExperienceSystem.cpp:416-440`
///
/// Called from `exp_change_inner()` when `exp_seal_status` is true.
///
/// If sealed_exp >= 1B and amount > 0: auto-disable seal and give as real XP.
/// Otherwise: add to sealed_exp (capped at 1B), then send update packet.
pub async fn exp_seal_change_exp(world: &WorldState, sid: SessionId, amount: u64) {
    let ch = match world.get_character_info(sid) {
        Some(c) => c,
        None => return,
    };

    // If already at cap and gaining positive XP, disable seal and pass through
    // C++ Reference: line 418-423
    if ch.sealed_exp >= MAX_SEALED_EXP && amount > 0 {
        // Disable seal
        world.update_character_stats(sid, |ch| {
            ch.exp_seal_status = false;
        });

        // Send seal-off notification
        let mut off_pkt = Packet::new(Opcode::WizExpSeal as u8);
        off_pkt.write_u8(2); // OFF
        off_pkt.write_u8(1);
        world.send_to_session_owned(sid, off_pkt);

        // Apply as normal XP via ExpChange (boxed to break recursion cycle)
        // C++ Reference: line 421 — `ExpChange("exp seal", amount);`
        Box::pin(crate::handler::level::exp_change(world, sid, amount as i64)).await;
        return;
    }

    // Accumulate sealed XP
    // C++ Reference: line 425-429
    let new_sealed = ch
        .sealed_exp
        .saturating_add(amount as u32)
        .min(MAX_SEALED_EXP);
    world.update_character_stats(sid, |ch| {
        ch.sealed_exp = new_sealed;
    });

    // Send sealed XP update packet
    // C++ Reference: line 432-434
    send_sealed_exp_update(world, sid);

    // Also send WIZ_EXP_CHANGE so client updates XP bar
    // C++ Reference: line 436-439
    let ch = match world.get_character_info(sid) {
        Some(c) => c,
        None => return,
    };
    let mut exp_pkt = Packet::new(Opcode::WizExpChange as u8);
    exp_pkt.write_u8(1); // flag 1 = exp seal update
    exp_pkt.write_i64(ch.exp as i64);
    world.send_to_session_owned(sid, exp_pkt);
}

/// Send the sealed XP update packet to the player.
///
/// C++ Reference: `UserLevelExperienceSystem.cpp:432-434`
///
/// Packet: `WIZ_EXP_SEAL [u8 0x04] [u32 sealed_exp] [u32 0]`
fn send_sealed_exp_update(world: &WorldState, sid: SessionId) {
    let sealed_exp = world
        .get_character_info(sid)
        .map(|ch| ch.sealed_exp)
        .unwrap_or(0);

    let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
    pkt.write_u8(0x04);
    pkt.write_u32(sealed_exp);
    pkt.write_u32(0);
    world.send_to_session_owned(sid, pkt);
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, Packet, PacketReader};

    use super::*;

    // ── Packet format tests ─────────────────────────────────────────

    #[test]
    fn test_seal_on_packet_format() {
        // Client request: [u8 sub=1]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(1);

        assert_eq!(pkt.opcode, Opcode::WizExpSeal as u8);
        assert_eq!(pkt.data.len(), 1);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_seal_off_packet_format() {
        // Client request: [u8 sub=2]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(2);

        assert_eq!(pkt.opcode, Opcode::WizExpSeal as u8);
        assert_eq!(pkt.data.len(), 1);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_seal_status_response_on() {
        // Server response for ON: [u8 1] [u8 1]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(1); // ON
        pkt.write_u8(1); // success

        assert_eq!(pkt.data.len(), 2);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_seal_status_response_off() {
        // Server response for OFF: [u8 2] [u8 1]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(2); // OFF
        pkt.write_u8(1); // success

        assert_eq!(pkt.data.len(), 2);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_sealed_potion_request_format() {
        // Client request: [u8 sub=3] [u32 jar_id] [u8 pos] [u64 exp]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(3);
        pkt.write_u32(SEALED_JAR);
        pkt.write_u8(5); // inventory position
        pkt.write_u64(500_000_000);

        // 1 + 4 + 1 + 8 = 14 bytes
        assert_eq!(pkt.data.len(), 14);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(3));
        assert_eq!(r.read_u32(), Some(SEALED_JAR));
        assert_eq!(r.read_u8(), Some(5));
        assert_eq!(r.read_u64(), Some(500_000_000));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_sealed_potion_response_format() {
        // Server response: [u8 0x03] [u32 jar_id] [u8 pos] [u32 exp_lo] [u32 exp_hi]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(0x03);
        pkt.write_u32(SEALED_JAR_500M);
        pkt.write_u8(5);
        pkt.write_u32(500_000_000);
        pkt.write_u32(0);

        // 1 + 4 + 1 + 4 + 4 = 14 bytes
        assert_eq!(pkt.data.len(), 14);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x03));
        assert_eq!(r.read_u32(), Some(SEALED_JAR_500M));
        assert_eq!(r.read_u8(), Some(5));
        assert_eq!(r.read_u32(), Some(500_000_000));
        assert_eq!(r.read_u32(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_sealed_exp_update_packet_format() {
        // Server → client: [u8 0x04] [u32 sealed_exp] [u32 0]
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(0x04);
        pkt.write_u32(750_000_000);
        pkt.write_u32(0);

        // 1 + 4 + 4 = 9 bytes
        assert_eq!(pkt.data.len(), 9);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x04));
        assert_eq!(r.read_u32(), Some(750_000_000));
        assert_eq!(r.read_u32(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_potion_fail_packet_format() {
        // Fail response: just [u8 0x03] with no additional data
        let mut pkt = Packet::new(Opcode::WizExpSeal as u8);
        pkt.write_u8(0x03);

        assert_eq!(pkt.data.len(), 1);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x03));
        assert_eq!(r.remaining(), 0);
    }

    // ── Constant validation tests ───────────────────────────────────

    #[test]
    fn test_sealed_jar_constants() {
        assert_eq!(SEALED_JAR, 810_354_000);
        assert_eq!(SEALED_JAR_100M, 810_357_000);
        assert_eq!(SEALED_JAR_500M, 810_355_000);
        assert_eq!(SEALED_JAR_1B, 810_356_000);
    }

    #[test]
    fn test_max_sealed_exp() {
        assert_eq!(MAX_SEALED_EXP, 1_000_000_000);
    }

    #[test]
    fn test_sealed_exp_cap_logic() {
        // Verify the capping logic matches C++ behavior
        let current: u32 = 900_000_000;
        let amount: u32 = 200_000_000;
        let result = current.saturating_add(amount).min(MAX_SEALED_EXP);
        assert_eq!(result, MAX_SEALED_EXP); // capped at 1B
    }

    #[test]
    fn test_sealed_exp_normal_add() {
        let current: u32 = 100_000_000;
        let amount: u32 = 50_000_000;
        let result = current.saturating_add(amount).min(MAX_SEALED_EXP);
        assert_eq!(result, 150_000_000);
    }

    #[test]
    fn test_sealed_exp_deduction() {
        let current: u32 = 500_000_000;
        let deduct: u32 = 100_000_000;
        let result = current.saturating_sub(deduct);
        assert_eq!(result, 400_000_000);
    }

    #[test]
    fn test_exp_seal_opcode_value() {
        assert_eq!(Opcode::WizExpSeal as u8, 0x9A);
    }
}
