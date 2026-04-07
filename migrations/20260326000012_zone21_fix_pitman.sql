-- Pitman (31511): Walking NPC — orijinalde TEK instance, yürüyor
-- Sniffer'da aynı npc_id=46396 iki pozisyonda görüldü (628,421) ve (671,416)
-- Fazla spawn'ı sil, tek spawn bırak
DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 31511;
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES (21, 31511, false, 1, 663, 411, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0);
