local NPC = 29182;

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

-- [AUTO-GEN] quest=917 status=2 n_index=6672
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 917)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6674);
	end
end

-- [AUTO-GEN] quest=917 status=0 n_index=6670
if (EVENT == 1000) then
	SelectMsg(UID, 4, 917, 10199, NPC, 457, 1001, 23, -1);
end

-- [AUTO-GEN] quest=917 status=0 n_index=6670
if (EVENT == 1001) then
	SaveEvent(UID, 6671);
end

-- [AUTO-GEN] quest=917 status=1 n_index=6671
if (EVENT == 1002) then
	SelectMsg(UID, 2, 917, 10199, NPC, 10, -1);
end

-- [AUTO-GEN] quest=917 status=1 n_index=6671
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 917)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6672);
	end
end

-- [AUTO-GEN] quest=917 status=1 n_index=6671
if (EVENT == 1004) then
	ShowMap(UID, 2);
end

