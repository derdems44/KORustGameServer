local NPC = 16095;

local savenum = -1;

if (EVENT == 100) then
	SelectMsg(UID, 2, savenum, 4441, NPC, 4188, 101, 4005, -1);
end

if (EVENT == 101) then
	ITEM_COUNTA = HowmuchItem(UID, 389190000);
	if (ITEM_COUNTA > 0) then
		RobItem(UID, 389190000, 1);
		ZoneChangeParty(UID, 31, 940, 186)
	else
		SelectMsg(UID, 2, savenum, 4437, NPC, 18, 102);
	end
end

if (EVENT == 102) then
	ShowMap(UID, 440);
end

if (EVENT == 103) then
	ShowMap(UID, 445);
end

if (EVENT == 104) then
	ShowMap(UID, 446);
end

if (EVENT == 105) then
	ShowMap(UID, 447);
end