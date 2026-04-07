-- Zone 21: Add missing NPC templates (18034, 31774) and spawn for 12200.
-- Data extracted from sniffer REQ_NPCIN packets (session 38).

-- Proto 18034: NPC, pic=31539, npc_type=46, nation=3, level=70, pos(814,709)
INSERT INTO npc_template (s_sid, is_monster, str_name, s_pid, s_size, by_type, i_selling_group, s_level)
VALUES (18034, false, 'Event NPC', 31539, 100, 46, 0, 70)
ON CONFLICT (s_sid, is_monster) DO NOTHING;

-- Proto 31774: NPC, pic=30001, npc_type=174, nation=3, level=50, pos(830,668)
INSERT INTO npc_template (s_sid, is_monster, str_name, s_pid, s_size, by_type, i_selling_group, s_level)
VALUES (31774, false, 'Unknown NPC', 30001, 100, 174, 0, 50)
ON CONFLICT (s_sid, is_monster) DO NOTHING;

-- Spawns for all 3 missing NPCs
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES
  -- 18034: Event NPC — pos(814,709), direction=90
  (21, 18034, false, 1, 814, 709, 0, 0, 0, 0, 0, 0, 0, 90, 0, '', 0),
  -- 31774: Unknown NPC — pos(830,668), direction=180
  (21, 31774, false, 1, 830, 668, 0, 0, 0, 0, 0, 0, 0, 180, 0, '', 0),
  -- 12200: [Arms Merchant]Pallus — pos(1688,1384), direction=1
  -- Position outside normal Moradon bounds but sniffer-verified
  (21, 12200, false, 1, 1688, 1384, 0, 0, 0, 0, 0, 0, 0, 1, 0, '', 0)
ON CONFLICT DO NOTHING;
