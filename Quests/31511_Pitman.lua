local NPC = 31511;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 241, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 101) then
		NpcMsg(UID, 9170, NPC)
	else
		EVENT = QuestNum
	end
end

-- EVENT 101 removed: merged into EVENT 100 (original flow)

if (EVENT == 400) then
	QuestStatusCheck = GetQuestStatus(UID, 433)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 9172, NPC, 22, 402, 23, -1);
		else
			SelectMsg(UID, 2, -1, 9173, NPC, 4466, 401,4467,-1);
	end
end

if (EVENT == 401) then
	QuestStatusCheck = GetQuestStatus(UID, 433)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 9172, NPC, 10, -1);
		else
			SelectMsg(UID, 2, -1, 9174, NPC, 10, -1);
			SlotCheck = CheckGiveSlot(UID, 1)
			if SlotCheck then
				GiveItem(UID, 389132000, 1);
			end
			SaveEvent(UID, 20043);
	end
end

if (EVENT == 402) then
	ITEM001 = HowmuchItem(UID, 900000000);
	ITEM002 = HowmuchItem(UID, 389132000);
		if (ITEM001 < 1000000) then
			SelectMsg(UID, 2, -1, 9181, NPC, 18, 1000);
		else
		if (ITEM002 > 0) then
			SelectMsg(UID, 2, -1, 9169, NPC, 27, -1);
		else
			SlotCheck = CheckGiveSlot(UID, 1)
			if SlotCheck then
				GoldLose(UID, 1000000);
				GiveItem(UID, 389132000, 1);
			end
			SaveEvent(UID, 20050);
		end
	end
end

if (EVENT == 1000) then
	ShowMap(UID, 336);
end

if (EVENT == 200) then
	SelectMsg(UID, 19, -1, 9169, NPC, 10, -1);
end

if (EVENT == 300) then
	QuestStatusCheck = GetQuestStatus(UID, 619)
	QuestStatusCheckII = GetQuestStatus(UID, 620)	
		if(QuestStatusCheck == 1 or QuestStatusCheckII == 1) then
			SelectMsg(UID, 3, -1, 9171, NPC, 7253, 301,7254,302,7164,304);
		else
			SelectMsg(UID, 2, -1, 9171, NPC, 7253, 301,7254,302);
	end
end

if (EVENT == 301) then
	MysteriousORE = HowmuchItem(UID, 399210000);  
		if (MysteriousORE < 1) then 
			SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
		else
			EVENT = 305
	end
end

if (EVENT == 302) then
	MysteriousGOLDORE = HowmuchItem(UID, 399200000);  
		if (MysteriousGOLDORE < 1) then 
			SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
		else
			EVENT = 306
	end
end

if (EVENT == 305) then
	MysteriousORE = HowmuchItem(UID, 399210000);  
	if (MysteriousORE < 1) then 
		SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
	else
		SlotCheck = CheckGiveSlot(UID, 1)
		if SlotCheck == false then
		else
			RunMiningExchange(UID,2);
			SelectMsg(UID, 2, -1, 9176, NPC, 10, -1);
		end  
	end
end
	
if (EVENT == 306) then
	MysteriousGOLDORE = HowmuchItem(UID, 399200000);  
	if (MysteriousGOLDORE < 1) then 
		SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
	else
		SlotCheck = CheckGiveSlot(UID, 2)
		if SlotCheck == false then
		else
			RunMiningExchange(UID,1);
			SelectMsg(UID, 2, -1, 9176, NPC, 10, -1); 
		end	
	end
end 

if (EVENT == 304) then
    SelectMsg(UID, 2, -1, 22154, NPC, 7253, 307,7254,308);
end

if (EVENT == 307) then
	MysteriousORE = HowmuchItem(UID, 399210000);  
		if (MysteriousORE < 1) then 
			SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
		else
			EVENT = 309
	end
end

if (EVENT == 308) then
	MysteriousGOLDORE = HowmuchItem(UID, 399200000);  
		if (MysteriousGOLDORE < 1) then 
			SelectMsg(UID, 2, -1, 9179, NPC, 10, -1); 
		else
			EVENT = 310
	end
end

if (EVENT == 309) then
	SlotCheck = CheckGiveSlot(UID, 1)
		if SlotCheck == false then
        else
			SlotCheck = CheckGiveSlot(UID, 1)
			if SlotCheck then
				GiveItem(UID,389770000,1);
			end
			RobItem(UID, 399210000, 1);
	end
end

if (EVENT == 310) then
	SlotCheck = CheckGiveSlot(UID, 1)
		if SlotCheck == false then
			
        else
			SlotCheck = CheckGiveSlot(UID, 1)
			if SlotCheck then
				GiveItem(UID,389770000,1);
			end
			RobItem(UID, 399200000, 1);
	end
end

-- Custom Sling shop removed — restored to original v2600 behavior



