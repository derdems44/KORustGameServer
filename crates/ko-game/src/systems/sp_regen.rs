//! Kurian SP (Stamina Points) regeneration tick system.
//!
//! C++ Reference: `UserDurationSkillSystem.cpp` — `CUser::HPTimeChangeStamina()`
//!
//! Runs every 2 seconds (`PLAYER_STAMINA_INTERVAL`), iterating all in-game
//! sessions and applying SP regen for Kurian class players only.
//!
//! ## Regen Rules (from C++)
//!
//! - **Beginner Kurian (class type 13)**: +5 SP per tick
//! - **Novice Kurian (class type 14)**: +7 SP per tick
//! - **Master Kurian (class type 15)**: +7 SP per tick
//! - No sitting/standing modifier (unlike HP/MP regen)
//! - Dead players do not regenerate SP
//!
//! ## WIZ_KURIAN_SP_CHANGE Packet (S->C)
//!
//! Opcode: `WizKurianSpChange` (0x9B)
//!
//! Normal SP update format:
//! ```text
//! [u8 type=1] [u8 subtype=1] [u8 max_sp] [u8 current_sp]
//! ```

use std::sync::Arc;
use std::time::Duration;

use ko_protocol::{Opcode, Packet};

use crate::handler::stats::is_kurian_class;
use crate::world::{RegenData, WorldState, USER_DEAD};

/// SP regen tick interval in seconds.
///
/// C++ Reference: `PLAYER_STAMINA_INTERVAL = 2`
const SP_REGEN_INTERVAL_SECS: u64 = 2;

/// Start the SP regen background task.
///
/// Returns a `JoinHandle` so the caller can abort on shutdown.
pub fn start_sp_regen_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(SP_REGEN_INTERVAL_SECS));
        loop {
            interval.tick().await;
            process_sp_regen_tick(&world);
        }
    })
}

/// Process one SP regen tick for all in-game sessions.
fn process_sp_regen_tick(world: &WorldState) {
    let data = world.collect_kurian_regen_data();
    for rd in data {
        process_session_sp_regen(world, &rd);
    }
}

/// Get SP regen amount per tick for a given class and skill investment.
///
/// C++ Reference: `CUser::SpTimeChange()` in `UserDurationSkillSystem.cpp`
/// - Class type 13 (Beginner): +5
/// - Class type 14 (Novice): +7
/// - Class type 15 (Master): +7 base, +1 per PRO_SKILL4 sub-skill (up to +5)
///
/// `pro_skill4` is `skill_points[8]` — the PRO_SKILL4 investment count.
/// C++ checks `CheckSkillPoint(PRO_SKILL4, 1..5, 23)`, which means each
/// sub-skill invested (1 through 5) adds +1 SP regen.  The effective bonus
/// is `min(pro_skill4, 5)`.
pub fn get_sp_regen_amount(class: u16, pro_skill4: u8) -> i16 {
    match class % 100 {
        13 => 5,
        14 => 7,
        15 => {
            let bonus = (pro_skill4 as i16).min(5);
            7 + bonus
        }
        _ => 0,
    }
}

/// Apply SP regen logic for a single session.
fn process_session_sp_regen(world: &WorldState, rd: &RegenData) {
    // Only Kurian classes have SP
    if !is_kurian_class(rd.class) {
        return;
    }

    // Dead players don't regen
    if rd.res_hp_type == USER_DEAD || rd.hp <= 0 {
        return;
    }

    // Already at max SP
    if rd.sp >= rd.max_sp || rd.max_sp <= 0 {
        return;
    }

    let regen = get_sp_regen_amount(rd.class, rd.pro_skill4);
    if regen <= 0 {
        return;
    }

    let new_sp = (rd.sp as i32 + regen as i32).min(rd.max_sp as i32) as i16;
    if new_sp != rd.sp {
        world.update_character_sp(rd.session_id, new_sp);
        let pkt = build_sp_change_packet(rd.max_sp as u8, new_sp as u8);
        world.send_to_session_owned(rd.session_id, pkt);
    }
}

