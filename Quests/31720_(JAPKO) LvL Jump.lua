local Ret = 0;
local NPC = 31720;

-- [(JAPKO) LvL Jump]
-- Auto-generated from sniffer capture by dialog_tree_builder.py
-- 2 unique menus, 4 truly unknown branches

-- ROOT: header=44646
if (EVENT == 100) then
	SelectMsg(UID, 2, 0, 44646, NPC, 40686, 3001 --[[ TODO: unknown target ]], 40687, 3001 --[[ TODO: unknown target ]], 40688, 3001 --[[ TODO: unknown target ]], 40689, 3001 --[[ TODO: unknown target ]]);
end

-- header=44596
if (EVENT == 101) then
	SelectMsg(UID, 2, 0, 44596, NPC, 27, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
