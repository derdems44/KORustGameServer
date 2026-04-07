-- Monster resource strings and death notice configuration.
-- Source: MSSQL MONSTER_RESOURCE (4 rows)

CREATE TABLE IF NOT EXISTS monster_resource (
    sid          SMALLINT     NOT NULL,
    sid_name     VARCHAR(50)  NOT NULL,
    resource     VARCHAR(255) NOT NULL,
    notice_zone  SMALLINT     NOT NULL DEFAULT 0,
    notice_type  SMALLINT     NOT NULL DEFAULT 8,
    PRIMARY KEY (sid)
);

INSERT INTO monster_resource (sid, sid_name, resource, notice_zone, notice_type) VALUES
    (9250, 'Dark Dragon', 'Dark Dragon of Delos Castle dungeon has fallen. A blessing has been sent to all.', 0, 8),
    (9503, 'Mammoth the 3rd', 'Crasher Gimmick has appeared.', 1, 8),
    (9507, 'Crasher Gimmick', 'Crasher Gimmick has fallen.', 1, 8),
    (9518, 'Pluwitoon', 'Pluwiton the Destroyer of Fire has fallen.', 1, 8)
ON CONFLICT DO NOTHING;
