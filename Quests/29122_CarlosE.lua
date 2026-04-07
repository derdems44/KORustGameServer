local NPC = 29122;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8779, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=840 status=2 n_index=6316
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 840)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 243);
		SaveEvent(UID, 6318);
	end
end

-- [AUTO-GEN] quest=848 status=2 n_index=6384
if (EVENT == 201) then
	QuestStatusCheck = GetQuestStatus(UID, 848)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1587);
		SaveEvent(UID, 6386);
	end
end

-- [AUTO-GEN] quest=840 status=0 n_index=6314
if (EVENT == 1602) then
	SelectMsg(UID, 4, 840, 9819, NPC, 1963, 1603, 23, -1);
end

-- [AUTO-GEN] quest=840 status=0 n_index=6314
if (EVENT == 1603) then
	SaveEvent(UID, 6315);
end

-- [AUTO-GEN] quest=840 status=1 n_index=6315
if (EVENT == 1605) then
	QuestStatusCheck = GetQuestStatus(UID, 840)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 243);
		SaveEvent(UID, 6316);
	end
end

-- [AUTO-GEN] quest=840 status=1 n_index=6315
if (EVENT == 1606) then
	ItemA = HowmuchItem(UID, 900371000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 840, 9819, NPC, 18, 1607);
	else
		SelectMsg(UID, 4, 840, 9819, NPC, 41, 1605, 27, -1);
	end
end

-- [AUTO-GEN] quest=840 status=1 n_index=6315
if (EVENT == 1607) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=842 status=0 n_index=6324
if (EVENT == 1702) then
	SelectMsg(UID, 4, 842, 9820, NPC, 1964, 1703, 23, -1);
end

-- [AUTO-GEN] quest=842 status=0 n_index=6324
if (EVENT == 1703) then
	SaveEvent(UID, 6325);
end

-- [AUTO-GEN] quest=842 status=1 n_index=6325
if (EVENT == 1705) then
	QuestStatusCheck = GetQuestStatus(UID, 842)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 245);
		SaveEvent(UID, 6326);
	end
end

-- [AUTO-GEN] quest=842 status=1 n_index=6325
if (EVENT == 1706) then
	ItemA = HowmuchItem(UID, 900375000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 842, 9820, NPC, 18, 1707);
	else
		SelectMsg(UID, 4, 842, 9820, NPC, 41, 1705, 27, -1);
	end
end

-- [AUTO-GEN] quest=842 status=1 n_index=6325
if (EVENT == 1707) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=848 status=0 n_index=6382
if (EVENT == 2102) then
	SelectMsg(UID, 4, 848, 9822, NPC, 1967, 2103, 23, -1);
end

-- [AUTO-GEN] quest=848 status=0 n_index=6382
if (EVENT == 2103) then
	SaveEvent(UID, 6383);
end

-- [AUTO-GEN] quest=848 status=1 n_index=6383
if (EVENT == 2105) then
	QuestStatusCheck = GetQuestStatus(UID, 848)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1587);
		SaveEvent(UID, 6384);
	end
end

-- [AUTO-GEN] quest=848 status=1 n_index=6383
if (EVENT == 2106) then
	ItemA = HowmuchItem(UID, 900369000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 848, 9822, NPC, 18, 2107);
	else
		SelectMsg(UID, 4, 848, 9822, NPC, 41, 2105, 27, -1);
	end
end

-- [AUTO-GEN] quest=848 status=1 n_index=6383
if (EVENT == 2107) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=850 status=0 n_index=6392
if (EVENT == 2202) then
	SelectMsg(UID, 4, 850, 9822, NPC, 1968, 2203, 23, -1);
end

-- [AUTO-GEN] quest=850 status=0 n_index=6392
if (EVENT == 2203) then
	SaveEvent(UID, 6393);
end

-- [AUTO-GEN] quest=850 status=1 n_index=6393
if (EVENT == 2205) then
	QuestStatusCheck = GetQuestStatus(UID, 850)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1634);
		SaveEvent(UID, 6394);
	end
end

-- [AUTO-GEN] quest=850 status=1 n_index=6393
if (EVENT == 2206) then
	ItemA = HowmuchItem(UID, 900370000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 850, 9822, NPC, 18, 2207);
	else
		SelectMsg(UID, 4, 850, 9822, NPC, 41, 2205, 27, -1);
	end
end

-- [AUTO-GEN] quest=850 status=1 n_index=6393
if (EVENT == 2207) then
	ShowMap(UID, 2);
end

