local Ret = 0;
local NPC = 31741;

-- [Trader] Julia
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 5 menus, 8 mapped, 0 inferred, 18 unknown

-- ROOT: header=44704 flag=2
if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44704, NPC, 40776, 101, 40777, 101, 40800, 102, 40831, 3001 --[[ TODO: unknown ]], 45330, 104);
end

-- header=44707 flag=2
if (EVENT == 101) then
	SelectMsg(UID, 2, -1, 44707, NPC, 27, 3001);
end

-- header=44704 flag=2
if (EVENT == 102) then
	SelectMsg(UID, 2, -1, 44704, NPC, 40851, 3001 --[[ TODO: unknown ]], 40802, 3001 --[[ TODO: unknown ]], 40853, 3001 --[[ TODO: unknown ]], 40804, 3001 --[[ TODO: unknown ]], 40854, 3001 --[[ TODO: unknown ]], 40805, 3001 --[[ TODO: unknown ]], 45337, 103, 40857, 3001 --[[ TODO: unknown ]], 40806, 3001 --[[ TODO: unknown ]]);
end

-- header=44705 flag=2
if (EVENT == 103) then
	SelectMsg(UID, 2, -1, 44705, NPC, 27, 3001);
end

-- header=45422 flag=2
if (EVENT == 104) then
	SelectMsg(UID, 2, -1, 45422, NPC, 45054, 3001 --[[ TODO: unknown ]], 40852, 3001 --[[ TODO: unknown ]], 45331, 3001 --[[ TODO: unknown ]], 45356, 3001 --[[ TODO: unknown ]], 45332, 3001 --[[ TODO: unknown ]], 45357, 3001 --[[ TODO: unknown ]], 40855, 103, 40856, 3001 --[[ TODO: unknown ]], 45359, 3001 --[[ TODO: unknown ]], 45333, 3001 --[[ TODO: unknown ]]);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
