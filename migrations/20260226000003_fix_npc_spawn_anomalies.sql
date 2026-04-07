-- Sprint 371: Fix NPC spawn anomalies from MSSQL import
--
-- 1. Remove wrong Saber Tooth (NPC 603) spawn in Moradon (zone 21) at (816, 641)
--    NPC 603 belongs in Zone 1/2 (castle areas), not Moradon.
--
-- 2. Add Magic Anvil (NPC 5001) spawns to all 3 main zones.
--    Magic Anvil templates (5001-5005) exist but had NO spawns anywhere.
--    Required for item upgrade (WIZ_ITEM_UPGRADE) functionality.

-- Remove wrong Saber Tooth from Moradon
DELETE FROM npc_spawn
WHERE zone_id = 21
  AND npc_id = 603
  AND is_monster = TRUE
  AND left_x = 816
  AND top_z = 641;

-- Add Magic Anvil spawns (NPC 5001, type=24 NPC_ANVIL)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES
    -- Zone 21 (Moradon) near Blacksmith Heppa (14301 at 713, 594)
    (21, 5001, FALSE, 100, 0, 0, 0, 0, 720, 600, 1, 0, 30, 45, 0, NULL, 0),
    -- Zone 1 (Luferson Castle) near service NPC cluster (~420, 1630)
    (1, 5001, FALSE, 100, 0, 0, 0, 0, 420, 1635, 1, 0, 30, 25, 0, NULL, 0),
    -- Zone 2 (El Morad Castle) near service NPC cluster (~1650, 390)
    (2, 5001, FALSE, 100, 0, 0, 0, 0, 1655, 390, 1, 0, 30, 110, 0, NULL, 0);