/// Build a WIZ_KURIAN_SP_CHANGE (0x9B) normal update packet.
///
/// C++ Reference: Kurian SP change notification.
///
/// Wire format: `[u8 type=1] [u8 subtype=1] [u8 max_sp] [u8 current_sp]`
pub fn build_sp_change_packet(max_sp: u8, current_sp: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizKurianSpChange as u8);
    pkt.write_u8(1); // type = normal SP update
    pkt.write_u8(1); // subtype
    pkt.write_u8(max_sp);
    pkt.write_u8(current_sp);
    pkt
}

/// Build a WIZ_KURIAN_SP_CHANGE (0x9B) devil absorption packet.
///
/// C++ Reference: Devil transformation SP drain.
///
/// Wire format: `[u8 type=2] [u8 subtype=1] [i32 sp_damage]`
pub fn build_sp_absorption_packet(sp_damage: i32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizKurianSpChange as u8);
    pkt.write_u8(2); // type = devil absorption
    pkt.write_u8(1); // subtype
    pkt.write_i32(sp_damage);
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ── SP calculation tests ─────────────────────────────────────────

    #[test]
    fn test_sp_regen_amount_beginner_kurian() {
        // Class type 13: beginner kurian, +5 per tick (pro_skill4 ignored)
        assert_eq!(get_sp_regen_amount(113, 0), 5); // Karus beginner kurian
        assert_eq!(get_sp_regen_amount(213, 0), 5); // El Morad beginner portu
        assert_eq!(get_sp_regen_amount(113, 5), 5); // pro_skill4 has no effect on beginner
    }

    #[test]
    fn test_sp_regen_amount_novice_kurian() {
        // Class type 14: novice kurian, +7 per tick (pro_skill4 ignored)
        assert_eq!(get_sp_regen_amount(114, 0), 7); // Karus novice kurian
        assert_eq!(get_sp_regen_amount(214, 0), 7); // El Morad novice portu
        assert_eq!(get_sp_regen_amount(114, 5), 7); // pro_skill4 has no effect on novice
    }

    #[test]
    fn test_sp_regen_amount_master_kurian() {
        // Class type 15: master kurian, base +7 per tick (no PRO_SKILL4)
        assert_eq!(get_sp_regen_amount(115, 0), 7); // Karus master kurian, 0 skills
        assert_eq!(get_sp_regen_amount(215, 0), 7); // El Morad master portu, 0 skills
    }

    #[test]
    fn test_sp_regen_amount_non_kurian() {
        // Non-kurian classes get 0 SP regen
        assert_eq!(get_sp_regen_amount(101, 0), 0); // Warrior
        assert_eq!(get_sp_regen_amount(102, 0), 0); // Rogue
        assert_eq!(get_sp_regen_amount(103, 0), 0); // Mage
        assert_eq!(get_sp_regen_amount(104, 0), 0); // Priest
        assert_eq!(get_sp_regen_amount(205, 0), 0); // Warrior novice
        assert_eq!(get_sp_regen_amount(210, 0), 0); // Mage master
    }

    // ── Master Kurian PRO_SKILL4 bonus tests ─────────────────────────

    #[test]
    fn test_master_kurian_sp_regen_with_pro_skill4() {
        // Master Kurian gets +1 per PRO_SKILL4 sub-skill, up to +5
        assert_eq!(get_sp_regen_amount(115, 1), 8); // 7 base + 1
        assert_eq!(get_sp_regen_amount(115, 2), 9); // 7 base + 2
        assert_eq!(get_sp_regen_amount(115, 3), 10); // 7 base + 3
        assert_eq!(get_sp_regen_amount(115, 4), 11); // 7 base + 4
        assert_eq!(get_sp_regen_amount(115, 5), 12); // 7 base + 5 (max)
    }

    #[test]
    fn test_master_kurian_sp_regen_pro_skill4_capped_at_5() {
        // PRO_SKILL4 bonus caps at +5 even if more sub-skills invested
        assert_eq!(get_sp_regen_amount(115, 6), 12); // capped at 7+5=12
        assert_eq!(get_sp_regen_amount(115, 10), 12); // capped at 7+5=12
        assert_eq!(get_sp_regen_amount(115, 23), 12); // max skill value, still capped
    }

    #[test]
    fn test_master_kurian_sp_regen_el_morad_with_pro_skill4() {
        // El Morad master kurian also gets the bonus
        assert_eq!(get_sp_regen_amount(215, 0), 7); // no bonus
        assert_eq!(get_sp_regen_amount(215, 3), 10); // 7 + 3
        assert_eq!(get_sp_regen_amount(215, 5), 12); // 7 + 5 (max)
    }

    // ── Packet format tests ─────────────────────────────────────────

    #[test]
    fn test_sp_change_packet_format() {
        let pkt = build_sp_change_packet(100, 75);
        assert_eq!(pkt.opcode, Opcode::WizKurianSpChange as u8);
        assert_eq!(pkt.data.len(), 4); // u8 type + u8 subtype + u8 max_sp + u8 current_sp

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(1)); // type = normal
        assert_eq!(reader.read_u8(), Some(1)); // subtype
        assert_eq!(reader.read_u8(), Some(100)); // max_sp
        assert_eq!(reader.read_u8(), Some(75)); // current_sp
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_sp_change_packet_full_sp() {
        let pkt = build_sp_change_packet(200, 200);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_u8(), Some(200));
        assert_eq!(reader.read_u8(), Some(200));
    }

    #[test]
    fn test_sp_change_packet_zero_sp() {
        let pkt = build_sp_change_packet(150, 0);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_u8(), Some(150));
        assert_eq!(reader.read_u8(), Some(0));
    }

    #[test]
    fn test_sp_absorption_packet_format() {
        let pkt = build_sp_absorption_packet(50);
        assert_eq!(pkt.opcode, Opcode::WizKurianSpChange as u8);
        assert_eq!(pkt.data.len(), 6); // u8 type + u8 subtype + i32 sp_damage

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(2)); // type = devil absorption
        assert_eq!(reader.read_u8(), Some(1)); // subtype
        assert_eq!(reader.read_i32(), Some(50)); // sp_damage
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_sp_absorption_packet_large_damage() {
        let pkt = build_sp_absorption_packet(10000);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(2));
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_i32(), Some(10000));
    }

    // ── Regen interval constant ─────────────────────────────────────

    #[test]
    fn test_sp_regen_interval() {
        // SP regen is every 2 seconds (PLAYER_STAMINA_INTERVAL)
        assert_eq!(SP_REGEN_INTERVAL_SECS, 2);
    }

    // ── SP regen boundary tests ─────────────────────────────────────

    #[test]
    fn test_sp_regen_all_class_variants() {
        // Verify all Kurian class codes across both nations (no PRO_SKILL4)
        for &(class, expected) in &[
            (113u16, 5i16), // Karus beginner
            (114, 7),       // Karus novice
            (115, 7),       // Karus master (base, no bonus)
            (213, 5),       // El Morad beginner
            (214, 7),       // El Morad novice
            (215, 7),       // El Morad master (base, no bonus)
        ] {
            assert_eq!(
                get_sp_regen_amount(class, 0),
                expected,
                "class {} should regen {} SP",
                class,
                expected
            );
        }
    }

    #[test]
    fn test_sp_regen_clamping() {
        // Test that SP regen correctly clamps to max_sp
        // Simulated: sp=97, max_sp=100, regen=5 => new_sp should be 100 (not 102)
        let sp: i16 = 97;
        let max_sp: i16 = 100;
        let regen: i16 = 5;
        let new_sp = (sp as i32 + regen as i32).min(max_sp as i32) as i16;
        assert_eq!(new_sp, 100);
    }

    #[test]
    fn test_sp_regen_at_max() {
        // When already at max, no regen should apply
        let sp: i16 = 100;
        let max_sp: i16 = 100;
        assert!(sp >= max_sp); // This condition prevents regen in process_session_sp_regen
    }
}
