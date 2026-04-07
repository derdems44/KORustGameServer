//! Canonical NPC type and interaction range constants.
//! NPC service type identifiers and interaction distance limits live here.
//! Every module that validates NPC types or interaction range should import
//! from this module rather than defining its own copies.

// ── NPC service types ───────────────────────────────────────────────────

/// NPC type: general merchant
pub const NPC_MERCHANT: u8 = 21;

/// NPC type: tinker / repair NPC
pub const NPC_TINKER: u8 = 22;

/// NPC type: warehouse NPC
pub const NPC_WAREHOUSE: u8 = 31;

/// NPC type: loyalty point merchant
pub const NPC_LOYALTY_MERCHANT: u8 = 170;

// ── Environment / non-combat NPC types ──────────────────────────────────

/// NPC type: tree object (`NPC_TREE = 2`). Fixed 20 damage.
pub const NPC_TREE: u8 = 2;

/// NPC type: flag / banner object
pub const NPC_OBJECT_FLAG: u8 = 15;

/// NPC type: refugee NPC (`NPC_REFUGEE = 46`). Proto-dependent damage.
pub const NPC_REFUGEE: u8 = 46;

/// NPC type: fossil ore node (`NPC_FOSIL = 173`). Requires pickaxe.
pub const NPC_FOSIL: u8 = 173;

/// NPC type: partner / companion NPC
pub const NPC_PARTNER_TYPE: u8 = 213;

/// NPC type: prison NPC (`NPC_PRISON = 220`). Requires punishment stick.
pub const NPC_PRISON: u8 = 220;

/// NPC type: scarecrow / training dummy
pub const NPC_SCARECROW: u8 = 171;

// ── Gate / door NPC types ───────────────────────────────────────────────

/// NPC type: standard gate
pub const NPC_GATE: u8 = 50;

/// NPC type: phoenix gate
pub const NPC_PHOENIX_GATE: u8 = 51;

/// NPC type: special gate, auto-cycles in war
pub const NPC_SPECIAL_GATE: u8 = 52;

/// NPC type: victory gate
pub const NPC_VICTORY_GATE: u8 = 53;

/// NPC type: burning wood object (`NPC_OBJECT_WOOD = 54`). 80% HP damage.
pub const NPC_OBJECT_WOOD: u8 = 54;

/// NPC type: rolling stone (`NPC_ROLLINGSTONE = 181`). Instant death.
pub const NPC_ROLLINGSTONE: u8 = 181;

/// NPC type: gate lever
pub const NPC_GATE_LEVER: u8 = 55;

/// NPC type: second gate variant
pub const NPC_GATE2: u8 = 150;

// ── Tower / guard NPC types ─────────────────────────────────────────────

/// NPC type: guard tower 1 (`NPC_GUARD_TOWER1 = 62`). Immune to attack.
pub const NPC_GUARD_TOWER1: u8 = 62;

/// NPC type: guard tower 2 (`NPC_GUARD_TOWER2 = 63`). Immune to attack.
pub const NPC_GUARD_TOWER2: u8 = 63;

// ── Monument NPC types ──────────────────────────────────────────────────

/// NPC type: destroyed artifact / CSW monument
pub const NPC_DESTROYED_ARTIFACT: u8 = 61;

/// NPC type: Karus nation monument
pub const NPC_KARUS_MONUMENT: u8 = 121;

/// NPC type: El Morad nation monument
pub const NPC_HUMAN_MONUMENT: u8 = 122;

/// NPC type: Bifrost monument
pub const NPC_BIFROST_MONUMENT: u8 = 155;

/// NPC type: soccer baal (`NPC_SOCCER_BAAL = 197`). Immune to attack.
pub const NPC_SOCCER_BAAL: u8 = 197;

/// NPC type: PVP zone monument
pub const NPC_PVP_MONUMENT: u8 = 210;

/// NPC type: battle zone monument
pub const NPC_BATTLE_MONUMENT: u8 = 211;

/// NPC type: border monument / BDW altar
pub const NPC_BORDER_MONUMENT: u8 = 212;

/// NPC type: clan war monument
pub const NPC_CLAN_WAR_MONUMENT: u8 = 224;

// ── Warder / gatekeeper NPC types ───────────────────────────────────────

/// NPC type: Karus warder 1
pub const NPC_KARUS_WARDER1: u8 = 190;

/// NPC type: Karus warder 2
pub const NPC_KARUS_WARDER2: u8 = 191;

/// NPC type: El Morad warder 1
pub const NPC_ELMORAD_WARDER1: u8 = 192;

/// NPC type: El Morad warder 2
pub const NPC_ELMORAD_WARDER2: u8 = 193;

