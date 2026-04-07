local NPC = 29181;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 10227, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 10227, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=927 status=2 n_index=6722
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 927)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6724);
	end
end

-- [AUTO-GEN] quest=927 status=0 n_index=6720
if (EVENT == 1000) then
	SelectMsg(UID, 4, 927, 10229, NPC, 460, 1001, 23, -1);
end

-- [AUTO-GEN] quest=927 status=0 n_index=6720
if (EVENT == 1001) then
	SaveEvent(UID, 6721);
end

-- [AUTO-GEN] quest=927 status=1 n_index=6721
if (EVENT == 1002) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 927, 10229, NPC, 22, 1003, 23, -1);
	else
		SelectMsg(UID, 2, 927, 10229, NPC, 18, 1004);
	end
end

-- [AUTO-GEN] quest=927 status=1 n_index=6721
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 927)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6722);
	end
end

-- [AUTO-GEN] quest=927 status=1 n_index=6721
if (EVENT == 1004) then
	ShowMap(UID, 21);
end

