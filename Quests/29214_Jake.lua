local NPC = 29214;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 323, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 323, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=966 status=2 n_index=6919
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 966)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6921);
	end
end

-- [AUTO-GEN] quest=966 status=0 n_index=6917
if (EVENT == 1000) then
	SelectMsg(UID, 4, 966, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=966 status=0 n_index=6917
if (EVENT == 1001) then
	SaveEvent(UID, 6918);
end

-- [AUTO-GEN] quest=966 status=1 n_index=6918
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

