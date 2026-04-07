-- Sprint 632: Knights Rating Compute Function
-- C++ Reference: KnightsRankSet.h — loads KNIGHTS_RATING table (nRank, shIndex, nPoints)
-- C++ Reference: LoadServerData.cpp:813-844 — LoadKnightsRankTable()
-- C++ Reference: KnightsManager.cpp:1554-1587 — RecvKnightsAllList() (rank + grade update)
--
-- The C++ server computes per-nation clan rankings via an MSSQL stored procedure,
-- then loads the result into m_KnightsRatingArray[KARUS] and m_KnightsRatingArray[ELMORAD].
-- Each clan's m_byRanking is set to its per-nation rank position (1 = top).
--
-- This function replicates that: rank all clans with points > 0 per nation,
-- write to knights_rating, and update the ranking column on the knights table.

-- Step 1: Add nation column to knights_rating (existing table has no nation column)
ALTER TABLE knights_rating ADD COLUMN IF NOT EXISTS nation SMALLINT NOT NULL DEFAULT 0;

-- Step 2: Replace PK — old PK was (rank_pos) which doesn't support per-nation ranks
ALTER TABLE knights_rating DROP CONSTRAINT IF EXISTS knights_rating_pkey;
ALTER TABLE knights_rating ADD PRIMARY KEY (nation, rank_pos);

-- Step 3: Create the compute function
CREATE OR REPLACE FUNCTION compute_knights_rating() RETURNS void AS $$
BEGIN
    -- Clear old rankings
    DELETE FROM knights_rating;

    -- Insert per-nation rankings: ROW_NUMBER within each nation, ordered by points DESC.
    -- Tie-break by clan ID ascending (lower = older clan = higher rank).
    -- C++ Reference: MSSQL stored proc populates KNIGHTS_RATING per nation.
    INSERT INTO knights_rating (nation, rank_pos, clan_id, points)
    SELECT
        k.nation,
        ROW_NUMBER() OVER (PARTITION BY k.nation ORDER BY k.points DESC, k.id_num ASC)::INTEGER,
        k.id_num,
        k.points
    FROM knights k
    WHERE k.points > 0;

    -- Update ranking column on the knights table from computed ratings.
    -- First reset all to 0, then set ranked clans.
    UPDATE knights SET ranking = 0;

    UPDATE knights k
    SET ranking = kr.rank_pos::SMALLINT
    FROM knights_rating kr
    WHERE k.id_num = kr.clan_id;
END;
$$ LANGUAGE plpgsql;
