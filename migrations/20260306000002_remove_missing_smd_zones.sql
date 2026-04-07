-- Remove zone_info entries whose SMD files don't exist on disk.
-- These zones cause WARN log spam on every server startup:
--   zone_id 62 (BattleZone_b.smd) — Alseids Prairie
--   zone_id 63 (BattleZone_d.smd) — Nieds Triangle
--   zone_id 64 (BattleZone_e.smd) — Nereids Island
--   zone_id 69 (BattleZone_b.smd) — Snow War
--   zone_id 96 (In_dungeon06.smd) — Party War 1
--   zone_id 97 (In_dungeon06.smd) — Party War 2
--   zone_id 98 (In_dungeon06.smd) — Party War 3
--   zone_id 99 (In_dungeon06.smd) — Party War 4
-- Also remove NPC spawns for these zones (no FK constraint but keep data clean).
DELETE FROM npc_spawn WHERE zone_id IN (62, 63, 64, 69, 96, 97, 98, 99);
DELETE FROM zone_info WHERE zone_no IN (62, 63, 64, 69, 96, 97, 98, 99);
