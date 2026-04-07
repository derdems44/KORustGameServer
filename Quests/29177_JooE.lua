local NPC = 29177;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 187, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 187, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=924 status=2 n_index=6707
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 924)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5010);
		SaveEvent(UID, 6709);
	end
end

-- [AUTO-GEN] quest=924 status=0 n_index=6705
if (EVENT == 1000) then
	SelectMsg(UID, 4, 924, 10215, NPC, 458, 1001, 23, -1);
end

-- [AUTO-GEN] quest=924 status=0 n_index=6705
if (EVENT == 1001) then
	SaveEvent(UID, 6706);
end

-- [AUTO-GEN] quest=924 status=1 n_index=6706
if (EVENT == 1002) then
	ItemA = HowmuchItem(UID, 900429000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 924, 10215, NPC, 18, 1004);
	else
		SelectMsg(UID, 4, 924, 10215, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=924 status=1 n_index=6706
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 924)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5010);
		SaveEvent(UID, 6707);
	end
end

-- [AUTO-GEN] quest=924 status=1 n_index=6706
if (EVENT == 1004) then
	ShowMap(UID, 2);
end

