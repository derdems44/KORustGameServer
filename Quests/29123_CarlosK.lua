local NPC = 29123;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8778, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8778, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=839 status=2 n_index=6311
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 839)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 243);
		SaveEvent(UID, 6313);
	end
end

-- [AUTO-GEN] quest=847 status=2 n_index=6379
if (EVENT == 201) then
	QuestStatusCheck = GetQuestStatus(UID, 847)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1587);
		SaveEvent(UID, 6381);
	end
end

-- [AUTO-GEN] quest=839 status=0 n_index=6309
if (EVENT == 1602) then
	SelectMsg(UID, 4, 839, 9814, NPC, 1963, 1603, 23, -1);
end

-- [AUTO-GEN] quest=839 status=0 n_index=6309
if (EVENT == 1603) then
	SaveEvent(UID, 6310);
end

-- [AUTO-GEN] quest=839 status=1 n_index=6310
if (EVENT == 1605) then
	QuestStatusCheck = GetQuestStatus(UID, 839)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 243);
		SaveEvent(UID, 6311);
	end
end

-- [AUTO-GEN] quest=839 status=1 n_index=6310
if (EVENT == 1606) then
	ItemA = HowmuchItem(UID, 900371000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 839, 9814, NPC, 18, 1607);
	else
		SelectMsg(UID, 4, 839, 9814, NPC, 41, 1605, 27, -1);
	end
end

-- [AUTO-GEN] quest=839 status=1 n_index=6310
if (EVENT == 1607) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=841 status=0 n_index=6319
if (EVENT == 1702) then
	SelectMsg(UID, 4, 841, 9815, NPC, 1964, 1703, 23, -1);
end

-- [AUTO-GEN] quest=841 status=0 n_index=6319
if (EVENT == 1703) then
	SaveEvent(UID, 6320);
end

-- [AUTO-GEN] quest=841 status=1 n_index=6320
if (EVENT == 1705) then
	QuestStatusCheck = GetQuestStatus(UID, 841)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 245);
		SaveEvent(UID, 6321);
	end
end

-- [AUTO-GEN] quest=841 status=1 n_index=6320
if (EVENT == 1706) then
	ItemA = HowmuchItem(UID, 900375000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 841, 9815, NPC, 18, 1707);
	else
		SelectMsg(UID, 4, 841, 9815, NPC, 41, 1705, 27, -1);
	end
end

-- [AUTO-GEN] quest=841 status=1 n_index=6320
if (EVENT == 1707) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=847 status=0 n_index=6377
if (EVENT == 2102) then
	SelectMsg(UID, 4, 847, 9817, NPC, 1967, 2103, 23, -1);
end

-- [AUTO-GEN] quest=847 status=0 n_index=6377
if (EVENT == 2103) then
	SaveEvent(UID, 6378);
end

-- [AUTO-GEN] quest=847 status=1 n_index=6378
if (EVENT == 2105) then
	QuestStatusCheck = GetQuestStatus(UID, 847)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1587);
		SaveEvent(UID, 6379);
	end
end

-- [AUTO-GEN] quest=847 status=1 n_index=6378
if (EVENT == 2106) then
	ItemA = HowmuchItem(UID, 900369000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 847, 9817, NPC, 18, 2107);
	else
		SelectMsg(UID, 4, 847, 9817, NPC, 41, 2105, 27, -1);
	end
end

-- [AUTO-GEN] quest=847 status=1 n_index=6378
if (EVENT == 2107) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=849 status=0 n_index=6387
if (EVENT == 2202) then
	SelectMsg(UID, 4, 849, 9817, NPC, 1968, 2203, 23, -1);
end

-- [AUTO-GEN] quest=849 status=0 n_index=6387
if (EVENT == 2203) then
	SaveEvent(UID, 6388);
end

-- [AUTO-GEN] quest=849 status=1 n_index=6388
if (EVENT == 2205) then
	QuestStatusCheck = GetQuestStatus(UID, 849)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1634);
		SaveEvent(UID, 6389);
	end
end

-- [AUTO-GEN] quest=849 status=1 n_index=6388
if (EVENT == 2206) then
	ItemA = HowmuchItem(UID, 900370000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 849, 9817, NPC, 18, 2207);
	else
		SelectMsg(UID, 4, 849, 9817, NPC, 41, 2205, 27, -1);
	end
end

-- [AUTO-GEN] quest=849 status=1 n_index=6388
if (EVENT == 2207) then
	ShowMap(UID, 1);
end

