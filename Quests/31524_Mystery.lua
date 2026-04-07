local Ret = 0;
local NPC = 31524;

-- Mysterious old man Â
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 4 menus, 7 mapped, 0 inferred, 1 unknown

-- ROOT: header=21215 flag=3
if (EVENT == 100) then
	SelectMsg(UID, 3, 1745, 21215, NPC, 7494, 3001, 8351, 101, 8788, 102, 45307, 101);
end

-- header=21215 flag=70
if (EVENT == 101) then
	SelectMsg(UID, 70, 616, 21215, NPC, 10, 3001);
end

-- header=12247 flag=2
if (EVENT == 102) then
	SelectMsg(UID, 2, 1745, 12247, NPC, 8785, 103, 8786, 3001 --[[ TODO: unknown ]]);
end

-- header=12248 flag=2
if (EVENT == 103) then
	SelectMsg(UID, 2, 1745, 12248, NPC, 8787, 3001);
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
