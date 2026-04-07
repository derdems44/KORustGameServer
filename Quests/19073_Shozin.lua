local Ret = 0;
local NPC = 19073;

-- [[Hepa Pupil] Shozin]
-- Auto-generated from sniffer capture by dialog_tree_builder.py
-- 12 unique menus, 14 truly unknown branches

-- ROOT: header=845
if (EVENT == 100) then
	SelectMsg(UID, 2, 0, 845, NPC, 4520, 3001 --[[ TODO: unknown target ]], 4521, 3001 --[[ TODO: unknown target ]], 4526, 3001 --[[ TODO: unknown target ]], 40368, 3001 --[[ TODO: unknown target ]], 4522, 3001 --[[ TODO: unknown target ]], 4523, 3001 --[[ TODO: unknown target ]]);
end

-- header=846
if (EVENT == 101) then
	SelectMsg(UID, 2, 0, 846, NPC, 4419, 3001 --[[ TODO: unknown target ]]);
end

-- header=847
if (EVENT == 102) then
	SelectMsg(UID, 2, 0, 847, NPC, 4419, 3001 --[[ TODO: unknown target ]]);
end

-- header=849
if (EVENT == 103) then
	SelectMsg(UID, 2, 0, 849, NPC, 4527, 3001, 4528, 3001);
end

-- header=851
if (EVENT == 104) then
	SelectMsg(UID, 2, 0, 851, NPC, 10, 3001);
end

-- header=44228
if (EVENT == 105) then
	SelectMsg(UID, 2, 0, 44228, NPC, 40365, 3001 --[[ TODO: unknown target ]], 40369, 3001 --[[ TODO: unknown target ]], 40367, 3001 --[[ TODO: unknown target ]]);
end

-- header=45328
if (EVENT == 106) then
	SelectMsg(UID, 2, 0, 45328, NPC, 10, 3001);
end

-- header=44221
if (EVENT == 107) then
	SelectMsg(UID, 2, 0, 44221, NPC, 40147, 3001 --[[ TODO: unknown target ]]);
end

-- header=44222
if (EVENT == 108) then
	SelectMsg(UID, 2, 0, 44222, NPC, 22, 3001, 23, 3001);
end

-- header=44225
if (EVENT == 109) then
	SelectMsg(UID, 2, 0, 44225, NPC, 4161, 3001, 4162, 3001);
end

-- header=44227
if (EVENT == 110) then
	SelectMsg(UID, 2, 0, 44227, NPC, 27, 3001);
end

-- header=848
if (EVENT == 111) then
	SelectMsg(UID, 2, 0, 848, NPC, 4524, 3001 --[[ TODO: unknown target ]], 4525, 3001 --[[ TODO: unknown target ]]);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
