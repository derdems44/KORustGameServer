local NPC = 31777;

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

-- [AUTO-GEN] quest=10026 status=1 n_index=14384
if (EVENT == 15000) then
	ItemA = HowmuchItem(UID, 900005000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 10026, 48285, NPC, 18, 15002);
	else
		SelectMsg(UID, 4, 10026, 48285, NPC, 41, 15001, 27, -1);
	end
end

-- [AUTO-GEN] quest=10026 status=2 n_index=14385
if (EVENT == 15001) then
	QuestStatusCheck = GetQuestStatus(UID, 10026)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 16241);
		SaveEvent(UID, 14387);
	end
end

-- [AUTO-GEN] quest=10026 status=1 n_index=14384
if (EVENT == 15002) then
	ShowMap(UID, 21);
end

