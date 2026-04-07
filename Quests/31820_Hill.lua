local NPC = 31820;

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

-- [AUTO-GEN] quest=1762 status=0 n_index=14705
if (EVENT == 102) then
	SelectMsg(UID, 4, 1762, 0, NPC, 3573, 103, 23, -1);
end

-- [AUTO-GEN] quest=1762 status=0 n_index=14705
if (EVENT == 103) then
	SaveEvent(UID, 14706);
end

-- [AUTO-GEN] quest=1737 status=0 n_index=14546
if (EVENT == 9000) then
	SelectMsg(UID, 4, 1737, 45397, NPC, 3565, 9001, 23, -1);
end

-- [AUTO-GEN] quest=1737 status=0 n_index=14546
if (EVENT == 9001) then
	SaveEvent(UID, 14547);
end

-- [AUTO-GEN] quest=1737 status=1 n_index=14547
if (EVENT == 9003) then
	QuestStatusCheck = GetQuestStatus(UID, 1737)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16254);
		SaveEvent(UID, 14548);
	end
end

-- [AUTO-GEN] quest=1737 status=1 n_index=14547
if (EVENT == 9004) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1737, 45397, NPC, 22, 9003, 23, -1);
	else
		SelectMsg(UID, 2, 1737, 45397, NPC, 18, 9005);
	end
end

-- [AUTO-GEN] quest=1737 status=1 n_index=14547
if (EVENT == 9005) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=10037 status=0 n_index=14556
if (EVENT == 10000) then
	SelectMsg(UID, 4, 10037, 45399, NPC, 3566, 10001, 23, -1);
end

-- [AUTO-GEN] quest=10037 status=0 n_index=14556
if (EVENT == 10001) then
	SaveEvent(UID, 14557);
end

-- [AUTO-GEN] quest=10037 status=1 n_index=14557
if (EVENT == 10002) then
	QuestStatusCheck = GetQuestStatus(UID, 10037)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16256);
		SaveEvent(UID, 14558);
	end
end

-- [AUTO-GEN] quest=10037 status=1 n_index=14557
if (EVENT == 10003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10037, 45399, NPC, 18, 10004);
	else
		SelectMsg(UID, 4, 10037, 45399, NPC, 41, 10002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10037 status=1 n_index=14557
if (EVENT == 10004) then
	ShowMap(UID, 71);
end

