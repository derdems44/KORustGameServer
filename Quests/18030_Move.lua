local NPC = 18030;

if (EVENT == 240) then
	SelectMsg(UID, 2, -1, 40001, NPC, 4071, 102, 4072, -1);
end

if (EVENT == 102) then
	NATION = CheckNation(UID);
	if (NATION == 2) then
		ZoneChange(UID, 2, 1705, 306)
	else
		ZoneChange(UID, 1, 360, 1742)
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1704 status=2 n_index=14291
if (EVENT == 100) then
	SearchQuest(UID, 18030);
end

-- [AUTO-GEN] quest=1704 status=0 n_index=14289
if (EVENT == 510) then
	SelectMsg(UID, 4, 1704, 45191, NPC, 3557, 511, 23, -1);
end

-- [AUTO-GEN] quest=1704 status=0 n_index=14289
if (EVENT == 511) then
	SaveEvent(UID, 14290);
end

-- [AUTO-GEN] quest=1704 status=1 n_index=14290
if (EVENT == 521) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1704, 45191, NPC, 22, 523, 23, -1);
	else
		SelectMsg(UID, 2, 1704, 45191, NPC, 18, 522);
	end
end

-- [AUTO-GEN] quest=1704 status=1 n_index=14290
if (EVENT == 522) then
	ShowMap(UID, 93);
end

-- [AUTO-GEN] quest=1704 status=1 n_index=14290
if (EVENT == 523) then
	QuestStatusCheck = GetQuestStatus(UID, 1704)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6852);
		SaveEvent(UID, 14291);
	end
end

-- [AUTO-GEN] quest=1705 status=0 n_index=14294
if (EVENT == 610) then
	SelectMsg(UID, 4, 1705, 45191, NPC, 3558, 611, 23, -1);
end

-- [AUTO-GEN] quest=1705 status=0 n_index=14294
if (EVENT == 611) then
	SaveEvent(UID, 14295);
end

-- [AUTO-GEN] quest=1705 status=1 n_index=14295
if (EVENT == 621) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1705, 45191, NPC, 22, 623, 23, -1);
	else
		SelectMsg(UID, 2, 1705, 45191, NPC, 18, 622);
	end
end

-- [AUTO-GEN] quest=1705 status=1 n_index=14295
if (EVENT == 622) then
	ShowMap(UID, 94);
end

-- [AUTO-GEN] quest=1705 status=1 n_index=14295
if (EVENT == 623) then
	QuestStatusCheck = GetQuestStatus(UID, 1705)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6853);
		SaveEvent(UID, 14296);
	end
end

