-- Sprint 620: Rename col_N columns from ko-tbl-import to named columns expected by Rust code.
-- Also fix column types (bigint -> integer/smallint) to match sqlx::FromRow struct types.
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

CREATE OR REPLACE FUNCTION _safe_add_column(t TEXT, col TEXT, col_type TEXT, col_default TEXT) RETURNS void AS $$
BEGIN
  EXECUTE format('ALTER TABLE %I ADD COLUMN %I %s DEFAULT %s', t, col, col_type, col_default);
EXCEPTION WHEN duplicate_column THEN NULL;
END; $$ LANGUAGE plpgsql;


--------------------------------------------------------------------------------
-- 1. item_op (4 cols, direct 1:1 mapping with MSSQL ITEM_OP)
--    Rust: ItemOpRow { item_id: i32, trigger_type: i16, skill_id: i32, trigger_rate: i16 }
--------------------------------------------------------------------------------
SELECT _safe_rename('item_op', 'col_0', 'item_id');
SELECT _safe_rename('item_op', 'col_1', 'trigger_type');
SELECT _safe_rename('item_op', 'col_2', 'skill_id');
SELECT _safe_rename('item_op', 'col_3', 'trigger_rate');

SELECT _safe_alter_type('item_op', 'item_id', 'integer');
SELECT _safe_alter_type('item_op', 'skill_id', 'integer');

--------------------------------------------------------------------------------
-- 2. set_item (39 cols: 25 from MSSQL SET_ITEM + 14 extra TBL-only unk fields)
--    Rust: SetItemRow { set_index: i32, set_name: Option<String>, ac_bonus..np_bonus: i16, unk1..unk14: i16 }
--------------------------------------------------------------------------------
SELECT _safe_rename('set_item', 'col_0', 'set_index');
SELECT _safe_rename('set_item', 'col_1', 'set_name');
SELECT _safe_rename('set_item', 'col_2', 'ac_bonus');
SELECT _safe_rename('set_item', 'col_3', 'hp_bonus');
SELECT _safe_rename('set_item', 'col_4', 'mp_bonus');
SELECT _safe_rename('set_item', 'col_5', 'strength_bonus');
SELECT _safe_rename('set_item', 'col_6', 'stamina_bonus');
SELECT _safe_rename('set_item', 'col_7', 'dexterity_bonus');
SELECT _safe_rename('set_item', 'col_8', 'intel_bonus');
SELECT _safe_rename('set_item', 'col_9', 'charisma_bonus');
SELECT _safe_rename('set_item', 'col_10', 'flame_resistance');
SELECT _safe_rename('set_item', 'col_11', 'glacier_resistance');
SELECT _safe_rename('set_item', 'col_12', 'lightning_resistance');
SELECT _safe_rename('set_item', 'col_13', 'poison_resistance');
SELECT _safe_rename('set_item', 'col_14', 'magic_resistance');
SELECT _safe_rename('set_item', 'col_15', 'curse_resistance');
SELECT _safe_rename('set_item', 'col_16', 'xp_bonus_percent');
SELECT _safe_rename('set_item', 'col_17', 'coin_bonus_percent');
SELECT _safe_rename('set_item', 'col_18', 'ap_bonus_percent');
SELECT _safe_rename('set_item', 'col_19', 'ap_bonus_class_type');
SELECT _safe_rename('set_item', 'col_20', 'ap_bonus_class_percent');
SELECT _safe_rename('set_item', 'col_21', 'ac_bonus_class_type');
SELECT _safe_rename('set_item', 'col_22', 'ac_bonus_class_percent');
SELECT _safe_rename('set_item', 'col_23', 'max_weight_bonus');
SELECT _safe_rename('set_item', 'col_24', 'np_bonus');
-- col_25..col_38 are TBL-only extra columns (14 unknowns)
SELECT _safe_rename('set_item', 'col_25', 'unk1');
SELECT _safe_rename('set_item', 'col_26', 'unk2');
SELECT _safe_rename('set_item', 'col_27', 'unk3');
SELECT _safe_rename('set_item', 'col_28', 'unk4');
SELECT _safe_rename('set_item', 'col_29', 'unk5');
SELECT _safe_rename('set_item', 'col_30', 'unk6');
SELECT _safe_rename('set_item', 'col_31', 'unk7');
SELECT _safe_rename('set_item', 'col_32', 'unk8');
SELECT _safe_rename('set_item', 'col_33', 'unk9');
SELECT _safe_rename('set_item', 'col_34', 'unk10');
SELECT _safe_rename('set_item', 'col_35', 'unk11');
SELECT _safe_rename('set_item', 'col_36', 'unk12');
SELECT _safe_rename('set_item', 'col_37', 'unk13');
SELECT _safe_rename('set_item', 'col_38', 'unk14');

