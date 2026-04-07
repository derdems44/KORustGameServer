local NPC = 25065;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 43829, NPC)
	else
		EVENT = QuestNum
	end
end

if(EVENT == 112) then 
	SelectMsg(UID, 4, 1242, 43829, NPC, 22, 113, 23, -1);
end

if(EVENT == 113) then
	QuestStatus = GetQuestStatus(UID, 1242)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7560);
	end
end

if(EVENT == 117) then
	QuestStatus = GetQuestStatus(UID, 1242)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1242, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1242, 43829, NPC, 10, 116);
		else
			SaveEvent(UID, 7562);
		end
	end
end

if (EVENT == 115) then
	QuestStatus = GetQuestStatus(UID, 1242)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1242, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1242, 43829, NPC, 10, 116);
		else
			SelectMsg(UID, 4, 1242, 43829, NPC, 10, 118, 27, -1);
		end
	end
end

if(EVENT == 116) then
	ShowMap(UID, 1337);
end

if(EVENT == 118) then
	QuestStatus = GetQuestStatus(UID, 1242)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1242, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1242, 43829, NPC, 10, 116);
		else
			RunQuestExchange(UID,6038);
			SaveEvent(UID, 7561);
		end
	end
end

if(EVENT == 122) then 
	SelectMsg(UID, 4, 1243, 43826, NPC, 10, 123, 23, -1);
end

if(EVENT == 123) then
	QuestStatus = GetQuestStatus(UID, 1243)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7566);
	end
end

if(EVENT == 127) then
	QuestStatus = GetQuestStatus(UID, 1243)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1243, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1243, 43826, NPC, 10, 126);
		else
			SaveEvent(UID, 7568);
		end
	end
end
	
if (EVENT == 125) then
	QuestStatus = GetQuestStatus(UID, 1243)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1243, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1243, 43826, NPC, 10, 126);
		else
			SelectMsg(UID, 4, 1243, 43826, NPC, 10, 128, 27, -1);
		end
	end
end

if(EVENT == 126) then
	ShowMap(UID, 114);
end

if(EVENT == 128) then
	QuestStatus = GetQuestStatus(UID, 1243)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1243, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1243, 43826, NPC, 10, 126);
		else
			RunQuestExchange(UID,6039);
			SaveEvent(UID, 7567);
		end
	end
end

if (EVENT == 132) then
	SelectMsg(UID, 4, 1244, 43833, NPC, 10, 133, 23, -1);
end

if(EVENT == 133) then
	QuestStatus = GetQuestStatus(UID, 1244)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7572);
	end
end

if(EVENT == 137) then
	QuestStatus = GetQuestStatus(UID, 1244)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	KINGAIF = HowmuchItem(UID, 900653000)
		if( KINGAIF < 1) then
			SelectMsg(UID, 2, 1244, 43833, NPC, 18, 136);
		else
			SaveEvent(UID, 7574);
		end
	end
end
	
if(EVENT == 135) then
	QuestStatus = GetQuestStatus(UID, 1244)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	KINGAIF = HowmuchItem(UID, 900653000)
		if( KINGAIF < 1) then
			SelectMsg(UID, 2, 1244, 43833, NPC, 18, 136);
		else
			SelectMsg(UID, 4, 1244, 43833, NPC, 10, 138, 27, -1);
		end
	end
end

if(EVENT == 136) then
	ShowMap(UID, 1246);
end

if(EVENT == 138) then
	QuestStatus = GetQuestStatus(UID, 1244)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	KINGAIF = HowmuchItem(UID, 900653000)
		if( KINGAIF < 1) then
			SelectMsg(UID, 2, 1244, 43833, NPC, 18, 136);
		else
			RunQuestExchange(UID,6040);
			SaveEvent(UID, 7573);
		end
	end
end

if (EVENT == 152) then
	SelectMsg(UID, 4, 1246, 43835, NPC, 10, 153, 23, -1);
end

if(EVENT == 153) then
	QuestStatus = GetQuestStatus(UID, 1246)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7584);
	end
end

if(EVENT == 157) then
	QuestStatus = GetQuestStatus(UID, 1246)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
	EARRIN = HowmuchItem(UID, 900655000) 
		if (BRACE < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 156);
		elseif (EARRIN < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 159);
		else
			SaveEvent(UID, 7586);
		end
	end
end

if(EVENT == 155) then
	QuestStatus = GetQuestStatus(UID, 1246)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
	EARRIN = HowmuchItem(UID, 900655000) 
		if (BRACE < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 156);
		elseif (EARRIN < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 159);
		else
			SelectMsg(UID, 4, 1246, 43833, NPC, 10, 158, 27, -1);
		end
	end
end

if(EVENT == 156) then
	ShowMap(UID, 1314);
end		
	
if(EVENT == 159) then
	ShowMap(UID, 1316);
end	

if(EVENT == 158) then
	QuestStatus = GetQuestStatus(UID, 1246)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
	EARRIN = HowmuchItem(UID, 900655000) 
		if (BRACE < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 156);
		elseif (EARRIN < 1) then
			SelectMsg(UID, 2, 1246, 43833, NPC, 18, 159);
		else
			RunQuestExchange(UID,6042);
			SaveEvent(UID, 7585);
		end
	end
end

if (EVENT == 162) then
	SelectMsg(UID, 4, 1247, 43839, NPC, 10, 163, 23, -1);
end

if(EVENT == 163) then
	QuestStatus = GetQuestStatus(UID, 1247)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7590);
	end
end

