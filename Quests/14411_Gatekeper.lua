local NPC = 14411;

if (EVENT == 217) then
	ITEM = HowmuchItem(UID, 389620000);
	if (ITEM < 1) then
		SelectMsg(UID, 2, -1, 1269, NPC, 18, 218);
	else
		SelectMsg(UID, 2, -1, 692, NPC, 58, 221, 4005, -1);
	end
end

if (EVENT == 218) then
	ShowMap(UID, 77);
end

if (EVENT == 221) then
	ITEM = HowmuchItem(UID, 389620000);
	if (ITEM < 1) then
		SelectMsg(UID, 2, -1, 1269, NPC, 18, 218);
	else
SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then
       
         else
		--RobItem(UID, 389620000, 1);
		ExpChange(UID, 5000);
		--SaveEvent(UID, 612);
		GiveItem(UID, 910087000, 1);
		SelectMsg(UID, 2, 66, 694, NPC, 56, -1);
	end
    end
	end