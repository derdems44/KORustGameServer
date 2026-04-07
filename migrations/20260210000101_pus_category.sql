-- PUS (Premium User Store) Category table
-- Source: MSSQL PUS_CATEGORY (5 rows)
-- Categories organize the cash shop into browsable sections.

CREATE TABLE IF NOT EXISTS pus_category (
    id              SMALLINT    NOT NULL PRIMARY KEY,
    category_name   VARCHAR(30) NOT NULL,
    description     VARCHAR(50) NOT NULL,
    category_id     SMALLINT    NOT NULL,
    status          SMALLINT    NOT NULL DEFAULT 1
);

-- Seed data from MSSQL PUS_CATEGORY
INSERT INTO pus_category (id, category_name, description, category_id, status) VALUES
(1, 'Scrolls',          'Scrollar',         1, 1),
(2, 'Premium-Other',    'Premium-Others',   2, 1),
(3, 'Trina-Pot-Scroll', 'Power-UP',         3, 1),
(4, 'Cospre items',     'Cosplay',          4, 1),
(5, 'TL & Knight KC',   'TL & Knight KC',   5, 1)
ON CONFLICT (id) DO NOTHING;
