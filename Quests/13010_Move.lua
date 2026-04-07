local NPC = 13010;

if (EVENT == 500) then
	SelectMsg(UID, 2, -1, 4072, NPC, 4072, 501, 4072, -1);
end

if (EVENT == 501) then
	ZoneChange(UID, 21, 0, 0)
end