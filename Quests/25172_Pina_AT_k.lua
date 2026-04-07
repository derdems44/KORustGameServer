

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1359 status=2 n_index=3865
if (EVENT == 100) then
	SearchQuest(UID, 25172);
end

-- [AUTO-GEN] quest=1359 status=255 n_index=3862
if (EVENT == 1120) then
	SaveEvent(UID, 3863);
end

-- [AUTO-GEN] quest=1359 status=0 n_index=3863
if (EVENT == 1122) then
	SelectMsg(UID, 4, 1359, 44147, NPC, 775, 1123, 23, -1);
end

-- [AUTO-GEN] quest=1359 status=0 n_index=3863
if (EVENT == 1123) then
	SaveEvent(UID, 3864);
end

-- [AUTO-GEN] quest=1359 status=1 n_index=3864
if (EVENT == 1125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1359, 44147, NPC, 22, 1127, 23, -1);
	else
		SelectMsg(UID, 2, 1359, 44147, NPC, 18, 1126);
	end
end

-- [AUTO-GEN] quest=1359 status=1 n_index=3864
if (EVENT == 1126) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1359 status=1 n_index=3864
if (EVENT == 1127) then
	QuestStatusCheck = GetQuestStatus(UID, 1359)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6153);
		SaveEvent(UID, 3865);
	end
end

