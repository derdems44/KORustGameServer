-- Sprint 625: Fix compute_daily_ranks() — add authority + level filters.
--
-- MSSQL UPDATE_RANKS: only ranks players with Authority=1 AND Level>30.
-- Without this, GMs (authority=0) and low-level chars pollute rankings.
--
-- Also filter user_daily_rank_stats subqueries by joining userdata to
-- exclude banned/GM characters from all rank types.

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
    -- Base: UNION of all ELIGIBLE characters (authority=1, level>30)
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
        -- Characters with any non-zero daily rank stat (filtered by authority+level)
        SELECT drs.char_id FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE u.authority = 1 AND u.level > 30
          AND (drs.gm_total_sold > 0 OR drs.mh_total_kill > 0 OR drs.sh_total_exchange > 0
               OR drs.cw_counter_win > 0 OR drs.up_counter_bles > 0)
        UNION
        -- Characters with loyalty (Knight Adonis rank source)
        SELECT str_user_id AS char_id FROM userdata
        WHERE loyalty_monthly > 0 AND authority = 1 AND level > 30
    ) ac
    LEFT JOIN (
        SELECT drs.char_id, ROW_NUMBER() OVER (ORDER BY drs.gm_total_sold DESC) AS rank_pos
        FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE drs.gm_total_sold > 0 AND u.authority = 1 AND u.level > 30
    ) gm ON gm.char_id = ac.char_id
    LEFT JOIN (
        SELECT drs.char_id, ROW_NUMBER() OVER (ORDER BY drs.mh_total_kill DESC) AS rank_pos
        FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE drs.mh_total_kill > 0 AND u.authority = 1 AND u.level > 30
    ) mh ON mh.char_id = ac.char_id
    LEFT JOIN (
        SELECT drs.char_id, ROW_NUMBER() OVER (ORDER BY drs.sh_total_exchange DESC) AS rank_pos
        FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE drs.sh_total_exchange > 0 AND u.authority = 1 AND u.level > 30
    ) sh ON sh.char_id = ac.char_id
    LEFT JOIN (
        SELECT str_user_id AS char_id, ROW_NUMBER() OVER (ORDER BY loyalty_monthly DESC) AS rank_pos
        FROM userdata WHERE loyalty_monthly > 0 AND authority = 1 AND level > 30
    ) ak ON ak.char_id = ac.char_id
    LEFT JOIN (
        SELECT drs.char_id, ROW_NUMBER() OVER (ORDER BY drs.cw_counter_win DESC) AS rank_pos
        FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE drs.cw_counter_win > 0 AND u.authority = 1 AND u.level > 30
    ) cw ON cw.char_id = ac.char_id
    LEFT JOIN (
        SELECT drs.char_id, ROW_NUMBER() OVER (ORDER BY drs.up_counter_bles DESC) AS rank_pos
        FROM user_daily_rank_stats drs
        INNER JOIN userdata u ON u.str_user_id = drs.char_id
        WHERE drs.up_counter_bles > 0 AND u.authority = 1 AND u.level > 30
    ) up ON up.char_id = ac.char_id
    LEFT JOIN _dr_prev prev ON prev.char_id = ac.char_id;

    -- Cleanup
    DROP TABLE IF EXISTS _dr_prev;
END;
$$ LANGUAGE plpgsql;
