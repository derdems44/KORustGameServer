local NPC = 32287;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 906, NPC, 4076, 102, 4255, -1);
end

if (EVENT == 102) then
	ZoneChange(UID, 71, 630, 919)
end