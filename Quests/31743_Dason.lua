local NPC = 31743;

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

-- [AUTO-GEN] quest=1631 status=0 n_index=10828
if (EVENT == 1000) then
	SelectMsg(UID, 4, 1631, 44742, NPC, 3529, 1001, 23, -1);
end

-- [AUTO-GEN] quest=1631 status=1 n_index=10829
if (EVENT == 1001) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1631, 44742, NPC, 22, 1003, 23, -1);
	else
		SelectMsg(UID, 2, 1631, 44742, NPC, 18, 1002);
	end
end

-- [AUTO-GEN] quest=1631 status=1 n_index=10829
if (EVENT == 1002) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1631 status=1 n_index=10829
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 1631)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6819);
		SaveEvent(UID, 10830);
	end
end

