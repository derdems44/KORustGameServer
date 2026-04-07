-- Right-top title messages (admin notices displayed in client top-right corner).
-- C++ Reference: GameDefine.h:2784 — _RIGHT_TOP_TITLE_MSG
-- C++ Reference: RightTopTitleSet.h — SELECT id, strMessage, strTitle FROM RIGHT_TOP_TITLE

CREATE TABLE IF NOT EXISTS right_top_title (
    id          INTEGER NOT NULL PRIMARY KEY,
    str_title   VARCHAR(128) NOT NULL DEFAULT '',
    str_message VARCHAR(256) NOT NULL DEFAULT ''
);
