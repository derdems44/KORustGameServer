-- Skill bar shortcut persistence.
-- C++ Reference: USERDATA_SKILLSHORTCUT table (strCharID, nCount, strSkillData).
-- Each skill slot is a uint32 (4 bytes). Max 80 slots = 320 bytes.
-- The binary data is stored as-is (client format) in BYTEA.

CREATE TABLE IF NOT EXISTS user_skill_shortcuts (
    character_id VARCHAR(21) NOT NULL PRIMARY KEY,
    count        SMALLINT    NOT NULL DEFAULT 0,
    skill_data   BYTEA       NOT NULL DEFAULT E'\\x'
);

-- Foreign key to userdata
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'fk_skill_shortcuts_character'
    ) THEN
        ALTER TABLE user_skill_shortcuts
            ADD CONSTRAINT fk_skill_shortcuts_character
            FOREIGN KEY (character_id) REFERENCES userdata(str_user_id) ON DELETE CASCADE;
    END IF;
END$$;
