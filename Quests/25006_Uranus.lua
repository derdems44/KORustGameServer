local NPC = 25006;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 4515, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 4516, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 112) then
	SelectMsg(UID, 4, 1285, 44135, NPC, 22, 113, 23, -1);
end

if (EVENT == 113) then
	QuestStatus = GetQuestStatus(UID, 1285)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7856);
	end
end

if (EVENT == 114) then
	QuestStatus = GetQuestStatus(UID, 1285)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7858);
	end
end

if (EVENT == 115) then
	QuestStatus = GetQuestStatus(UID, 1285)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1285, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1285, 44135, NPC, 18, 116);
		else
			SelectMsg(UID, 4, 1285, 44135, NPC, 41, 117, 27, -1);
		end
	end
end

if (EVENT == 116) then
	ShowMap(UID, 1245);
end

if (EVENT == 117) then
	QuestStatus = GetQuestStatus(UID, 1285)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1285, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1285, 44135, NPC, 18, 116);
		else
			RunQuestExchange(UID,6081);
			SaveEvent(UID, 7857);
		end
	end
end

if (EVENT == 122) then
	SelectMsg(UID, 4, 1286, 44136, NPC, 22, 123, 23, -1);
end

if (EVENT == 123) then
	QuestStatus = GetQuestStatus(UID, 1286)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7862);
	end
end

if (EVENT == 124) then
	QuestStatus = GetQuestStatus(UID, 1286)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1286, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 1286, 44136, NPC, 18, 126);
		else
			SaveEvent(UID, 7864);
		end
	end
end

if (EVENT == 125) then
	QuestStatus = GetQuestStatus(UID, 1286)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1286, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 1286, 44136, NPC, 18, 126);
		else
			SelectMsg(UID, 4, 1286, 44136, NPC, 41, 127, 27, -1);
		end
	end
end

if (EVENT == 126) then
	ShowMap(UID, 488);
end

if (EVENT == 127) then
	QuestStatus = GetQuestStatus(UID, 1286)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1286, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 1286, 44136, NPC, 18, 126);
		else
			RunQuestExchange(UID,6082);
			SaveEvent(UID, 7863);   
		end
	end
end

if (EVENT == 132) then
	SelectMsg(UID, 4, 1287, 44137, NPC, 22, 133, 23, -1);
end

if (EVENT == 133) then
	QuestStatus = GetQuestStatus(UID, 1287)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7868);
	end
end

if (EVENT == 137) then
	QuestStatus = GetQuestStatus(UID, 1287)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	CONTTA = HowmuchItem(UID, 900687000);
		if (CONTTA < 10) then
			SelectMsg(UID, 2, 1287, 44137, NPC, 19, 134);
		else
			SaveEvent(UID, 7870);
		end
	end
end

if (EVENT == 134) then
	ShowMap(UID, 488);
end

if (EVENT == 135) then
	QuestStatus = GetQuestStatus(UID, 1287)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	CONTTA = HowmuchItem(UID, 900687000);
		if (CONTTA < 10) then
			SelectMsg(UID, 2, 1287, 44137, NPC, 19, 134);
		else
			SelectMsg(UID, 4, 1287, 44137, NPC, 22, 136, 23, -1);
		end
	end
end

if (EVENT == 136) then
	QuestStatus = GetQuestStatus(UID, 1287)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	CONTTA = HowmuchItem(UID, 900687000);
		if (CONTTA < 10) then
			SelectMsg(UID, 2, 1287, 44137, NPC, 19, 134);
		else
			RunQuestExchange(UID,6083);
			SaveEvent(UID, 7869);
		end
	end
end

if (EVENT == 142) then
	SelectMsg(UID, 4, 1288, 44138, NPC, 22, 143, 23, -1);
end

if (EVENT == 143) then
	SaveEvent(UID, 7874);
	SaveEvent(UID, 7876);
end

if (EVENT == 145) then
	SelectMsg(UID, 4, 1288, 44138, NPC, 22, 146, 23, -1);
end

if (EVENT == 146) then
	RunQuestExchange(UID,6084);
	SaveEvent(UID, 7875);   
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1285 status=255 n_index=7854
if (EVENT == 110) then
	SaveEvent(UID, 7855);
