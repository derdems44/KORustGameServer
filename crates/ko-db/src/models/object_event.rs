//! Object event position model.
//!
//! C++ Reference: `_OBJECT_EVENT` struct in `shared/database/structs.h`
//! Data source: `K_OBJECTPOS2369` table (MSSQL) → `object_event_pos` (PostgreSQL)

/// A row from the `object_event_pos` table — defines an interactive object
/// in a zone (bind point, warp gate, lever, anvil, etc.).
///
/// C++ Reference: `_OBJECT_EVENT` in `structs.h:308-321`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ObjectEventRow {
    /// Auto-increment primary key.
    pub id: i32,
    /// Zone where this object resides.
    pub zone_id: i16,
    /// Nation restriction (0=all, 1=karus, 2=elmorad).
    pub belong: i16,
    /// Object index within the zone.
    pub s_index: i16,
    /// Object type (ObjectType enum).
    pub obj_type: i16,
    /// Associated NPC ID or warp group.
    pub control_npc: i16,
    /// Status: 0=inactive, 1=active.
    pub status: i16,
    /// World X coordinate.
    pub pos_x: f32,
    /// World Y coordinate.
    pub pos_y: f32,
    /// World Z coordinate.
    pub pos_z: f32,
    /// Life flag.
    pub by_life: i16,
}
