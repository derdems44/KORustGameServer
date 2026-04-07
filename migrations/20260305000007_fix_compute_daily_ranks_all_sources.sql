-- Fix compute_daily_ranks(): use ALL data sources as base character set
-- Problem: Previous version used user_daily_rank_stats as sole base table.
-- If that table was empty (no player logged out since table creation),
-- ALL rank types returned zero rows — even KNIGHT_ADONIS (loyalty_monthly)
-- which has data in userdata. This caused empty daily rank screen on client.
--
-- Fix: UNION of user_daily_rank_stats + userdata for base character set,
-- and preserve previous ranks (rank_prev) across recomputes.
CREATE OR REPLACE FUNCTION compute_daily_ranks() RETURNS void AS $$
BEGIN
    -- Step 1: Save current ranks to temp table (will become prev_rank)
    DROP TABLE IF EXISTS _dr_prev;
    CREATE TEMP TABLE _dr_prev AS
        SELECT char_id,
               gm_rank_cur AS gm_prev, mh_rank_cur AS mh_prev,
               sh_rank_cur AS sh_prev, ak_rank_cur AS ak_prev,
               cw_rank_cur AS cw_prev, up_rank_cur AS up_prev
        FROM daily_rank;

    -- Step 2: Clear daily_rank
    DELETE FROM daily_rank;

    -- Step 3: Reinsert with computed ranks + preserved previous ranks
    -- Base: UNION of all characters from ANY ranking data source
    INSERT INTO daily_rank (
        char_id,
        gm_rank_cur, gm_rank_prev,
        mh_rank_cur, mh_rank_prev,
        sh_rank_cur, sh_rank_prev,
        ak_rank_cur, ak_rank_prev,
        cw_rank_cur, cw_rank_prev,
        up_rank_cur, up_rank_prev
    )
    SELECT
        ac.char_id,
        COALESCE(gm.rank_pos, 0)::INTEGER,
        COALESCE(prev.gm_prev, 0)::INTEGER,
        COALESCE(mh.rank_pos, 0)::INTEGER,
        COALESCE(prev.mh_prev, 0)::INTEGER,
        COALESCE(sh.rank_pos, 0)::INTEGER,
        COALESCE(prev.sh_prev, 0)::INTEGER,
        COALESCE(ak.rank_pos, 0)::INTEGER,
        COALESCE(prev.ak_prev, 0)::INTEGER,
        COALESCE(cw.rank_pos, 0)::INTEGER,
        COALESCE(prev.cw_prev, 0)::INTEGER,
        COALESCE(up.rank_pos, 0)::INTEGER,
        COALESCE(prev.up_prev, 0)::INTEGER
    FROM (
        -- Characters with any non-zero daily rank stat
        SELECT char_id FROM user_daily_rank_stats
        WHERE gm_total_sold > 0 OR mh_total_kill > 0 OR sh_total_exchange > 0
              OR cw_counter_win > 0 OR up_counter_bles > 0
        UNION
        -- Characters with loyalty (Knight Adonis rank source)
        SELECT str_user_id AS char_id FROM userdata WHERE loyalty_monthly > 0
    ) ac
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY gm_total_sold DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE gm_total_sold > 0
    ) gm ON gm.char_id = ac.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY mh_total_kill DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE mh_total_kill > 0
    ) mh ON mh.char_id = ac.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY sh_total_exchange DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE sh_total_exchange > 0
    ) sh ON sh.char_id = ac.char_id
    LEFT JOIN (
        SELECT str_user_id AS char_id, ROW_NUMBER() OVER (ORDER BY loyalty_monthly DESC) AS rank_pos
        FROM userdata WHERE loyalty_monthly > 0
    ) ak ON ak.char_id = ac.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY cw_counter_win DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE cw_counter_win > 0
    ) cw ON cw.char_id = ac.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY up_counter_bles DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE up_counter_bles > 0
    ) up ON up.char_id = ac.char_id
    LEFT JOIN _dr_prev prev ON prev.char_id = ac.char_id;

    -- Cleanup
    DROP TABLE IF EXISTS _dr_prev;
END;
$$ LANGUAGE plpgsql;