/// NPC type: Karus gatekeeper
pub const NPC_KARUS_GATEKEEPER: u8 = 198;

/// NPC type: El Morad gatekeeper
pub const NPC_ELMORAD_GATEKEEPER: u8 = 199;

/// NPC type: chaos stone
pub const NPC_CHAOS_STONE: u8 = 200;

/// NPC type: Santa Claus event NPC
pub const NPC_SANTA: u8 = 219;

// ── Interaction range constants ─────────────────────────────────────────

/// Maximum NPC interaction range (game units).
pub const MAX_NPC_RANGE: f32 = 30.0;

/// Maximum object interaction range (game units).
pub const MAX_OBJECT_RANGE: f32 = 100.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// Service NPC types have distinct values.
    #[test]
    fn test_service_npc_types() {
        assert_eq!(NPC_MERCHANT, 21);
        assert_eq!(NPC_TINKER, 22);
        assert_eq!(NPC_WAREHOUSE, 31);
        assert_eq!(NPC_LOYALTY_MERCHANT, 170);
    }

    /// Gate NPC types range 50-55 + 150.
    #[test]
    fn test_gate_npc_types() {
        assert_eq!(NPC_GATE, 50);
        assert_eq!(NPC_PHOENIX_GATE, 51);
        assert_eq!(NPC_SPECIAL_GATE, 52);
        assert_eq!(NPC_VICTORY_GATE, 53);
        assert_eq!(NPC_GATE_LEVER, 55);
        assert_eq!(NPC_GATE2, 150);
    }

    /// Monument NPC types.
    #[test]
    fn test_monument_npc_types() {
        assert_eq!(NPC_DESTROYED_ARTIFACT, 61);
        assert_eq!(NPC_KARUS_MONUMENT, 121);
        assert_eq!(NPC_HUMAN_MONUMENT, 122);
        assert_eq!(NPC_BIFROST_MONUMENT, 155);
    }

    /// Interaction range constants.
    #[test]
    fn test_interaction_ranges() {
        assert_eq!(MAX_NPC_RANGE, 30.0);
        assert_eq!(MAX_OBJECT_RANGE, 100.0);
        assert!(MAX_OBJECT_RANGE > MAX_NPC_RANGE);
    }

    /// Warder types are symmetric: Karus 190/191, Elmorad 192/193.
    #[test]
    fn test_warder_symmetry() {
        assert_eq!(NPC_KARUS_WARDER1, 190);
        assert_eq!(NPC_KARUS_WARDER2, 191);
        assert_eq!(NPC_ELMORAD_WARDER1, 192);
        assert_eq!(NPC_ELMORAD_WARDER2, 193);
        assert_eq!(NPC_KARUS_GATEKEEPER, 198);
        assert_eq!(NPC_ELMORAD_GATEKEEPER, 199);
    }

    // ── Sprint 938: Additional coverage ──────────────────────────────

    /// Environment NPC types (tree, scarecrow, fossil, prison).
    #[test]
    fn test_environment_npc_types() {
        assert_eq!(NPC_TREE, 2);
        assert_eq!(NPC_SCARECROW, 171);
        assert_eq!(NPC_FOSIL, 173);
        assert_eq!(NPC_PRISON, 220);
        assert_eq!(NPC_REFUGEE, 46);
    }

    /// Guard tower types are adjacent (62, 63).
    #[test]
    fn test_guard_tower_pair() {
        assert_eq!(NPC_GUARD_TOWER1, 62);
        assert_eq!(NPC_GUARD_TOWER2, 63);
        assert_eq!(NPC_GUARD_TOWER2 - NPC_GUARD_TOWER1, 1);
    }

    /// Chaos stone and special event NPCs.
    #[test]
    fn test_special_npc_types() {
        assert_eq!(NPC_CHAOS_STONE, 200);
        assert_eq!(NPC_SANTA, 219);
        assert_eq!(NPC_SOCCER_BAAL, 197);
        assert_eq!(NPC_PARTNER_TYPE, 213);
    }

    /// PVP/Battle/Border monument types are sequential 210-212.
    #[test]
    fn test_pvp_monument_sequential() {
        assert_eq!(NPC_PVP_MONUMENT, 210);
        assert_eq!(NPC_BATTLE_MONUMENT, 211);
        assert_eq!(NPC_BORDER_MONUMENT, 212);
        assert_eq!(NPC_CLAN_WAR_MONUMENT, 224);
    }

    /// Object flag and rolling stone types.
    #[test]
    fn test_misc_npc_types() {
        assert_eq!(NPC_OBJECT_FLAG, 15);
        assert_eq!(NPC_OBJECT_WOOD, 54);
        assert_eq!(NPC_ROLLINGSTONE, 181);
    }
}
