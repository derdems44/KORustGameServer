//! Coefficient repository — loads class coefficient data from PostgreSQL.
//!
//! C++ Reference:
//! - `GameServer/LoadServerData.cpp` — `LoadCoefficientTable()`
//! - `shared/database/CoefficientSet.h` — `CCoefficientSet`

use crate::models::CoefficientRow;
use crate::DbPool;

/// Repository for `coefficient` table access.
pub struct CoefficientRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> CoefficientRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all class coefficient rows (bulk load at startup).
    ///
    /// Returns one row per class (30 rows total: 101-115 Karus, 201-215 El Morad).
    /// C++ Reference: `CGameServerDlg::LoadCoefficientTable()`
    pub async fn load_all_coefficients(&self) -> Result<Vec<CoefficientRow>, sqlx::Error> {
        sqlx::query_as::<_, CoefficientRow>(
            "SELECT s_class, short_sword, jamadar, sword, axe, club, spear, pole, \
             staff, bow, hp, mp, sp, ac, hitrate, evasionrate \
             FROM coefficient ORDER BY s_class",
        )
        .fetch_all(self.pool)
        .await
    }
}
