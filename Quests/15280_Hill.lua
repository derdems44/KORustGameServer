local NPC = 15280;

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

-- [AUTO-GEN] quest=1661 status=0 n_index=10959
if (EVENT == 410) then
	SelectMsg(UID, 4, 1661, 0, NPC, 3542, 411, 23, -1);
end

-- [AUTO-GEN] quest=1661 status=0 n_index=10959
if (EVENT == 411) then
	SaveEvent(UID, 10960);
end

-- [AUTO-GEN] quest=1661 status=1 n_index=10960
if (EVENT == 412) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1659 status=0 n_index=10949
if (EVENT == 900) then
	SelectMsg(UID, 4, 1659, 44864, NPC, 3540, 901, 23, -1);
end

-- [AUTO-GEN] quest=1659 status=0 n_index=10949
if (EVENT == 901) then
	SaveEvent(UID, 10950);
end

-- [AUTO-GEN] quest=1659 status=1 n_index=10950
if (EVENT == 903) then
	QuestStatusCheck = GetQuestStatus(UID, 1659)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16241);
		SaveEvent(UID, 10951);
	end
end

-- [AUTO-GEN] quest=1659 status=1 n_index=10950
if (EVENT == 904) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1659, 44864, NPC, 22, 903, 23, -1);
	else
		SelectMsg(UID, 2, 1659, 44864, NPC, 18, 905);
	end
end

-- [AUTO-GEN] quest=1659 status=1 n_index=10950
if (EVENT == 905) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1729 status=0 n_index=14471
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1729, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1729 status=0 n_index=14471
if (EVENT == 9001) then
	SaveEvent(UID, 14472);
end

-- [AUTO-GEN] quest=1729 status=1 n_index=14472
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1729)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16245);
		SaveEvent(UID, 14473);
	end
end

-- [AUTO-GEN] quest=1729 status=1 n_index=14472
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1729, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1729, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1729 status=1 n_index=14472
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10032 status=0 n_index=14491
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10032, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10032 status=0 n_index=14491
if (EVENT == 10001) then
	SaveEvent(UID, 14492);
end

-- [AUTO-GEN] quest=10032 status=1 n_index=14492
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10032)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16252);
		SaveEvent(UID, 14493);
	end
end

-- [AUTO-GEN] quest=10032 status=1 n_index=14492
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10032, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10032, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10032 status=1 n_index=14492
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