SELECT _safe_alter_type('set_item', 'set_index', 'integer');

--------------------------------------------------------------------------------
-- 3. item_exchange (27 cols: col_2 is TBL-only extra, col_26 = exchange_item_time4,
--    exchange_item_time5 is missing — must be added)
--    Rust: ItemExchangeRow { n_index: i32, random_flag: i16, origin_item_num1..5: i32,
--      origin_item_count1..5: i32, exchange_item_num1..5: i32, exchange_item_count1..5: i32,
--      exchange_item_time1..5: i32 }
--    Verified: col_2 is always an extra column between random_flag and origin_item_num1.
--------------------------------------------------------------------------------
SELECT _safe_rename('item_exchange', 'col_0', 'n_index');
SELECT _safe_rename('item_exchange', 'col_1', 'random_flag');
-- col_2 is TBL-only extra (leave as tbl_extra_1)
SELECT _safe_rename('item_exchange', 'col_2', 'tbl_extra_1');
SELECT _safe_rename('item_exchange', 'col_3', 'origin_item_num1');
SELECT _safe_rename('item_exchange', 'col_4', 'origin_item_count1');
SELECT _safe_rename('item_exchange', 'col_5', 'origin_item_num2');
SELECT _safe_rename('item_exchange', 'col_6', 'origin_item_count2');
SELECT _safe_rename('item_exchange', 'col_7', 'origin_item_num3');
SELECT _safe_rename('item_exchange', 'col_8', 'origin_item_count3');
SELECT _safe_rename('item_exchange', 'col_9', 'origin_item_num4');
SELECT _safe_rename('item_exchange', 'col_10', 'origin_item_count4');
SELECT _safe_rename('item_exchange', 'col_11', 'origin_item_num5');
SELECT _safe_rename('item_exchange', 'col_12', 'origin_item_count5');
SELECT _safe_rename('item_exchange', 'col_13', 'exchange_item_num1');
SELECT _safe_rename('item_exchange', 'col_14', 'exchange_item_count1');
SELECT _safe_rename('item_exchange', 'col_15', 'exchange_item_num2');
SELECT _safe_rename('item_exchange', 'col_16', 'exchange_item_count2');
SELECT _safe_rename('item_exchange', 'col_17', 'exchange_item_num3');
SELECT _safe_rename('item_exchange', 'col_18', 'exchange_item_count3');
SELECT _safe_rename('item_exchange', 'col_19', 'exchange_item_num4');
SELECT _safe_rename('item_exchange', 'col_20', 'exchange_item_count4');
SELECT _safe_rename('item_exchange', 'col_21', 'exchange_item_num5');
SELECT _safe_rename('item_exchange', 'col_22', 'exchange_item_count5');
SELECT _safe_rename('item_exchange', 'col_23', 'exchange_item_time1');
SELECT _safe_rename('item_exchange', 'col_24', 'exchange_item_time2');
SELECT _safe_rename('item_exchange', 'col_25', 'exchange_item_time3');
SELECT _safe_rename('item_exchange', 'col_26', 'exchange_item_time4');
-- exchange_item_time5 is missing from TBL (always 0 in MSSQL) — add it
SELECT _safe_add_column('item_exchange', 'exchange_item_time5', 'integer', '0');

