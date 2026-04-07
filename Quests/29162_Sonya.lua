local NPC = 29162;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9796, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 9796, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=969 status=2 n_index=6934
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 969)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6936);
	end
end

-- [AUTO-GEN] quest=969 status=0 n_index=6932
if (EVENT == 1000) then
	SelectMsg(UID, 4, 969, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=969 status=0 n_index=6932
if (EVENT == 1001) then
	SaveEvent(UID, 6933);
end

-- [AUTO-GEN] quest=969 status=1 n_index=6933
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

