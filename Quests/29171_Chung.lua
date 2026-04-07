local NPC = 29171;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 10191, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 10191, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=914 status=2 n_index=6657
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 914)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5000);
		SaveEvent(UID, 6659);
	end
end

-- [AUTO-GEN] quest=914 status=0 n_index=6655
if (EVENT == 1000) then
	SelectMsg(UID, 4, 914, 10192, NPC, 454, 1001, 23, -1);
end

-- [AUTO-GEN] quest=914 status=0 n_index=6655
if (EVENT == 1001) then
	SaveEvent(UID, 6656);
end

-- [AUTO-GEN] quest=914 status=1 n_index=6656
if (EVENT == 1002) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 914, 10192, NPC, 22, 1000, 23, -1);
	else
		SelectMsg(UID, 2, 914, 10192, NPC, 18, 1003);
	end
end

-- [AUTO-GEN] quest=914 status=1 n_index=6656
if (EVENT == 1003) then
	ShowMap(UID, 21);
end

