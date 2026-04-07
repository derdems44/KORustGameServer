local NPC = 31521;

if (EVENT == 100) then
	NATION = CheckNation(UID);
	if (NATION == 2) then
		SlotCheck = CheckGiveSlot(UID, 2)
		if SlotCheck then
			GiveItem(UID, 910228000, 1);
		end
		SelectMsg(UID, 2, -1, 20511, NPC, 10, -1);
	else
	    GiveItem(UID, 900071000, 1);
		SelectMsg(UID, 2, -1, 20511, NPC, 10, -1);
	end
end