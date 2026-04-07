local NPC = 31780;

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

-- [AUTO-GEN] quest=10027 status=1 n_index=14379
if (EVENT == 1001) then
	ItemA = HowmuchItem(UID, 900005000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10027, 48285, NPC, 18, 1003);
	else
		SelectMsg(UID, 4, 10027, 48285, NPC, 41, 1002, 27, -1);
	end
end

-- [AUTO-GEN] quest=10027 status=2 n_index=14380
if (EVENT == 1002) then
	QuestStatusCheck = GetQuestStatus(UID, 10027)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16241);
		SaveEvent(UID, 14382);
	end
end

-- [AUTO-GEN] quest=10027 status=1 n_index=14379
if (EVENT == 1003) then
	ShowMap(UID, 21);
end

