local NPC = 31515;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then 
		SelectMsg(UID, 2, -1, 4703, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then 
		NpcMsg(UID, 21715, NPC)
	else
		EVENT = QuestNum
	end
end

if(EVENT == 1002)then 
	SelectMsg(UID, 2, 539, 20314, NPC, 10, 1004);
end

if(EVENT == 1003)then 
	SelectMsg(UID, 2, 539, 20314, NPC, 10, 1004);
end

if(EVENT == 1004)then 
	SelectMsg(UID, 4, 539, 20315, NPC,22, 1005, 27, -1); 
	SaveEvent(UID, 11310);
end

if(EVENT == 1005)then 
	SaveEvent(UID, 11309);
	SaveEvent(UID, 11320);
end

if (EVENT == 1101)then
	SelectMsg(UID, 4, 540, 20048, NPC, 22, 1102,23,-1);
end

if(EVENT == 1102)then 
	QuestStatusCheck = GetQuestStatus(UID, 540)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 11320);
	end
end

if(EVENT == 1106)then 
	QuestStatusCheck = GetQuestStatus(UID, 540)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEM1_COUNT = HowmuchItem(UID, 508106000);   
		if (ITEM1_COUNT < 5) then
			SelectMsg(UID, 2, 540, 20048, NPC, 18,1104);
		else
			SaveEvent(UID, 11322);
		end
	end
end

if (EVENT == 1103) then
	QuestStatusCheck = GetQuestStatus(UID, 540)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEM1_COUNT = HowmuchItem(UID, 508106000);   
		if (ITEM1_COUNT < 5) then
			SelectMsg(UID, 2, 540, 20048, NPC, 18,1104);
		else
			SelectMsg(UID, 4, 540, 20048, NPC, 22, 1107, 27, -1); 
		end
	end
end

if (EVENT == 1104 ) then
	ShowMap(UID, 371)
end

if (EVENT == 1107)then
	QuestStatusCheck = GetQuestStatus(UID, 540)	
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEM1_COUNT = HowmuchItem(UID, 508106000);   
		if (ITEM1_COUNT < 5) then
			SelectMsg(UID, 2, 540, 20048, NPC, 18,1104);
		else
			RunQuestExchange(UID,3027);
			SaveEvent(UID,11321);
			SaveEvent(UID,11332);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=539 status=2 n_index=11309
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 539)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3026);
		SaveEvent(UID, 11311);
	end
end

-- [AUTO-GEN] quest=539 status=255 n_index=11306
if (EVENT == 1000) then
	SaveEvent(UID, 11307);
end

-- [AUTO-GEN] quest=540 status=255 n_index=11318
if (EVENT == 1100) then
	SaveEvent(UID, 11319);
end

