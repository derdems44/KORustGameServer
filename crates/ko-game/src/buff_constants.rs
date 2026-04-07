//! Canonical BUFF_TYPE constants — Type4 buff identifiers.
//! `CMagicProcess::Type4Process()` and buff cleanup logic.
//! These constants identify buff effects by their `bBuffType` field in the
//! `MAGIC_TYPE4` table. Each buff type maps to a specific stat modifier,
//! status effect, or transformation.
//! Previously duplicated across magic_process.rs (~50), buff_tick.rs (38),
//! attack.rs (3), home.rs (1), stealth.rs (1), flash.rs (1), bdw.rs (1).
//! Consolidated here as the single source of truth.

/// Size transformation (bezoar/cake shrink/grow).
pub const BUFF_TYPE_SIZE: i32 = 3;
/// Attack speed modifier.
pub const BUFF_TYPE_ATTACK_SPEED: i32 = 5;
/// Movement speed modifier (also used for AOE speed and speed removal).
pub const BUFF_TYPE_SPEED: i32 = 6;
/// Experience gain modifier.
pub const BUFF_TYPE_EXPERIENCE: i32 = 11;
/// Carry weight modifier.
pub const BUFF_TYPE_WEIGHT: i32 = 12;
/// Weapon damage modifier.
pub const BUFF_TYPE_WEAPON_DAMAGE: i32 = 13;
/// Loyalty (NP) gain rate modifier.
pub const BUFF_TYPE_LOYALTY: i32 = 15;
/// Gold (Noah) bonus modifier.
pub const BUFF_TYPE_NOAH_BONUS: i32 = 16;
/// Premium merchant buff.
pub const BUFF_TYPE_PREMIUM_MERCHANT: i32 = 17;
/// Armor-based attack speed modifier.
pub const BUFF_TYPE_ATTACK_SPEED_ARMOR: i32 = 18;
/// Disable targeting (target cannot be selected).
pub const BUFF_TYPE_DISABLE_TARGETING: i32 = 20;
/// Blind (reduces hit rate / screen effect).
pub const BUFF_TYPE_BLIND: i32 = 21;
/// Freeze (immobilize target, cannot move or attack).
pub const BUFF_TYPE_FREEZE: i32 = 22;
/// Instant magic cast (removes cast time).
pub const BUFF_TYPE_INSTANT_MAGIC: i32 = 23;
/// Decrease magic resistance.
pub const BUFF_TYPE_DECREASE_RESIST: i32 = 24;
/// Mage armor (magic shield absorb).
pub const BUFF_TYPE_MAGE_ARMOR: i32 = 25;
/// Prohibit invisibility (prevents stealth).
pub const BUFF_TYPE_PROHIBIT_INVIS: i32 = 26;
/// Resistance and magic damage modifier.
pub const BUFF_TYPE_RESIS_AND_MAGIC_DMG: i32 = 27;
/// Block curse effects.
pub const BUFF_TYPE_BLOCK_CURSE: i32 = 29;
/// Block and reflect curse effects.
pub const BUFF_TYPE_BLOCK_CURSE_REFLECT: i32 = 30;
/// Mana absorb on hit.
pub const BUFF_TYPE_MANA_ABSORB: i32 = 31;
/// Ignore weapon defense.
pub const BUFF_TYPE_IGNORE_WEAPON: i32 = 32;
/// Various stat effects (multi-modifier buff).
pub const BUFF_TYPE_VARIOUS_EFFECTS: i32 = 33;
/// Secondary speed modifier (speed2 — stacking speed buff).
pub const BUFF_TYPE_SPEED2: i32 = 40;
/// Loyalty amount modifier (flat NP bonus).
pub const BUFF_TYPE_LOYALTY_AMOUNT: i32 = 42;
/// Mirror damage to party members (damage reflect share).
pub const BUFF_TYPE_MIRROR_DAMAGE_PARTY: i32 = 44;
/// Dagger/bow defense modifier.
pub const BUFF_TYPE_DAGGER_BOW_DEFENSE: i32 = 45;
/// Stun (immobilize, cannot act).
pub const BUFF_TYPE_STUN: i32 = 47;
/// Fishing buff.
pub const BUFF_TYPE_FISHING: i32 = 48;
/// Devil transformation (Kurian).
pub const BUFF_TYPE_DEVIL_TRANSFORM: i32 = 49;
/// Fragment of Manes (BDW event buff).
pub const BUFF_TYPE_FRAGMENT_OF_MANES: i32 = 52;
/// Jackpot event buff.
pub const BUFF_TYPE_JACKPOT: i32 = 77;
/// Invisibility (stealth / Type9).
pub const BUFF_TYPE_INVISIBILITY: i32 = 100;
/// No recall (prevent warp/teleport).
pub const BUFF_TYPE_NO_RECALL: i32 = 150;
/// Reduce target (debuff — lower stats).
pub const BUFF_TYPE_REDUCE_TARGET: i32 = 151;
/// Silence target (prevent casting).
pub const BUFF_TYPE_SILENCE_TARGET: i32 = 152;
/// No potions (prevent potion use).
pub const BUFF_TYPE_NO_POTIONS: i32 = 153;
/// Kaul transformation.
pub const BUFF_TYPE_KAUL_TRANSFORMATION: i32 = 154;
/// Undead transformation.
pub const BUFF_TYPE_UNDEAD: i32 = 155;
/// Unsight (cannot see other players).
pub const BUFF_TYPE_UNSIGHT: i32 = 156;
/// Block all physical damage.
pub const BUFF_TYPE_BLOCK_PHYSICAL_DAMAGE: i32 = 157;
/// Block all magical damage.
pub const BUFF_TYPE_BLOCK_MAGICAL_DAMAGE: i32 = 158;
/// NP drop + Noah bonus combined.
pub const BUFF_TYPE_NP_DROP_NOAH: i32 = 169;
/// Snowman Titi transformation.
pub const BUFF_TYPE_SNOWMAN_TITI: i32 = 170;

