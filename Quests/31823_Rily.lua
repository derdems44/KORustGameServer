local NPC = 31823;

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

-- [AUTO-GEN] quest=1763 status=0 n_index=14710
if (EVENT == 101) then
	SelectMsg(UID, 4, 1763, 0, NPC, 3573, 102, 23, -1);
end

-- [AUTO-GEN] quest=1763 status=0 n_index=14710
if (EVENT == 102) then
	SaveEvent(UID, 14711);
end

-- [AUTO-GEN] quest=1740 status=0 n_index=14573
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1740, 45486, NPC, 3571, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1740 status=0 n_index=14573
if (EVENT == 9001) then
	SaveEvent(UID, 14574);
end

-- [AUTO-GEN] quest=1740 status=1 n_index=14574
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1740)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16261);
		SaveEvent(UID, 14575);
	end
end

-- [AUTO-GEN] quest=1740 status=1 n_index=14574
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1740, 45486, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1740, 45486, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1740 status=1 n_index=14574
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1749 status=0 n_index=14635
if (EVENT == 9100) then
	SelectMsg(UID, 4, 1749, 45514, NPC, 3576, 9101, 23, -1);
end

-- [AUTO-GEN] quest=1749 status=0 n_index=14635
if (EVENT == 9101) then
	SaveEvent(UID, 14636);
end

-- [AUTO-GEN] quest=1749 status=1 n_index=14636
if (EVENT == 9103) then
	QuestStatusCheck = GetQuestStatus(UID, 1749)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16266);
		SaveEvent(UID, 14637);
	end
end

-- [AUTO-GEN] quest=1749 status=1 n_index=14636
if (EVENT == 9104) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1749, 45514, NPC, 22, 9103, 23, -1);
	else
		SelectMsg(UID, 2, 1749, 45514, NPC, 18, 9105);
	end
end

-- [AUTO-GEN] quest=1749 status=1 n_index=14636
if (EVENT == 9105) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1757 status=0 n_index=14680
if (EVENT == 9200) then
	SelectMsg(UID, 4, 1757, 45562, NPC, 3578, 9201, 23, -1);
end

-- [AUTO-GEN] quest=1757 status=0 n_index=14680
if (EVENT == 9201) then
	SaveEvent(UID, 14681);
end

-- [AUTO-GEN] quest=1757 status=1 n_index=14681
if (EVENT == 9203) then
	QuestStatusCheck = GetQuestStatus(UID, 1757)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16269);
		SaveEvent(UID, 14682);
	end
end

-- [AUTO-GEN] quest=1757 status=1 n_index=14681
if (EVENT == 9204) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1757, 45562, NPC, 22, 9203, 23, -1);
	else
		SelectMsg(UID, 2, 1757, 45562, NPC, 18, 9205);
	end
end

-- [AUTO-GEN] quest=1757 status=1 n_index=14681
if (EVENT == 9205) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10038 status=0 n_index=14583
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10038, 45491, NPC, 3572, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10038 status=0 n_index=14583
if (EVENT == 10001) then
	SaveEvent(UID, 14584);
end

-- [AUTO-GEN] quest=10038 status=1 n_index=14584
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10038)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16263);
		SaveEvent(UID, 14585);
	end
end

-- [AUTO-GEN] quest=10038 status=1 n_index=14584
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10038, 45491, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10038, 45491, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10038 status=1 n_index=14584
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

