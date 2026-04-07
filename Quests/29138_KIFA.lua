local NPC = 29138;

if (EVENT == 101) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 820, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 820, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=0 status=0 n_index=5548
if (EVENT == 100) then
	SearchQuest(UID, 29138);
end

-- [AUTO-GEN] quest=873 status=0 n_index=6444
if (EVENT == 1000) then
	SelectMsg(UID, 4, 873, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=873 status=0 n_index=6444
if (EVENT == 1001) then
	SaveEvent(UID, 6445);
end

-- [AUTO-GEN] quest=873 status=1 n_index=6445
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=872 status=1 n_index=6440
if (EVENT == 1003) then
	QuestStatusCheck = GetQuestStatus(UID, 872)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2453);
		SaveEvent(UID, 6441);
	end
end

-- [AUTO-GEN] quest=872 status=1 n_index=6440
if (EVENT == 1004) then
	ItemA = HowmuchItem(UID, 900378000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 872, 0, NPC, 18, 1005);
	else
		SelectMsg(UID, 4, 872, 0, NPC, 41, 1003, 27, -1);
	end
end

-- [AUTO-GEN] quest=872 status=1 n_index=6440
if (EVENT == 1005) then
	ShowMap(UID, 21);
end

