-- Fix update_ranks() function: column name was 'struid' but actual column is 'str_user_id'
-- Bug: "column u.struid does not exist" error at server startup

CREATE OR REPLACE FUNCTION update_ranks() RETURNS void AS $$
DECLARE
    max_rank CONSTANT INT := 100;
BEGIN
    -- Truncate old rankings
    DELETE FROM user_personal_rank;
    DELETE FROM user_knights_rank;

    -- user_personal_rank: pair Karus rank N with Elmorad rank N
    -- ROW_NUMBER within each nation, ordered by loyalty_monthly DESC
    INSERT INTO user_personal_rank (rank_pos, rank_name, karus_user_id, karus_clan_name, karus_knights, karus_loyalty, elmo_user_id, elmo_clan_name, elmo_knights, elmo_loyalty, salary)
    SELECT
        COALESCE(k.rn, e.rn) AS rank_pos,
        '' AS rank_name,
        COALESCE(k.str_user_id, '') AS karus_user_id,
        COALESCE(k.clan_name, '') AS karus_clan_name,
        COALESCE(k.knights, 0) AS karus_knights,
        COALESCE(k.loyalty_monthly, 0) AS karus_loyalty,
        COALESCE(e.str_user_id, '') AS elmo_user_id,
        COALESCE(e.clan_name, '') AS elmo_clan_name,
        COALESCE(e.knights, 0) AS elmo_knights,
        COALESCE(e.loyalty_monthly, 0) AS elmo_loyalty,
        0 AS salary
    FROM (
        SELECT ROW_NUMBER() OVER (ORDER BY u.loyalty_monthly DESC, u.str_user_id ASC) AS rn,
               u.str_user_id, COALESCE(kt.id_name, '') AS clan_name, u.knights, u.loyalty_monthly
        FROM userdata u
        LEFT JOIN knights kt ON kt.id_num = u.knights
        WHERE u.nation = 1 AND u.loyalty_monthly > 0
        LIMIT max_rank
    ) k
    FULL OUTER JOIN (
        SELECT ROW_NUMBER() OVER (ORDER BY u.loyalty_monthly DESC, u.str_user_id ASC) AS rn,
               u.str_user_id, COALESCE(kt.id_name, '') AS clan_name, u.knights, u.loyalty_monthly
        FROM userdata u
        LEFT JOIN knights kt ON kt.id_num = u.knights
        WHERE u.nation = 2 AND u.loyalty_monthly > 0
        LIMIT max_rank
    ) e ON k.rn = e.rn;

    -- user_knights_rank: same but only users who belong to a clan (knights > 0)
    INSERT INTO user_knights_rank (rank_pos, rank_name, karus_user_id, karus_knights_name, karus_knights, karus_loyalty, elmo_user_id, elmo_knights_name, elmo_knights, elmo_loyalty, salary)
    SELECT
        COALESCE(k.rn, e.rn) AS rank_pos,
        '' AS rank_name,
        COALESCE(k.str_user_id, '') AS karus_user_id,
        COALESCE(k.clan_name, '') AS karus_knights_name,
        COALESCE(k.knights, 0) AS karus_knights,
        COALESCE(k.loyalty_monthly, 0) AS karus_loyalty,
        COALESCE(e.str_user_id, '') AS elmo_user_id,
        COALESCE(e.clan_name, '') AS elmo_knights_name,
        COALESCE(e.knights, 0) AS elmo_knights,
        COALESCE(e.loyalty_monthly, 0) AS elmo_loyalty,
        0 AS salary
    FROM (
        SELECT ROW_NUMBER() OVER (ORDER BY u.loyalty_monthly DESC, u.str_user_id ASC) AS rn,
               u.str_user_id, COALESCE(kt.id_name, '') AS clan_name, u.knights, u.loyalty_monthly
        FROM userdata u
        LEFT JOIN knights kt ON kt.id_num = u.knights
        WHERE u.nation = 1 AND u.knights > 0 AND u.loyalty_monthly > 0
        LIMIT max_rank
    ) k
    FULL OUTER JOIN (
        SELECT ROW_NUMBER() OVER (ORDER BY u.loyalty_monthly DESC, u.str_user_id ASC) AS rn,
               u.str_user_id, COALESCE(kt.id_name, '') AS clan_name, u.knights, u.loyalty_monthly
        FROM userdata u
        LEFT JOIN knights kt ON kt.id_num = u.knights
        WHERE u.nation = 2 AND u.knights > 0 AND u.loyalty_monthly > 0
        LIMIT max_rank
    ) e ON k.rn = e.rn;
END;
$$ LANGUAGE plpgsql;
