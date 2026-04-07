local NPC = 29095;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9745, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 9745, NPC)
	else
		EVENT = QuestNum
	end
end


if (EVENT == 1801) then
	SelectMsg(UID, 4, 838, 9745, NPC, 22, 1802, 23, -1);
end


if (EVENT == 1802) then
	SelectMsg(UID, 2, -1, 9745, NPC, 10, -1);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=838 status=2 n_index=6256
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 838)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1566);
		SaveEvent(UID, 6258);
	end
end

-- [AUTO-GEN] quest=838 status=1 n_index=6255
if (EVENT == 1805) then
	QuestStatusCheck = GetQuestStatus(UID, 838)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1566);
		SaveEvent(UID, 6256);
	end
end

-- [AUTO-GEN] quest=838 status=1 n_index=6255
if (EVENT == 1806) then
	ItemA = HowmuchItem(UID, 900348000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 838, 9745, NPC, 18, 1807);
	else
		SelectMsg(UID, 4, 838, 9745, NPC, 41, 1805, 27, -1);
	end
end

-- [AUTO-GEN] quest=838 status=1 n_index=6255
if (EVENT == 1807) then
	ShowMap(UID, 21);
end

