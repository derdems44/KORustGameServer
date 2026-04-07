//! Canonical race/class template constants.
//!
//! All character race identifiers live here. Every module that needs
//! race validation (class change, gender change, nation transfer, NTS)
//! should import from this module rather than defining its own copies.
//!
//! C++ Reference: `GameDefine.h:45-54` — `enum Race`

// ── Karus races ─────────────────────────────────────────────────────────

/// Arch Tuarek — Warriors (C++ `KARUS_BIG = 1`).
pub const KARUS_BIG: u8 = 1;

/// Tuarek — Rogues & Priests (C++ `KARUS_MIDDLE = 2`).
pub const KARUS_MIDDLE: u8 = 2;

/// Wrinkle Tuarek — Magicians (C++ `KARUS_SMALL = 3`).
pub const KARUS_SMALL: u8 = 3;

/// Puri Tuarek — Priests (C++ `KARUS_WOMAN = 4`).
pub const KARUS_WOMAN: u8 = 4;

/// Kurian race (C++ `KURIAN = 6`).
pub const KURIAN: u8 = 6;

// ── El Morad races ──────────────────────────────────────────────────────

/// Barbarian — Warriors (C++ `BABARIAN = 11`).
pub const BABARIAN: u8 = 11;

/// El Morad Male (C++ `ELMORAD_MAN = 12`).
pub const ELMORAD_MAN: u8 = 12;

/// El Morad Female (C++ `ELMORAD_WOMAN = 13`).
pub const ELMORAD_WOMAN: u8 = 13;

/// Porutu (C++ `PORUTU = 14`).
pub const PORUTU: u8 = 14;
