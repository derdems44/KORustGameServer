-- Sprint 102: Add skill0-skill9 columns to userdata table
-- Issue #6: "column skill0 of relation userdata does not exist"
--
-- C++ Reference: m_bstrSkill[10] in globals.h — 10 skill point categories (uint8)
-- MSSQL stores these as a single varchar 'strSkill', but our code uses
-- normalized columns for save_stat_points() and name_change clone queries.

ALTER TABLE userdata
    ADD COLUMN skill0  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill1  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill2  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill3  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill4  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill5  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill6  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill7  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill8  SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN skill9  SMALLINT NOT NULL DEFAULT 0;

-- Migrate existing str_skill data (comma-separated) into the new columns.
-- str_skill format: "free,cat1,cat2,cat3,master,cat1b,cat2b,cat3b,masterb"
-- Some rows may use colon separator from apply_starting_stats.
UPDATE userdata SET
    skill0 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 1), '')::SMALLINT, 0),
    skill1 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 2), '')::SMALLINT, 0),
    skill2 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 3), '')::SMALLINT, 0),
    skill3 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 4), '')::SMALLINT, 0),
    skill4 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 5), '')::SMALLINT, 0),
    skill5 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 6), '')::SMALLINT, 0),
    skill6 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 7), '')::SMALLINT, 0),
    skill7 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 8), '')::SMALLINT, 0),
    skill8 = COALESCE(NULLIF(split_part(REPLACE(COALESCE(str_skill, ''), ':', ','), ',', 9), '')::SMALLINT, 0),
    skill9 = 0
WHERE str_skill IS NOT NULL AND str_skill != '';
