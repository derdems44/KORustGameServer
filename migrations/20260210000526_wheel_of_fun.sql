-- Wheel of Fun event — prize items and drop settings.
-- Source: MSSQL WHEEL_OF_FUN_ITEM (15 rows) + WHEEL_SETTINGS (15 rows)

CREATE TABLE IF NOT EXISTS wheel_of_fun_item (
    id      SMALLINT     NOT NULL,
    name    VARCHAR(100) NOT NULL DEFAULT '',
    num     INTEGER      NOT NULL DEFAULT 0,
    count   INTEGER      NOT NULL DEFAULT 1,
    percent INTEGER      NOT NULL DEFAULT 50,
    days    INTEGER      NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

INSERT INTO wheel_of_fun_item (id, name, num, count, percent, days) VALUES
    (1,  'Unidentified Potion',               900067000, 1,   50, 0),
    (2,  'Gryphons Helmet Certificate',        800230000, 1,   50, 0),
    (3,  'Gryphons armor Certificate',         800240000, 1,   50, 0),
    (4,  'Certificate of Pathos Glove',        800250000, 1,   50, 0),
    (5,  'War Premium',                        399292764, 1,   50, 0),
    (6,  'DISC Premium',                       399281685, 1,    5, 0),
    (7,  'Scroll of Armor 400',                800077000, 30,  50, 0),
    (8,  'Blue Potion',                        900128000, 1,   50, 0),
    (9,  'Red Potion',                         900129000, 1,   50, 0),
    (10, 'Spirit of Genie',                    810378000, 1,   50, 0),
    (11, 'HP Scroll 60%',                      800079000, 30,  50, 0),
    (12, 'NP increase item',                   800074000, 1,   50, 0),
    (13, 'Duration Item',                      800022000, 10,  50, 0),
    (14, 'Water of Ibexs [Limited Edition]',   810247000, 100, 50, 0),
    (15, 'Potion of Crisis [Limited Edition]', 810248000, 100, 50, 0)
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS wheel_of_fun_settings (
    idx         SMALLINT     NOT NULL,
    item_name   VARCHAR(100) NOT NULL DEFAULT '',
    item_id     INTEGER      NOT NULL DEFAULT 0,
    item_count  INTEGER      NOT NULL DEFAULT 1,
    rental_time INTEGER      NOT NULL DEFAULT 1,
    flag        SMALLINT     NOT NULL DEFAULT 1,
    drop_rate   INTEGER      NOT NULL DEFAULT 100,
    PRIMARY KEY (idx)
);

INSERT INTO wheel_of_fun_settings (idx, item_name, item_id, item_count, rental_time, flag, drop_rate) VALUES
    (16, 'Switching Premium',           399295859, 1,  1, 1, 20),
    (17, 'War Premium',                 399292764, 1,  1, 1, 50),
    (18, 'DC Premium',                  399281685, 1,  1, 1, 50),
    (19, 'Certificate 100 Knight Cash', 700082000, 1,  1, 1, 150),
    (20, 'Certificate 350 Knight Cash', 700083000, 1,  1, 1, 80),
    (21, 'Trina',                       700002000, 1,  1, 1, 80),
    (22, 'HP Scroll 60%',               800079000, 1,  1, 1, 200),
    (23, 'Scroll of Armor 400',         800077000, 1,  1, 1, 200),
    (24, 'Scroll of 2000 HP',           800078000, 1,  1, 1, 500),
    (25, 'Scroll of Armor 350',         800076000, 1,  1, 1, 500),
    (26, 'Premium Potion HP',           389390000, 1,  1, 1, 400),
    (27, 'Premium Potion MP',           389400000, 1,  1, 1, 400),
    (28, 'Duration Item',               800022000, 1,  1, 1, 600),
    (29, 'Red Potion',                  900129000, 10, 1, 1, 750),
    (30, 'Blue Potion',                 900128000, 10, 1, 1, 750)
ON CONFLICT DO NOTHING;
