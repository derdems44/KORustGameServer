local NPC = 29098;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9754, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 9754, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 1802) then
	SelectMsg(UID, 4, 837, 9754, NPC, 22, 1803, 23, -1);
end


if (EVENT == 1803) then
	SelectMsg(UID, 2, -1, 9754, NPC, 10, -1);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=837 status=2 n_index=6237
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 837)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 240);
		SaveEvent(UID, 6239);
	end
end

