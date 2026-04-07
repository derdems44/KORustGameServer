local Ret = 0;
local NPC = 31506;

-- [Lunar Lady] Magpie
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 5 menus, 6 mapped, 0 inferred, 1 unknown

-- ROOT: header=45268 flag=3
if (EVENT == 100) then
	SelectMsg(UID, 3, 1745, 45268, NPC, 40600, 101, 40599, 103);
end

-- header=44507 flag=2
if (EVENT == 101) then
	SelectMsg(UID, 2, 1745, 44507, NPC, 8419, 102, 40158, 3001 --[[ TODO: unknown ]]);
end

-- header=44509 flag=3
if (EVENT == 102) then
	SelectMsg(UID, 3, 1745, 44509, NPC, 27, 3001);
end

-- header=44506 flag=3
if (EVENT == 103) then
	SelectMsg(UID, 3, 1745, 44506, NPC, 4006, 104);
end

-- header=44510 flag=3
if (EVENT == 104) then
	SelectMsg(UID, 3, 1745, 44510, NPC, 27, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
