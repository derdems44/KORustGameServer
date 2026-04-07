local NPC = 29156;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9124, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 9124, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=928 status=2 n_index=6727
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 928)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6729);
	end
end

-- [AUTO-GEN] quest=928 status=0 n_index=6725
if (EVENT == 1000) then
	SelectMsg(UID, 4, 928, 0, NPC, 461, 1001, 23, -1);
end

-- [AUTO-GEN] quest=928 status=0 n_index=6725
if (EVENT == 1001) then
	SaveEvent(UID, 6726);
end

-- [AUTO-GEN] quest=928 status=1 n_index=6726
if (EVENT == 1002) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 928, 0, NPC, 22, 1003, 23, -1);
	else
		SelectMsg(UID, 2, 928, 0, NPC, 18, 1004);
	end
end

-- [AUTO-GEN] quest=928 status=1 n_index=6726
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 928)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6727);
	end
end

-- [AUTO-GEN] quest=928 status=1 n_index=6726
if (EVENT == 1004) then
	ShowMap(UID, 21);
end

