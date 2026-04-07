local Ret = 0;
local NPC = 14301;

-- [Blacksmith] Heppa
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 1 menus, 0 mapped, 1 inferred, 0 unknown

-- header=615 flag=1
if (EVENT == 240) then
	SelectMsg(UID, 1, 94, 615, NPC, 14, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
