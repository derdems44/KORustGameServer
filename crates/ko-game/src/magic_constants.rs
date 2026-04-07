//! Canonical magic process opcode and skill moral constants.
//!
//! All WIZ_MAGIC_PROCESS sub-opcodes and SkillMoral values live here.
//! Every module that sends/receives magic packets should import from
//! this module rather than defining its own copies.
//!
//! C++ Reference: `MagicProcess.h` — `enum MagicOpcode`, `enum SkillMoral`

// ── MagicOpcode constants ───────────────────────────────────────────────

/// Phase 1: Cast animation — broadcast to region.
///
/// C++ Reference: `packets.h:560` — `MAGIC_CASTING = 1`
pub const MAGIC_CASTING: u8 = 1;

/// Phase 2: Projectile in flight (archer skills).
///
/// C++ Reference: `packets.h:561` — `MAGIC_FLYING = 2`
pub const MAGIC_FLYING: u8 = 2;

/// Phase 3: Effect applied (damage, heal, buff).
///
/// C++ Reference: `packets.h:562` — `MAGIC_EFFECTING = 3`
pub const MAGIC_EFFECTING: u8 = 3;

/// Skill failed — sent only to caster.
///
/// C++ Reference: `packets.h:563` — `MAGIC_FAIL = 4`
pub const MAGIC_FAIL: u8 = 4;

/// Buff duration expired — sent with buff type in sData[3].
///
/// C++ Reference: `packets.h:564` — `MAGIC_DURATION_EXPIRED = 5`
pub const MAGIC_DURATION_EXPIRED: u8 = 5;

/// Client requests buff removal.
///
/// C++ Reference: `packets.h:565` — `MAGIC_CANCEL = 6`
pub const MAGIC_CANCEL: u8 = 6;

/// Cancel transformation.
///
/// C++ Reference: `packets.h:566` — `MAGIC_CANCEL_TRANSFORMATION = 7`
pub const MAGIC_CANCEL_TRANSFORMATION: u8 = 7;

/// Extend type 4 buff duration.
///
/// C++ Reference: `packets.h:567` — `MAGIC_TYPE4_EXTEND = 8`
pub const MAGIC_TYPE4_EXTEND: u8 = 8;

/// Second cancel opcode — identical behavior to MAGIC_CANCEL.
///
/// C++ Reference: `packets.h:572` — `MAGIC_CANCEL2 = 13`
pub const MAGIC_CANCEL2: u8 = 13;

// ── SkillMoral constants ────────────────────────────────────────────────

/// Self-cast only.
pub const MORAL_SELF: i16 = 1;

/// Friendly units including self.
pub const MORAL_FRIEND_WITHME: i16 = 2;

/// Friends except self.
pub const MORAL_FRIEND_EXCEPTME: i16 = 3;

/// Party members.
pub const MORAL_PARTY: i16 = 4;

/// NPC targets.
pub const MORAL_NPC: i16 = 5;

/// Full party with area.
pub const MORAL_PARTY_ALL: i16 = 6;

/// Single enemy.
pub const MORAL_ENEMY: i16 = 7;

/// All units.
pub const MORAL_ALL: i16 = 8;

/// AOE enemy.
pub const MORAL_AREA_ENEMY: i16 = 10;

/// AOE friendly.
pub const MORAL_AREA_FRIEND: i16 = 11;

/// AOE all.
pub const MORAL_AREA_ALL: i16 = 12;

/// AOE centered on self.
pub const MORAL_SELF_AREA: i16 = 13;

// ── Skill fail message constants ────────────────────────────────────────

/// Sent as data[3] when a skill attack deals 0 damage — client shows "skill missed".
///
/// C++ Reference: `packets.h:580` — `SKILLMAGIC_FAIL_ATTACKZERO = -104`
pub const SKILLMAGIC_FAIL_ATTACKZERO: i32 = -104;

/// Sent as data[3] when skill has no effect — client shows "<skill name> failed".
///
/// C++ Reference: `packets.h:579` — `SKILLMAGIC_FAIL_NOEFFECT = -103`
pub const SKILLMAGIC_FAIL_NOEFFECT: i32 = -103;

// ── Transformation type constants ───────────────────────────────────────
// C++ Reference: Unit.h — enum TransformationType

