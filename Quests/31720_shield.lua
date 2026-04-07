local Ret = 0;
local NPC = 31720;

-- (JAPKO) LvL Jump
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 2 menus, 5 mapped, 0 inferred, 0 unknown

-- ROOT: header=44646 flag=2
if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44646, NPC, 40686, 101, 40687, 101, 40688, 101, 40689, 101);
end

-- header=44596 flag=3
if (EVENT == 101) then
	SelectMsg(UID, 3, -1, 44596, NPC, 27, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
