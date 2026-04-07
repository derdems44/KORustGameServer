//! Canonical clan (knights) role and limit constants.
//! Clan fame values, membership limits, and creation requirements live here.
//! Every module that checks clan roles or limits should import from this
//! module rather than defining its own copies.

// ── Clan role (fame) constants ────────────────────────────────────────────

/// Clan chief / leader.
pub const CHIEF: u8 = 1;

/// Vice chief / assistant leader.
pub const VICECHIEF: u8 = 2;

/// Knight rank.
pub const KNIGHT: u8 = 3;

/// Officer rank.
pub const OFFICER: u8 = 4;

/// Trainee rank (default for new members).
pub const TRAINEE: u8 = 5;

/// War commander captain fame.
pub const COMMAND_CAPTAIN: u8 = 100;

/// Authority change sub-type for fame updates.
pub const COMMAND_AUTHORITY: u8 = 1;

// ── Clan limit constants ──────────────────────────────────────────────────

/// Maximum members per clan.
pub const MAX_CLAN_USERS: u16 = 50;

/// Gold cost to create a clan.
pub const CLAN_COIN_REQUIREMENT: u32 = 500_000;

/// Minimum level to create a clan.
pub const CLAN_LEVEL_REQUIREMENT: u8 = 30;

// ── Clan type flag constants ──────────────────────────────────────────

/// Training clan (newly created).
pub const CLAN_TYPE_TRAINING: u8 = 1;

/// Promoted clan (approved by chief donation).
pub const CLAN_TYPE_PROMOTED: u8 = 2;

/// First accredited tier (flag >= 3 → grade forced to 1).
pub const CLAN_TYPE_ACCREDITED5: u8 = 3;

/// Royal grade 1 clan (auto-royal setting).
pub const CLAN_TYPE_ROYAL1: u8 = 12;

#[cfg(test)]
mod tests {
    use super::*;

    /// Clan roles are sequential 1-5.
    #[test]
    fn test_clan_roles_sequential() {
        assert_eq!(CHIEF, 1);
        assert_eq!(VICECHIEF, 2);
        assert_eq!(KNIGHT, 3);
        assert_eq!(OFFICER, 4);
        assert_eq!(TRAINEE, 5);
    }

    /// Clan creation requirements.
    #[test]
    fn test_clan_creation_requirements() {
        assert_eq!(CLAN_COIN_REQUIREMENT, 500_000);
        assert_eq!(CLAN_LEVEL_REQUIREMENT, 30);
        assert_eq!(MAX_CLAN_USERS, 50);
    }

    /// Clan type progression: training → promoted → accredited.
    #[test]
    fn test_clan_type_progression() {
        assert_eq!(CLAN_TYPE_TRAINING, 1);
        assert_eq!(CLAN_TYPE_PROMOTED, 2);
        assert_eq!(CLAN_TYPE_ACCREDITED5, 3);
        assert!(CLAN_TYPE_ROYAL1 > CLAN_TYPE_ACCREDITED5);
    }

    /// COMMAND_CAPTAIN is 100 (distinct from regular roles).
    #[test]
    fn test_command_captain() {
        assert_eq!(COMMAND_CAPTAIN, 100);
        assert!(COMMAND_CAPTAIN > TRAINEE);
    }

    /// COMMAND_AUTHORITY is 1.
    #[test]
    fn test_command_authority() {
        assert_eq!(COMMAND_AUTHORITY, 1);
    }

    // ── Sprint 939: Additional coverage ──────────────────────────────

    /// Clan roles are all distinct values.
    #[test]
    fn test_clan_roles_distinct() {
        let roles = [CHIEF, VICECHIEF, KNIGHT, OFFICER, TRAINEE];
        for i in 0..roles.len() {
            for j in (i + 1)..roles.len() {
                assert_ne!(roles[i], roles[j]);
            }
        }
    }

    /// CHIEF is the highest rank (lowest value = highest rank).
    #[test]
    fn test_chief_highest_rank() {
        assert!(CHIEF < VICECHIEF);
        assert!(VICECHIEF < KNIGHT);
        assert!(KNIGHT < OFFICER);
        assert!(OFFICER < TRAINEE);
    }

    /// Clan type values span 1 to 12.
    #[test]
    fn test_clan_type_range() {
        assert_eq!(CLAN_TYPE_TRAINING, 1);
        assert_eq!(CLAN_TYPE_ROYAL1, 12);
        assert_eq!(CLAN_TYPE_ROYAL1 - CLAN_TYPE_TRAINING, 11);
    }

    /// MAX_CLAN_USERS fits in u8.
    #[test]
    fn test_max_clan_fits_u8() {
        assert!(MAX_CLAN_USERS <= u8::MAX as u16);
    }

    /// Clan coin requirement is 500K gold.
    #[test]
    fn test_clan_coin_500k() {
        assert_eq!(CLAN_COIN_REQUIREMENT, 500 * 1000);
        assert!(CLAN_COIN_REQUIREMENT < u32::MAX);
    }
}
