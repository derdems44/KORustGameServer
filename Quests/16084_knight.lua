local NPC = 16084;

if (EVENT == 500) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 1179, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 1179, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1703 status=2 n_index=14286
if (EVENT == 100) then
	SearchQuest(UID, 16084);
end

-- [AUTO-GEN] quest=1703 status=0 n_index=14284
if (EVENT == 510) then
	SelectMsg(UID, 4, 1703, 45191, NPC, 3556, 511, 23, -1);
end

-- [AUTO-GEN] quest=1703 status=0 n_index=14284
if (EVENT == 511) then
	SaveEvent(UID, 14285);
end

-- [AUTO-GEN] quest=1703 status=1 n_index=14285
if (EVENT == 521) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1703, 45191, NPC, 22, 523, 23, -1);
	else
		SelectMsg(UID, 2, 1703, 45191, NPC, 18, 522);
	end
end

-- [AUTO-GEN] quest=1703 status=1 n_index=14285
if (EVENT == 522) then
	ShowMap(UID, 54);
end

-- [AUTO-GEN] quest=1703 status=1 n_index=14285
if (EVENT == 523) then
	QuestStatusCheck = GetQuestStatus(UID, 1703)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6851);
		SaveEvent(UID, 14286);
	end
end

