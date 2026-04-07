//! King system repository — loads and updates king system data from PostgreSQL.
//! - `KingSystem.cpp` — `CKingSystem` loaded from `KING_SYSTEM` table
//! - `DBAgent.cpp` — database request handlers for king events/tax/elections

use crate::models::{
    KingCandidacyNoticeBoardRow, KingElectionListRow, KingNominationListRow, KingSystemRow,
};
use crate::DbPool;

/// Repository for `king_system` table access.
pub struct KingRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> KingRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all king system rows (bulk load at startup).
    ///
    /// Returns one row per nation (2 rows total: nation 1=Karus, 2=Elmorad).
    ///
    pub async fn load_all(&self) -> Result<Vec<KingSystemRow>, sqlx::Error> {
        sqlx::query_as::<_, KingSystemRow>(
            "SELECT by_nation, by_type, s_year, by_month, by_day, by_hour, by_minute, \
             by_im_type, s_im_year, by_im_month, by_im_day, by_im_hour, by_im_minute, \
             by_noah_event, by_noah_event_day, by_noah_event_hour, by_noah_event_minute, \
             s_noah_event_duration, \
             by_exp_event, by_exp_event_day, by_exp_event_hour, by_exp_event_minute, \
             s_exp_event_duration, \
             n_tribute, by_territory_tariff, n_territory_tax, n_national_treasury, \
             str_king_name, s_king_clan_id, str_im_request_id, \
             str_new_king_name, king_votes, total_votes \
             FROM king_system ORDER BY by_nation",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Update king event state (noah/exp event activation).
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn update_noah_event(
        &self,
        nation: i16,
        by_noah_event: i16,
        by_noah_event_day: i16,
        by_noah_event_hour: i16,
        by_noah_event_minute: i16,
        s_noah_event_duration: i16,
        n_national_treasury: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_system SET \
             by_noah_event = $1, by_noah_event_day = $2, by_noah_event_hour = $3, \
             by_noah_event_minute = $4, s_noah_event_duration = $5, \
             n_national_treasury = $6 \
             WHERE by_nation = $7",
        )
        .bind(by_noah_event)
        .bind(by_noah_event_day)
        .bind(by_noah_event_hour)
        .bind(by_noah_event_minute)
        .bind(s_noah_event_duration)
        .bind(n_national_treasury)
        .bind(nation)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update EXP event state.
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn update_exp_event(
        &self,
        nation: i16,
        by_exp_event: i16,
        by_exp_event_day: i16,
        by_exp_event_hour: i16,
        by_exp_event_minute: i16,
        s_exp_event_duration: i16,
        n_national_treasury: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_system SET \
             by_exp_event = $1, by_exp_event_day = $2, by_exp_event_hour = $3, \
             by_exp_event_minute = $4, s_exp_event_duration = $5, \
             n_national_treasury = $6 \
             WHERE by_nation = $7",
        )
        .bind(by_exp_event)
        .bind(by_exp_event_day)
        .bind(by_exp_event_hour)
        .bind(by_exp_event_minute)
        .bind(s_exp_event_duration)
        .bind(n_national_treasury)
        .bind(nation)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update territory tariff rate.
    ///
    pub async fn update_tariff(
        &self,
        nation: i16,
        by_territory_tariff: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE king_system SET by_territory_tariff = $1 WHERE by_nation = $2")
            .bind(by_territory_tariff)
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Collect territory tax (set to 0 after king collects).
    ///
    pub async fn collect_territory_tax(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE king_system SET n_territory_tax = 0 WHERE by_nation = $1")
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Update national treasury after event spending.
    pub async fn update_treasury(
        &self,
        nation: i16,
        n_national_treasury: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE king_system SET n_national_treasury = $1 WHERE by_nation = $2")
            .bind(n_national_treasury)
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Update election status type.
    ///
    pub async fn update_election_status(
        &self,
        nation: i16,
        by_type: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE king_system SET by_type = $1 WHERE by_nation = $2")
            .bind(by_type)
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Update the king name and clan after election.
    ///
    pub async fn update_king(
        &self,
        nation: i16,
        king_name: &str,
        king_clan_id: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_system SET str_king_name = $1, s_king_clan_id = $2 WHERE by_nation = $3",
        )
        .bind(king_name)
        .bind(king_clan_id)
        .bind(nation)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Save election result (new king name + vote counts) for crash recovery.
    ///
    /// Called when election enters TERM_ENDED phase. Cleared when king is assigned.
    pub async fn save_election_result(
        &self,
        nation: i16,
        new_king_name: &str,
        king_votes: i32,
        total_votes: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_system SET str_new_king_name = $1, king_votes = $2, \
             total_votes = $3 WHERE by_nation = $4",
        )
        .bind(new_king_name)
        .bind(king_votes)
        .bind(total_votes)
        .bind(nation)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Clear election result after king has been assigned.
    pub async fn clear_election_result(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_system SET str_new_king_name = '', king_votes = 0, \
             total_votes = 0 WHERE by_nation = $1",
        )
        .bind(nation)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    // ── Election List Operations ────────────────────────────────────────

    /// Load all election list entries for a nation.
    ///
    pub async fn load_election_list(
        &self,
        nation: i16,
    ) -> Result<Vec<KingElectionListRow>, sqlx::Error> {
        sqlx::query_as::<_, KingElectionListRow>(
            "SELECT by_nation, by_type, str_name, n_knights, n_money \
             FROM king_election_list WHERE by_nation = $1 ORDER BY by_type, str_name",
        )
        .bind(nation)
        .fetch_all(self.pool)
        .await
    }

    /// Insert or update an election list entry (senator or candidate).
    ///
    pub async fn upsert_election_list(
        &self,
        nation: i16,
        by_type: i16,
        name: &str,
        knights_id: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO king_election_list (by_nation, by_type, str_name, n_knights) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (by_nation, by_type, str_name) DO UPDATE SET n_knights = $4",
        )
        .bind(nation)
        .bind(by_type)
        .bind(name)
        .bind(knights_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete an election list entry.
    pub async fn delete_election_list_entry(
        &self,
        nation: i16,
        by_type: i16,
        name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM king_election_list WHERE by_nation = $1 AND by_type = $2 AND str_name = $3",
        )
        .bind(nation)
        .bind(by_type)
        .bind(name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Clear all election lists for a nation (used during election reset).
    pub async fn clear_election_lists(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM king_election_list WHERE by_nation = $1")
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Add a vote increment to a candidate's entry.
    pub async fn increment_votes(
        &self,
        nation: i16,
        candidate_name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE king_election_list SET n_money = n_money + 1 \
             WHERE by_nation = $1 AND by_type = 4 AND str_name = $2",
        )
        .bind(nation)
        .bind(candidate_name)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    // ── Nomination List Operations ──────────────────────────────────────

    /// Load all nomination list entries for a nation.
    ///
    pub async fn load_nomination_list(
        &self,
        nation: i16,
    ) -> Result<Vec<KingNominationListRow>, sqlx::Error> {
        sqlx::query_as::<_, KingNominationListRow>(
            "SELECT by_nation, str_nominator, str_nominee \
             FROM king_nomination_list WHERE by_nation = $1",
        )
        .bind(nation)
        .fetch_all(self.pool)
        .await
    }

    /// Insert a nomination.
    pub async fn insert_nomination(
        &self,
        nation: i16,
        nominator: &str,
        nominee: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO king_nomination_list (by_nation, str_nominator, str_nominee) \
             VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        )
        .bind(nation)
        .bind(nominator)
        .bind(nominee)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete a nomination entry.
    pub async fn delete_nomination(&self, nation: i16, nominee: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM king_nomination_list WHERE by_nation = $1 AND str_nominee = $2")
            .bind(nation)
            .bind(nominee)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Clear all nominations for a nation.
    pub async fn clear_nominations(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM king_nomination_list WHERE by_nation = $1")
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    // ── Notice Board Operations ─────────────────────────────────────────

    /// Load all candidacy notice board entries for a nation.
    ///
    pub async fn load_notice_board(
        &self,
        nation: i16,
    ) -> Result<Vec<KingCandidacyNoticeBoardRow>, sqlx::Error> {
        sqlx::query_as::<_, KingCandidacyNoticeBoardRow>(
            "SELECT by_nation, str_user_id, str_notice \
             FROM king_candidacy_notice_board WHERE by_nation = $1",
        )
        .bind(nation)
        .fetch_all(self.pool)
        .await
    }

    /// Insert or update a candidacy notice board entry.
    pub async fn upsert_notice_board(
        &self,
        nation: i16,
        user_id: &str,
        notice: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO king_candidacy_notice_board (by_nation, str_user_id, str_notice) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (by_nation, str_user_id) DO UPDATE SET str_notice = $3",
        )
        .bind(nation)
        .bind(user_id)
        .bind(notice)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Clear all notice board entries for a nation.
    pub async fn clear_notice_board(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM king_candidacy_notice_board WHERE by_nation = $1")
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    // ── Vote Tracking ───────────────────────────────────────────────────

    /// Record a vote (returns false if account already voted).
    pub async fn record_vote(
        &self,
        nation: i16,
        account_id: &str,
        user_id: &str,
        nominee: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO king_election_votes (by_nation, str_account_id, str_user_id, str_nominee) \
             VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        )
        .bind(nation)
        .bind(account_id)
        .bind(user_id)
        .bind(nominee)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Clear all votes for a nation (used during election reset).
    pub async fn clear_votes(&self, nation: i16) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM king_election_votes WHERE by_nation = $1")
            .bind(nation)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
