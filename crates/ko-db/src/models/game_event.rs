//! Game event model — maps to the `game_event` PostgreSQL table.

/// A single game event row from the database.
/// event_type: 1 = ZONE_CHANGE, 2 = TRAP_DEAD, 3 = TRAP_AREA
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameEventRow {
    pub zone_no: i16,
    pub event_num: i16,
    pub event_type: i16,
    pub cond1: i32,
    pub cond2: i32,
    pub cond3: i32,
    pub cond4: i32,
    pub cond5: i32,
    pub exec1: i32,
    pub exec2: i32,
    pub exec3: i32,
}
