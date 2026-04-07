//! Repository for Under The Castle monster spawn data.

use sqlx::PgPool;

use crate::models::under_castle::MonsterUnderTheCastleRow;

/// Repository for loading Under The Castle spawn configuration.
pub struct UnderCastleRepository;

impl UnderCastleRepository {
    /// Fetch all monster/NPC spawn entries for the Under The Castle event.
    pub async fn fetch_all(pool: &PgPool) -> Result<Vec<MonsterUnderTheCastleRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterUnderTheCastleRow>(
            "SELECT s_index, s_sid, str_name, b_type, trap_number, x, y, z, by_direction, s_count, b_radius
             FROM monster_under_the_castle
             ORDER BY s_index",
        )
        .fetch_all(pool)
        .await
    }
}
