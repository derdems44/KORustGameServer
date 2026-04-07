local NPC = 29157;

if (EVENT == 101) then
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

-- [AUTO-GEN] quest=913 status=0 n_index=6650
if (EVENT == 1000) then
	SelectMsg(UID, 4, 913, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=913 status=0 n_index=6650
if (EVENT == 1001) then
	SaveEvent(UID, 6651);
end

-- [AUTO-GEN] quest=913 status=1 n_index=6651
if (EVENT == 1002) then
	SelectMsg(UID, 2, 913, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=913 status=1 n_index=6651
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 913)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6652);
	end
end

-- [AUTO-GEN] quest=913 status=1 n_index=6651
if (EVENT == 1004) then
	ShowMap(UID, 84);
end

