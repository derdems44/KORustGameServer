local Ret = 0;
local NPC = 19073;

-- [Hepa Pupil] Shozin
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 12 menus, 19 mapped, 3 inferred, 1 unknown [actions=QUEST]

-- ROOT: header=845 flag=20
if (EVENT == 100) then
	SelectMsg(UID, 20, -1, 845, NPC, 4520, 101, 4521, 102, 4526, 103, 40368, 105, 4522, 111, 4523, 3001);
end

-- header=846 flag=19
if (EVENT == 101) then
	SelectMsg(UID, 19, -1, 846, NPC, 4419, 100);
end

-- header=847 flag=19
if (EVENT == 102) then
	SelectMsg(UID, 19, -1, 847, NPC, 4419, 100);
end

-- header=849 flag=19
if (EVENT == 103) then
	SelectMsg(UID, 19, -1, 849, NPC, 4527, 104, 4528, 3001);
end

-- header=851 flag=19
if (EVENT == 104) then
	SelectMsg(UID, 19, -1, 851, NPC, 10, 3001);
end

-- header=44228 flag=2
if (EVENT == 105) then
	SelectMsg(UID, 2, 1745, 44228, NPC, 40365, 106, 40369, 107, 40367, 109);
end

-- header=45328 flag=2
if (EVENT == 106) then
	SelectMsg(UID, 2, 1745, 45328, NPC, 10, 3001);
end

-- header=44221 flag=2
if (EVENT == 107) then
	SelectMsg(UID, 2, 1745, 44221, NPC, 40147, 108);
end

-- header=44222 flag=2
if (EVENT == 108) then
	SelectMsg(UID, 2, 1506, 44222, NPC, 22, 5000, 23, 3001);
end

-- header=44225 flag=2
if (EVENT == 109) then
	SelectMsg(UID, 2, 1745, 44225, NPC, 4161, 110, 4162, 3001);
end

-- header=44227 flag=3
if (EVENT == 110) then
	SelectMsg(UID, 3, 1745, 44227, NPC, 27, 3001);
end

-- header=848 flag=19
if (EVENT == 111) then
	SelectMsg(UID, 19, -1, 848, NPC, 4524, 3001, 4525, 3001 --[[ TODO: unknown ]]);
end

-- ═══ Action handlers (sniffer-verified) ═══

-- QUEST action (btn_text=22)
if (EVENT == 5000) then
	-- TODO: wire quest logic (SaveEvent, RunExchange, etc.)
	Ret = 1;
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
