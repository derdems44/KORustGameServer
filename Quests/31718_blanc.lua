local Ret = 0;
local NPC = 31718;

-- [Lunar Order Priest] Blanc
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 1 menus, 1 mapped, 0 inferred, 0 unknown

-- ROOT: header=45188 flag=3
if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 45188, NPC, 27, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
