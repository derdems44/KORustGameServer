local Ret = 0;
local NPC = 13013;

-- [Sentinel] Patrick
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 2 menus, 0 mapped, 2 inferred, 0 unknown

-- header=8526 flag=1
if (EVENT == 165) then
	SelectMsg(UID, 1, 98, 8526, NPC, 14, 3001);
end

-- header=8518 flag=1
if (EVENT == 166) then
	SelectMsg(UID, 1, 97, 8518, NPC, 14, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
