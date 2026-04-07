local NPC = 29166;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9149, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 9149, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=909 status=2 n_index=6632
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 909)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6634);
	end
end

-- [AUTO-GEN] quest=909 status=0 n_index=6630
if (EVENT == 1000) then
	SelectMsg(UID, 4, 909, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=909 status=0 n_index=6630
if (EVENT == 1001) then
	SaveEvent(UID, 6631);
end

-- [AUTO-GEN] quest=909 status=1 n_index=6631
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

