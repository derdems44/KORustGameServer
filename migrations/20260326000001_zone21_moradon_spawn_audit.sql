-- Zone 21 (Moradon) NPC/Monster Spawn Audit
-- Source: Original server sniffer capture (session 38, 2026-03-25)
-- 127,582 packets decoded, 265 unique NPC/Monster instances identified
--
-- Changes:
--   1. Fix num_npc=0 bugs (9274 Mana Predator, 9275 Shadow Spectre, 9641 Looter)
--   2. Add missing spawn entries (9276 + 10 NPCs with existing templates)
--   3. Add extra spawn points for NPCs seen at new positions
--   4. Add extra Kecoon spawns (+3 to match original server count of ~13)

-- ============================================================================
-- 1. FIX num_npc=0 BUG — these monsters have spawn points but 0 count
-- ============================================================================

-- 9274 Mana Predator: sniffer saw 1 instance at (389, 218)
-- Existing spawn at (351, 225) — set num_npc=1
UPDATE npc_spawn SET num_npc = 1 WHERE zone_id = 21 AND npc_id = 9274 AND is_monster = true AND num_npc = 0;

-- 9275 Shadow Spectre: sniffer saw 2 instances at (385-392, 217-237)
-- Existing spawn at (368, 213) — set num_npc=2
UPDATE npc_spawn SET num_npc = 2 WHERE zone_id = 21 AND npc_id = 9275 AND is_monster = true AND num_npc = 0;

-- 9641 Looter: sniffer saw 6 instances at (786-833, 339-384)
-- Existing 13 spawn points all at num_npc=0 — set each to 1 (total becomes 13, enough for 6 visible)
UPDATE npc_spawn SET num_npc = 1 WHERE zone_id = 21 AND npc_id = 9641 AND is_monster = true AND num_npc = 0;

-- ============================================================================
-- 2. ADD MISSING SPAWN ENTRIES — template exists, no spawn in zone 21
-- ============================================================================

-- 9276 [Leader] Shadow of Krowaz (MONSTER, level 52)
-- Sniffer: 1 instance at (396, 204)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 9276, true, 1, 0, 0, 0, 0, 396, 204, 1, 15, 60, 0, 0, 0);

-- 12200 [Arms Merchant] Pallus (NPC, level 50, nation 2)
-- Sniffer: 1 instance at (1688, 1384) — far east Moradon
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 12200, false, 0, 0, 0, 0, 0, 1688, 1384, 1, 0, 0, 1, 0, 0);

-- 14401 [Rental booth] Helard (NPC, level 70, nation 2)
-- Sniffer: 1 instance at (821, 607)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 14401, false, 0, 0, 0, 0, 0, 821, 607, 1, 0, 0, 90, 0, 0);

-- 16073 Karus Hero Statue (NPC, level 90, nation 2)
-- Sniffer: 1 instance at (784, 684)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 16073, false, 0, 0, 0, 0, 0, 784, 684, 1, 0, 0, 90, 0, 0);

-- 16074 Elmorad Hero Statue (NPC, level 90, nation 2)
-- Sniffer: 1 instance at (847, 680)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 16074, false, 0, 0, 0, 0, 0, 847, 680, 1, 0, 0, 90, 0, 0);

-- 16097 [InnHostess] Nia (NPC, level 50, nation 3)
-- Sniffer: 1 instance at (868, 649)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 16097, false, 0, 0, 0, 0, 0, 868, 649, 1, 0, 0, 135, 0, 0);

-- 19057 Diary lost man (NPC, level 30, quest NPC)
-- Sniffer: 1 instance at (921, 569)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19057, false, 0, 0, 0, 0, 0, 921, 569, 1, 0, 0, 158, 0, 0);

-- 19060 Psssimist (NPC, level 30, quest NPC)
-- Sniffer: 1 instance at (908, 633)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19060, false, 0, 0, 0, 0, 0, 908, 633, 1, 0, 0, 126, 0, 0);

