//! Draki Tower instance dungeon models.
//!                `DRAKI_MONSTER_LIST`, `USER_DRAKI_TOWER_DATA`, `DRAKI_TOWER_RIFT_RANK`

/// A row from the `draki_tower_stages` table -- defines dungeon/sub-stage structure.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DrakiTowerStageRow {
    /// Primary key (1-41).
    pub id: i16,
    /// Dungeon number (1-5).
    pub draki_stage: i16,
    /// Sub-stage within the dungeon (1-8).
    pub draki_sub_stage: i16,
    /// 0 = monster stage, 1 = NPC (safe/rest) stage.
    pub draki_tower_npc_state: i16,
}

/// A row from the `draki_monster_list` table -- monsters to spawn per stage.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DrakiMonsterListRow {
    /// Primary key (1-166).
    pub id: i32,
    /// Stage ID (references draki_tower_stages.id).
    pub stage_id: i16,
    /// NPC template ID (s_sid) to spawn.
    pub monster_id: i16,
    /// X coordinate.
    pub pos_x: i16,
    /// Z coordinate.
    pub pos_z: i16,
    /// Facing direction (0-360).
    pub s_direction: i16,
    /// 0 = non-monster NPC, 1 = monster.
    pub is_monster: bool,
}

/// A row from the `user_draki_tower_data` table -- per-user progress.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDrakiTowerDataRow {
    /// Character name (primary key).
    pub str_user_id: String,
    /// Class ID (1=Warrior, 2=Rogue, etc.).
    pub class: i32,
    /// Class name for display.
    pub class_name: String,
    /// Best time in seconds.
    pub i_draki_time: i32,
    /// Best stage reached (stage ID index).
    pub b_draki_stage: i16,
    /// Remaining entrance attempts today (max 3, resets at 18:00).
    pub b_draki_enterance_limit: i16,
}

/// A row from the `draki_tower_rift_rank` table -- ranking leaderboard.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DrakiTowerRiftRankRow {
    /// Auto-increment index.
    pub s_index: i32,
    /// Class ID.
    pub class: i32,
    /// Class name for display.
    pub class_name: String,
    /// Rank within class (1-based).
    pub rank_id: i32,
    /// Character name.
    pub str_user_id: String,
    /// Stage reached.
    pub b_stage: i16,
    /// Completion time in seconds.
    pub finish_time: i32,
}
