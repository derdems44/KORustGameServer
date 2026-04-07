local NPC = 31770;

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

-- [AUTO-GEN] quest=1707 status=0 n_index=14331
if (EVENT == 2100) then
	SelectMsg(UID, 4, 1707, 44955, NPC, 22, 2101, 23, -1);
end

-- [AUTO-GEN] quest=1707 status=0 n_index=14331
if (EVENT == 2101) then
	SaveEvent(UID, 14332);
end

-- [AUTO-GEN] quest=1708 status=0 n_index=14336
if (EVENT == 2200) then
	SelectMsg(UID, 4, 1708, 44957, NPC, 3550, 2201, 23, -1);
end

-- [AUTO-GEN] quest=1708 status=0 n_index=14336
if (EVENT == 2201) then
	SaveEvent(UID, 14337);
end

-- [AUTO-GEN] quest=1708 status=1 n_index=14337
if (EVENT == 2202) then
	QuestStatusCheck = GetQuestStatus(UID, 1708)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6854);
		SaveEvent(UID, 14338);
	end
end

-- [AUTO-GEN] quest=1708 status=1 n_index=14337
if (EVENT == 2203) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1708, 44957, NPC, 22, 2202, 23, -1);
	else
		SelectMsg(UID, 2, 1708, 44957, NPC, 18, 2204);
	end
end

-- [AUTO-GEN] quest=1708 status=1 n_index=14337
if (EVENT == 2204) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=1709 status=0 n_index=14341
if (EVENT == 2300) then
	SelectMsg(UID, 4, 1709, 44961, NPC, 3551, 2301, 23, -1);
end

-- [AUTO-GEN] quest=1709 status=0 n_index=14341
if (EVENT == 2301) then
	SaveEvent(UID, 14342);
end

-- [AUTO-GEN] quest=1709 status=1 n_index=14342
if (EVENT == 2302) then
	QuestStatusCheck = GetQuestStatus(UID, 1709)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6855);
		SaveEvent(UID, 14343);
	end
end

-- [AUTO-GEN] quest=1709 status=1 n_index=14342
if (EVENT == 2303) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1709, 44961, NPC, 22, 2302, 23, -1);
	else
		SelectMsg(UID, 2, 1709, 44961, NPC, 18, 2304);
	end
end

-- [AUTO-GEN] quest=1709 status=1 n_index=14342
if (EVENT == 2304) then
	ShowMap(UID, 21);
end