end

-- [AUTO-GEN] quest=1286 status=255 n_index=7860
if (EVENT == 120) then
	SaveEvent(UID, 7861);
end

-- [AUTO-GEN] quest=1287 status=255 n_index=7866
if (EVENT == 130) then
	SaveEvent(UID, 7867);
end

-- [AUTO-GEN] quest=1288 status=255 n_index=7872
if (EVENT == 140) then
	SaveEvent(UID, 7873);
end

-- [AUTO-GEN] quest=1288 status=1 n_index=7874
if (EVENT == 147) then
	QuestStatusCheck = GetQuestStatus(UID, 1288)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6084);
		SaveEvent(UID, 7875);
	end
end

-- [AUTO-GEN] quest=1294 status=255 n_index=7908
if (EVENT == 200) then
	SaveEvent(UID, 7909);
end

-- [AUTO-GEN] quest=1294 status=0 n_index=7909
if (EVENT == 202) then
	SelectMsg(UID, 4, 1294, 44142, NPC, 764, 203, 23, -1);
end

-- [AUTO-GEN] quest=1294 status=0 n_index=7909
if (EVENT == 203) then
	SaveEvent(UID, 7910);
end

-- [AUTO-GEN] quest=1294 status=1 n_index=7910
if (EVENT == 205) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1294, 44142, NPC, 22, 207, 23, -1);
	else
		SelectMsg(UID, 2, 1294, 44142, NPC, 18, 206);
	end
end

-- [AUTO-GEN] quest=1294 status=1 n_index=7910
if (EVENT == 206) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1294 status=1 n_index=7910
if (EVENT == 207) then
	QuestStatusCheck = GetQuestStatus(UID, 1294)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6090);
		SaveEvent(UID, 7911);
	end
end

-- [AUTO-GEN] quest=1295 status=255 n_index=7940
if (EVENT == 210) then
	SaveEvent(UID, 7941);
end

-- [AUTO-GEN] quest=1295 status=0 n_index=7941
if (EVENT == 212) then
	SelectMsg(UID, 4, 1295, 44143, NPC, 765, 213, 23, -1);
end

-- [AUTO-GEN] quest=1295 status=0 n_index=7941
if (EVENT == 213) then
	SaveEvent(UID, 7942);
end

-- [AUTO-GEN] quest=1295 status=1 n_index=7942
if (EVENT == 215) then
	ItemA = HowmuchItem(UID, 900657000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1295, 44143, NPC, 18, 216);
	else
		SelectMsg(UID, 4, 1295, 44143, NPC, 41, 217, 27, -1);
	end
end

-- [AUTO-GEN] quest=1295 status=1 n_index=7942
if (EVENT == 216) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1295 status=1 n_index=7942
if (EVENT == 217) then
	QuestStatusCheck = GetQuestStatus(UID, 1295)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6091);
		SaveEvent(UID, 7943);
	end
end

-- [AUTO-GEN] quest=1296 status=255 n_index=7946
if (EVENT == 220) then
	SaveEvent(UID, 7947);
end

-- [AUTO-GEN] quest=1296 status=0 n_index=7947
if (EVENT == 222) then
	SelectMsg(UID, 4, 1296, 44144, NPC, 766, 223, 23, -1);
end

-- [AUTO-GEN] quest=1296 status=0 n_index=7947
if (EVENT == 223) then
	SaveEvent(UID, 7948);
end

-- [AUTO-GEN] quest=1296 status=1 n_index=7948
if (EVENT == 225) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1296, 44144, NPC, 22, 227, 23, -1);
	else
		SelectMsg(UID, 2, 1296, 44144, NPC, 18, 226);
	end
end

-- [AUTO-GEN] quest=1296 status=1 n_index=7948
if (EVENT == 226) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1296 status=1 n_index=7948
if (EVENT == 227) then
	QuestStatusCheck = GetQuestStatus(UID, 1296)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6092);
		SaveEvent(UID, 7949);
	end
end

-- [AUTO-GEN] quest=1297 status=255 n_index=7952
if (EVENT == 230) then
	SaveEvent(UID, 7953);
end

