local NPC = 32609;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 43944, NPC, 1737, 502 ,1738, 510,4005, 168 )
end

if (EVENT == 168) then
	Ret = 1;
end

if (EVENT == 502) then
	EXPPREM = HowmuchItem(UID, 900000000);
	if (EXPPREM < 100000000 or EXPPREM == 0) then
		SelectMsg(UID, 2, -1, 43945, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	

		else
	    GoldLose(UID, 100000000); -- Userin üzerinden direkt noah çeker.
	    GiveBalance(UID, 0, 20); -- Usere 1. Satırda TL , 2. Satırda KC verir.
		SelectMsg(UID, 2, -1, 6361, NPC, 10, -1);	
		end
	end
end

if (EVENT == 510) then
	EXPPREM = HowmuchItem(UID, 900000000);
	if (EXPPREM < 500000000 or EXPPREM == 0) then
		SelectMsg(UID, 2, -1, 43947, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
		else
	    GoldLose(UID, 500000000); -- Userin üzerinden direkt noah çeker.
	    GiveBalance(UID, 0, 100); -- Usere 1. Satırda TL , 2. Satırda KC verir.
		SelectMsg(UID, 2, -1, 6361, NPC, 10, -1);	
		end
	end
end
