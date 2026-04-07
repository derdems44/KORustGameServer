local NPC = 29222;

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

-- [AUTO-GEN] quest=932 status=2 n_index=10103
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 932)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 10105);
	end
end

-- [AUTO-GEN] quest=957 status=255 n_index=10111
if (EVENT == 105) then
	SaveEvent(UID, 10113);
end

-- [AUTO-GEN] quest=932 status=0 n_index=10101
if (EVENT == 1000) then
	SelectMsg(UID, 4, 932, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=932 status=0 n_index=10101
if (EVENT == 1001) then
	SaveEvent(UID, 10102);
end

-- [AUTO-GEN] quest=932 status=1 n_index=10102
if (EVENT == 1002) then
	SelectMsg(UID, 2, 932, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=932 status=1 n_index=10102
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 932)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 10103);
	end
end

-- [AUTO-GEN] quest=932 status=1 n_index=10102
if (EVENT == 1004) then
	ShowMap(UID, 3);
end

