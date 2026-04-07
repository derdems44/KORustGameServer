//! Canonical race/class template constants.
//! All character race identifiers live here. Every module that needs
//! race validation (class change, gender change, nation transfer, NTS)
//! should import from this module rather than defining its own copies.

// ── Karus races ─────────────────────────────────────────────────────────

/// Arch Tuarek — Warriors
pub const KARUS_BIG: u8 = 1;

/// Tuarek — Rogues & Priests
pub const KARUS_MIDDLE: u8 = 2;

/// Wrinkle Tuarek — Magicians
pub const KARUS_SMALL: u8 = 3;

/// Puri Tuarek — Priests
pub const KARUS_WOMAN: u8 = 4;

/// Kurian race
pub const KURIAN: u8 = 6;

// ── El Morad races ──────────────────────────────────────────────────────

/// Barbarian — Warriors
pub const BABARIAN: u8 = 11;

/// El Morad Male
pub const ELMORAD_MAN: u8 = 12;

/// El Morad Female
pub const ELMORAD_WOMAN: u8 = 13;

/// Porutu
pub const PORUTU: u8 = 14;
