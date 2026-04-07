-- World Boss system persistence (v2525 WIZ_WORLD_BOSS 0xD5/0xD6).
-- Boss configuration (4 panel slots) and per-event player rankings.

-- World boss slot configuration (up to 4 bosses)
CREATE TABLE IF NOT EXISTS world_boss_config (
    slot_id       SMALLINT PRIMARY KEY,           -- 1-4 (client panel slot)
    boss_name     VARCHAR(50) NOT NULL DEFAULT '', -- display name
    npc_proto_id  INTEGER  NOT NULL DEFAULT 0,     -- NPC template ID for spawning
    boss_type     SMALLINT NOT NULL DEFAULT 1,     -- 1-4 (gauge clamp type)
    boss_info_id  SMALLINT NOT NULL DEFAULT 0,     -- animation resource lookup
    spawn_zone    SMALLINT NOT NULL DEFAULT 0,     -- zone ID for boss spawn
    spawn_x       REAL     NOT NULL DEFAULT 0.0,
    spawn_z       REAL     NOT NULL DEFAULT 0.0,
    enabled       BOOLEAN  NOT NULL DEFAULT FALSE  -- whether this slot is active
);

-- Insert 4 empty boss slots
INSERT INTO world_boss_config (slot_id) VALUES (1), (2), (3), (4)
ON CONFLICT DO NOTHING;

-- Per-event player damage/ranking tracking
CREATE TABLE IF NOT EXISTS world_boss_ranking (
    id            SERIAL PRIMARY KEY,
    slot_id       SMALLINT NOT NULL,              -- boss slot (1-4)
    character_id  VARCHAR(21) NOT NULL,            -- player name
    damage_dealt  BIGINT   NOT NULL DEFAULT 0,     -- total damage in current event
    kill_count    INTEGER  NOT NULL DEFAULT 0,      -- times participated in kill
    last_hit      BOOLEAN  NOT NULL DEFAULT FALSE,  -- got the last hit
    event_time    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (slot_id, character_id, event_time)
);

CREATE INDEX IF NOT EXISTS idx_world_boss_ranking_slot ON world_boss_ranking (slot_id, damage_dealt DESC);
