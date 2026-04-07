-- Fix: userdata.id and character_seal_mapping.unique_id use COALESCE(MAX, 0)+1
-- which races under concurrent inserts. Add proper PostgreSQL sequences.

-- ── userdata.id ──────────────────────────────────────────────────────

-- 1. Create a sequence for userdata.id
CREATE SEQUENCE IF NOT EXISTS userdata_id_seq;

-- 2. Set the sequence to current max id (so nextval starts at max+1)
SELECT setval('userdata_id_seq', GREATEST(COALESCE((SELECT MAX(id) FROM userdata), 0), 1));

-- 3. Set the column default to use the sequence
ALTER TABLE userdata ALTER COLUMN id SET DEFAULT nextval('userdata_id_seq');

-- 4. Add unique constraint on id (prevents duplicates)
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'uq_userdata_id') THEN
        ALTER TABLE userdata ADD CONSTRAINT uq_userdata_id UNIQUE (id);
    END IF;
END $$;

-- ── character_seal_mapping.unique_id ─────────────────────────────────

-- Already has UNIQUE constraint, but still races between SELECT MAX and INSERT.
CREATE SEQUENCE IF NOT EXISTS character_seal_unique_id_seq;

SELECT setval('character_seal_unique_id_seq',
    GREATEST(COALESCE((SELECT MAX(unique_id) FROM character_seal_mapping), 0), 1));
