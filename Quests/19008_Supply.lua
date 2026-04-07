local Ret = 0;
local NPC = 19008;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 8903, NPC, 4519, 101);
end   

if(EVENT == 101) then
	SelectMsg(UID, 2, -1, 9208, NPC, 40206, 102)
end

if(EVENT == 102) then
	Class = CheckClass(UID);
    if (Class == 1 or Class == 5 or Class == 6) then
		EVENT = 103
    elseif (Class == 2 or Class == 7 or Class == 8) then
		EVENT = 113
    elseif (Class == 3 or Class == 9 or Class == 10) then
		EVENT = 123
    elseif (Class == 4 or Class == 11 or Class == 12) then
		EVENT = 133
	end
end

if(EVENT == 103) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	if( LEVEL > 59 and COIN > 99999 ) then
		SelectMsg(UID, 2, -1, 8889, NPC, 4466, 104, 3019, 193 )
	elseif(LEVEL > 35 and LEVEL < 60 and COIN > 599999) then
		SelectMsg(UID, 2, -1, 8885, NPC, 4466, 105, 3019, 193 )
	else
		EVENT = 193
	end
end

if(EVENT == 113) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	if( LEVEL > 59 and COIN > 99999 ) then
		SelectMsg(UID, 2, -1, 8890, NPC, 4466, 114, 3019, 193 )
	elseif(LEVEL > 35 and LEVEL < 60 and COIN > 599999) then
		SelectMsg(UID, 2, -1, 8886, NPC, 4466, 115, 3019, 193 )
	else
		EVENT = 193
	end
end

if(EVENT == 123) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	if( LEVEL > 59 and COIN > 99999 ) then
		SelectMsg(UID, 2, -1, 8891, NPC, 4466, 124, 3019, 193 )
	elseif(LEVEL > 35 and LEVEL < 60 and COIN > 599999) then
		SelectMsg(UID, 2, -1, 8887, NPC, 4466, 125, 3019, 193 )
	else
		EVENT = 193
	end
end

if(EVENT == 133) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	if( LEVEL > 59 and COIN > 99999 ) then
		SelectMsg(UID, 2, -1, 8892, NPC, 4466, 134, 3019, 193 )
	elseif(LEVEL > 35 and LEVEL < 60 and COIN > 599999) then
		SelectMsg(UID, 2, -1, 8888, NPC, 4466, 135, 3019, 193 )
	else
		EVENT = 193
	end
end

if(EVENT == 193 ) then
	SelectMsg(UID, 2, -1, 9503, NPC, 10, -1 )
end

