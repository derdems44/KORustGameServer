local NPC = 25279;

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

-- [AUTO-GEN] quest=1724 status=0 n_index=14446
if (EVENT == 340) then
	SelectMsg(UID, 4, 1724, 44282, NPC, 3563, 341, 23, -1);
end

-- [AUTO-GEN] quest=1724 status=0 n_index=14446
if (EVENT == 341) then
	SaveEvent(UID, 14447);
end

-- [AUTO-GEN] quest=1724 status=1 n_index=14447
if (EVENT == 342) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1724, 44282, NPC, 22, 341, 23, -1);
	else
		SelectMsg(UID, 2, 1724, 44282, NPC, 18, 343);
	end
end

-- [AUTO-GEN] quest=1724 status=1 n_index=14447
if (EVENT == 343) then
	ShowMap(UID, 12);
end

-- [AUTO-GEN] quest=1647 status=0 n_index=10909
if (EVENT == 400) then
	SelectMsg(UID, 4, 1647, 0, NPC, 3533, 401, 23, -1);
end

-- [AUTO-GEN] quest=1647 status=0 n_index=10909
if (EVENT == 401) then
	SaveEvent(UID, 10910);
end

-- [AUTO-GEN] quest=1647 status=1 n_index=10910
if (EVENT == 402) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1651 status=0 n_index=10934
if (EVENT == 410) then
	SelectMsg(UID, 4, 1651, 0, NPC, 3537, 411, 23, -1);
end

-- [AUTO-GEN] quest=1651 status=0 n_index=10934
if (EVENT == 411) then
	SaveEvent(UID, 10935);
end

-- [AUTO-GEN] quest=1651 status=1 n_index=10935
if (EVENT == 412) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1645 status=0 n_index=10899
if (EVENT == 840) then
	SelectMsg(UID, 4, 1645, 44844, NPC, 3531, 841, 23, -1);
end

-- [AUTO-GEN] quest=1645 status=0 n_index=10899
if (EVENT == 841) then
	SaveEvent(UID, 10900);
end

-- [AUTO-GEN] quest=1645 status=1 n_index=10900
if (EVENT == 842) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1645, 44844, NPC, 22, 841, 23, -1);
	else
		SelectMsg(UID, 2, 1645, 44844, NPC, 18, 843);
	end
end

-- [AUTO-GEN] quest=1645 status=1 n_index=10900
if (EVENT == 843) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1649 status=0 n_index=10924
if (EVENT == 900) then
	SelectMsg(UID, 4, 1649, 44868, NPC, 3535, 901, 23, -1);
end

-- [AUTO-GEN] quest=1649 status=0 n_index=10924
if (EVENT == 901) then
	SaveEvent(UID, 10925);
end

-- [AUTO-GEN] quest=1649 status=1 n_index=10925
if (EVENT == 903) then
	QuestStatusCheck = GetQuestStatus(UID, 1649)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16240);
		SaveEvent(UID, 10926);
	end
end

-- [AUTO-GEN] quest=1649 status=1 n_index=10925
if (EVENT == 904) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1649, 44868, NPC, 22, 903, 23, -1);
	else
		SelectMsg(UID, 2, 1649, 44868, NPC, 18, 905);
	end
end

-- [AUTO-GEN] quest=1649 status=1 n_index=10925
if (EVENT == 905) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1726 status=0 n_index=14456
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1726, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1726 status=0 n_index=14456
if (EVENT == 9001) then
	SaveEvent(UID, 14457);
end

-- [AUTO-GEN] quest=1726 status=1 n_index=14457
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1726)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16244);
		SaveEvent(UID, 14458);
	end
end

-- [AUTO-GEN] quest=1726 status=1 n_index=14457
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1726, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1726, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1726 status=1 n_index=14457
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10029 status=0 n_index=14476
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10029, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10029 status=0 n_index=14476
if (EVENT == 10001) then
	SaveEvent(UID, 14477);
end

-- [AUTO-GEN] quest=10029 status=1 n_index=14477
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10029)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16251);
		SaveEvent(UID, 14478);
	end
end

-- [AUTO-GEN] quest=10029 status=1 n_index=14477
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10029, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10029, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10029 status=1 n_index=14477
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

