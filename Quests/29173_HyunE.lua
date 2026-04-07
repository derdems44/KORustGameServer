local NPC = 29173;

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

-- [AUTO-GEN] quest=916 status=2 n_index=6667
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 916)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5002);
		SaveEvent(UID, 6669);
	end
end

-- [AUTO-GEN] quest=916 status=0 n_index=6665
if (EVENT == 1000) then
	SelectMsg(UID, 4, 916, 10199, NPC, 457, 1001, 23, -1);
end

-- [AUTO-GEN] quest=916 status=0 n_index=6665
if (EVENT == 1001) then
	SaveEvent(UID, 6666);
end

-- [AUTO-GEN] quest=916 status=1 n_index=6666
if (EVENT == 1002) then
	ItemA = HowmuchItem(UID, 900428000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 916, 10199, NPC, 18, 1004);
	else
		SelectMsg(UID, 4, 916, 10199, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=916 status=1 n_index=6666
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 916)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 5002);
		SaveEvent(UID, 6667);
	end
end

-- [AUTO-GEN] quest=916 status=1 n_index=6666
if (EVENT == 1004) then
	ShowMap(UID, 2);
end

