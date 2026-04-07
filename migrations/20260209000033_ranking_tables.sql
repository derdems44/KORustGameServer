-- Ranking system tables
-- C++ Reference: UserPersonalRankSet.h, UserKnightsRankSet.h, KnightsRankSet.h

-- Personal user rankings per nation (loaded from DB periodically)
-- C++ tables: USER_PERSONAL_RANK, USER_KNIGHTS_RANK
CREATE TABLE IF NOT EXISTS user_personal_rank (
    rank_pos        SMALLINT        NOT NULL,
    rank_name       VARCHAR(100)    NOT NULL DEFAULT '',
    elmo_user_id    VARCHAR(21)     NOT NULL DEFAULT '',
    elmo_clan_name  VARCHAR(21)     NOT NULL DEFAULT '',
    elmo_knights    SMALLINT        NOT NULL DEFAULT 0,
    elmo_loyalty    INTEGER         NOT NULL DEFAULT 0,
    karus_user_id   VARCHAR(21)     NOT NULL DEFAULT '',
    karus_clan_name VARCHAR(21)     NOT NULL DEFAULT '',
    karus_knights   SMALLINT        NOT NULL DEFAULT 0,
    karus_loyalty   INTEGER         NOT NULL DEFAULT 0,
    salary          INTEGER         NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_pos)
);

CREATE TABLE IF NOT EXISTS user_knights_rank (
    rank_pos            SMALLINT        NOT NULL,
    rank_name           VARCHAR(100)    NOT NULL DEFAULT '',
    elmo_user_id        VARCHAR(21)     NOT NULL DEFAULT '',
    elmo_knights_name   VARCHAR(21)     NOT NULL DEFAULT '',
    elmo_knights        SMALLINT        NOT NULL DEFAULT 0,
    elmo_loyalty        INTEGER         NOT NULL DEFAULT 0,
    karus_user_id       VARCHAR(21)     NOT NULL DEFAULT '',
    karus_knights_name  VARCHAR(21)     NOT NULL DEFAULT '',
    karus_knights       SMALLINT        NOT NULL DEFAULT 0,
    karus_loyalty       INTEGER         NOT NULL DEFAULT 0,
    salary              INTEGER         NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_pos)
);

-- Knights (clan) ranking table
-- C++ table: KNIGHTS_RATING
CREATE TABLE IF NOT EXISTS knights_rating (
    rank_pos    INTEGER     NOT NULL,
    clan_id     SMALLINT    NOT NULL DEFAULT 0,
    points      INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_pos)
);

CREATE INDEX IF NOT EXISTS idx_knights_rating_clan ON knights_rating (clan_id);

-- Daily rank tracking table
-- C++ struct: _DAILY_RANK keyed by character name
CREATE TABLE IF NOT EXISTS daily_rank (
    char_id         VARCHAR(21)     NOT NULL,
    gm_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    gm_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    mh_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    mh_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    sh_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    sh_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    ak_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    ak_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    cw_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    cw_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    up_rank_cur     INTEGER         NOT NULL DEFAULT 0,
    up_rank_prev    INTEGER         NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id)
);

-- Draki tower daily ranking (for DRAKI_RANK daily rank type)
CREATE TABLE IF NOT EXISTS draki_tower_daily_rank (
    char_id     VARCHAR(21)     NOT NULL,
    class_id    INTEGER         NOT NULL DEFAULT 0,
    draki_stage SMALLINT        NOT NULL DEFAULT 0,
    draki_time  INTEGER         NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id)
);

CREATE INDEX IF NOT EXISTS idx_draki_class ON draki_tower_daily_rank (class_id);
