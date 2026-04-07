local NPC = 29094;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8662, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8662, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=828 status=2 n_index=2881
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 828)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 2883);
	end
end

-- [AUTO-GEN] quest=828 status=0 n_index=2879
if (EVENT == 1000) then
	SelectMsg(UID, 4, 828, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=828 status=0 n_index=2879
if (EVENT == 1001) then
	SaveEvent(UID, 2880);
end

-- [AUTO-GEN] quest=828 status=1 n_index=2880
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 828)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 2881);
	end
end

-- [AUTO-GEN] quest=828 status=1 n_index=2880
if (EVENT == 1004) then
	SelectMsg(UID, 2, 828, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=828 status=1 n_index=2880
if (EVENT == 1005) then
	ShowMap(UID, 21);
end