-- 31719 [CSW Manager] Aaron (NPC, level 50, nation 1)
-- Sniffer: 1 instance at (804, 608)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 31719, false, 0, 0, 0, 0, 0, 804, 608, 1, 0, 0, 90, 0, 0);

-- 31720 (JAPKO) LvL Jump (NPC, level 50, nation 1)
-- Sniffer: 1 instance at (817, 524)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 31720, false, 0, 0, 0, 0, 0, 817, 524, 1, 0, 0, 180, 0, 0);

-- ============================================================================
-- 3. ADD EXTRA NPC SPAWN POINTS — seen at new positions in sniffer
-- ============================================================================

-- Scarecrows: sniffer saw additional instances at slightly different positions
-- 19067 Leather Scarecrow: existing at (758,416), sniffer also at (759,416) and (759,423)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19067, false, 0, 0, 0, 0, 0, 759, 423, 1, 0, 0, 90, 0, 0);

-- 19068 Chain Scarecrow: existing at (767,416), sniffer also at (769,416) and (769,423)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19068, false, 0, 0, 0, 0, 0, 769, 423, 1, 0, 0, 90, 0, 0);

-- 19069 Iron Scarecrow: existing at (776,416), sniffer also at (778,416) and (778,423)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19069, false, 0, 0, 0, 0, 0, 778, 423, 1, 0, 0, 91, 0, 0);

-- 19070 Leather Scarecrow (small): existing at (758,406), sniffer also at (769,406)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19070, false, 0, 0, 0, 0, 0, 769, 406, 1, 0, 0, 90, 0, 0);

-- 19071 Chain Scarecrow (small): existing at (758,398), sniffer also at (768,398)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19071, false, 0, 0, 0, 0, 0, 768, 398, 1, 0, 0, 90, 0, 0);

-- 19072 Iron Scarecrow (small): existing at (758,390), sniffer also at (768,390)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 19072, false, 0, 0, 0, 0, 0, 768, 390, 1, 0, 0, 90, 0, 0);

-- 5001 Magic anvil: existing at (720,600), sniffer also at (815,608) and (816,607)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES
  (21, 5001, false, 0, 0, 0, 0, 0, 815, 608, 1, 0, 0, 0, 0, 0),
  (21, 5001, false, 0, 0, 0, 0, 0, 816, 607, 1, 0, 0, 0, 0, 0);

-- 25174 Pulchino: existing at (828,587), sniffer also at (803,587)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 25174, false, 0, 0, 0, 0, 0, 803, 587, 1, 0, 0, 44, 0, 0);

-- 19019 Kaul: existing 8 spawns, sniffer saw 13
-- New positions from sniffer: (716,522), (710,543), (715,516), (718,545), (726,518)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES
  (21, 19019, false, 0, 0, 0, 0, 0, 716, 522, 1, 0, 0, 90, 0, 0),
  (21, 19019, false, 0, 0, 0, 0, 0, 710, 543, 1, 0, 0, 90, 0, 0),
  (21, 19019, false, 0, 0, 0, 0, 0, 715, 516, 1, 0, 0, 90, 0, 0),
  (21, 19019, false, 0, 0, 0, 0, 0, 718, 545, 1, 0, 0, 90, 0, 0),
  (21, 19019, false, 0, 0, 0, 0, 0, 726, 518, 1, 0, 0, 90, 0, 0);

-- ============================================================================
-- 4. INCREASE MONSTER SPAWN COUNTS — Kecoon area needs more mobs
-- ============================================================================

-- 150 Kecoon: current 10 (2×5), sniffer saw 13
-- Add 1 more spawn point with num_npc=3 near existing clusters
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 150, true, 1, 0, 0, 0, 0, 625, 328, 3, 7, 30, 0, 0, 0);

-- 852 Scavenger Bandicoot: current 10, sniffer saw 11
-- Add 1 more at a new position (633, 483)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, room)
VALUES (21, 852, true, 1, 0, 0, 0, 0, 633, 483, 1, 7, 30, 0, 0, 0);
