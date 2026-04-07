-- Zone 21: Add 4 monsters present in sniffer but missing from C++ import
-- Using 2 spawn groups per proto with num_npc=5 (standard KO pattern)

-- 159: Kecoon captain (sniffer: 14 positions → ~2 groups of 5)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES
  (21, 159, true, 5, 658, 291, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 159, true, 5, 670, 276, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
-- 452: Stegodon (sniffer: 3 positions → 1 group of 3)
  (21, 452, true, 3, 481, 850, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
-- 1056: Battalion (sniffer: 12 positions → 2 groups of 5)
  (21, 1056, true, 5, 283, 937, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
  (21, 1056, true, 5, 305, 960, 1, 0, 0, 0, 0, 7, 30, 0, 0, '', 0),
-- 9021: Wandering Spectre (sniffer: 1 position → 1 spawn)
  (21, 9021, true, 1, 292, 259, 1, 0, 0, 0, 0, 7, 60, 0, 0, '', 0);
