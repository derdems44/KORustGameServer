//! King system model — maps to the `king_system` PostgreSQL table.
//! - `KingSystem.h` — `CKingSystem` class fields
//! - MSSQL `KING_SYSTEM` table (30 columns, 2 rows)
//! One row per nation (1=Karus, 2=Elmorad), storing election schedule,
//! impeachment state, active events, treasury, tax, and current king info.

/// A single king system row from the database.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KingSystemRow {
    /// Nation identifier: 1=Karus, 2=Elmorad.
    pub by_nation: i16,

    /// Election type (`ElectionType` enum).
    /// 0=NO_TERM, 1=NOMINATION, 2=PRE_ELECTION, 3=ELECTION, 6=TERM_STARTED, 7=TERM_ENDED.
    pub by_type: i16,

    /// Scheduled election year.
    pub s_year: i16,
    /// Scheduled election month (1-12).
    pub by_month: i16,
    /// Scheduled election day.
    pub by_day: i16,
    /// Scheduled election hour.
    pub by_hour: i16,
    /// Scheduled election minute.
    pub by_minute: i16,

    /// Impeachment type/state.
    pub by_im_type: i16,
    /// Impeachment schedule year.
    pub s_im_year: i16,
    /// Impeachment schedule month.
    pub by_im_month: i16,
    /// Impeachment schedule day.
    pub by_im_day: i16,
    /// Impeachment schedule hour.
    pub by_im_hour: i16,
    /// Impeachment schedule minute.
    pub by_im_minute: i16,

    /// Noah (coin) event multiplier (0=off, 1-3=amount).
    pub by_noah_event: i16,
    /// Day the noah event started.
    pub by_noah_event_day: i16,
    /// Hour the noah event started.
    pub by_noah_event_hour: i16,
    /// Minute the noah event started.
    pub by_noah_event_minute: i16,
    /// Noah event duration in minutes.
    pub s_noah_event_duration: i16,

    /// EXP event multiplier (0=off, 10/30/50).
    pub by_exp_event: i16,
    /// Day the EXP event started.
    pub by_exp_event_day: i16,
    /// Hour the EXP event started.
    pub by_exp_event_hour: i16,
    /// Minute the EXP event started.
    pub by_exp_event_minute: i16,
    /// EXP event duration in minutes.
    pub s_exp_event_duration: i16,

    /// Tribute amount.
    pub n_tribute: i32,
    /// Territory tariff rate (0-10).
    pub by_territory_tariff: i16,
    /// Territory tax collected.
    pub n_territory_tax: i32,
    /// National treasury balance.
    pub n_national_treasury: i32,

    /// Current king's character name (empty if no king).
    pub str_king_name: String,
    /// Current king's clan ID.
    pub s_king_clan_id: i16,

    /// Impeachment requester's character name.
    pub str_im_request_id: String,

    /// New king name determined after election (persisted during TERM_ENDED phase).
    pub str_new_king_name: String,
    /// Votes for the winning king candidate.
    pub king_votes: i32,
    /// Total votes cast in the election.
    pub total_votes: i32,
}

/// A row from the `king_election_list` table.
/// byType: 3=senator, 4=candidate for King.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KingElectionListRow {
    pub by_nation: i16,
    pub by_type: i16,
    pub str_name: String,
    pub n_knights: i16,
    pub n_money: i32,
}

/// A row from the `king_nomination_list` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KingNominationListRow {
    pub by_nation: i16,
    pub str_nominator: String,
    pub str_nominee: String,
}

/// A row from the `king_candidacy_notice_board` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KingCandidacyNoticeBoardRow {
    pub by_nation: i16,
    pub str_user_id: String,
    pub str_notice: String,
}
