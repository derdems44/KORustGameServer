

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1360 status=2 n_index=3871
if (EVENT == 100) then
	SearchQuest(UID, 25173);
end

-- [AUTO-GEN] quest=1360 status=255 n_index=3868
if (EVENT == 1120) then
	SaveEvent(UID, 3869);
end

-- [AUTO-GEN] quest=1360 status=0 n_index=3869
if (EVENT == 1122) then
	SelectMsg(UID, 4, 1360, 44148, NPC, 776, 1123, 23, -1);
end

-- [AUTO-GEN] quest=1360 status=0 n_index=3869
if (EVENT == 1123) then
	SaveEvent(UID, 3870);
end

-- [AUTO-GEN] quest=1360 status=1 n_index=3870
if (EVENT == 1125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1360, 44148, NPC, 22, 1127, 23, -1);
	else
		SelectMsg(UID, 2, 1360, 44148, NPC, 18, 1126);
	end
end

-- [AUTO-GEN] quest=1360 status=1 n_index=3870
if (EVENT == 1126) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1360 status=1 n_index=3870
if (EVENT == 1127) then
	QuestStatusCheck = GetQuestStatus(UID, 1360)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6154);
		SaveEvent(UID, 3871);
	end
end