/// Monster transformation (e.g. transform scrolls).
///
/// C++ Reference: `Unit::TransformationType::TransformationMonster`
pub const TRANSFORMATION_MONSTER: u8 = 1;

/// NPC transformation (non-combat disguise).
///
/// C++ Reference: `Unit::TransformationType::TransformationNPC`
pub const TRANSFORMATION_NPC: u8 = 2;

/// Siege transformation (war vehicle).
///
/// C++ Reference: `Unit::TransformationType::TransformationSiege`
pub const TRANSFORMATION_SIEGE: u8 = 3;

// ── Abnormal type constants ─────────────────────────────────────────────
// C++ Reference: `GameDefine.h:1395-1407`

/// Invisible (GM stealth).
///
/// C++ Reference: `GameDefine.h:1395` — `ABNORMAL_INVISIBLE = 0`
pub const ABNORMAL_INVISIBLE: u32 = 0;

/// Normal appearance (visible, not blinking).
///
/// C++ Reference: `GameDefine.h:1397` — `ABNORMAL_NORMAL = 1`
pub const ABNORMAL_NORMAL: u32 = 1;

/// Giant / enlarged (Bezoar scroll).
///
/// C++ Reference: `GameDefine.h:1398` — `ABNORMAL_GIANT = 2`
pub const ABNORMAL_GIANT: u32 = 2;

/// Dwarf / shrunk (Rice cake / Minimize scroll).
///
/// C++ Reference: `GameDefine.h:1399` — `ABNORMAL_DWARF = 3`
pub const ABNORMAL_DWARF: u32 = 3;

/// Blinking (Type 9 invisibility expiring).
///
/// C++ Reference: `GameDefine.h:1400` — `ABNORMAL_BLINKING = 4`
pub const ABNORMAL_BLINKING: u32 = 4;

/// Giant applied to target (Maximize Scroll).
///
/// C++ Reference: `GameDefine.h:1401` — `ABNORMAL_GIANT_TARGET = 6`
pub const ABNORMAL_GIANT_TARGET: u32 = 6;

/// Chaos/dungeon-defence normal (non-blinking form for special zones).
///
/// C++ Reference: `GameDefine.h:1402` — `ABNORMAL_CHAOS_NORMAL = 7`
pub const ABNORMAL_CHAOS_NORMAL: u32 = 7;

// ── User status type constants (SendUserStatusUpdate) ─────────────────

/// Status cured (clear DOT/poison/speed debuff).
///
/// C++ Reference: `Define.h:551` — `USER_STATUS_CURE = 0`
pub const USER_STATUS_CURE: u8 = 0;

/// Damage over time active.
///
/// C++ Reference: `Define.h:553` — `USER_STATUS_DOT = 1`
pub const USER_STATUS_DOT: u8 = 1;

/// Poison active.
///
/// C++ Reference: `Define.h:554` — `USER_STATUS_POISON = 2`
pub const USER_STATUS_POISON: u8 = 2;

