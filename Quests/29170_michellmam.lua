local NPC = 29170;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9149, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 9149, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=910 status=2 n_index=6637
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 910)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6639);
	end
end

-- [AUTO-GEN] quest=910 status=0 n_index=6635
if (EVENT == 1000) then
	SelectMsg(UID, 4, 910, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=910 status=0 n_index=6635
if (EVENT == 1001) then
	SaveEvent(UID, 6636);
end

-- [AUTO-GEN] quest=910 status=1 n_index=6636
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

