//! Character Seal repository — DB operations for character sealing/unsealing.

use sqlx::PgPool;

use crate::models::character_seal::{CharacterSealItemRow, CharacterSealMappingRow};

/// Repository for character seal operations.
pub struct CharacterSealRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CharacterSealRepository<'a> {
    /// Create a new character seal repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Get all seal mappings for an account.
    ///
    pub async fn get_seal_list(
        &self,
        account_id: &str,
    ) -> Result<Vec<CharacterSealMappingRow>, sqlx::Error> {
        sqlx::query_as::<_, CharacterSealMappingRow>(
            "SELECT id, unique_id, seal_item_id, account_id
             FROM character_seal_mapping WHERE account_id = $1",
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load a sealed character item by its unique_id.
    ///
    /// `m_CharacterSealItemArray.GetData(nItemSerial)`
    pub async fn load_seal_item_by_unique_id(
        &self,
        unique_id: i32,
    ) -> Result<Option<CharacterSealItemRow>, sqlx::Error> {
        sqlx::query_as::<_, CharacterSealItemRow>(
            "SELECT csi.*
             FROM character_seal_items csi
             INNER JOIN character_seal_mapping csm ON csm.seal_item_id = csi.id
             WHERE csm.unique_id = $1",
        )
        .bind(unique_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Seal a character: snapshot userdata into character_seal_items.
    ///
    ///
    /// Returns the seal_item_id (PK) on success.
    pub async fn seal_character(
        &self,
        account_id: &str,
        char_name: &str,
        item_serial: i64,
    ) -> Result<i32, sqlx::Error> {
        // Snapshot character data into character_seal_items
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO character_seal_items (
                account_id, char_name, race, class, level, rebirth_level,
                face, hair_rgb, rank, title, exp, loyalty, loyalty_monthly,
                manner_point, fame, city, knights, hp, mp, sp, zone_id,
                strong, sta, dex, intel, cha, authority, free_points, gold,
                skill_cat1, skill_cat2, skill_cat3, skill_master,
                inventory_data, item_serial
             )
             SELECT $1, str_user_id, race, class, level, rebirth_level,
                face, hair_rgb, rank, title, exp, loyalty, loyalty_monthly,
                COALESCE(manner_point, 0), fame, city, knights, hp, mp, sp,
                COALESCE(zone, 0),
                strong, sta, dex, intel, cha, authority, free_points, gold,
                COALESCE(skill_cat1, 0), COALESCE(skill_cat2, 0),
                COALESCE(skill_cat3, 0), COALESCE(skill_master, 0),
                NULL, $3
             FROM userdata WHERE str_user_id = $2
             RETURNING id",
        )
        .bind(account_id)
        .bind(char_name)
        .bind(item_serial)
        .fetch_one(self.pool)
        .await?;

        Ok(row.0)
    }

    /// Create a mapping from unique_id to seal_item_id.
    pub async fn create_mapping(
        &self,
        unique_id: i32,
        seal_item_id: i32,
        account_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO character_seal_mapping (unique_id, seal_item_id, account_id)
             VALUES ($1, $2, $3)",
        )
        .bind(unique_id)
        .bind(seal_item_id)
        .bind(account_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete a sealed character and its mapping (unseal cleanup).
    pub async fn delete_seal(&self, unique_id: i32) -> Result<(), sqlx::Error> {
        // CASCADE on FK will delete the mapping too
        sqlx::query(
            "DELETE FROM character_seal_items
             WHERE id = (SELECT seal_item_id FROM character_seal_mapping WHERE unique_id = $1)",
        )
        .bind(unique_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Restore a sealed character back to userdata.
    ///
    pub async fn unseal_character(
        &self,
        unique_id: i32,
        account_id: &str,
        char_slot: u8,
    ) -> Result<Option<String>, sqlx::Error> {
        // Load sealed character data
        let seal = match self.load_seal_item_by_unique_id(unique_id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        // Re-insert into userdata
        sqlx::query(
            "INSERT INTO userdata (
                str_user_id, nation, race, class, level, rebirth_level,
                face, hair_rgb, rank, title, exp, loyalty, loyalty_monthly,
                manner_point, fame, city, knights, hp, mp, sp,
                strong, sta, dex, intel, cha, authority, free_points, gold,
                skill_cat1, skill_cat2, skill_cat3, skill_master
             ) VALUES (
                $1, CASE WHEN $3 / 100 = 1 THEN 2 ELSE 1 END,
                $2, $3, $4, $5,
                $6, $7, $8, $9, $10, $11, $12,
                $13, $14, $15, $16, $17, $18, $19,
                $20, $21, $22, $23, $24, $25, $26, $27,
                $28, $29, $30, $31
             )",
        )
        .bind(&seal.char_name) // $1
        .bind(seal.race) // $2
        .bind(seal.class) // $3
        .bind(seal.level) // $4
        .bind(seal.rebirth_level) // $5
        .bind(seal.face) // $6
        .bind(seal.hair_rgb) // $7
        .bind(seal.rank) // $8
        .bind(seal.title) // $9
        .bind(seal.exp) // $10
        .bind(seal.loyalty) // $11
        .bind(seal.loyalty_monthly) // $12
        .bind(seal.manner_point) // $13
        .bind(seal.fame) // $14
        .bind(seal.city) // $15
        .bind(seal.knights) // $16
        .bind(seal.hp) // $17
        .bind(seal.mp) // $18
        .bind(seal.sp) // $19
        .bind(seal.strong) // $20
        .bind(seal.sta) // $21
        .bind(seal.dex) // $22
        .bind(seal.intel) // $23
        .bind(seal.cha) // $24
        .bind(seal.authority) // $25
        .bind(seal.free_points) // $26
        .bind(seal.gold) // $27
        .bind(seal.skill_cat1) // $28
        .bind(seal.skill_cat2) // $29
        .bind(seal.skill_cat3) // $30
        .bind(seal.skill_master) // $31
        .execute(self.pool)
        .await?;

        // Update account_char to assign the character to the slot
        let slot_col = match char_slot {
            0 => "str_char_id1",
            1 => "str_char_id2",
            2 => "str_char_id3",
            _ => "str_char_id4",
        };
        let sql = format!(
            "UPDATE account_char SET {slot_col} = $2, b_char_num = b_char_num + 1 \
             WHERE str_account_id = $1"
        );
        sqlx::query(&sql)
            .bind(account_id)
            .bind(&seal.char_name)
            .execute(self.pool)
            .await?;

        let char_name = seal.char_name.clone();

        // Delete seal data
        self.delete_seal(unique_id).await?;

        Ok(Some(char_name))
    }

    /// Delete character data (userdata + user_items) after sealing.
    ///
    pub async fn delete_character_data(&self, char_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_items WHERE str_user_id = $1")
            .bind(char_name)
            .execute(self.pool)
            .await?;
        sqlx::query("DELETE FROM userdata WHERE str_user_id = $1")
            .bind(char_name)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Load a sealed character summary by item serial number.
    ///
    /// — looks up `m_CharacterSealItemArray.GetData(nSerialNum)`.
    ///
    /// Returns (unique_id, char_name, class, level, exp, race) or None.
    pub async fn load_seal_summary_by_serial(
        &self,
        item_serial: i64,
    ) -> Result<Option<(i32, String, i16, i16, i64, i16)>, sqlx::Error> {
        let row: Option<(i32, String, i16, i16, i64, i16)> = sqlx::query_as(
            "SELECT csm.unique_id, csi.char_name, csi.class, csi.level, \
             COALESCE(csi.exp, 0), COALESCE(csi.race, 0) \
             FROM character_seal_items csi \
             INNER JOIN character_seal_mapping csm ON csm.seal_item_id = csi.id \
             WHERE csi.item_serial = $1",
        )
        .bind(item_serial)
        .fetch_optional(self.pool)
        .await?;
        Ok(row)
    }

    /// Generate a new unique ID for a seal mapping (sequence-based, race-safe).
    pub async fn next_unique_id(&self) -> Result<i32, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT nextval('character_seal_unique_id_seq')")
            .fetch_one(self.pool)
            .await?;
        Ok(row.0 as i32)
    }
}
