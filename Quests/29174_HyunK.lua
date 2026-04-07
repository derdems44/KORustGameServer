local NPC = 29174;

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

-- [AUTO-GEN] quest=920 status=2 n_index=6687
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 920)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5006);
		SaveEvent(UID, 6689);
	end
end

-- [AUTO-GEN] quest=920 status=0 n_index=6685
if (EVENT == 1000) then
	SelectMsg(UID, 4, 920, 10207, NPC, 457, 1001, 23, -1);
end

-- [AUTO-GEN] quest=920 status=0 n_index=6685
if (EVENT == 1001) then
	SaveEvent(UID, 6686);
end

-- [AUTO-GEN] quest=920 status=1 n_index=6686
if (EVENT == 1002) then
	ItemA = HowmuchItem(UID, 900428000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 920, 10207, NPC, 18, 1004);
	else
		SelectMsg(UID, 4, 920, 10207, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=920 status=1 n_index=6686
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 920)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5006);
		SaveEvent(UID, 6687);
	end
end

-- [AUTO-GEN] quest=920 status=1 n_index=6686
if (EVENT == 1004) then
	ShowMap(UID, 1);
end

