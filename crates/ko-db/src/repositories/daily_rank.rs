//! Daily rank repository — load ranking data from DB.
//!
//! C++ Reference: `CDailyRankSet` — loaded at startup via `LoadDailyRank()`.

use sqlx::PgPool;

use crate::models::daily_rank::{DailyRankRow, DrakiTowerDailyRankRow, UserDailyRankStatsRow};

/// Repository for daily ranking data.
pub struct DailyRankRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> DailyRankRepository<'a> {
    /// Create a new daily rank repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all daily rank entries from the `daily_rank` table.
    ///
    /// C++ Reference: `LoadDailyRank()` — loads entire table at startup.
    pub async fn load_all(&self) -> Result<Vec<DailyRankRow>, sqlx::Error> {
        sqlx::query_as::<_, DailyRankRow>("SELECT * FROM daily_rank")
            .fetch_all(self.pool)
            .await
    }

    /// Load Draki Tower daily rank entries filtered by class.
    ///
    /// C++ Reference: `LoadDrakiTowerDailyRank(class_id)` — async DB query
    /// sorted by stage DESC, time ASC (highest stage first, fastest time wins).
    pub async fn load_draki_by_class(
        &self,
        class_id: i32,
    ) -> Result<Vec<DrakiTowerDailyRankRow>, sqlx::Error> {
        sqlx::query_as::<_, DrakiTowerDailyRankRow>(
            "SELECT * FROM draki_tower_daily_rank \
             WHERE class_id = $1 \
             ORDER BY draki_stage DESC, draki_time ASC",
        )
        .bind(class_id)
        .fetch_all(self.pool)
        .await
    }

    /// Load daily rank raw stats for a single character (on login).
    ///
    /// C++ Reference: `LOAD_DAILY_RANK_USER(?)` stored procedure in `DBAgent.cpp:5471-5498`
    pub async fn load_user_stats(
        &self,
        char_id: &str,
    ) -> Result<Option<UserDailyRankStatsRow>, sqlx::Error> {
        sqlx::query_as::<_, UserDailyRankStatsRow>(
            "SELECT * FROM user_daily_rank_stats WHERE char_id = $1",
        )
        .bind(char_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save daily rank raw stats for a character (on logout / periodic save).
    ///
    /// C++ Reference: `UPDATE_USER_DAILY_RANK(?, ?, ?, ?, ?, ?)` in `DBAgent.cpp:5529-5544`
    pub async fn save_user_stats(
        &self,
        char_id: &str,
        gm_total_sold: i64,
        mh_total_kill: i64,
        sh_total_exchange: i64,
        cw_counter_win: i64,
        up_counter_bles: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_daily_rank_stats \
             (char_id, gm_total_sold, mh_total_kill, sh_total_exchange, cw_counter_win, up_counter_bles) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             ON CONFLICT (char_id) DO UPDATE SET \
                gm_total_sold = $2, \
                mh_total_kill = $3, \
                sh_total_exchange = $4, \
                cw_counter_win = $5, \
                up_counter_bles = $6",
        )
        .bind(char_id)
        .bind(gm_total_sold)
        .bind(mh_total_kill)
        .bind(sh_total_exchange)
        .bind(cw_counter_win)
        .bind(up_counter_bles)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Initialize daily_rank row for a newly created character.
    ///
    /// C++ Reference: `CREATE_NEW_CHAR` SP — `INSERT INTO DAILY_RANK(StrUserID) VALUES (@strCharID)`
    pub async fn init_for_new_char(&self, char_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO daily_rank (char_id) VALUES ($1) ON CONFLICT (char_id) DO NOTHING",
        )
        .bind(char_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Initialize user_daily_rank_stats row for a newly created character.
    ///
    /// C++ Reference: `CREATE_NEW_CHAR` SP — ensures rank stats exist from character creation.
    pub async fn init_stats_for_new_char(&self, char_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_daily_rank_stats (char_id) VALUES ($1) \
             ON CONFLICT (char_id) DO NOTHING",
        )
        .bind(char_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Execute compute_daily_ranks() to recompute rank positions.
    ///
    /// C++ Reference: `UPDATE_RANKS` stored procedure, called at startup.
    pub async fn compute_ranks(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT compute_daily_ranks()")
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
