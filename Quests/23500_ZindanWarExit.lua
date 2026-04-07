local NPC = 23500;

if (EVENT == 100) then
SelectMsg(UID, 2, -1, 9496, NPC,7182,1010);
end

if (EVENT == 1010) then
ZONEKONTROL = GetZoneID(UID);
if (ZONEKONTROL == 91) then
ZoneChange(UID, 21, 0.0, 0.0);
end
end
