-- Sprint 620: Rename col_N columns for quest_menu and quest_talk tables.
-- These were missed in the first rename migration (20260307000003).
-- Made idempotent: safe to run even when columns are already named (fresh DB).

CREATE OR REPLACE FUNCTION _safe_rename(t TEXT, old_col TEXT, new_col TEXT) RETURNS void AS $$
BEGIN
  EXECUTE format('ALTER TABLE %I RENAME COLUMN %I TO %I', t, old_col, new_col);
EXCEPTION WHEN undefined_column THEN NULL;
END; $$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION _safe_alter_type(t TEXT, col TEXT, new_type TEXT) RETURNS void AS $$
BEGIN
  EXECUTE format('ALTER TABLE %I ALTER COLUMN %I TYPE %s USING %I::%s', t, col, new_type, col, new_type);
EXCEPTION WHEN OTHERS THEN NULL;
END; $$ LANGUAGE plpgsql;

--------------------------------------------------------------------------------
-- 11. quest_menu (2 cols, 1:1 with MSSQL QUEST_MENU_US)
--     Rust: QuestMenuRow { i_num: i32, str_menu: String }
--------------------------------------------------------------------------------
SELECT _safe_rename('quest_menu', 'col_0', 'i_num');
SELECT _safe_rename('quest_menu', 'col_1', 'str_menu');

SELECT _safe_alter_type('quest_menu', 'i_num', 'integer');

--------------------------------------------------------------------------------
-- 12. quest_talk (4 cols: 2 from MSSQL + 2 TBL extras)
--     Rust: QuestTalkRow { i_num: i32, str_talk: String }
--     col_2 and col_3 are TBL-only extras (not in MSSQL).
--------------------------------------------------------------------------------
SELECT _safe_rename('quest_talk', 'col_0', 'i_num');
SELECT _safe_rename('quest_talk', 'col_1', 'str_talk');
SELECT _safe_rename('quest_talk', 'col_2', 'tbl_extra_1');
SELECT _safe_rename('quest_talk', 'col_3', 'tbl_extra_2');

SELECT _safe_alter_type('quest_talk', 'i_num', 'integer');
SELECT _safe_alter_type('quest_talk', 'tbl_extra_1', 'integer');
SELECT _safe_alter_type('quest_talk', 'tbl_extra_2', 'integer');

DROP FUNCTION IF EXISTS _safe_rename(TEXT, TEXT, TEXT);
DROP FUNCTION IF EXISTS _safe_alter_type(TEXT, TEXT, TEXT);
