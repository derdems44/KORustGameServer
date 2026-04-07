-- Mining Exchange table — ore-to-item crafting via mining NPC.
--
-- C++ Reference: GameDefine.h:2614-2625 (_MINING_EXCHANGE struct)
-- MSSQL source: MINING_EXCHANGE (0 rows in 25xx backup — schema only)
CREATE TABLE IF NOT EXISTS mining_exchange (
    n_index       SMALLINT    NOT NULL PRIMARY KEY,
    s_npc_id      SMALLINT    NOT NULL DEFAULT 0,
    give_effect   SMALLINT    NOT NULL DEFAULT 0,
    ore_type      SMALLINT    NOT NULL DEFAULT 0,
    n_origin_item_num  INTEGER NOT NULL DEFAULT 0,
    n_give_item_num    INTEGER NOT NULL DEFAULT 0,
    n_give_item_count  SMALLINT NOT NULL DEFAULT 1,
    success_rate  INTEGER     NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_mining_exchange_ore_npc
    ON mining_exchange (ore_type, s_npc_id);
