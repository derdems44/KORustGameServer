local Ret = 0;
local NPC = 16047;

-- [Operator] Moira
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 2 menus, 2 mapped, 0 inferred, 0 unknown

-- header=8917 flag=2
if (EVENT == 240) then
	SelectMsg(UID, 2, 16, 8917, NPC, 10, 3001);
end

-- header=4032 flag=2
if (EVENT == 241) then
	SelectMsg(UID, 2, 11, 4032, NPC, 10, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
