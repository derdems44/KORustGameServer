local NPC = 29185;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 10199, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 10199, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=921 status=2 n_index=6692
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 921)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6694);
	end
end

-- [AUTO-GEN] quest=921 status=0 n_index=6690
if (EVENT == 1000) then
	SelectMsg(UID, 4, 921, 10207, NPC, 457, 1001, 23, -1);
end

-- [AUTO-GEN] quest=921 status=0 n_index=6690
if (EVENT == 1001) then
	SaveEvent(UID, 6691);
end

-- [AUTO-GEN] quest=921 status=1 n_index=6691
if (EVENT == 1002) then
	SelectMsg(UID, 2, 921, 10207, NPC, 10, -1);
end

-- [AUTO-GEN] quest=921 status=1 n_index=6691
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 921)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6692);
	end
end

-- [AUTO-GEN] quest=921 status=1 n_index=6691
if (EVENT == 1004) then
	ShowMap(UID, 1);
end

