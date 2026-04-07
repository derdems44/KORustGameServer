local NPC = 29180;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 10233, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 10233, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=926 status=2 n_index=6717
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 926)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6719);
	end
end

-- [AUTO-GEN] quest=926 status=0 n_index=6715
if (EVENT == 1000) then
	SelectMsg(UID, 4, 926, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=926 status=0 n_index=6715
if (EVENT == 1001) then
	SaveEvent(UID, 6716);
end

-- [AUTO-GEN] quest=926 status=1 n_index=6716
if (EVENT == 1002) then
	SelectMsg(UID, 2, 926, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=926 status=1 n_index=6716
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 926)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6717);
	end
end

-- [AUTO-GEN] quest=926 status=1 n_index=6716
if (EVENT == 1004) then
	ShowMap(UID, 21);
end