if(EVENT == 167) then
	QuestStatus = GetQuestStatus(UID, 1247)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1247, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1247, 43839, NPC, 10, 166);
		else
			SaveEvent(UID, 7592);
		end
	end
end

if (EVENT == 165) then
	QuestStatus = GetQuestStatus(UID, 1247)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1247, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1247, 43839, NPC, 10, 166);
		else
			SelectMsg(UID, 4, 1247, 43839, NPC, 10, 168, 27, -1);
		end
	end
end

if(EVENT == 166) then
	ShowMap(UID, 1314);
end

if(EVENT == 168) then
	QuestStatus = GetQuestStatus(UID, 1247)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1247, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1247, 43839, NPC, 10, 166);
		else
			RunQuestExchange(UID,6043);
			SaveEvent(UID, 7591);
		end
	end
end

if (EVENT == 172) then
	SelectMsg(UID, 4, 1248, 43842, NPC, 10, 173, 23, -1);
end

if(EVENT == 173) then
	QuestStatus = GetQuestStatus(UID, 1248)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7596);
	end
end

if(EVENT == 177) then
	QuestStatus = GetQuestStatus(UID, 1248)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1248, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1248, 43842, NPC, 10, 176);
		else
			SaveEvent(UID, 7598);
		end
	end
end

if (EVENT == 175) then
	QuestStatus = GetQuestStatus(UID, 1248)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1248, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1248, 43842, NPC, 10, 176);
		else
			SelectMsg(UID, 4, 1248, 43842, NPC, 10, 178, 27, -1);
		end
	end
end

if(EVENT == 176) then
ShowMap(UID, 106);
end

if(EVENT == 178) then
	QuestStatus = GetQuestStatus(UID, 1248)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1248, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1248, 43842, NPC, 10, 176);
		else
			RunQuestExchange(UID,6044);
			SaveEvent(UID, 7597);
		end
	end
end

if (EVENT == 182) then
	SelectMsg(UID, 4, 1249, 43845, NPC, 10, 183, 23, -1);
end

if(EVENT == 183) then
	QuestStatus = GetQuestStatus(UID, 1249)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7602);
	end
end

if(EVENT == 187) then
	QuestStatus = GetQuestStatus(UID, 1249)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1249, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1249, 43845, NPC, 10, 186);
		else
			SaveEvent(UID, 7604);
		end
	end
end

if (EVENT == 185) then
	QuestStatus = GetQuestStatus(UID, 1249)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1249, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1249, 43845, NPC, 10, 186);
		else
			SelectMsg(UID, 4, 1249, 43845, NPC, 10, 188, 27, -1);
		end
	end
end

if(EVENT == 186) then
ShowMap(UID, 1316);
end

if(EVENT == 188) then
	QuestStatus = GetQuestStatus(UID, 1249)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1249, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1249, 43845, NPC, 10, 186);
		else
			RunQuestExchange(UID,6045);
			SaveEvent(UID, 7603);
		end
	end
end

if (EVENT == 192) then
	SelectMsg(UID, 4, 1250, 43848, NPC, 10, 193, 23, -1);
end

if(EVENT == 193) then
	QuestStatus = GetQuestStatus(UID, 1250)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
			SaveEvent(UID, 7608);
	end
end

if(EVENT == 197) then
	QuestStatus = GetQuestStatus(UID, 1250)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1250, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1250, 43848, NPC, 10, 196);
		else
			SaveEvent(UID, 7610);
		end
	end
end

if (EVENT == 195) then
	QuestStatus = GetQuestStatus(UID, 1250)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1250, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1250, 43848, NPC, 10, 196);
		else
			SelectMsg(UID, 4, 1250, 43848, NPC, 10, 198, 27, -1);
		end
	end
end

if(EVENT == 196) then
	ShowMap(UID, 546);
end

if(EVENT == 198) then
	QuestStatus = GetQuestStatus(UID, 1250)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43829, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1250, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1250, 43848, NPC, 10, 196);
		else
			RunQuestExchange(UID,6046);
			SaveEvent(UID, 7609);
		end
	end
end

if (EVENT == 202) then
	SelectMsg(UID, 4, 1251, 43851, NPC, 10, 205, 23, -1);
end

if(EVENT == 205) then
    RunQuestExchange(UID,6047);
    SaveEvent(UID, 7614);
	SaveEvent(UID, 7616);
	SaveEvent(UID, 7615);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1242 status=255 n_index=7558
if (EVENT == 110) then
	SaveEvent(UID, 7559);
end

-- [AUTO-GEN] quest=1243 status=255 n_index=7564
if (EVENT == 120) then
	SaveEvent(UID, 7565);
end

-- [AUTO-GEN] quest=1244 status=255 n_index=7570
if (EVENT == 130) then
	SaveEvent(UID, 7571);
end

-- [AUTO-GEN] quest=1246 status=255 n_index=7582
if (EVENT == 150) then
	SaveEvent(UID, 7583);
end

-- [AUTO-GEN] quest=1247 status=255 n_index=7588
if (EVENT == 160) then
	SaveEvent(UID, 7589);
end

-- [AUTO-GEN] quest=1248 status=255 n_index=7594
if (EVENT == 170) then
	SaveEvent(UID, 7595);
end

-- [AUTO-GEN] quest=1249 status=255 n_index=7600
if (EVENT == 180) then
	SaveEvent(UID, 7601);
end

-- [AUTO-GEN] quest=1250 status=255 n_index=7606
if (EVENT == 190) then
	SearchQuest(UID, 25065);
end

-- [AUTO-GEN] quest=1251 status=255 n_index=7612
if (EVENT == 200) then
	SaveEvent(UID, 7613);
end

