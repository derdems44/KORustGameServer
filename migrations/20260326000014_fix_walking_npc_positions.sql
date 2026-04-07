-- Fix walking NPC positions — migration 013 had wrong column order
-- C++ import column order: zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room

DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id IN (19019, 21021, 11021, 19056);

-- Kaul (19019) — 8 spawn points from C++ import
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room) VALUES
(21,19019,FALSE,101,0,0,0,0,725,538,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,700,538,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,725,518,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,900,476,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,900,483,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,898,473,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,911,459,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,926,488,1,0,30,0,0,'',0);

-- Karus Guard (21021) — 4 spawns
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room) VALUES
(21,21021,FALSE,101,0,0,0,0,726,615,1,0,300,0,0,'',0),
(21,21021,FALSE,101,0,0,0,0,726,608,1,0,300,0,0,'',0),
(21,21021,FALSE,101,0,0,0,0,840,491,1,0,300,0,0,'',0),
(21,21021,FALSE,101,0,0,0,0,835,491,1,0,300,0,0,'',0);

-- Elmorad Guard (11021) — 4 spawns
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room) VALUES
(21,11021,FALSE,101,0,0,0,0,800,491,1,0,300,0,0,'',0),
(21,11021,FALSE,101,0,0,0,0,795,491,1,0,300,0,0,'',0),
(21,11021,FALSE,101,0,0,0,0,905,618,1,0,300,0,0,'',0),
(21,11021,FALSE,101,0,0,0,0,820,502,1,0,300,0,0,'',0);

-- Homeless (19056) — 1 spawn
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, act_type, regen_type, dungeon_family, special_type, trap_number, left_x, top_z, num_npc, spawn_range, regen_time, direction, dot_cnt, path, room) VALUES
(21,19056,FALSE,0,0,0,0,0,829,559,1,0,0,0,0,'',0);
