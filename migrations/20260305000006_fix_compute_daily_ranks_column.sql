-- Fix compute_daily_ranks() function: column name was 'struid' but actual column is 'str_user_id'
-- Bug: "column struid does not exist" error when compute_daily_ranks() runs
-- Same root cause as Sprint 589 fix for update_ranks()
CREATE OR REPLACE FUNCTION compute_daily_ranks() RETURNS void AS $$
BEGIN
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
        COALESCE(gm.rank_pos, 0)::INTEGER AS gm_rank_cur,
        0 AS gm_rank_prev,
        COALESCE(mh.rank_pos, 0)::INTEGER AS mh_rank_cur,
        0 AS mh_rank_prev,
        COALESCE(sh.rank_pos, 0)::INTEGER AS sh_rank_cur,
        0 AS sh_rank_prev,
        COALESCE(ak.rank_pos, 0)::INTEGER AS ak_rank_cur,
        0 AS ak_rank_prev,
        COALESCE(cw.rank_pos, 0)::INTEGER AS cw_rank_cur,
        0 AS cw_rank_prev,
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
        SELECT str_user_id AS char_id, ROW_NUMBER() OVER (ORDER BY loyalty_monthly DESC) AS rank_pos
        FROM userdata WHERE loyalty_monthly > 0
    ) ak ON ak.char_id = s.char_id
    WHERE COALESCE(gm.rank_pos, mh.rank_pos, sh.rank_pos, ak.rank_pos, cw.rank_pos, up.rank_pos) IS NOT NULL;
END;
$$ LANGUAGE plpgsql;
