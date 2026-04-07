local NPC = 3300;

if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 45541, NPC, 1721, 101,1722,102,1723,103,1724,104,1725,105,1726,106,1727,107,1728,108,1729,109,1730,110);
end

if (EVENT == 101) then --Voucher of Infinite Arrow	800605000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 100) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,100);
		GiveItem(UID, 800605000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 102) then --Voucher of oto mining		800610000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 200) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,200);
		GiveItem(UID, 800610000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 103) then --300 Voucher of Auto Loot		750680000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 300) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,300);
		GiveItem(UID, 750680000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end


if (EVENT == 104) then --400 Voucher of Infinite All		346390000	
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 400) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,400);
		GiveItem(UID, 346390000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 105) then --500 X-wing Voucher			820358000	
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 500) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,500);
		GiveItem(UID, 820358000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 106) then -- 600 Automatic Mining + Auto Loot Voucher	511000000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 600) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,600);
		GiveItem(UID, 511000000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 107) then --700 Tears of Karivdis		379258000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 700) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,700);
		GiveItem(UID, 379258000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 108) then --800 Trina's Piece			700002000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 800) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,800);
		GiveItem(UID, 700002000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 109) then --900 Accessory Trina's Piece		354000000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 900) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,900);
		GiveItem(UID, 354000000,1);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end

if (EVENT == 110) then --1000 Automatic Upgrade		544000000
	CHEST = HowmuchItem(UID, 820418000);
	if (CHEST < 1000) then
		SelectMsg(UID, 2, -1, 45542, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 820418000,1000);
		GiveItem(UID, 544000000,1,15);
		SelectMsg(UID, 2, -1, 45543, NPC, 27, -1);	
    	end
    end
end