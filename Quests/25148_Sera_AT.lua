

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=0 status=0 n_index=6008
if (EVENT == 100) then
	SearchQuest(UID, 25148);
end

-- [AUTO-GEN] quest=1300 status=255 n_index=7992
if (EVENT == 120) then
	SaveEvent(UID, 7993);
end

-- [AUTO-GEN] quest=1300 status=0 n_index=7993
if (EVENT == 122) then
	SelectMsg(UID, 4, 1300, 44148, NPC, 776, 123, 23, -1);
end

-- [AUTO-GEN] quest=1300 status=0 n_index=7993
if (EVENT == 123) then
	SaveEvent(UID, 7994);
end

-- [AUTO-GEN] quest=1300 status=1 n_index=7994
if (EVENT == 125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1300, 44148, NPC, 22, 127, 23, -1);
	else
		SelectMsg(UID, 2, 1300, 44148, NPC, 18, 126);
	end
end

-- [AUTO-GEN] quest=1300 status=1 n_index=7994
if (EVENT == 126) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1300 status=1 n_index=7994
if (EVENT == 127) then
	QuestStatusCheck = GetQuestStatus(UID, 1300)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6099);
		SaveEvent(UID, 7995);
	end
end

