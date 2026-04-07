local NPC = 29212;

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

-- [AUTO-GEN] quest=962 status=2 n_index=6899
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 962)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6901);
	end
end

-- [AUTO-GEN] quest=962 status=0 n_index=6897
if (EVENT == 1000) then
	SelectMsg(UID, 4, 962, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=962 status=0 n_index=6897
if (EVENT == 1001) then
	SaveEvent(UID, 6898);
end

-- [AUTO-GEN] quest=962 status=1 n_index=6898
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

