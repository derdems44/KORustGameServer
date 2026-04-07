-- Anti-AFK NPC list — NPCs used for AFK detection.
-- Source: MSSQL ANTIAFKLIST (4 rows)

CREATE TABLE IF NOT EXISTS anti_afk_list (
    idx    SMALLINT NOT NULL,
    npc_id SMALLINT NOT NULL,
    PRIMARY KEY (idx)
);

INSERT INTO anti_afk_list (idx, npc_id) VALUES
    (56, 8401),
    (57, 8402),
    (58, 8403),
    (59, 8404)
ON CONFLICT DO NOTHING;
