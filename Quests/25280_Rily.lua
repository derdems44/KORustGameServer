local NPC = 25280;

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

-- [AUTO-GEN] quest=1660 status=0 n_index=10954
if (EVENT == 410) then
	SelectMsg(UID, 4, 1660, 0, NPC, 3541, 411, 23, -1);
end

-- [AUTO-GEN] quest=1660 status=0 n_index=10954
if (EVENT == 411) then
	SaveEvent(UID, 10955);
end

-- [AUTO-GEN] quest=1660 status=1 n_index=10955
if (EVENT == 412) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1658 status=0 n_index=10944
if (EVENT == 900) then
	SelectMsg(UID, 4, 1658, 44868, NPC, 3539, 901, 23, -1);
end

-- [AUTO-GEN] quest=1658 status=0 n_index=10944
if (EVENT == 901) then
	SaveEvent(UID, 10945);
end

-- [AUTO-GEN] quest=1658 status=1 n_index=10945
if (EVENT == 903) then
	QuestStatusCheck = GetQuestStatus(UID, 1658)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16242);
		SaveEvent(UID, 10946);
	end
end

-- [AUTO-GEN] quest=1658 status=1 n_index=10945
if (EVENT == 904) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1658, 44868, NPC, 22, 903, 23, -1);
	else
		SelectMsg(UID, 2, 1658, 44868, NPC, 18, 905);
	end
end

-- [AUTO-GEN] quest=1658 status=1 n_index=10945
if (EVENT == 905) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1728 status=0 n_index=14466
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1728, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1728 status=0 n_index=14466
if (EVENT == 9001) then
	SaveEvent(UID, 14467);
end

-- [AUTO-GEN] quest=1728 status=1 n_index=14467
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1728)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16246);
		SaveEvent(UID, 14468);
	end
end

-- [AUTO-GEN] quest=1728 status=1 n_index=14467
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1728, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1728, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1728 status=1 n_index=14467
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10031 status=0 n_index=14486
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10031, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10031 status=0 n_index=14486
if (EVENT == 10001) then
	SaveEvent(UID, 14487);
end

-- [AUTO-GEN] quest=10031 status=1 n_index=14487
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10031)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16253);
		SaveEvent(UID, 14488);
	end
end

-- [AUTO-GEN] quest=10031 status=1 n_index=14487
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10031, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10031, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10031 status=1 n_index=14487
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

