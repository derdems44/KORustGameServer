-- Character Seal system tables.
--
-- C++ Reference: `SealHandler.cpp` — CHARACTER_SEAL_PROCEED / CHARACTER_UNSEAL_PROCEED
-- Stores a snapshot of a sealed character's data (stats + inventory).

-- Sealed character snapshot (all relevant stats + serialized inventory).
CREATE TABLE IF NOT EXISTS character_seal_items (
    id            SERIAL PRIMARY KEY,
    account_id    VARCHAR(50) NOT NULL,
    char_name     VARCHAR(50) NOT NULL,
    race          SMALLINT NOT NULL DEFAULT 0,
    class         SMALLINT NOT NULL DEFAULT 0,
    level         SMALLINT NOT NULL DEFAULT 0,
    rebirth_level SMALLINT NOT NULL DEFAULT 0,
    face          SMALLINT NOT NULL DEFAULT 0,
    hair_rgb      INT NOT NULL DEFAULT 0,
    rank          SMALLINT NOT NULL DEFAULT 0,
    title         SMALLINT NOT NULL DEFAULT 0,
    exp           BIGINT NOT NULL DEFAULT 0,
    loyalty       INT NOT NULL DEFAULT 0,
    loyalty_monthly INT NOT NULL DEFAULT 0,
    manner_point  INT NOT NULL DEFAULT 0,
    fame          SMALLINT NOT NULL DEFAULT 0,
    city          SMALLINT NOT NULL DEFAULT 0,
    knights       SMALLINT NOT NULL DEFAULT 0,
    hp            SMALLINT NOT NULL DEFAULT 0,
    mp            SMALLINT NOT NULL DEFAULT 0,
    sp            SMALLINT NOT NULL DEFAULT 0,
    zone_id       SMALLINT NOT NULL DEFAULT 0,
    strong        SMALLINT NOT NULL DEFAULT 0,
    sta           SMALLINT NOT NULL DEFAULT 0,
    dex           SMALLINT NOT NULL DEFAULT 0,
    intel         SMALLINT NOT NULL DEFAULT 0,
    cha           SMALLINT NOT NULL DEFAULT 0,
    authority     SMALLINT NOT NULL DEFAULT 1,
    free_points   SMALLINT NOT NULL DEFAULT 0,
    gold          INT NOT NULL DEFAULT 0,
    skill_cat1    SMALLINT NOT NULL DEFAULT 0,
    skill_cat2    SMALLINT NOT NULL DEFAULT 0,
    skill_cat3    SMALLINT NOT NULL DEFAULT 0,
    skill_master  SMALLINT NOT NULL DEFAULT 0,
    inventory_data BYTEA,
    item_serial   BIGINT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Maps cypher ring unique_id to sealed character item record.
CREATE TABLE IF NOT EXISTS character_seal_mapping (
    id            SERIAL PRIMARY KEY,
    unique_id     INT NOT NULL UNIQUE,
    seal_item_id  INT NOT NULL REFERENCES character_seal_items(id) ON DELETE CASCADE,
    account_id    VARCHAR(50) NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_character_seal_items_account ON character_seal_items(account_id);
CREATE INDEX IF NOT EXISTS idx_character_seal_mapping_account ON character_seal_mapping(account_id);