SELECT _safe_alter_type('item_exchange', 'n_index', 'integer');
SELECT _safe_alter_type('item_exchange', 'tbl_extra_1', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_num1', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_count1', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_num2', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_num3', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_num4', 'integer');
SELECT _safe_alter_type('item_exchange', 'origin_item_num5', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_num1', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_num2', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_num3', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_num4', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_num5', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_time1', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_time2', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_time3', 'integer');
SELECT _safe_alter_type('item_exchange', 'exchange_item_time4', 'integer');

--------------------------------------------------------------------------------
-- 4. item_exchange_exp (12 cols in TBL, Rust expects 17 — missing exchange_item_time1..5)
--    Rust: ItemExchangeExpRow { n_index: i32, random_flag: Option<i16>,
--      exchange_item_num1..5: Option<i32>, exchange_item_count1..5: Option<i32>,
--      exchange_item_time1..5: Option<i32> }
--    TBL 1:1 with MSSQL for first 12 cols, times not in TBL.
--------------------------------------------------------------------------------
SELECT _safe_rename('item_exchange_exp', 'col_0', 'n_index');
SELECT _safe_rename('item_exchange_exp', 'col_1', 'random_flag');
SELECT _safe_rename('item_exchange_exp', 'col_2', 'exchange_item_num1');
SELECT _safe_rename('item_exchange_exp', 'col_3', 'exchange_item_count1');
SELECT _safe_rename('item_exchange_exp', 'col_4', 'exchange_item_num2');
SELECT _safe_rename('item_exchange_exp', 'col_5', 'exchange_item_count2');
SELECT _safe_rename('item_exchange_exp', 'col_6', 'exchange_item_num3');
SELECT _safe_rename('item_exchange_exp', 'col_7', 'exchange_item_count3');
SELECT _safe_rename('item_exchange_exp', 'col_8', 'exchange_item_num4');
SELECT _safe_rename('item_exchange_exp', 'col_9', 'exchange_item_count4');
SELECT _safe_rename('item_exchange_exp', 'col_10', 'exchange_item_num5');
SELECT _safe_rename('item_exchange_exp', 'col_11', 'exchange_item_count5');
-- Add missing time columns (not in TBL, always 0 in MSSQL)
SELECT _safe_add_column('item_exchange_exp', 'exchange_item_time1', 'integer', '0');
SELECT _safe_add_column('item_exchange_exp', 'exchange_item_time2', 'integer', '0');
SELECT _safe_add_column('item_exchange_exp', 'exchange_item_time3', 'integer', '0');
SELECT _safe_add_column('item_exchange_exp', 'exchange_item_time4', 'integer', '0');
SELECT _safe_add_column('item_exchange_exp', 'exchange_item_time5', 'integer', '0');

SELECT _safe_alter_type('item_exchange_exp', 'n_index', 'integer');
SELECT _safe_alter_type('item_exchange_exp', 'exchange_item_count1', 'integer');
SELECT _safe_alter_type('item_exchange_exp', 'exchange_item_count2', 'integer');
SELECT _safe_alter_type('item_exchange_exp', 'exchange_item_count3', 'integer');
SELECT _safe_alter_type('item_exchange_exp', 'exchange_item_count4', 'integer');
SELECT _safe_alter_type('item_exchange_exp', 'exchange_item_count5', 'integer');

--------------------------------------------------------------------------------
-- 5. achieve_main (14 cols, 1:1 with MSSQL ACHIEVE_MAIN)
--    Rust: AchieveMainRow { s_index: i32, type: i16, title_id: i16, point: i16,
--      item_num: i32, count: i32, zone_id: i16, unknown2: i16, achieve_type: i16,
--      req_time: i16, byte1: i16, byte2: i16 }
--    col_10 (strName) and col_11 (strDesc) are NOT in Rust struct — leave with descriptive names.
--    SELECT * with FromRow will ignore unmatched columns.
--------------------------------------------------------------------------------
SELECT _safe_rename('achieve_main', 'col_0', 's_index');
SELECT _safe_rename('achieve_main', 'col_1', 'type');
SELECT _safe_rename('achieve_main', 'col_2', 'title_id');
SELECT _safe_rename('achieve_main', 'col_3', 'point');
SELECT _safe_rename('achieve_main', 'col_4', 'item_num');
SELECT _safe_rename('achieve_main', 'col_5', 'count');
SELECT _safe_rename('achieve_main', 'col_6', 'zone_id');
SELECT _safe_rename('achieve_main', 'col_7', 'unknown2');
SELECT _safe_rename('achieve_main', 'col_8', 'achieve_type');
SELECT _safe_rename('achieve_main', 'col_9', 'req_time');
SELECT _safe_rename('achieve_main', 'col_10', 'str_name');
SELECT _safe_rename('achieve_main', 'col_11', 'str_desc');
SELECT _safe_rename('achieve_main', 'col_12', 'byte1');
SELECT _safe_rename('achieve_main', 'col_13', 'byte2');

SELECT _safe_alter_type('achieve_main', 's_index', 'integer');
SELECT _safe_alter_type('achieve_main', 'item_num', 'integer');

--------------------------------------------------------------------------------
-- 6. achieve_war (5 cols: 3 from MSSQL + 2 TBL extras)
--    Rust: AchieveWarRow { s_index: i32, type: i16, s_count: i32 }
--    col_2 and col_4 are TBL-only extras (always 0).
--------------------------------------------------------------------------------
SELECT _safe_rename('achieve_war', 'col_0', 's_index');
SELECT _safe_rename('achieve_war', 'col_1', 'type');
SELECT _safe_rename('achieve_war', 'col_2', 'tbl_extra_1');
SELECT _safe_rename('achieve_war', 'col_3', 's_count');
SELECT _safe_rename('achieve_war', 'col_4', 'tbl_extra_2');

SELECT _safe_alter_type('achieve_war', 's_index', 'integer');

--------------------------------------------------------------------------------
-- 7. achieve_normal (3 cols, 1:1 with MSSQL ACHIEVE_NORMAL)
--    Rust: AchieveNormalRow { s_index: i32, type: i16, count: i32 }
--------------------------------------------------------------------------------
SELECT _safe_rename('achieve_normal', 'col_0', 's_index');
SELECT _safe_rename('achieve_normal', 'col_1', 'type');
SELECT _safe_rename('achieve_normal', 'col_2', 'count');

SELECT _safe_alter_type('achieve_normal', 's_index', 'integer');

--------------------------------------------------------------------------------
-- 8. achieve_com (4 cols, 1:1 with MSSQL ACHIEVE_COM)
--    Rust: AchieveComRow { s_index: i32, type: i16, req1: i32, req2: i32 }
--------------------------------------------------------------------------------
SELECT _safe_rename('achieve_com', 'col_0', 's_index');
SELECT _safe_rename('achieve_com', 'col_1', 'type');
SELECT _safe_rename('achieve_com', 'col_2', 'req1');
SELECT _safe_rename('achieve_com', 'col_3', 'req2');

SELECT _safe_alter_type('achieve_com', 's_index', 'integer');

--------------------------------------------------------------------------------
-- 9. achieve_title (28 cols: col_0=sIndex, col_1=strName(text), col_2=TBL extra,
--    col_3..col_27 = stat fields)
--    Rust: AchieveTitleRow { s_index: i32, str: i16, hp: i16, dex: i16, int: i16,
--      mp: i16, attack: i16, defence: i16, s_loyalty_bonus..s_poison_resist: i16 }
--    col_1 (strName) not in Rust struct. col_2 is TBL-only extra.
--------------------------------------------------------------------------------
SELECT _safe_rename('achieve_title', 'col_0', 's_index');
SELECT _safe_rename('achieve_title', 'col_1', 'str_name');
SELECT _safe_rename('achieve_title', 'col_2', 'tbl_extra_1');
SELECT _safe_rename('achieve_title', 'col_3', 'str');
SELECT _safe_rename('achieve_title', 'col_4', 'hp');
SELECT _safe_rename('achieve_title', 'col_5', 'dex');
SELECT _safe_rename('achieve_title', 'col_6', 'int');
SELECT _safe_rename('achieve_title', 'col_7', 'mp');
SELECT _safe_rename('achieve_title', 'col_8', 'attack');
SELECT _safe_rename('achieve_title', 'col_9', 'defence');
SELECT _safe_rename('achieve_title', 'col_10', 's_loyalty_bonus');
SELECT _safe_rename('achieve_title', 'col_11', 's_exp_bonus');
SELECT _safe_rename('achieve_title', 'col_12', 's_short_sword_ac');
SELECT _safe_rename('achieve_title', 'col_13', 's_jamadar_ac');
SELECT _safe_rename('achieve_title', 'col_14', 's_sword_ac');
SELECT _safe_rename('achieve_title', 'col_15', 's_blow_ac');
SELECT _safe_rename('achieve_title', 'col_16', 's_axe_ac');
SELECT _safe_rename('achieve_title', 'col_17', 's_spear_ac');
SELECT _safe_rename('achieve_title', 'col_18', 's_arrow_ac');
SELECT _safe_rename('achieve_title', 'col_19', 's_fire_bonus');
SELECT _safe_rename('achieve_title', 'col_20', 's_ice_bonus');
SELECT _safe_rename('achieve_title', 'col_21', 's_light_bonus');
SELECT _safe_rename('achieve_title', 'col_22', 's_fire_resist');
SELECT _safe_rename('achieve_title', 'col_23', 's_ice_resist');
SELECT _safe_rename('achieve_title', 'col_24', 's_light_resist');
SELECT _safe_rename('achieve_title', 'col_25', 's_magic_resist');
SELECT _safe_rename('achieve_title', 'col_26', 's_curse_resist');
SELECT _safe_rename('achieve_title', 'col_27', 's_poison_resist');

SELECT _safe_alter_type('achieve_title', 's_index', 'integer');

--------------------------------------------------------------------------------
-- 10. quest_helper (21 cols: 19 from MSSQL + 2 TBL extras at col_3 and col_5)
--     Rust: QuestHelperRow { n_index: i32, b_message_type: i16, b_level: i16,
--       n_exp: i32, b_class: i16, b_nation: i16, b_quest_type: i16, b_zone: i16,
--       s_npc_id: i16, s_event_data_index: i16, b_event_status: i16,
--       n_event_trigger_index: i32, n_event_complete_index: i32, n_exchange_index: i32,
--       n_event_talk_index: i32, str_lua_filename: String, s_quest_menu: i32,
--       s_npc_main: i32, s_quest_solo: i16 }
--     col_3 and col_5 are TBL-only extras.
--------------------------------------------------------------------------------
SELECT _safe_rename('quest_helper', 'col_0', 'n_index');
SELECT _safe_rename('quest_helper', 'col_1', 'b_message_type');
SELECT _safe_rename('quest_helper', 'col_2', 'b_level');
SELECT _safe_rename('quest_helper', 'col_3', 'tbl_extra_1');
SELECT _safe_rename('quest_helper', 'col_4', 'n_exp');
SELECT _safe_rename('quest_helper', 'col_5', 'tbl_extra_2');
SELECT _safe_rename('quest_helper', 'col_6', 'b_class');
SELECT _safe_rename('quest_helper', 'col_7', 'b_nation');
SELECT _safe_rename('quest_helper', 'col_8', 'b_quest_type');
SELECT _safe_rename('quest_helper', 'col_9', 'b_zone');
SELECT _safe_rename('quest_helper', 'col_10', 's_npc_id');
SELECT _safe_rename('quest_helper', 'col_11', 's_event_data_index');
SELECT _safe_rename('quest_helper', 'col_12', 'b_event_status');
SELECT _safe_rename('quest_helper', 'col_13', 'n_event_trigger_index');
SELECT _safe_rename('quest_helper', 'col_14', 'n_event_complete_index');
SELECT _safe_rename('quest_helper', 'col_15', 'n_exchange_index');
SELECT _safe_rename('quest_helper', 'col_16', 'n_event_talk_index');
SELECT _safe_rename('quest_helper', 'col_17', 'str_lua_filename');
SELECT _safe_rename('quest_helper', 'col_18', 's_quest_menu');
SELECT _safe_rename('quest_helper', 'col_19', 's_npc_main');
SELECT _safe_rename('quest_helper', 'col_20', 's_quest_solo');

SELECT _safe_alter_type('quest_helper', 'n_index', 'integer');
SELECT _safe_alter_type('quest_helper', 'tbl_extra_1', 'integer');
SELECT _safe_alter_type('quest_helper', 'n_exp', 'integer');
SELECT _safe_alter_type('quest_helper', 's_npc_id', 'smallint');
SELECT _safe_alter_type('quest_helper', 's_event_data_index', 'smallint');
SELECT _safe_alter_type('quest_helper', 'n_exchange_index', 'integer');
SELECT _safe_alter_type('quest_helper', 'n_event_talk_index', 'integer');


DROP FUNCTION IF EXISTS _safe_rename(TEXT, TEXT, TEXT);
DROP FUNCTION IF EXISTS _safe_alter_type(TEXT, TEXT, TEXT);
DROP FUNCTION IF EXISTS _safe_add_column(TEXT, TEXT, TEXT, TEXT);