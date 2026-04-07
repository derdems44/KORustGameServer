local NPC = 29157;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 20543, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 20543, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=898 status=2 n_index=6571
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 898)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6573);
	end
end

-- [AUTO-GEN] quest=898 status=1 n_index=6570
if (EVENT == 1004) then
	SelectMsg(UID, 2, 898, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=898 status=1 n_index=6570
if (EVENT == 1005) then
	QuestStatusCheck = GetQuestStatus(UID, 898)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6571);
	end
end

-- [AUTO-GEN] quest=898 status=1 n_index=6570
if (EVENT == 1006) then
	ShowMap(UID, 21);
end

