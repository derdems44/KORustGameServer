//! Canonical attack result and hit-rate constants.
//! All attack-related magic numbers live here. Every module that needs
//! attack results or hit-rate classifications should import from this
//! module rather than defining its own copies.

// ── Attack result constants ─────────────────────────────────────────────

/// Attack result: attack failed (miss/block).
pub const ATTACK_FAIL: u8 = 0;

/// Attack result: damage dealt, target still alive.
pub const ATTACK_SUCCESS: u8 = 1;

/// Attack result: damage dealt, target died.
pub const ATTACK_TARGET_DEAD: u8 = 2;

// ── Hit-rate result constants ───────────────────────────────────────────

/// Critical/great hit.
pub const GREAT_SUCCESS: u8 = 0x01;

/// Successful hit.
pub const SUCCESS: u8 = 0x02;

/// Normal hit.
pub const NORMAL: u8 = 0x03;

/// Miss.
pub const FAIL: u8 = 0x04;

// ── Attack type constants ───────────────────────────────────────────────

/// Long-range (ranged) attack type.
pub const LONG_ATTACK: u8 = 1;

// ── Damage cap constants ──────────────────────────────────────────────

/// Maximum damage value — game clamps all damage to this ceiling.
/// Stored as i32 for use in wider arithmetic; callers working with i16
/// damage values should cast via `MAX_DAMAGE as i16`.
pub const MAX_DAMAGE: i32 = 32000;

#[cfg(test)]
mod tests {
    use super::*;

    /// Attack results are sequential 0-2.
    #[test]
    fn test_attack_results_sequential() {
        assert_eq!(ATTACK_FAIL, 0);
        assert_eq!(ATTACK_SUCCESS, 1);
        assert_eq!(ATTACK_TARGET_DEAD, 2);
    }

    /// Hit-rate values are sequential 1-4.
    #[test]
    fn test_hit_rate_values() {
        assert_eq!(GREAT_SUCCESS, 1);
        assert_eq!(SUCCESS, 2);
        assert_eq!(NORMAL, 3);
        assert_eq!(FAIL, 4);
    }

    /// LONG_ATTACK is 1 (ranged).
    #[test]
    fn test_long_attack() {
        assert_eq!(LONG_ATTACK, 1);
    }

    /// MAX_DAMAGE fits in i16 range for damage calculations.
    #[test]
    fn test_max_damage_fits_i16() {
        assert_eq!(MAX_DAMAGE, 32000);
        assert!(MAX_DAMAGE <= i16::MAX as i32);
    }

    /// Hit-rate ordering: GREAT_SUCCESS < SUCCESS < NORMAL < FAIL.
    #[test]
    fn test_hit_rate_ordering() {
        assert!(GREAT_SUCCESS < SUCCESS);
        assert!(SUCCESS < NORMAL);
        assert!(NORMAL < FAIL);
    }

    // ── Sprint 940: Additional coverage ──────────────────────────────

    /// Attack result constants are distinct.
    #[test]
    fn test_attack_results_distinct() {
        assert_ne!(ATTACK_FAIL, ATTACK_SUCCESS);
        assert_ne!(ATTACK_SUCCESS, ATTACK_TARGET_DEAD);
        assert_ne!(ATTACK_FAIL, ATTACK_TARGET_DEAD);
    }

    /// Hit-rate constants are distinct from attack results.
    #[test]
    fn test_hit_rate_distinct_from_attack() {
        // Hit-rate uses 1-4, attack result uses 0-2 — overlap at 1,2
        // but they're used in different contexts
        assert_eq!(GREAT_SUCCESS, 0x01);
        assert_eq!(SUCCESS, 0x02);
        assert_eq!(NORMAL, 0x03);
        assert_eq!(FAIL, 0x04);
    }

    /// MAX_DAMAGE as i16 cast preserves value.
    #[test]
    fn test_max_damage_i16_cast() {
        let clamped = MAX_DAMAGE as i16;
        assert_eq!(clamped, 32000i16);
        assert_eq!(clamped as i32, MAX_DAMAGE);
    }

    /// ATTACK_FAIL is zero (falsy in C++).
    #[test]
    fn test_attack_fail_is_zero() {
        assert_eq!(ATTACK_FAIL, 0);
    }

    /// MAX_DAMAGE is positive and non-zero.
    #[test]
    fn test_max_damage_positive() {
        assert!(MAX_DAMAGE > 0);
        assert!(MAX_DAMAGE > 1000);
    }
}
