//! Model for the `monster_under_the_castle` table.
//!
//! C++ Reference: `m_MonsterUnderTheCastleArray` in `GameServerDlg.h`

/// A row from the `monster_under_the_castle` table.
///
/// Each row defines a monster or NPC spawn for the Under The Castle event.
/// `b_type == 0` = monster, `b_type == 1` = NPC (merchants, observers).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterUnderTheCastleRow {
    pub s_index: i16,
    pub s_sid: i16,
    pub str_name: String,
    pub b_type: i16,
    pub trap_number: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub by_direction: i16,
    pub s_count: i16,
    pub b_radius: i16,
}
