local NPC = 29142;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 723, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 723, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=887 status=2 n_index=6516
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 887)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2454);
		SaveEvent(UID, 6518);
	end
end

-- [AUTO-GEN] quest=888 status=0 n_index=6519
if (EVENT == 1000) then
	SelectMsg(UID, 4, 888, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=888 status=0 n_index=6519
if (EVENT == 1001) then
	SaveEvent(UID, 6520);
end

-- [AUTO-GEN] quest=888 status=1 n_index=6520
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=887 status=1 n_index=6515
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 887)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2454);
		SaveEvent(UID, 6516);
	end
end

-- [AUTO-GEN] quest=887 status=1 n_index=6515
if (EVENT == 1004) then
	ItemA = HowmuchItem(UID, 508230000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 887, 0, NPC, 18, 1005);
	else
		SelectMsg(UID, 4, 887, 0, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=887 status=1 n_index=6515
if (EVENT == 1005) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=895 status=0 n_index=6554
if (EVENT == 5000) then
	SelectMsg(UID, 4, 895, 0, NPC, 1969, 5001, 23, -1);
end

-- [AUTO-GEN] quest=895 status=0 n_index=6554
if (EVENT == 5001) then
	SaveEvent(UID, 6555);
end

-- [AUTO-GEN] quest=895 status=1 n_index=6555
if (EVENT == 5005) then
	QuestStatusCheck = GetQuestStatus(UID, 895)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2503);
		SaveEvent(UID, 6556);
	end
end

-- [AUTO-GEN] quest=895 status=1 n_index=6555
if (EVENT == 5006) then
	SelectMsg(UID, 4, 895, 0, NPC, 41, 5005, 27, -1);
end

-- [AUTO-GEN] quest=895 status=1 n_index=6555
if (EVENT == 5007) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=897 status=0 n_index=6564
if (EVENT == 7000) then
	SelectMsg(UID, 4, 897, 0, NPC, 1969, 7001, 23, -1);
end

-- [AUTO-GEN] quest=897 status=0 n_index=6564
if (EVENT == 7001) then
	SaveEvent(UID, 6565);
end

-- [AUTO-GEN] quest=897 status=1 n_index=6565
if (EVENT == 7005) then
	QuestStatusCheck = GetQuestStatus(UID, 897)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2503);
		SaveEvent(UID, 6566);
	end
end

-- [AUTO-GEN] quest=897 status=1 n_index=6565
if (EVENT == 7006) then
	SelectMsg(UID, 4, 897, 0, NPC, 41, 7005, 27, -1);
end

-- [AUTO-GEN] quest=897 status=1 n_index=6565
if (EVENT == 7007) then
	ShowMap(UID, 21);
end

