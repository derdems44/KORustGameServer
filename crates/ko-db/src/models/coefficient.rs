//! Coefficient model — maps to the `coefficient` PostgreSQL table.
//!
//! C++ Reference:
//! - `GameDefine.h` — `_CLASS_COEFFICIENT` struct
//! - `shared/database/CoefficientSet.h` — DB loader
//!
//! Each row contains weapon proficiency and stat scaling coefficients for a
//! single class. Keyed by `s_class` (101-115 Karus, 201-215 El Morad).

/// A single class coefficient row from the database.
///
/// Used by the ability system to compute weapon damage, max HP/MP/SP,
/// armor class, hit rate, and evasion rate per class.
/// C++ equivalent: `_CLASS_COEFFICIENT` (GameDefine.h:1418).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CoefficientRow {
    /// Class identifier (101-115 Karus, 201-215 El Morad).
    pub s_class: i16,
    /// Short sword weapon coefficient.
    pub short_sword: f64,
    /// Jamadar (dagger) weapon coefficient.
    pub jamadar: f64,
    /// Sword weapon coefficient.
    pub sword: f64,
    /// Axe weapon coefficient.
    pub axe: f64,
    /// Club/mace weapon coefficient.
    pub club: f64,
    /// Spear weapon coefficient.
    pub spear: f64,
    /// Pole-arm weapon coefficient.
    pub pole: f64,
    /// Staff weapon coefficient.
    pub staff: f64,
    /// Bow weapon coefficient.
    pub bow: f64,
    /// HP scaling coefficient.
    pub hp: f64,
    /// MP scaling coefficient.
    pub mp: f64,
    /// SP (stamina) scaling coefficient.
    pub sp: f64,
    /// Armor class coefficient.
    pub ac: f64,
    /// Hit rate coefficient.
    pub hitrate: f64,
    /// Evasion rate coefficient.
    pub evasionrate: f64,
}