-- [AUTO-GEN] quest=1297 status=0 n_index=7953
if (EVENT == 232) then
	SelectMsg(UID, 4, 1297, 44145, NPC, 767, 233, 23, -1);
end

-- [AUTO-GEN] quest=1297 status=0 n_index=7953
if (EVENT == 233) then
	SaveEvent(UID, 7954);
end

-- [AUTO-GEN] quest=1297 status=1 n_index=7954
if (EVENT == 235) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1297, 44145, NPC, 22, 237, 23, -1);
	else
		SelectMsg(UID, 2, 1297, 44145, NPC, 18, 236);
	end
end

-- [AUTO-GEN] quest=1297 status=1 n_index=7954
if (EVENT == 236) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1297 status=1 n_index=7954
if (EVENT == 237) then
	QuestStatusCheck = GetQuestStatus(UID, 1297)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6093);
		SaveEvent(UID, 7955);
	end
end

-- [AUTO-GEN] quest=1301 status=255 n_index=8004
if (EVENT == 240) then
	SaveEvent(UID, 8005);
end

-- [AUTO-GEN] quest=1301 status=0 n_index=8005
if (EVENT == 242) then
	SelectMsg(UID, 4, 1301, 44149, NPC, 777, 243, 23, -1);
end

-- [AUTO-GEN] quest=1301 status=0 n_index=8005
if (EVENT == 243) then
	SaveEvent(UID, 8006);
end

-- [AUTO-GEN] quest=1301 status=1 n_index=8006
if (EVENT == 245) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1301, 44149, NPC, 22, 247, 23, -1);
	else
		SelectMsg(UID, 2, 1301, 44149, NPC, 18, 246);
	end
end

-- [AUTO-GEN] quest=1301 status=1 n_index=8006
if (EVENT == 246) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1301 status=1 n_index=8006
if (EVENT == 247) then
	QuestStatusCheck = GetQuestStatus(UID, 1301)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6094);
		SaveEvent(UID, 8007);
	end
end

-- [AUTO-GEN] quest=1302 status=255 n_index=8075
if (EVENT == 250) then
	SaveEvent(UID, 8076);
end

-- [AUTO-GEN] quest=1302 status=0 n_index=8076
if (EVENT == 252) then
	SelectMsg(UID, 4, 1302, 44150, NPC, 778, 253, 23, -1);
end

-- [AUTO-GEN] quest=1302 status=0 n_index=8076
if (EVENT == 253) then
	SaveEvent(UID, 8077);
end

-- [AUTO-GEN] quest=1302 status=1 n_index=8077
if (EVENT == 255) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1302, 44150, NPC, 22, 257, 23, -1);
	else
		SelectMsg(UID, 2, 1302, 44150, NPC, 18, 256);
	end
end

-- [AUTO-GEN] quest=1302 status=1 n_index=8077
if (EVENT == 256) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1302 status=1 n_index=8077
if (EVENT == 257) then
	QuestStatusCheck = GetQuestStatus(UID, 1302)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6095);
		SaveEvent(UID, 8078);
	end
end

-- [AUTO-GEN] quest=1303 status=255 n_index=8081
if (EVENT == 260) then
	SaveEvent(UID, 8082);
end

-- [AUTO-GEN] quest=1303 status=0 n_index=8082
if (EVENT == 262) then
	SelectMsg(UID, 4, 1303, 44151, NPC, 779, 263, 23, -1);
end

-- [AUTO-GEN] quest=1303 status=0 n_index=8082
if (EVENT == 263) then
	SaveEvent(UID, 8083);
end

-- [AUTO-GEN] quest=1303 status=1 n_index=8083
if (EVENT == 265) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1303, 44151, NPC, 18, 266);
	else
		SelectMsg(UID, 4, 1303, 44151, NPC, 41, 267, 27, -1);
	end
end

-- [AUTO-GEN] quest=1303 status=1 n_index=8083
if (EVENT == 266) then
	ShowMap(UID, 71);
end

-- [AUTO-GEN] quest=1303 status=1 n_index=8083
if (EVENT == 267) then
	QuestStatusCheck = GetQuestStatus(UID, 1303)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6096);
		SaveEvent(UID, 8084);
	end
end

