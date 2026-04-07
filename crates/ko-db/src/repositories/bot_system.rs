//! Bot system repository — loading farm bots, merchant bots, rankings.
//!
//! C++ Reference: `CDBAgent::LoadBotTable()`, `CDBAgent::LoadBotHandlerMerchantTable()`

use crate::models::bot_system::{
    BotHandlerFarmRow, BotHandlerMerchantRow, BotKnightsRankRow, BotMerchantDataRow,
    BotPersonalRankRow, UserBotRow,
};
use crate::DbPool;

/// Repository for bot system data.
pub struct BotSystemRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> BotSystemRepository<'a> {
    /// Create a new bot system repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all farm bot records.
    ///
    /// C++ Reference: `CDBAgent::LoadBotTable()` -> `LOAD_BOT_HANDLER` stored proc
    pub async fn load_all_farm_bots(&self) -> Result<Vec<BotHandlerFarmRow>, sqlx::Error> {
        sqlx::query_as::<_, BotHandlerFarmRow>(
            "SELECT id, str_user_id, nation, race, class, hair_rgb, level, face, \
             knights, fame, zone, px, pz, py, str_item, cover_title, reb_level, \
             str_skill, gold, points, strong, sta, dex, intel, cha, \
             loyalty, loyalty_monthly, donated_np \
             FROM bot_handler_farm ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all merchant bot templates.
    ///
    /// C++ Reference: `CDBAgent::LoadBotHandlerMerchantTable()` -> `LOAD_BOT_HANDLER_MERCHANT`
    pub async fn load_all_merchant_templates(
        &self,
    ) -> Result<Vec<BotHandlerMerchantRow>, sqlx::Error> {
        sqlx::query_as::<_, BotHandlerMerchantRow>(
            "SELECT s_index, bot_merchant_type, bot_item_num, bot_item_count, \
             bot_item_price, bot_merchant_message \
             FROM bot_handler_merchant ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all pre-configured merchant bot stall data.
    ///
    /// C++ Reference: `_BOT_SAVE_DATA` used in merchant bot spawning
    pub async fn load_all_merchant_data(&self) -> Result<Vec<BotMerchantDataRow>, sqlx::Error> {
        sqlx::query_as::<_, BotMerchantDataRow>(
            "SELECT n_index, advert_message, \
             n_num1, n_price1, s_count1, s_duration1, is_kc1, \
             n_num2, n_price2, s_count2, s_duration2, is_kc2, \
             n_num3, n_price3, s_count3, s_duration3, is_kc3, \
             n_num4, n_price4, s_count4, s_duration4, is_kc4, \
             n_num5, n_price5, s_count5, s_duration5, is_kc5, \
             n_num6, n_price6, s_count6, s_duration6, is_kc6, \
             n_num7, n_price7, s_count7, s_duration7, is_kc7, \
             n_num8, n_price8, s_count8, s_duration8, is_kc8, \
             n_num9, n_price9, s_count9, s_duration9, is_kc9, \
             n_num10, n_price10, s_count10, s_duration10, is_kc10, \
             n_num11, n_price11, s_count11, s_duration11, is_kc11, \
             n_num12, n_price12, s_count12, s_duration12, is_kc12, \
             px, pz, py, minute, zone, s_direction, merchant_type \
             FROM bot_merchant_data ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all user bot records.
    pub async fn load_all_user_bots(&self) -> Result<Vec<UserBotRow>, sqlx::Error> {
        sqlx::query_as::<_, UserBotRow>(
            "SELECT id, str_user_id, nation, race, class, hair_rgb, level, face, \
             knights, fame, zone, px, pz, py, str_item, cover_title, reb_level, \
             str_skill, gold, points, strong, sta, dex, intel, cha \
             FROM user_bots ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all bot knights ranking rows.
    ///
    /// C++ Reference: `GetBotRank()` for knights ranking
    pub async fn load_all_knights_ranks(&self) -> Result<Vec<BotKnightsRankRow>, sqlx::Error> {
        sqlx::query_as::<_, BotKnightsRankRow>(
            "SELECT sh_index, str_name, str_elmo_user_id, str_elmo_knights_name, \
             s_elmo_knights, n_elmo_loyalty, str_karus_user_id, str_karus_knights_name, \
             s_karus_knights, n_karus_loyalty, n_money \
             FROM bot_knights_rank ORDER BY sh_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all bot personal ranking rows.
    ///
    /// C++ Reference: `GetBotRank()` for personal ranking
    pub async fn load_all_personal_ranks(&self) -> Result<Vec<BotPersonalRankRow>, sqlx::Error> {
        sqlx::query_as::<_, BotPersonalRankRow>(
            "SELECT n_rank, str_rank_name, n_elmo_up, str_elmo_user_id, str_elmo_clan_name, \
             s_elmo_knights, n_elmo_loyalty_monthly, n_elmo_check, n_karus_up, \
             str_karus_user_id, str_karus_clan_name, s_karus_knights, \
             n_karus_loyalty_monthly, n_karus_check, n_salary, update_date \
             FROM bot_personal_rank ORDER BY n_rank",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Update bot loyalty values (called when bot disconnects).
    ///
    /// C++ Reference: `CDBAgent::UpdateBotUser()`
    pub async fn update_bot_loyalty(
        &self,
        str_user_id: &str,
        loyalty: i32,
        loyalty_monthly: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE bot_handler_farm SET loyalty = $1, loyalty_monthly = $2 \
             WHERE str_user_id = $3",
        )
        .bind(loyalty)
        .bind(loyalty_monthly)
        .bind(str_user_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
