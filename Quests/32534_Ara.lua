local NPC = 32534;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8986, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8986, NPC)
	else
		EVENT = QuestNum
	end
end

local savenum = 1041;

if (EVENT == 102) then
	SaveEvent(UID, 983);
end

if (EVENT == 104) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, savenum, 1476, NPC, 22, 105, 23, 106);
	else
		SelectMsg(UID, 2, savenum, 1476, NPC, 10, -1);
	end
end

if (EVENT == 105) then
	SaveEvent(UID, 984);
end

if (EVENT == 106) then
	SaveEvent(UID, 987);
end

if (EVENT == 108) then
	SaveEvent(UID, 986);
end

if (EVENT == 109) then
	MonsterCount = CountMonsterQuestSub(UID, 1041, 1);
	if (MonsterCount < 5) then
		SelectMsg(UID, 2, savenum, 1476, NPC, 18, 110);
	else
		SelectMsg(UID, 4, savenum, 1476, NPC, 41, 111, 23, 110);
	end
end

if (EVENT == 110) then
	ShowMap(UID, 197);
end

if (EVENT == 111) then
	ExpChange(UID, 1400000);
	SaveEvent(UID, 985);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=250 status=2 n_index=985
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 250)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 169);
		SaveEvent(UID, 987);
	end
end

-- [AUTO-GEN] quest=252 status=0 n_index=993
if (EVENT == 112) then
	SelectMsg(UID, 4, 252, 1476, NPC, 154, 113, 23, -1);
end

-- [AUTO-GEN] quest=252 status=0 n_index=993
if (EVENT == 113) then
	SaveEvent(UID, 994);
end

-- [AUTO-GEN] quest=252 status=1 n_index=994
if (EVENT == 116) then
	QuestStatusCheck = GetQuestStatus(UID, 252)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 171);
		SaveEvent(UID, 995);
	end
end

-- [AUTO-GEN] quest=252 status=1 n_index=994
if (EVENT == 117) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 252, 1476, NPC, 22, 116, 23, -1);
	else
		SelectMsg(UID, 2, 252, 1476, NPC, 18, 118);
	end
end

-- [AUTO-GEN] quest=252 status=1 n_index=994
if (EVENT == 118) then
	ShowMap(UID, 73);
end

-- [AUTO-GEN] quest=254 status=0 n_index=1003
if (EVENT == 120) then
	SelectMsg(UID, 4, 254, 1476, NPC, 154, 121, 23, -1);
end

-- [AUTO-GEN] quest=254 status=0 n_index=1003
if (EVENT == 121) then
	SaveEvent(UID, 1004);
end

-- [AUTO-GEN] quest=254 status=1 n_index=1004
if (EVENT == 124) then
	QuestStatusCheck = GetQuestStatus(UID, 254)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 173);
		SaveEvent(UID, 1005);
	end
end

-- [AUTO-GEN] quest=254 status=1 n_index=1004
if (EVENT == 125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 254, 1476, NPC, 22, 124, 23, -1);
	else
		SelectMsg(UID, 2, 254, 1476, NPC, 18, 126);
	end
end

-- [AUTO-GEN] quest=254 status=1 n_index=1004
if (EVENT == 126) then
	ShowMap(UID, 73);
end

-- [AUTO-GEN] quest=256 status=0 n_index=1013
if (EVENT == 130) then
	SelectMsg(UID, 4, 256, 1476, NPC, 154, 131, 23, -1);
end

-- [AUTO-GEN] quest=256 status=0 n_index=1013
if (EVENT == 131) then
	SaveEvent(UID, 1014);
end

-- [AUTO-GEN] quest=256 status=1 n_index=1014
if (EVENT == 134) then
	QuestStatusCheck = GetQuestStatus(UID, 256)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 175);
		SaveEvent(UID, 1015);
	end
end

-- [AUTO-GEN] quest=256 status=1 n_index=1014
if (EVENT == 135) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 256, 1476, NPC, 22, 134, 23, -1);
	else
		SelectMsg(UID, 2, 256, 1476, NPC, 18, 136);
	end
end

-- [AUTO-GEN] quest=256 status=1 n_index=1014
if (EVENT == 136) then
	ShowMap(UID, 73);
end

-- [AUTO-GEN] quest=258 status=0 n_index=1023
if (EVENT == 140) then
	SelectMsg(UID, 4, 258, 1476, NPC, 154, 141, 23, -1);
end

-- [AUTO-GEN] quest=258 status=0 n_index=1023
if (EVENT == 141) then
	SaveEvent(UID, 1024);
end

-- [AUTO-GEN] quest=258 status=1 n_index=1024
if (EVENT == 144) then
	QuestStatusCheck = GetQuestStatus(UID, 258)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 177);
		SaveEvent(UID, 1025);
	end
end

-- [AUTO-GEN] quest=258 status=1 n_index=1024
if (EVENT == 145) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 258, 1476, NPC, 22, 144, 23, -1);
	else
		SelectMsg(UID, 2, 258, 1476, NPC, 18, 146);
	end
end

-- [AUTO-GEN] quest=258 status=1 n_index=1024
if (EVENT == 146) then
	ShowMap(UID, 73);
end

-- [AUTO-GEN] quest=260 status=0 n_index=1033
if (EVENT == 150) then
	SelectMsg(UID, 4, 260, 1476, NPC, 154, 151, 23, -1);
end

-- [AUTO-GEN] quest=260 status=0 n_index=1033
if (EVENT == 151) then
	SaveEvent(UID, 1034);
end

-- [AUTO-GEN] quest=260 status=1 n_index=1034
if (EVENT == 154) then
	QuestStatusCheck = GetQuestStatus(UID, 260)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 179);
		SaveEvent(UID, 1035);
	end
end

-- [AUTO-GEN] quest=260 status=1 n_index=1034
if (EVENT == 155) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 260, 1476, NPC, 22, 154, 23, -1);
	else
		SelectMsg(UID, 2, 260, 1476, NPC, 18, 156);
	end
end

-- [AUTO-GEN] quest=260 status=1 n_index=1034
if (EVENT == 156) then
	ShowMap(UID, 73);
end

-- [AUTO-GEN] quest=262 status=0 n_index=1043
if (EVENT == 160) then
	SelectMsg(UID, 4, 262, 1476, NPC, 154, 161, 23, -1);
end

-- [AUTO-GEN] quest=262 status=0 n_index=1043
if (EVENT == 161) then
	SaveEvent(UID, 1044);
end

-- [AUTO-GEN] quest=262 status=1 n_index=1044
if (EVENT == 164) then
	QuestStatusCheck = GetQuestStatus(UID, 262)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 181);
		SaveEvent(UID, 1045);
	end
end

-- [AUTO-GEN] quest=262 status=1 n_index=1044
if (EVENT == 165) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 262, 1476, NPC, 22, 164, 23, -1);
	else
		SelectMsg(UID, 2, 262, 1476, NPC, 18, 166);
	end
end

-- [AUTO-GEN] quest=262 status=1 n_index=1044
if (EVENT == 166) then
	ShowMap(UID, 73);
end

