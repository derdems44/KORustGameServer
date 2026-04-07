//! Zindan war stages model — maps to the `zindan_war_stages` PostgreSQL table.
//!
//! Source: MSSQL `ZINDAN_WAR_STAGES` table — stage progression for Zindan War event.

/// A row from the `zindan_war_stages` table — defines a single stage
/// in the Zindan War event progression.
///
/// Each stage has a type, ordering index, and time limit.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ZindanWarStageRow {
    /// Primary key index.
    pub idx: i32,
    /// Stage type identifier (determines stage behavior/rules).
    pub stage_type: i16,
    /// Stage ordering number within the war sequence.
    pub stage: i16,
    /// Time limit for this stage in minutes.
    pub time_min: i16,
}
