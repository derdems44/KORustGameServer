//! WIZ_STATE_CHANGE `bType` constants.
//!
//! C++ Reference: `User.cpp:2941-3007` — `StateChangeServerDirect(uint8 bType, uint32 nBuff)`
//!
//! Each constant maps to a `case` in the C++ switch statement.

/// bType 1 — HP regen status (standing/sitting/mining/fishing/flashing).
///
/// C++ Reference: `User.cpp:2946-2948` — `m_bResHpType = buff`
pub const STATE_CHANGE_HPTYPE: u8 = 1;

/// bType 2 — Need party flag (not looking / seeking / looking for member).
///
/// C++ Reference: `User.cpp:2950-2951` — `m_bNeedParty = buff`
pub const STATE_CHANGE_NEEDPARTY: u8 = 2;

/// bType 3 — Abnormal/view state (transformation, giant, dwarf, blinking).
///
/// C++ Reference: `User.cpp:2954-2963` — `m_bAbnormalType = nBuff`
pub const STATE_CHANGE_ABNORMAL: u8 = 3;

/// bType 5 — GM visibility toggle.
///
/// C++ Reference: `User.cpp:2966-2971` — `GmInOut(INOUT_OUT/IN)`
pub const STATE_CHANGE_GM_VISIBILITY: u8 = 5;

/// bType 6 — Party leader symbol ('P' icon above head).
///
/// C++ Reference: `User.cpp:2974-2975` — `m_bPartyLeader = nBuff`
pub const STATE_CHANGE_PARTY_LEADER: u8 = 6;

/// bType 7 — Invisibility type.
///
/// C++ Reference: `User.cpp:2978-2979` — `UpdateVisibility((InvisibilityType)buff)`
pub const STATE_CHANGE_INVISIBILITY: u8 = 7;

/// bType 11 — Team colour (red/blue/none for BDW, events).
///
/// C++ Reference: `User.cpp:2985-2992` — `m_teamColour` assignment
pub const STATE_CHANGE_TEAM_COLOUR: u8 = 11;

/// bType 12 — Devil/weapons disabled visual (no server-side logic).
///
/// C++ Reference: `User.cpp:2993-2994` — `case 12: break`
pub const STATE_CHANGE_WEAPONS_DISABLED: u8 = 12;
