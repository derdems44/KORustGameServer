-- Per-player daily rank raw stats table.
-- C++ Reference: _USER_DAILY_RANK struct in GameDefine.h:4457-4466
-- Tracks cumulative stats used to compute daily rankings.
-- Stats are loaded on login and saved on logout.

CREATE TABLE IF NOT EXISTS user_daily_rank_stats (
    char_id             VARCHAR(21)     NOT NULL,
    gm_total_sold       BIGINT          NOT NULL DEFAULT 0,
    mh_total_kill       BIGINT          NOT NULL DEFAULT 0,
    sh_total_exchange   BIGINT          NOT NULL DEFAULT 0,
    cw_counter_win      BIGINT          NOT NULL DEFAULT 0,
    up_counter_bles     BIGINT          NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id)
);

-- compute_daily_ranks() PostgreSQL function
-- C++ Reference: UPDATE_RANKS stored procedure (for daily rank portion)
-- Computes rank positions from raw stats and populates the daily_rank table.
-- Called at startup and periodically (every 15 minutes).
CREATE OR REPLACE FUNCTION compute_daily_ranks() RETURNS void AS $$
BEGIN
    -- Move current ranks to previous ranks, then recompute current ranks
    -- Step 1: For each rank type, compute rank positions from raw stats

    -- Clear and repopulate daily_rank table
    DELETE FROM daily_rank;

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
        s.char_id,
        -- Grand Merchant: ranked by gm_total_sold DESC
        COALESCE(gm.rank_pos, 0)::INTEGER AS gm_rank_cur,
        0 AS gm_rank_prev,
        -- Monster Hunter: ranked by mh_total_kill DESC
        COALESCE(mh.rank_pos, 0)::INTEGER AS mh_rank_cur,
        0 AS mh_rank_prev,
        -- Shozin: ranked by sh_total_exchange DESC
        COALESCE(sh.rank_pos, 0)::INTEGER AS sh_rank_cur,
        0 AS sh_rank_prev,
        -- Knight Adonis: ranked by loyalty_monthly from userdata
        COALESCE(ak.rank_pos, 0)::INTEGER AS ak_rank_cur,
        0 AS ak_rank_prev,
        -- Hero of Chaos: ranked by cw_counter_win DESC
        COALESCE(cw.rank_pos, 0)::INTEGER AS cw_rank_cur,
        0 AS cw_rank_prev,
        -- Disciple of Keron: ranked by up_counter_bles DESC
        COALESCE(up.rank_pos, 0)::INTEGER AS up_rank_cur,
        0 AS up_rank_prev
    FROM user_daily_rank_stats s
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY gm_total_sold DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE gm_total_sold > 0
    ) gm ON gm.char_id = s.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY mh_total_kill DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE mh_total_kill > 0
    ) mh ON mh.char_id = s.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY sh_total_exchange DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE sh_total_exchange > 0
    ) sh ON sh.char_id = s.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY cw_counter_win DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE cw_counter_win > 0
    ) cw ON cw.char_id = s.char_id
    LEFT JOIN (
        SELECT char_id, ROW_NUMBER() OVER (ORDER BY up_counter_bles DESC) AS rank_pos
        FROM user_daily_rank_stats WHERE up_counter_bles > 0
    ) up ON up.char_id = s.char_id
    LEFT JOIN (
        SELECT struid AS char_id, ROW_NUMBER() OVER (ORDER BY loyalty_monthly DESC) AS rank_pos
        FROM userdata WHERE loyalty_monthly > 0
    ) ak ON ak.char_id = s.char_id
    WHERE COALESCE(gm.rank_pos, mh.rank_pos, sh.rank_pos, ak.rank_pos, cw.rank_pos, up.rank_pos) IS NOT NULL;
END;
$$ LANGUAGE plpgsql;