if(EVENT == 104) then --Warrior
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000); 
	WarriorSlot = CheckGiveSlot(UID, 6)
	if WarriorSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 59 and COIN > 99999 ) then
		GiveItem(UID, 208003017, 1, 1)
		GiveItem(UID, 208001017, 1, 1)
		GiveItem(UID, 208002017, 1, 1)
		GiveItem(UID, 208004017, 1, 1)
		GiveItem(UID, 208005017, 1, 1)
		GiveItem(UID, 127430757, 1, 1)
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 105) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	WarriorSlot = CheckGiveSlot(UID, 6)
	if WarriorSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 35 and LEVEL < 60 and COIN > 99999 ) then
		GiveItem(UID, 208003017, 1, 1)
		GiveItem(UID, 208001017, 1, 1)
		GiveItem(UID, 208002017, 1, 1)
		GiveItem(UID, 208004017, 1, 1)
		GiveItem(UID, 208005017, 1, 1)
		GiveItem(UID, 127430757, 1, 1)
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 114) then -- Rogue
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000); 
	RogueSlot = CheckGiveSlot(UID, 8)
	if RogueSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 59 and COIN > 99999 ) then
		GiveItem(UID, 248003517, 1, 1);
		GiveItem(UID, 248001517, 1, 1);
		GiveItem(UID, 248002517, 1, 1);
		GiveItem(UID, 248004517, 1, 1);
		GiveItem(UID, 248005517, 1, 1);
		GiveItem(UID, 180450797, 1, 1);
		GiveItem(UID, 128410807, 1, 1);
		GiveItem(UID, 128410807, 1, 1);	
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 115) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	RogueaSlot = CheckGiveSlot(UID, 7)
	if RogueaSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 35 and LEVEL < 60 and COIN > 99999 ) then
		GiveItem(UID, 248003517, 1, 1);
		GiveItem(UID, 248001517, 1, 1);
		GiveItem(UID, 248002517, 1, 1);
		GiveItem(UID, 248004517, 1, 1);
		GiveItem(UID, 248005517, 1, 1);
		GiveItem(UID, 180450797, 1, 1);
		GiveItem(UID, 128410807, 1, 1);
		GiveItem(UID, 128410807, 1, 1);	
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 124) then -- Mage
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	MageSlot = CheckGiveSlot(UID, 8)
	if MageSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 59 and COIN > 99999 ) then
		GiveItem(UID, 268003507, 1, 1);
		GiveItem(UID, 268001507, 1, 1);
		GiveItem(UID, 268002507, 1, 1);
		GiveItem(UID, 268004507, 1, 1);
		GiveItem(UID, 268005507, 1, 1);
		GiveItem(UID, 182420807, 1, 1);
		GiveItem(UID, 183420837, 1, 1);
		GiveItem(UID, 184420867, 1, 1);
		GoldLose(UID, 100000);
	else              
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 125) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	MageSlot = CheckGiveSlot(UID, 8)
	if MageSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 35 and LEVEL < 60 and COIN > 99999 ) then
		GiveItem(UID, 268003507, 1, 1);
		GiveItem(UID, 268001507, 1, 1);
		GiveItem(UID, 268002507, 1, 1);
		GiveItem(UID, 268004507, 1, 1);
		GiveItem(UID, 268005507, 1, 1);
		GiveItem(UID, 182420807, 1, 1);
		GiveItem(UID, 183420837, 1, 1);
		GiveItem(UID, 184420867, 1, 1);
		GoldLose(UID, 100000); 
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 134) then -- Priest
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000);  
	PriestSlot = CheckGiveSlot(UID, 8)
	if PriestSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 59 and COIN > 99999 ) then
		GiveItem(UID, 288003507, 1, 1);
		GiveItem(UID, 288001507, 1, 1);
		GiveItem(UID, 288002507, 1, 1);
		GiveItem(UID, 288004507, 1, 1);
		GiveItem(UID, 288005507, 1, 1);
		GiveItem(UID, 198410427, 1, 1);
		GiveItem(UID, 197420347, 1, 1);
		GiveItem(UID, 127430757, 1, 1);
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if(EVENT == 135) then
	LEVEL = GetLevel(UID)
	COIN = HowmuchItem(UID, 900000000); 
	PriestSlot = CheckGiveSlot(UID, 8)
	if PriestSlot == false then
		SelectMsg(UID,2,-1,8900,NPC,10,-1)
	else
	if( LEVEL > 35 and LEVEL < 60 and COIN > 99999 ) then
		GiveItem(UID, 288003507, 1, 1);
		GiveItem(UID, 288001507, 1, 1);
		GiveItem(UID, 288002507, 1, 1);
		GiveItem(UID, 288004507, 1, 1);
		GiveItem(UID, 288005507, 1, 1);
		GiveItem(UID, 198410427, 1, 1);
		GiveItem(UID, 197420347, 1, 1);
		GiveItem(UID, 127430757, 1, 1);
		GoldLose(UID, 100000);
	else
		SelectMsg(UID, 2, -1, 8893, NPC, 4466, -1, 3019, 193 )
		end
	end
end

if (EVENT == 3001) then
	SelectMsg(UID, 2, -1, 213, NPC, 4446, 193);
end

