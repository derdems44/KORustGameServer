//! Character repository — USERDATA and USER_ITEMS access.

use sqlx::{PgPool, QueryBuilder};

use crate::models::{
    TrashItemRow, UserData, UserItem, VipWarehouseItemRow, VipWarehouseRow, WarehouseCoins,
    WarehouseItem,
};

/// Parameters for saving an inventory item slot.
pub struct SaveItemParams<'a> {
    pub char_id: &'a str,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}

/// Parameters for creating a new character.
pub struct CreateCharParams<'a> {
    pub char_id: &'a str,
    pub nation: i16,
    pub race: i16,
    pub class: i16,
    pub face: i16,
    pub hair: i32,
    pub strong: i16,
    pub sta: i16,
    pub dex: i16,
    pub intel: i16,
    pub cha: i16,
    pub zone: i16,
    pub px: i32,
    pub pz: i32,
    pub py: i32,
}

/// Parameters for saving character stats.
pub struct SaveStatsParams<'a> {
    pub char_id: &'a str,
    pub level: i16,
    pub hp: i16,
    pub mp: i16,
    pub sp: i16,
    pub exp: i64,
    pub gold: i32,
    pub loyalty: i32,
    pub loyalty_monthly: i32,
    pub manner_point: i32,
}

/// Parameters for saving stat and skill point allocations.
pub struct SaveStatPointsParams<'a> {
    pub char_id: &'a str,
    pub str_val: i16,
    pub sta: i16,
    pub dex: i16,
    pub intel: i16,
    pub cha: i16,
    pub free_points: i16,
    pub skill_points: [i16; 10],
}

/// Parameters for saving level and experience after level change.
pub struct SaveLevelExpParams<'a> {
    pub char_id: &'a str,
    pub level: i16,
    pub exp: i64,
    pub hp: i16,
    pub mp: i16,
    pub free_points: i16,
}

