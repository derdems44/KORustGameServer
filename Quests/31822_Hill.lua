local NPC = 31822;

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

-- [AUTO-GEN] quest=1764 status=0 n_index=14715
if (EVENT == 101) then
	SelectMsg(UID, 4, 1764, 0, NPC, 3573, 102, 23, -1);
end

-- [AUTO-GEN] quest=1764 status=0 n_index=14715
if (EVENT == 102) then
	SaveEvent(UID, 14716);
end

-- [AUTO-GEN] quest=1741 status=0 n_index=14578
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1741, 45489, NPC, 3571, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1741 status=0 n_index=14578
if (EVENT == 9001) then
	SaveEvent(UID, 14579);
end

-- [AUTO-GEN] quest=1741 status=1 n_index=14579
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1741)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16260);
		SaveEvent(UID, 14580);
	end
end

-- [AUTO-GEN] quest=1741 status=1 n_index=14579
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1741, 45489, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1741, 45489, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1741 status=1 n_index=14579
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1750 status=0 n_index=14640
if (EVENT == 9100) then
	SelectMsg(UID, 4, 1750, 45515, NPC, 3576, 9101, 23, -1);
end

-- [AUTO-GEN] quest=1750 status=0 n_index=14640
if (EVENT == 9101) then
	SaveEvent(UID, 14641);
end

-- [AUTO-GEN] quest=1750 status=1 n_index=14641
if (EVENT == 9103) then
	QuestStatusCheck = GetQuestStatus(UID, 1750)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16265);
		SaveEvent(UID, 14642);
	end
end

-- [AUTO-GEN] quest=1750 status=1 n_index=14641
if (EVENT == 9104) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1750, 45515, NPC, 22, 9103, 23, -1);
	else
		SelectMsg(UID, 2, 1750, 45515, NPC, 18, 9105);
	end
end

-- [AUTO-GEN] quest=1750 status=1 n_index=14641
if (EVENT == 9105) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1758 status=0 n_index=14685
if (EVENT == 9200) then
	SelectMsg(UID, 4, 1758, 45563, NPC, 3578, 9201, 23, -1);
end

-- [AUTO-GEN] quest=1758 status=0 n_index=14685
if (EVENT == 9201) then
	SaveEvent(UID, 14686);
end

-- [AUTO-GEN] quest=1758 status=1 n_index=14686
if (EVENT == 9203) then
	QuestStatusCheck = GetQuestStatus(UID, 1758)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16270);
		SaveEvent(UID, 14687);
	end
end

-- [AUTO-GEN] quest=1758 status=1 n_index=14686
if (EVENT == 9204) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1758, 45563, NPC, 22, 9203, 23, -1);
	else
		SelectMsg(UID, 2, 1758, 45563, NPC, 18, 9205);
	end
end

-- [AUTO-GEN] quest=1758 status=1 n_index=14686
if (EVENT == 9205) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10039 status=0 n_index=14588
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10039, 45491, NPC, 3572, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10039 status=0 n_index=14588
if (EVENT == 10001) then
	SaveEvent(UID, 14589);
end

-- [AUTO-GEN] quest=10039 status=1 n_index=14589
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10039)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16262);
		SaveEvent(UID, 14590);
	end
end

-- [AUTO-GEN] quest=10039 status=1 n_index=14589
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10039, 45491, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10039, 45491, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10039 status=1 n_index=14589
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

