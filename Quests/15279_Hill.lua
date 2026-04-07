local NPC = 15279;

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

-- [AUTO-GEN] quest=1725 status=0 n_index=14451
if (EVENT == 340) then
	SelectMsg(UID, 4, 1725, 44282, NPC, 3563, 341, 23, -1);
end

-- [AUTO-GEN] quest=1725 status=0 n_index=14451
if (EVENT == 341) then
	SaveEvent(UID, 14452);
end

-- [AUTO-GEN] quest=1725 status=1 n_index=14452
if (EVENT == 342) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1725, 44282, NPC, 22, 341, 23, -1);
	else
		SelectMsg(UID, 2, 1725, 44282, NPC, 18, 343);
	end
end

-- [AUTO-GEN] quest=1725 status=1 n_index=14452
if (EVENT == 343) then
	ShowMap(UID, 11);
end

-- [AUTO-GEN] quest=1648 status=0 n_index=10914
if (EVENT == 400) then
	SelectMsg(UID, 4, 1648, 0, NPC, 3534, 401, 23, -1);
end

-- [AUTO-GEN] quest=1648 status=0 n_index=10914
if (EVENT == 401) then
	SaveEvent(UID, 10915);
end

-- [AUTO-GEN] quest=1648 status=1 n_index=10915
if (EVENT == 402) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1657 status=0 n_index=10939
if (EVENT == 410) then
	SelectMsg(UID, 4, 1657, 0, NPC, 3538, 411, 23, -1);
end

-- [AUTO-GEN] quest=1657 status=0 n_index=10939
if (EVENT == 411) then
	SaveEvent(UID, 10940);
end

-- [AUTO-GEN] quest=1657 status=1 n_index=10940
if (EVENT == 412) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1646 status=0 n_index=10904
if (EVENT == 840) then
	SelectMsg(UID, 4, 1646, 44844, NPC, 3532, 841, 23, -1);
end

-- [AUTO-GEN] quest=1646 status=0 n_index=10904
if (EVENT == 841) then
	SaveEvent(UID, 10905);
end

-- [AUTO-GEN] quest=1646 status=1 n_index=10905
if (EVENT == 842) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1646, 44844, NPC, 22, 841, 23, -1);
	else
		SelectMsg(UID, 2, 1646, 44844, NPC, 18, 843);
	end
end

-- [AUTO-GEN] quest=1646 status=1 n_index=10905
if (EVENT == 843) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1650 status=0 n_index=10929
if (EVENT == 900) then
	SelectMsg(UID, 4, 1650, 44864, NPC, 3536, 901, 23, -1);
end

-- [AUTO-GEN] quest=1650 status=0 n_index=10929
if (EVENT == 901) then
	SaveEvent(UID, 10930);
end

-- [AUTO-GEN] quest=1650 status=1 n_index=10930
if (EVENT == 903) then
	QuestStatusCheck = GetQuestStatus(UID, 1650)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16239);
		SaveEvent(UID, 10931);
	end
end

-- [AUTO-GEN] quest=1650 status=1 n_index=10930
if (EVENT == 904) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1650, 44864, NPC, 22, 903, 23, -1);
	else
		SelectMsg(UID, 2, 1650, 44864, NPC, 18, 905);
	end
end

-- [AUTO-GEN] quest=1650 status=1 n_index=10930
if (EVENT == 905) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1727 status=0 n_index=14461
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1727, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1727 status=0 n_index=14461
if (EVENT == 9001) then
	SaveEvent(UID, 14462);
end

-- [AUTO-GEN] quest=1727 status=1 n_index=14462
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1727)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16243);
		SaveEvent(UID, 14463);
	end
end

-- [AUTO-GEN] quest=1727 status=1 n_index=14462
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1727, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1727, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1727 status=1 n_index=14462
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10030 status=0 n_index=14481
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10030, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10030 status=0 n_index=14481
if (EVENT == 10001) then
	SaveEvent(UID, 14482);
end

-- [AUTO-GEN] quest=10030 status=1 n_index=14482
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10030)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16250);
		SaveEvent(UID, 14483);
	end
end

-- [AUTO-GEN] quest=10030 status=1 n_index=14482
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10030, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10030, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10030 status=1 n_index=14482
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