/// Repository for character-related database operations.
pub struct CharacterRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CharacterRepository<'a> {
    /// Create a new character repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load character data by character ID.
    pub async fn load(&self, char_id: &str) -> Result<Option<UserData>, sqlx::Error> {
        sqlx::query_as::<_, UserData>("SELECT * FROM userdata WHERE str_user_id = $1")
            .bind(char_id)
            .fetch_optional(self.pool)
            .await
    }

    /// Load all characters for an account (via account_char mapping).
    pub async fn load_all_for_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<UserData>, sqlx::Error> {
        sqlx::query_as::<_, UserData>(
            "SELECT u.* FROM userdata u
             INNER JOIN account_char ac ON ac.str_account_id = $1
             WHERE u.str_user_id IN (ac.str_char_id1, ac.str_char_id2, ac.str_char_id3, ac.str_char_id4)",
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load inventory items for a character.
    pub async fn load_items(&self, char_id: &str) -> Result<Vec<UserItem>, sqlx::Error> {
        sqlx::query_as::<_, UserItem>(
            "SELECT * FROM user_items WHERE str_user_id = $1 ORDER BY slot_index",
        )
        .bind(char_id)
        .fetch_all(self.pool)
        .await
    }

    /// Batch-load equipped items for multiple characters (charsel optimization).
    /// Returns only equipment slots (0-13) to minimize data transfer.
    pub async fn load_equipped_items_batch(
        &self,
        char_ids: &[&str],
    ) -> Result<Vec<UserItem>, sqlx::Error> {
        sqlx::query_as::<_, UserItem>(
            "SELECT * FROM user_items
             WHERE str_user_id = ANY($1) AND slot_index < 14 AND item_id > 0
             ORDER BY str_user_id, slot_index",
        )
        .bind(char_ids)
        .fetch_all(self.pool)
        .await
    }

    /// Save (upsert) a single inventory item slot.
    pub async fn save_item(&self, params: &SaveItemParams<'_>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_items (str_user_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (str_user_id, slot_index) DO UPDATE SET
                item_id = $3, durability = $4, count = $5, flag = $6, original_flag = $7, serial_num = $8, expire_time = $9",
        )
        .bind(params.char_id)
        .bind(params.slot_index)
        .bind(params.item_id)
        .bind(params.durability)
        .bind(params.count)
        .bind(params.flag)
        .bind(params.original_flag)
        .bind(params.serial_num)
        .bind(params.expire_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Batch save (upsert) all inventory item slots in a single query.
    ///
    /// Reduces N individual round-trips to 1 for periodic/shutdown saves.
    pub async fn save_items_batch(&self, items: &[SaveItemParams<'_>]) -> Result<(), sqlx::Error> {
        if items.is_empty() {
            return Ok(());
        }
        let mut builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "INSERT INTO user_items (str_user_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time) ",
        );
        builder.push_values(items, |mut b, p| {
            b.push_bind(p.char_id)
                .push_bind(p.slot_index)
                .push_bind(p.item_id)
                .push_bind(p.durability)
                .push_bind(p.count)
                .push_bind(p.flag)
                .push_bind(p.original_flag)
                .push_bind(p.serial_num)
                .push_bind(p.expire_time);
        });
        builder.push(
            " ON CONFLICT (str_user_id, slot_index) DO UPDATE SET \
             item_id = EXCLUDED.item_id, durability = EXCLUDED.durability, \
             count = EXCLUDED.count, flag = EXCLUDED.flag, \
             original_flag = EXCLUDED.original_flag, serial_num = EXCLUDED.serial_num, \
             expire_time = EXCLUDED.expire_time",
        );
        builder.build().execute(self.pool).await?;
        Ok(())
    }

    /// Batch save (upsert) all warehouse item slots in a single query.
    pub async fn save_warehouse_items_batch(
        &self,
        items: &[SaveWarehouseItemParams<'_>],
    ) -> Result<(), sqlx::Error> {
        if items.is_empty() {
            return Ok(());
        }
        let mut builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "INSERT INTO user_warehouse (str_account_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time) ",
        );
        builder.push_values(items, |mut b, p| {
            b.push_bind(p.account_id)
                .push_bind(p.slot_index)
                .push_bind(p.item_id)
                .push_bind(p.durability)
                .push_bind(p.count)
                .push_bind(p.flag)
                .push_bind(p.original_flag)
                .push_bind(p.serial_num)
                .push_bind(p.expire_time);
        });
        builder.push(
            " ON CONFLICT (str_account_id, slot_index) DO UPDATE SET \
             item_id = EXCLUDED.item_id, durability = EXCLUDED.durability, \
             count = EXCLUDED.count, flag = EXCLUDED.flag, \
             original_flag = EXCLUDED.original_flag, serial_num = EXCLUDED.serial_num, \
             expire_time = EXCLUDED.expire_time",
        );
        builder.build().execute(self.pool).await?;
        Ok(())
    }

    /// Update character position and zone.
    pub async fn save_position(
        &self,
        char_id: &str,
        zone: i16,
        px: i32,
        py: i32,
        pz: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET zone = $2, px = $3, py = $4, pz = $5, dt_update_time = NOW()
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(zone)
        .bind(px)
        .bind(py)
        .bind(pz)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save character stats (level, HP, MP, SP, EXP, Gold, Loyalty, manner_point, etc.).
    pub async fn save_stats(&self, params: &SaveStatsParams<'_>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET level = $2, hp = $3, mp = $4, sp = $5, exp = $6, gold = $7, \
             loyalty = $8, loyalty_monthly = $9, manner_point = $10, dt_update_time = NOW()
             WHERE str_user_id = $1",
        )
        .bind(params.char_id)
        .bind(params.level)
        .bind(params.hp)
        .bind(params.mp)
        .bind(params.sp)
        .bind(params.exp)
        .bind(params.gold)
        .bind(params.loyalty)
        .bind(params.loyalty_monthly)
        .bind(params.manner_point)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save flash time/count/type for a character.
    ///
    /// C++ Reference: `DBAgent.cpp` — UpdateUser saves flash_time, flash_count, flash_type
    pub async fn save_flash(
        &self,
        char_id: &str,
        flash_time: i32,
        flash_count: i16,
        flash_type: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET flash_time = $2, flash_count = $3, flash_type = $4
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(flash_time)
        .bind(flash_count)
        .bind(flash_type)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save bind point (respawn location) for a character.
    ///
    /// C++ Reference: `DBAgent.cpp` — UpdateUser saves bind, bind_px, bind_pz
    pub async fn save_bind(
        &self,
        char_id: &str,
        bind_zone: i16,
        bind_px: i32,
        bind_pz: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET bind = $2, bind_px = $3, bind_pz = $4
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(bind_zone)
        .bind(bind_px)
        .bind(bind_pz)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update character gold.
    pub async fn update_gold(&self, char_id: &str, gold: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE userdata SET gold = $1 WHERE str_user_id = $2")
            .bind(gold)
            .bind(char_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Check if a character name already exists.
    pub async fn name_exists(&self, char_id: &str) -> Result<bool, sqlx::Error> {
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM userdata WHERE str_user_id = $1")
                .bind(char_id)
                .fetch_one(self.pool)
                .await?;
        Ok(count > 0)
    }

    /// Create a new character.
    pub async fn create(&self, params: &CreateCharParams<'_>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO userdata (id, str_user_id, nation, race, class, face, hair_rgb, strong, sta, dex, intel, cha, zone, px, pz, py)
             VALUES (
                 nextval('userdata_id_seq'),
                 $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
             )",
        )
        .bind(params.char_id)
        .bind(params.nation)
        .bind(params.race)
        .bind(params.class)
        .bind(params.face)
        .bind(params.hair)
        .bind(params.strong)
        .bind(params.sta)
        .bind(params.dex)
        .bind(params.intel)
        .bind(params.cha)
        .bind(params.zone)
        .bind(params.px)
        .bind(params.pz)
        .bind(params.py)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update character face and hair.
    pub async fn change_hair(
        &self,
        char_id: &str,
        face: i16,
        hair: i32,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE userdata SET face = $2, hair_rgb = $3, dt_update_time = NOW()
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(face)
        .bind(hair)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Save stat points, skill points, and free points to DB.
    ///
    /// Called after WIZ_POINT_CHANGE or WIZ_SKILLPT_CHANGE.
    /// C++ Reference: `UserSkillStatPointSystem.cpp` — stat/skill allocation saves.
    pub async fn save_stat_points(
        &self,
        params: &SaveStatPointsParams<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET strong = $2, sta = $3, dex = $4, intel = $5, cha = $6, \
             points = $7, \
             skill0 = $8, skill1 = $9, skill2 = $10, skill3 = $11, skill4 = $12, \
             skill5 = $13, skill6 = $14, skill7 = $15, skill8 = $16, skill9 = $17, \
             dt_update_time = NOW() \
             WHERE str_user_id = $1",
        )
        .bind(params.char_id)
        .bind(params.str_val)
        .bind(params.sta)
        .bind(params.dex)
        .bind(params.intel)
        .bind(params.cha)
        .bind(params.free_points)
        .bind(params.skill_points[0])
        .bind(params.skill_points[1])
        .bind(params.skill_points[2])
        .bind(params.skill_points[3])
        .bind(params.skill_points[4])
        .bind(params.skill_points[5])
        .bind(params.skill_points[6])
        .bind(params.skill_points[7])
        .bind(params.skill_points[8])
        .bind(params.skill_points[9])
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save level, experience, HP/MP, and free stat points after a level change.
    ///
    /// Called after `LevelChange()` to persist the new level, current HP/MP,
    /// and free stat points.  Max HP/MP are not stored — they are recalculated
    /// from the coefficient table at load time.
    ///
    /// C++ Reference: `CUser::LevelChange()` — saves updated level data.
    pub async fn save_level_exp(&self, params: &SaveLevelExpParams<'_>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET level = $2, exp = $3, hp = $4, mp = $5, \
             points = $6, dt_update_time = NOW() \
             WHERE str_user_id = $1",
        )
        .bind(params.char_id)
        .bind(params.level)
        .bind(params.exp)
        .bind(params.hp)
        .bind(params.mp)
        .bind(params.free_points)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save nation, race, and class change for a character (ClanNts).
    ///
    /// C++ Reference: `CDBAgent::SaveClanNationTransferUser(name, nation, race, class)`
    pub async fn save_nation_transfer_char(
        &self,
        char_name: &str,
        nation: i16,
        race: i16,
        class: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET nation = $2, race = $3, class = $4, dt_update_time = NOW() \
             WHERE str_user_id = $1",
        )
        .bind(char_name)
        .bind(nation)
        .bind(race)
        .bind(class)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save class and race change to DB.
    ///
    /// C++ Reference: `GenderJobChangeHandler.cpp` — updates `m_sClass` and `m_bRace`.
    pub async fn save_class_change(
        &self,
        char_id: &str,
        class: i16,
        race: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET class = $2, race = $3, dt_update_time = NOW() \
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(class)
        .bind(race)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save rebirth stats and level after rebirth change/reset.
    ///
    /// C++ Reference: `NPCHandler.cpp:301-410` — REB_STAT_CHANGE / REB_STAT_RESET
    #[allow(clippy::too_many_arguments)]
    pub async fn save_rebirth(
        &self,
        char_id: &str,
        rebirth_level: i16,
        reb_str: i16,
        reb_sta: i16,
        reb_dex: i16,
        reb_intel: i16,
        reb_cha: i16,
        exp: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET rebirth_level = $2, reb_str = $3, reb_sta = $4, \
             reb_dex = $5, reb_intel = $6, reb_cha = $7, exp = $8, dt_update_time = NOW() \
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(rebirth_level)
        .bind(reb_str)
        .bind(reb_sta)
        .bind(reb_dex)
        .bind(reb_intel)
        .bind(reb_cha)
        .bind(exp)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete a character and its items (cascade).
    pub async fn delete(&self, char_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM userdata WHERE str_user_id = $1")
            .bind(char_id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ── Warehouse (Inn) Methods ──────────────────────────────────────

    /// Load all warehouse items for an account.
    ///
    /// C++ Reference: `CUser::m_sWarehouseArray` — loaded from DB on WAREHOUSE_OPEN.
    pub async fn load_warehouse_items(
        &self,
        account_id: &str,
    ) -> Result<Vec<WarehouseItem>, sqlx::Error> {
        sqlx::query_as::<_, WarehouseItem>(
            "SELECT * FROM user_warehouse WHERE str_account_id = $1 ORDER BY slot_index",
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load warehouse coins (inn gold) for an account.
    ///
    /// C++ Reference: `CUser::GetInnCoins()` / `m_iBank`
    pub async fn load_warehouse_coins(&self, account_id: &str) -> Result<i32, sqlx::Error> {
        let result = sqlx::query_as::<_, WarehouseCoins>(
            "SELECT * FROM user_warehouse_coins WHERE str_account_id = $1",
        )
        .bind(account_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(result.map(|r| r.coins).unwrap_or(0))
    }

    /// Save (upsert) a single warehouse item slot.
    pub async fn save_warehouse_item(
        &self,
        params: &SaveWarehouseItemParams<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_warehouse (str_account_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (str_account_id, slot_index) DO UPDATE SET
                item_id = $3, durability = $4, count = $5, flag = $6, original_flag = $7, serial_num = $8, expire_time = $9",
        )
        .bind(params.account_id)
        .bind(params.slot_index)
        .bind(params.item_id)
        .bind(params.durability)
        .bind(params.count)
        .bind(params.flag)
        .bind(params.original_flag)
        .bind(params.serial_num)
        .bind(params.expire_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Rename a character across all relevant tables (transactional).
    ///
    /// C++ Reference: `CDBAgent::UpdateCharacterName` / `CHANGE_NEW_ID` stored proc.
    ///
    /// Uses a clone-and-swap approach to avoid FK constraint violations:
    /// 1. Insert a new userdata row with the new name (copied from old)
    /// 2. Update all child tables (user_items, user_quest, friend_list, etc.)
    /// 3. Update account_char slot references
    /// 4. Delete the old userdata row
    ///
    /// Returns: 3 on success, 2 if new name already exists, 0 on error.
    pub async fn rename_character(
        &self,
        account_id: &str,
        old_name: &str,
        new_name: &str,
    ) -> Result<u8, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Check inside transaction to avoid TOCTOU race
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM userdata WHERE str_user_id = $1")
            .bind(new_name)
            .fetch_one(&mut *tx)
            .await?;
        if count.0 > 0 {
            return Ok(2); // Name already taken
        }

        // 1. Clone userdata row with new name
        let rows = sqlx::query(
            "INSERT INTO userdata (
                id, str_user_id, nation, race, class, face, hair_rgb,
                strong, sta, dex, intel, cha, level, exp, loyalty, hp, mp, sp,
                authority, points, gold, zone, bind, bind_px, bind_pz,
                px, py, pz, knights, fame, skill0, skill1, skill2, skill3, skill4,
                skill5, skill6, skill7, skill8, skill9, dt_update_time
            )
            SELECT
                id, $2, nation, race, class, face, hair_rgb,
                strong, sta, dex, intel, cha, level, exp, loyalty, hp, mp, sp,
                authority, points, gold, zone, bind, bind_px, bind_pz,
                px, py, pz, knights, fame, skill0, skill1, skill2, skill3, skill4,
                skill5, skill6, skill7, skill8, skill9, NOW()
            FROM userdata WHERE str_user_id = $1",
        )
        .bind(old_name)
        .bind(new_name)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows == 0 {
            tx.rollback().await?;
            return Ok(0); // Character not found
        }

        // 2. Update child tables to reference new name
        sqlx::query("UPDATE user_items SET str_user_id = $2 WHERE str_user_id = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await?;

        // Optional tables (may not have rows, ignore errors)
        sqlx::query("UPDATE user_quest SET str_user_id = $2 WHERE str_user_id = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE user_achieve SET str_user_id = $2 WHERE str_user_id = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE user_achieve_summary SET str_user_id = $2 WHERE str_user_id = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE friend_list SET user_id = $2 WHERE user_id = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE friend_list SET friend_name = $2 WHERE friend_name = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE letter SET sender_name = $2 WHERE sender_name = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        sqlx::query("UPDATE letter SET recipient_name = $2 WHERE recipient_name = $1")
            .bind(old_name)
            .bind(new_name)
            .execute(&mut *tx)
            .await
            .ok();

        // 3. Update account_char slot references
        for col in &[
            "str_char_id1",
            "str_char_id2",
            "str_char_id3",
            "str_char_id4",
        ] {
            let sql = format!(
                "UPDATE account_char SET {} = $2 WHERE str_account_id = $3 AND {} = $1",
                col, col
            );
            sqlx::query(&sql)
                .bind(old_name)
                .bind(new_name)
                .bind(account_id)
                .execute(&mut *tx)
                .await?;
        }

        // 4. Delete old userdata row (children now point to new name)
        sqlx::query("DELETE FROM userdata WHERE str_user_id = $1")
            .bind(old_name)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(3) // Success
    }

    /// Save warehouse coins (inn gold) for an account.
    pub async fn save_warehouse_coins(
        &self,
        account_id: &str,
        coins: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_warehouse_coins (str_account_id, coins)
             VALUES ($1, $2)
             ON CONFLICT (str_account_id) DO UPDATE SET coins = $2",
        )
        .bind(account_id)
        .bind(coins)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    // ── Trash Item (Repurchase) Methods ──────────────────────────────

    /// Load non-expired trash items for repurchase.
    ///
    /// C++ Reference: `CDBAgent::LoadTrashItemList()` — called via
    /// `LOAD_TRASH_ITEMLIST` stored procedure on character login.
    /// Filters out items whose `delete_time` has passed.
    pub async fn load_trash_items(
        &self,
        char_id: &str,
        now_unix: i32,
    ) -> Result<Vec<TrashItemRow>, sqlx::Error> {
        sqlx::query_as::<_, TrashItemRow>(
            "SELECT * FROM trash_item_list \
             WHERE str_user_id = $1 AND delete_time > $2 \
             ORDER BY id",
        )
        .bind(char_id)
        .bind(now_unix)
        .fetch_all(self.pool)
        .await
    }

    /// Insert a trash item for repurchase.
    ///
    /// C++ Reference: `ItemHandler.cpp:2536-2556` — when selling a non-countable
    /// item, a `_DELETED_ITEM` is created and persisted via `WIZ_DB_SAVE_USER`.
    pub async fn insert_trash_item(
        &self,
        params: &InsertTrashItemParams<'_>,
    ) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO trash_item_list \
             (str_user_id, item_id, delete_time, duration, count, flag, serial_num) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING id",
        )
        .bind(params.char_id)
        .bind(params.item_id)
        .bind(params.delete_time)
        .bind(params.duration)
        .bind(params.count)
        .bind(params.flag)
        .bind(params.serial_num)
        .fetch_one(self.pool)
        .await?;
        Ok(row.0)
    }

    /// Delete a single trash item by its DB id (after successful buyback).
    ///
    /// C++ Reference: `CUser::RepurchaseGiveIDBack()` — removes from map after buyback.
    pub async fn delete_trash_item(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM trash_item_list WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Clear all trash items for a character.
    ///
    /// C++ Reference: `CUser::ResetRepurchaseData()` — clears the display list.
    /// We also provide a full DB clear for logout/cleanup.
    pub async fn clear_trash_items(&self, char_id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM trash_item_list WHERE str_user_id = $1")
            .bind(char_id)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Delete all expired trash items for a character.
    ///
    /// Called periodically or on list display to clean up expired entries.
    pub async fn delete_expired_trash_items(
        &self,
        char_id: &str,
        now_unix: i32,
    ) -> Result<u64, sqlx::Error> {
        let result =
            sqlx::query("DELETE FROM trash_item_list WHERE str_user_id = $1 AND delete_time <= $2")
                .bind(char_id)
                .bind(now_unix)
                .execute(self.pool)
                .await?;
        Ok(result.rows_affected())
    }

    // ── VIP Warehouse Methods ────────────────────────────────────────

    /// Load VIP warehouse metadata for an account.
    ///
    /// C++ Reference: `CDBAgent::LoadVIPStorage` — loads password, expiry, password_request.
    pub async fn load_vip_warehouse(
        &self,
        account_id: &str,
    ) -> Result<Option<VipWarehouseRow>, sqlx::Error> {
        sqlx::query_as::<_, VipWarehouseRow>(
            "SELECT * FROM vip_warehouse WHERE str_account_id = $1",
        )
        .bind(account_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Load VIP warehouse items for an account.
    ///
    /// C++ Reference: `CUser::m_sVIPWarehouseArray[VIPWAREHOUSE_MAX]`
    pub async fn load_vip_warehouse_items(
        &self,
        account_id: &str,
    ) -> Result<Vec<VipWarehouseItemRow>, sqlx::Error> {
        sqlx::query_as::<_, VipWarehouseItemRow>(
            "SELECT * FROM vip_warehouse_items WHERE str_account_id = $1 ORDER BY slot_index",
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
    }

    /// Save (upsert) VIP warehouse metadata.
    pub async fn save_vip_warehouse(
        &self,
        account_id: &str,
        password: &str,
        password_request: i16,
        vault_expiry: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO vip_warehouse (str_account_id, password, password_request, vault_expiry)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (str_account_id) DO UPDATE SET
                password = $2, password_request = $3, vault_expiry = $4",
        )
        .bind(account_id)
        .bind(password)
        .bind(password_request)
        .bind(vault_expiry)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save (upsert) a single VIP warehouse item slot.
    pub async fn save_vip_warehouse_item(
        &self,
        params: &SaveVipWarehouseItemParams<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO vip_warehouse_items (str_account_id, slot_index, item_id, durability, count, flag, original_flag, serial_num, expire_time)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (str_account_id, slot_index) DO UPDATE SET
                item_id = $3, durability = $4, count = $5, flag = $6, original_flag = $7, serial_num = $8, expire_time = $9",
        )
        .bind(params.account_id)
        .bind(params.slot_index)
        .bind(params.item_id)
        .bind(params.durability)
        .bind(params.count)
        .bind(params.flag)
        .bind(params.original_flag)
        .bind(params.serial_num)
        .bind(params.expire_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Apply starting equipment for a new character from CREATE_NEW_CHAR_SET data.
    ///
    /// Inserts all non-empty item slots into user_items.
    /// C++ Reference: `LOAD_NEW_CHAR_SET` stored procedure
    pub async fn apply_starting_equipment(
        &self,
        char_id: &str,
        items: &[crate::models::char_creation::CreateNewCharSetRow],
    ) -> Result<(), sqlx::Error> {
        let valid: Vec<_> = items.iter().filter(|i| i.item_id != 0).collect();
        if valid.is_empty() {
            return Ok(());
        }

        let mut builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "INSERT INTO user_items (str_user_id, slot_index, item_id, durability, count, flag, expire_time) ",
        );
        builder.push_values(&valid, |mut b, item| {
            b.push_bind(char_id)
                .push_bind(item.slot_id as i16)
                .push_bind(item.item_id)
                .push_bind(item.item_duration)
                .push_bind(item.item_count)
                .push_bind(item.item_flag)
                .push_bind(item.item_expire_time);
        });
        builder.push(
            " ON CONFLICT (str_user_id, slot_index) DO UPDATE \
             SET item_id = EXCLUDED.item_id, durability = EXCLUDED.durability, \
                 count = EXCLUDED.count, flag = EXCLUDED.flag, expire_time = EXCLUDED.expire_time",
        );
        builder.build().execute(self.pool).await?;
        Ok(())
    }

    /// Apply starting stats for a new character from CREATE_NEW_CHAR_VALUE data.
    ///
    /// Updates level, exp, stat points, skill points, and gold.
    /// C++ Reference: `LOAD_NEW_CHAR_VALUE` stored procedure
    pub async fn apply_starting_stats(
        &self,
        char_id: &str,
        stats: &crate::models::char_creation::CreateNewCharValueRow,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE userdata SET
                level = $2, exp = $3,
                strong = strong + $4, sta = sta + $5, dex = dex + $6, intel = intel + $7, cha = cha + $8,
                points = $9, skill0 = $10, skill1 = $11, skill2 = $12, skill3 = $13, skill4 = $14,
                gold = $15, dt_update_time = NOW()
             WHERE str_user_id = $1",
        )
        .bind(char_id)
        .bind(stats.level)
        .bind(stats.exp)
        .bind(stats.strength)
        .bind(stats.health)
        .bind(stats.dexterity)
        .bind(stats.intelligence)
        .bind(stats.magic_power)
        .bind(stats.free_points)
        .bind(stats.skill_point_free)
        .bind(stats.skill_point_cat1)
        .bind(stats.skill_point_cat2)
        .bind(stats.skill_point_cat3)
        .bind(stats.skill_point_master)
        .bind(stats.gold)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Reset loyalty_monthly to 0 for all users.
    ///
    /// C++ Reference: `DBAgent.cpp:2455-2463` — `RESET_LOYALTY_MONTHLY` stored proc.
    pub async fn reset_loyalty_monthly(&self) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("UPDATE userdata SET loyalty_monthly = 0")
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Update a character's authority level.
    ///
    /// C++ Reference: `CUser::HandleChangeGM` in `ChatHandler.cpp:2266` — sets `m_bAuthority`.
    pub async fn update_authority(
        &self,
        char_name: &str,
        authority: i16,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("UPDATE userdata SET authority = $1 WHERE str_user_id = $2")
            .bind(authority)
            .bind(char_name)
            .execute(self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Update a character's tag name and colour.
    ///
    /// C++ Reference: `TagChange.cpp:49-53` — persists to `userdata.tagname` + `tagname_rgb`.
    pub async fn update_tagname(
        &self,
        char_id: &str,
        tagname: &str,
        tagname_rgb: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE userdata SET tagname = $1, tagname_rgb = $2 WHERE str_user_id = $3")
            .bind(tagname)
            .bind(tagname_rgb)
            .bind(char_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Update the mute_status column for a character.
    ///
    /// C++ Reference: `DBAgent.cpp:1672-1680` — `UserAuthorityUpdate(MUTE)`
    ///
    /// - `mute_status = -1` → permanently muted
    /// - `mute_status = 0` → not muted
    pub async fn update_mute_status(
        &self,
        char_id: &str,
        mute_status: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE userdata SET mute_status = $1 WHERE str_user_id = $2")
            .bind(mute_status)
            .bind(char_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}

/// Parameters for saving a warehouse item slot.
pub struct SaveWarehouseItemParams<'a> {
    pub account_id: &'a str,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}

/// Parameters for inserting a trash item (repurchase candidate).
///
/// C++ Reference: `_DELETED_ITEM` fields stored in `TRASH_ITEMLIST`.
pub struct InsertTrashItemParams<'a> {
    pub char_id: &'a str,
    pub item_id: i32,
    pub delete_time: i32,
    pub duration: i16,
    pub count: i32,
    pub flag: i16,
    pub serial_num: i64,
}

/// Parameters for saving a VIP warehouse item slot.
pub struct SaveVipWarehouseItemParams<'a> {
    pub account_id: &'a str,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}