#[cfg(test)]
mod tests {
    use super::*;

    /// Core buff types used in combat.
    #[test]
    fn test_combat_buff_types() {
        assert_eq!(BUFF_TYPE_ATTACK_SPEED, 5);
        assert_eq!(BUFF_TYPE_SPEED, 6);
        assert_eq!(BUFF_TYPE_WEAPON_DAMAGE, 13);
        assert_eq!(BUFF_TYPE_FREEZE, 22);
        assert_eq!(BUFF_TYPE_STUN, 47);
    }

    /// CC (crowd control) debuff types.
    #[test]
    fn test_cc_debuff_types() {
        assert_eq!(BUFF_TYPE_BLIND, 21);
        assert_eq!(BUFF_TYPE_FREEZE, 22);
        assert_eq!(BUFF_TYPE_STUN, 47);
        assert_eq!(BUFF_TYPE_SILENCE_TARGET, 152);
        assert_eq!(BUFF_TYPE_NO_POTIONS, 153);
    }

    /// Transformation buff types.
    #[test]
    fn test_transformation_buff_types() {
        assert_eq!(BUFF_TYPE_SIZE, 3);
        assert_eq!(BUFF_TYPE_DEVIL_TRANSFORM, 49);
        assert_eq!(BUFF_TYPE_KAUL_TRANSFORMATION, 154);
        assert_eq!(BUFF_TYPE_UNDEAD, 155);
        assert_eq!(BUFF_TYPE_SNOWMAN_TITI, 170);
    }

    /// Invisibility type is 100.
    #[test]
    fn test_invisibility_type() {
        assert_eq!(BUFF_TYPE_INVISIBILITY, 100);
    }

    /// Economic buff types (EXP, loyalty, gold).
    #[test]
    fn test_economic_buff_types() {
        assert_eq!(BUFF_TYPE_EXPERIENCE, 11);
        assert_eq!(BUFF_TYPE_LOYALTY, 15);
        assert_eq!(BUFF_TYPE_NOAH_BONUS, 16);
        assert_eq!(BUFF_TYPE_LOYALTY_AMOUNT, 42);
        assert_eq!(BUFF_TYPE_NP_DROP_NOAH, 169);
    }

    // ── Sprint 939: Additional coverage ──────────────────────────────

    /// Defensive buff types.
    #[test]
    fn test_defensive_buff_types() {
        assert_eq!(BUFF_TYPE_MAGE_ARMOR, 25);
        assert_eq!(BUFF_TYPE_BLOCK_CURSE, 29);
        assert_eq!(BUFF_TYPE_BLOCK_CURSE_REFLECT, 30);
        assert_eq!(BUFF_TYPE_DAGGER_BOW_DEFENSE, 45);
        assert_eq!(BUFF_TYPE_BLOCK_PHYSICAL_DAMAGE, 157);
        assert_eq!(BUFF_TYPE_BLOCK_MAGICAL_DAMAGE, 158);
    }

    /// Utility buff types.
    #[test]
    fn test_utility_buff_types() {
        assert_eq!(BUFF_TYPE_WEIGHT, 12);
        assert_eq!(BUFF_TYPE_PREMIUM_MERCHANT, 17);
        assert_eq!(BUFF_TYPE_INSTANT_MAGIC, 23);
        assert_eq!(BUFF_TYPE_FISHING, 48);
        assert_eq!(BUFF_TYPE_JACKPOT, 77);
    }

    /// Debuff types in 150+ range.
    #[test]
    fn test_high_range_debuffs() {
        assert_eq!(BUFF_TYPE_NO_RECALL, 150);
        assert_eq!(BUFF_TYPE_REDUCE_TARGET, 151);
        assert_eq!(BUFF_TYPE_SILENCE_TARGET, 152);
        assert_eq!(BUFF_TYPE_NO_POTIONS, 153);
        assert_eq!(BUFF_TYPE_UNSIGHT, 156);
    }

    /// Stat modifier buff types.
    #[test]
    fn test_stat_modifier_buffs() {
        assert_eq!(BUFF_TYPE_DISABLE_TARGETING, 20);
        assert_eq!(BUFF_TYPE_DECREASE_RESIST, 24);
        assert_eq!(BUFF_TYPE_PROHIBIT_INVIS, 26);
        assert_eq!(BUFF_TYPE_RESIS_AND_MAGIC_DMG, 27);
        assert_eq!(BUFF_TYPE_VARIOUS_EFFECTS, 33);
    }

    /// Speed variants: SPEED(6), SPEED2(40), ATTACK_SPEED(5), ATTACK_SPEED_ARMOR(18).
    #[test]
    fn test_speed_variants() {
        assert_eq!(BUFF_TYPE_SPEED, 6);
        assert_eq!(BUFF_TYPE_SPEED2, 40);
        assert_eq!(BUFF_TYPE_ATTACK_SPEED, 5);
        assert_eq!(BUFF_TYPE_ATTACK_SPEED_ARMOR, 18);
        assert_ne!(BUFF_TYPE_SPEED, BUFF_TYPE_SPEED2);
    }
}
