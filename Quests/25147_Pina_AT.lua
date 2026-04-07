

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=0 status=0 n_index=6007
if (EVENT == 100) then
	SearchQuest(UID, 25147);
end

-- [AUTO-GEN] quest=1299 status=255 n_index=7986
if (EVENT == 120) then
	SaveEvent(UID, 7987);
end

-- [AUTO-GEN] quest=1299 status=0 n_index=7987
if (EVENT == 122) then
	SelectMsg(UID, 4, 1299, 44147, NPC, 775, 123, 23, -1);
end

-- [AUTO-GEN] quest=1299 status=0 n_index=7987
if (EVENT == 123) then
	SaveEvent(UID, 7988);
end

-- [AUTO-GEN] quest=1299 status=1 n_index=7988
if (EVENT == 125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1299, 44147, NPC, 22, 127, 23, -1);
	else
		SelectMsg(UID, 2, 1299, 44147, NPC, 18, 126);
	end
end

-- [AUTO-GEN] quest=1299 status=1 n_index=7988
if (EVENT == 126) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1299 status=1 n_index=7988
if (EVENT == 127) then
	QuestStatusCheck = GetQuestStatus(UID, 1299)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6098);
		SaveEvent(UID, 7989);
	end
end

