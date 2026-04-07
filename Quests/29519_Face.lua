local NPC = 29519;

if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 43611, NPC,40599,105);
end

if (EVENT == 105) then
	LIKE = HowmuchItem(UID, 910334000);
	if (LIKE < 1) then
		SelectMsg(UID, 2, -1, 43629, NPC, 10, -1);
	else
		SelectMsg(UID, 2, -1, 43611, NPC, 10, 107);
	end
end

if (EVENT == 107) then
SlotCheck = CheckGiveSlot(UID, 1)
if SlotCheck then
	GiveItem(UID,810333856,1,7);
end
RobItem(UID, 910334000, 1);
end
