//! Anti-AFK NPC list.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct AntiAfkEntry {
    pub idx: i16,
    pub npc_id: i16,
}
