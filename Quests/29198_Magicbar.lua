local NPC = 29198;

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

-- [AUTO-GEN] quest=953 status=2 n_index=6852
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 953)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6854);
	end
end

-- [AUTO-GEN] quest=953 status=0 n_index=6850
if (EVENT == 1000) then
	SelectMsg(UID, 4, 953, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=953 status=0 n_index=6850
if (EVENT == 1001) then
	SaveEvent(UID, 6851);
end

-- [AUTO-GEN] quest=953 status=1 n_index=6851
if (EVENT == 1002) then
	ShowMap(UID, 86);
end

