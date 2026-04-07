local NPC = 31821;

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

-- [AUTO-GEN] quest=1761 status=0 n_index=14700
if (EVENT == 102) then
	SelectMsg(UID, 4, 1761, 0, NPC, 3573, 103, 23, -1);
end

-- [AUTO-GEN] quest=1761 status=0 n_index=14700
if (EVENT == 103) then
	SaveEvent(UID, 14701);
end

-- [AUTO-GEN] quest=1736 status=0 n_index=14541
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1736, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1736 status=0 n_index=14541
if (EVENT == 9001) then
	SaveEvent(UID, 14542);
end

-- [AUTO-GEN] quest=1736 status=1 n_index=14542
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1736)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16255);
		SaveEvent(UID, 14543);
	end
end

-- [AUTO-GEN] quest=1736 status=1 n_index=14542
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1736, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1736, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1736 status=1 n_index=14542
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10036 status=0 n_index=14551
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10036, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10036 status=0 n_index=14551
if (EVENT == 10001) then
	SaveEvent(UID, 14552);
end

-- [AUTO-GEN] quest=10036 status=1 n_index=14552
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10036)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16257);
		SaveEvent(UID, 14553);
	end
end

-- [AUTO-GEN] quest=10036 status=1 n_index=14552
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10036, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10036, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10036 status=1 n_index=14552
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

