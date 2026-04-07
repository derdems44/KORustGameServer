-- King Election support tables
-- C++ Reference:
--   KING_ELECTION_LIST table — senators and candidates with vote counts
--   KING_NOMINATION_LIST table — who nominated whom
--   KING_CANDIDACY_NOTICE_BOARD table — candidate platform notices
--   KING_ELECTION_VOTES table — tracks who has voted (prevents double voting)

-----------------------------------------------
-- king_election_list: Senators (type=3) and candidates (type=4) per nation.
-- MSSQL source: KING_ELECTION_LIST (byNation, byType, strName, nKnights, nMoney)
-- C++ Reference: KingElectionListSet.h
-----------------------------------------------
CREATE TABLE IF NOT EXISTS king_election_list (
    id          BIGINT      GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    by_nation   SMALLINT    NOT NULL,
    by_type     SMALLINT    NOT NULL DEFAULT 3,
    str_name    VARCHAR(21) NOT NULL,
    n_knights   SMALLINT    NOT NULL DEFAULT 0,
    n_money     INTEGER     NOT NULL DEFAULT 0,

    UNIQUE (by_nation, by_type, str_name)
);

-----------------------------------------------
-- king_nomination_list: Who nominated whom for king.
-- MSSQL source: KING_NOMINATION_LIST (Nation, strNominator, strNominee)
-- C++ Reference: KingNominationListSet.h
-----------------------------------------------
CREATE TABLE IF NOT EXISTS king_nomination_list (
    id              BIGINT      GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    by_nation       SMALLINT    NOT NULL,
    str_nominator   VARCHAR(21) NOT NULL,
    str_nominee     VARCHAR(21) NOT NULL,

    UNIQUE (by_nation, str_nominee)
);

-----------------------------------------------
-- king_candidacy_notice_board: Platform statements by candidates.
-- MSSQL source: KING_CANDIDACY_NOTICE_BOARD (byNation, strUserID, strNotice)
-- C++ Reference: KingCandidacyNoticeBoardSet.h
-----------------------------------------------
CREATE TABLE IF NOT EXISTS king_candidacy_notice_board (
    id          BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    by_nation   SMALLINT        NOT NULL,
    str_user_id VARCHAR(21)     NOT NULL,
    str_notice  VARCHAR(1024)   NOT NULL DEFAULT '',

    UNIQUE (by_nation, str_user_id)
);

-----------------------------------------------
-- king_election_votes: Tracks which accounts have already voted.
-- Prevents double voting per account per election cycle.
-----------------------------------------------
CREATE TABLE IF NOT EXISTS king_election_votes (
    id                  BIGINT      GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    by_nation           SMALLINT    NOT NULL,
    str_account_id      VARCHAR(21) NOT NULL,
    str_user_id         VARCHAR(21) NOT NULL,
    str_nominee         VARCHAR(21) NOT NULL,

    UNIQUE (by_nation, str_account_id)
);
