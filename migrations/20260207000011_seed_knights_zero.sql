-- Seed knights table with id_num=0 (no clan placeholder).
-- Required because userdata.knights defaults to 0 with FK constraint.
INSERT INTO knights (id_num, flag, nation, id_name, chief)
VALUES (0, 0, 0, '', '')
ON CONFLICT (id_num) DO NOTHING;
