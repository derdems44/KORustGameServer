//! Magic repository — loads magic skill data from PostgreSQL.
//!
//! C++ Reference:
//! - `GameServer/LoadServerData.cpp` — `LoadMagicTable()`, `LoadMagicType1()` .. `LoadMagicType9()`
//! - `shared/database/MagicTableSet.h` through `MagicType9Set.h`
//!
//! All tables are bulk-loaded at server startup and cached in memory.

use crate::models::magic::{
    MagicRow, MagicType1Row, MagicType2Row, MagicType3Row, MagicType4Row, MagicType5Row,
    MagicType6Row, MagicType7Row, MagicType8Row, MagicType9Row,
};
use crate::DbPool;

/// Repository for magic system table access.
pub struct MagicRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> MagicRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all master magic/skill rows (bulk load at startup).
    ///
    /// Returns ~3,880 rows. Keyed by `magic_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicTable()`
    pub async fn load_magic_table(&self) -> Result<Vec<MagicRow>, sqlx::Error> {
        sqlx::query_as::<_, MagicRow>(
            "SELECT magic_num, en_name, kr_name, description, t_1, before_action, \
             target_action, self_effect, flying_effect, target_effect, moral, \
             skill_level, skill, msp, hp, s_sp, item_group, use_item, cast_time, \
             recast_time, success_rate, type1, type2, \"range\", etc, use_standing, \
             skill_check, icelightrate \
             FROM magic ORDER BY magic_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all melee attack skill parameters (bulk load at startup).
    ///
    /// Returns ~525 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType1()`
    pub async fn load_magic_type1(&self) -> Result<Vec<MagicType1Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType1Row>(
            "SELECT i_num, hit_type, hit_rate, hit, add_damage, combo_type, \
             combo_count, combo_damage, \"range\", delay, add_dmg_perc_to_user, \
             add_dmg_perc_to_npc \
             FROM magic_type1 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all ranged/archery attack skill parameters (bulk load at startup).
    ///
    /// Returns ~91 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType2()`
    pub async fn load_magic_type2(&self) -> Result<Vec<MagicType2Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType2Row>(
            "SELECT i_num, hit_type, hit_rate, add_damage, add_range, need_arrow, \
             add_dmg_perc_to_user, add_dmg_perc_to_npc \
             FROM magic_type2 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all DOT / direct magic damage parameters (bulk load at startup).
    ///
    /// Returns ~1,152 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType3()`
    pub async fn load_magic_type3(&self) -> Result<Vec<MagicType3Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType3Row>(
            "SELECT i_num, direct_type, first_damage, time_damage, duration, \
             attribute, radius, add_dmg_perc_to_user, add_dmg_perc_to_npc \
             FROM magic_type3 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all buff/debuff skill parameters (bulk load at startup).
    ///
    /// Returns ~1,917 rows. Keyed by `i_num`. Largest sub-table.
    /// C++ Reference: `CGameServerDlg::LoadMagicType4()`
    pub async fn load_magic_type4(&self) -> Result<Vec<MagicType4Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType4Row>(
            "SELECT i_num, buff_type, radius, duration, attack_speed, speed, ac, \
             ac_pct, attack, magic_attack, max_hp, max_hp_pct, max_mp, max_mp_pct, \
             str, sta, dex, intel, cha, fire_r, cold_r, lightning_r, magic_r, \
             disease_r, poison_r, exp_pct, special_amount, hit_rate, avoid_rate \
             FROM magic_type4 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all resurrection/recovery skill parameters (bulk load at startup).
    ///
    /// Returns ~59 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType5()`
    pub async fn load_magic_type5(&self) -> Result<Vec<MagicType5Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType5Row>(
            "SELECT i_num, \"type\", exp_recover, need_stone \
             FROM magic_type5 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all transformation skill parameters (bulk load at startup).
    ///
    /// Returns ~251 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType6()`
    pub async fn load_magic_type6(&self) -> Result<Vec<MagicType6Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType6Row>(
            "SELECT i_num, \"name\", description, \"size\", transform_id, duration, \
             max_hp, max_mp, speed, attack_speed, total_hit, total_ac, \
             total_hit_rate, total_evasion_rate, total_fire_r, total_cold_r, \
             total_lightning_r, total_magic_r, total_disease_r, total_poison_r, \
             class, user_skill_use, need_item, skill_success_rate, \
             monster_friendly, nation \
             FROM magic_type6 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all summoning/CC skill parameters (bulk load at startup).
    ///
    /// Returns ~26 rows. Keyed by `n_index`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType7()`
    pub async fn load_magic_type7(&self) -> Result<Vec<MagicType7Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType7Row>(
            "SELECT n_index, str_name, str_note, valid_group, nation_change, \
             monster_num, target_change, state_change, radius, hit_rate, \
             duration, damage, vision, need_item \
             FROM magic_type7 ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all teleportation/warp skill parameters (bulk load at startup).
    ///
    /// Returns ~121 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType8()`
    pub async fn load_magic_type8(&self) -> Result<Vec<MagicType8Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType8Row>(
            "SELECT i_num, \"name\", description, target, radius, warp_type, \
             exp_recover, kick_distance \
             FROM magic_type8 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all advanced CC/debuff skill parameters (bulk load at startup).
    ///
    /// Returns ~84 rows. Keyed by `i_num`.
    /// C++ Reference: `CGameServerDlg::LoadMagicType9()`
    pub async fn load_magic_type9(&self) -> Result<Vec<MagicType9Row>, sqlx::Error> {
        sqlx::query_as::<_, MagicType9Row>(
            "SELECT i_num, \"name\", description, valid_group, nation_change, \
             monster_num, target_change, state_change, radius, hit_rate, \
             duration, add_damage, vision, need_item \
             FROM magic_type9 ORDER BY i_num",
        )
        .fetch_all(self.pool)
        .await
    }
}