/// Speed debuff active.
///
/// C++ Reference: `Define.h:555` — `USER_STATUS_SPEED = 3`
pub const USER_STATUS_SPEED: u8 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    /// MagicOpcode constants form a sequential range 1-8, with CANCEL2 at 13.
    #[test]
    fn test_magic_opcode_sequence() {
        assert_eq!(MAGIC_CASTING, 1);
        assert_eq!(MAGIC_FLYING, 2);
        assert_eq!(MAGIC_EFFECTING, 3);
        assert_eq!(MAGIC_FAIL, 4);
        assert_eq!(MAGIC_DURATION_EXPIRED, 5);
        assert_eq!(MAGIC_CANCEL, 6);
        assert_eq!(MAGIC_CANCEL_TRANSFORMATION, 7);
        assert_eq!(MAGIC_TYPE4_EXTEND, 8);
        assert_eq!(MAGIC_CANCEL2, 13);
    }

    /// SkillMoral constants match C++ enum values.
    #[test]
    fn test_skill_moral_values() {
        assert_eq!(MORAL_SELF, 1);
        assert_eq!(MORAL_FRIEND_WITHME, 2);
        assert_eq!(MORAL_FRIEND_EXCEPTME, 3);
        assert_eq!(MORAL_PARTY, 4);
        assert_eq!(MORAL_NPC, 5);
        assert_eq!(MORAL_PARTY_ALL, 6);
        assert_eq!(MORAL_ENEMY, 7);
        assert_eq!(MORAL_ALL, 8);
        assert_eq!(MORAL_AREA_ENEMY, 10);
        assert_eq!(MORAL_AREA_FRIEND, 11);
        assert_eq!(MORAL_AREA_ALL, 12);
        assert_eq!(MORAL_SELF_AREA, 13);
    }

    /// Skill fail codes are negative.
    #[test]
    fn test_skill_fail_codes_negative() {
        assert!(SKILLMAGIC_FAIL_ATTACKZERO < 0);
        assert!(SKILLMAGIC_FAIL_NOEFFECT < 0);
        assert_eq!(SKILLMAGIC_FAIL_ATTACKZERO, -104);
        assert_eq!(SKILLMAGIC_FAIL_NOEFFECT, -103);
    }

    /// Transformation type constants.
    #[test]
    fn test_transformation_types() {
        assert_eq!(TRANSFORMATION_MONSTER, 1);
        assert_eq!(TRANSFORMATION_NPC, 2);
        assert_eq!(TRANSFORMATION_SIEGE, 3);
    }

    /// Abnormal type constants match C++ GameDefine.h.
    #[test]
    fn test_abnormal_types() {
        assert_eq!(ABNORMAL_INVISIBLE, 0);
        assert_eq!(ABNORMAL_NORMAL, 1);
        assert_eq!(ABNORMAL_GIANT, 2);
        assert_eq!(ABNORMAL_DWARF, 3);
        assert_eq!(ABNORMAL_BLINKING, 4);
        assert_eq!(ABNORMAL_GIANT_TARGET, 6);
        assert_eq!(ABNORMAL_CHAOS_NORMAL, 7);
    }

    // ── Sprint 938: Additional coverage ──────────────────────────────

    /// User status constants are sequential 0-3.
    #[test]
    fn test_user_status_sequential() {
        assert_eq!(USER_STATUS_CURE, 0);
        assert_eq!(USER_STATUS_DOT, 1);
        assert_eq!(USER_STATUS_POISON, 2);
        assert_eq!(USER_STATUS_SPEED, 3);
    }

    /// MAGIC_CANCEL2 (13) has a gap from MAGIC_TYPE4_EXTEND (8).
    #[test]
    fn test_magic_cancel2_gap() {
        assert_eq!(MAGIC_CANCEL2 - MAGIC_TYPE4_EXTEND, 5);
    }

    /// Abnormal ordering: INVISIBLE < NORMAL < GIANT < DWARF < BLINKING.
    #[test]
    fn test_abnormal_ordering() {
        assert!(ABNORMAL_INVISIBLE < ABNORMAL_NORMAL);
        assert!(ABNORMAL_NORMAL < ABNORMAL_GIANT);
        assert!(ABNORMAL_GIANT < ABNORMAL_DWARF);
        assert!(ABNORMAL_DWARF < ABNORMAL_BLINKING);
    }

    /// AOE moral group: AREA_ENEMY(10), AREA_FRIEND(11), AREA_ALL(12), SELF_AREA(13).
    #[test]
    fn test_moral_aoe_group() {
        assert_eq!(MORAL_AREA_ENEMY, 10);
        assert_eq!(MORAL_AREA_FRIEND, 11);
        assert_eq!(MORAL_AREA_ALL, 12);
        assert_eq!(MORAL_SELF_AREA, 13);
        // Gap between single-target ALL(8) and AOE AREA_ENEMY(10)
        assert_eq!(MORAL_AREA_ENEMY - MORAL_ALL, 2);
    }

    /// Transformation types are sequential 1-3.
    #[test]
    fn test_transformation_sequential() {
        assert_eq!(TRANSFORMATION_MONSTER, 1);
        assert_eq!(TRANSFORMATION_NPC, 2);
        assert_eq!(TRANSFORMATION_SIEGE, 3);
        assert_eq!(TRANSFORMATION_SIEGE - TRANSFORMATION_MONSTER, 2);
    }
}
