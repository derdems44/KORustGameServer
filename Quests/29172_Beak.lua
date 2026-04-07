local NPC = 29172;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 10195, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 10195, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=915 status=2 n_index=6662
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 915)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5020);
		SaveEvent(UID, 6662);
	end
end

-- [AUTO-GEN] quest=915 status=0 n_index=6660
if (EVENT == 1000) then
	SelectMsg(UID, 4, 915, 10196, NPC, 456, 1001, 23, -1);
end

-- [AUTO-GEN] quest=915 status=0 n_index=6660
if (EVENT == 1001) then
	SaveEvent(UID, 6660);
end

-- [AUTO-GEN] quest=915 status=1 n_index=6661
if (EVENT == 1002) then
	ItemA = HowmuchItem(UID, 900427000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 915, 10196, NPC, 18, 1004);
	else
		SelectMsg(UID, 4, 915, 10196, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=915 status=1 n_index=6661
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 915)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5020);
		SaveEvent(UID, 6661);
	end
end

-- [AUTO-GEN] quest=915 status=1 n_index=6661
if (EVENT == 1004) then
	ShowMap(UID, 21);
end

