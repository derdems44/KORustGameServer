local NPC = 14425;

if (EVENT == 190) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 4589, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 4590, NPC)
	else
		EVENT = QuestNum
	end
end

local savenum = 248;

if (EVENT == 530) then
	SaveEvent(UID, 4313);
	SelectMsg(UID, 2, savenum, 4591, NPC, 4080, -1);
end

if (EVENT == 532) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 2, savenum, 4592, NPC, 4228, 535, 4063, -1);
	else
		SelectMsg(UID, 2, savenum, 4593, NPC, 10, -1);
	end
end

if (EVENT == 535) then
	SelectMsg(UID, 4, savenum, 4647, NPC, 22, 533, 23, 534);
end

if (EVENT == 533) then
	SaveEvent(UID, 4314);
end

if (EVENT == 534) then
	SaveEvent(UID, 4317);
end

if (EVENT == 180) then
	SaveEvent(UID, 4316);
	SelectMsg(UID, 2, savenum, 4594, NPC, 14, -1);
end

if (EVENT == 536) then
	MonsterCount = CountMonsterQuestSub(UID, 248, 1);
	if (MonsterCount < 10) then
		SelectMsg(UID, 2, savenum, 4595, NPC, 18, 538);
	else
		SelectMsg(UID, 2, savenum, 4596, NPC, 4272, 537, 4273, -1);
	end
end

if (EVENT == 538) then
	ShowMap(UID, 488);
end

if (EVENT == 537) then
	MonsterCount = CountMonsterQuestSub(UID, 248, 1);
	if (MonsterCount < 10) then
		SelectMsg(UID, 2, savenum, 4595, NPC, 18, 538);
	else
	RunQuestExchange(UID,490)
	SaveEvent(UID, 4315); 
end
end

local savenum = 271;

if (EVENT == 9360) then -- 59 Level Ardream Kill 10
	SaveEvent(UID, 9393);
	SelectMsg(UID, 2, savenum, 4591, NPC, 4080, -1);
end

if (EVENT == 9362) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 2, savenum, 8685, NPC, 4228, 9363, 4063, -1);
	else
		SelectMsg(UID, 2, savenum, 8685, NPC, 10, -1);
	end
end

if (EVENT == 9363) then
	SelectMsg(UID, 4, savenum, 8685, NPC, 22, 9364, 23, 9368);
end

if (EVENT == 9364) then
	SaveEvent(UID, 9394);
end

if (EVENT == 9368) then
	SaveEvent(UID, 9397);
end

if (EVENT == 9365) then
	SaveEvent(UID, 9396);
	SelectMsg(UID, 2, savenum, 8685, NPC, 14, -1);
end

if (EVENT == 9367) then
	MonsterCount1 = CountMonsterQuestSub(UID, 271, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 271, 2);
	if (MonsterCount1 < 1 and MonsterCount2 < 1) then
		SelectMsg(UID, 2, savenum, 8685, NPC, 18, 9370);
	else
		SelectMsg(UID, 4, savenum, 8685, NPC, 4272, 9369, 4273, -1);
	end
end

if (EVENT == 9370) then
	ShowMap(UID, 488);
end

if (EVENT == 9369) then
	MonsterCount1 = CountMonsterQuestSub(UID, 271, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 271, 2);
	if (MonsterCount1 < 1 and MonsterCount2 < 1) then
		SelectMsg(UID, 2, savenum, 8685, NPC, 18, 9370);
	else
	RunQuestExchange(UID,1094)
	SaveEvent(UID, 9395);   
end
end

if (EVENT == 400) then
	SelectMsg(UID, 4, 440, 6121, NPC, 10, 401, 4005, -1);
end

if (EVENT == 401) then
    SelectMsg(UID, 15, -1, -1, NPC);
    RunQuestExchange(UID,55)
	SaveEvent(UID, 7134);
end

if (EVENT == 100) then
	SelectMsg(UID, 4, 189, 8878, NPC, 10, -1, 4005, -1);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=189 status=1 n_index=1276
if (EVENT == 120) then
	QuestStatusCheck = GetQuestStatus(UID, 189)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 190);
		SaveEvent(UID, 1277);
	end
end

-- [AUTO-GEN] quest=189 status=2 n_index=1277
if (EVENT == 193) then
	QuestStatusCheck = GetQuestStatus(UID, 189)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 190);
		SaveEvent(UID, 1279);
	end
end

-- [AUTO-GEN] quest=440 status=2 n_index=7134
if (EVENT == 240) then
	QuestStatusCheck = GetQuestStatus(UID, 440)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 55);
		SaveEvent(UID, 7136);
	end
end

