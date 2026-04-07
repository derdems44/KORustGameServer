local NPC = 29079;

if (EVENT == 101) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 1171, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 1171, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=967 status=0 n_index=6922
if (EVENT == 1000) then
	SelectMsg(UID, 4, 967, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=967 status=0 n_index=6922
if (EVENT == 1001) then
	SaveEvent(UID, 6923);
end

-- [AUTO-GEN] quest=967 status=1 n_index=6923
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

