-- Object event positions (K_OBJECTPOS2369 from MSSQL)
-- These define interactive objects in zones: bind points, warp gates, levers, anvils, etc.
CREATE TABLE IF NOT EXISTS object_event_pos (
    id          SERIAL PRIMARY KEY,
    zone_id     SMALLINT    NOT NULL,
    belong      SMALLINT    NOT NULL DEFAULT 0,   -- nation restriction (0=all, 1=karus, 2=elmorad)
    s_index     SMALLINT    NOT NULL,              -- object index within zone
    obj_type    SMALLINT    NOT NULL DEFAULT 0,    -- ObjectType enum (0=bind, 1=gate, 3=gate_lever, 5=warp_gate, 8=anvil, 12=krowaz_gate, 14=wood, 15=wood_lever)
    control_npc SMALLINT    NOT NULL DEFAULT 0,    -- associated NPC or warp group ID
    status      SMALLINT    NOT NULL DEFAULT 1,    -- 0=inactive, 1=active
    pos_x       REAL        NOT NULL DEFAULT 0,
    pos_y       REAL        NOT NULL DEFAULT 0,
    pos_z       REAL        NOT NULL DEFAULT 0,
    by_life     SMALLINT    NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_object_event_pos_zone ON object_event_pos (zone_id);
