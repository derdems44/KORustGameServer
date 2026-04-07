local NPC = 25005;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then 
		SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then 
		NpcMsg(UID, 43943, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 112) then 
	SelectMsg(UID, 2, 1274, 44124, NPC, 40307, 113);
end

if (EVENT == 113) then 
	SelectMsg(UID, 2, 1274, 44124, NPC, 40308, 114);
end

if (EVENT == 114) then 
	SelectMsg(UID, 2, 1274, 44124, NPC, 40309, 115);
end

if (EVENT == 115) then 
	SelectMsg(UID, 4, 1274, 44124, NPC, 65, 116, 27, -1);
end

if (EVENT == 116) then
	RunQuestExchange(UID,6070);
    SaveEvent(UID, 7790);
	SaveEvent(UID, 7792);
	SaveEvent(UID, 7791);
end

if (EVENT == 122) then 
	SelectMsg(UID, 4, 1275, 44125, NPC, 22, 123, 23, -1);
end

if (EVENT == 123) then
	QuestStatus = GetQuestStatus(UID, 1275)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7796);
	end
end

if (EVENT == 127) then
	QuestStatus = GetQuestStatus(UID, 1275)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MARCILTOKEN = HowmuchItem(UID, 900683000);
		if (MARCILTOKEN < 10) then
			SelectMsg(UID, 2, 1275, 44125, NPC, 19, -1);
		else
			SaveEvent(UID, 7798);
		end
	end
end

if (EVENT == 125) then
	QuestStatus = GetQuestStatus(UID, 1275)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MARCILTOKEN = HowmuchItem(UID, 900683000);
		if (MARCILTOKEN < 10) then
			SelectMsg(UID, 2, 1275, 44125, NPC, 19, -1);
		else
			SelectMsg(UID, 4, 1275, 44125, NPC, 22, 128, 23,-1);
		end
	end
end

if (EVENT == 128) then
	QuestStatus = GetQuestStatus(UID, 1275)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MARCILTOKEN = HowmuchItem(UID, 900683000);
		if (MARCILTOKEN < 10) then
			SelectMsg(UID, 2, 1275, 44125, NPC, 19, -1);
		else
			RunQuestExchange(UID,6071); 
			SaveEvent(UID, 7797);
		end
	end
end

if (EVENT == 132) then 
	SelectMsg(UID, 4, 1276, 44126, NPC, 22, 133, 23, -1);
end

if (EVENT == 133) then
	QuestStatus = GetQuestStatus(UID, 1276)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7802);
	end
end

if (EVENT == 137) then
	QuestStatus = GetQuestStatus(UID, 1276)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1276, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1276, 44126, NPC, 18, 134);
		else
			SaveEvent(UID, 7804);
		end
	end
end

if (EVENT == 135) then
	QuestStatus = GetQuestStatus(UID, 1276)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1276, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1276, 44126, NPC, 18, 134);
		else
			SelectMsg(UID, 4, 1276, 44126, NPC, 22, 136, 23, -1);
		end
	end
end

if (EVENT == 134) then
	ShowMap(UID, 1240);
end

if (EVENT == 136) then
	QuestStatus = GetQuestStatus(UID, 1276)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1276, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1276, 44126, NPC, 18, 134);
		else
			RunQuestExchange(UID,6072);
			SaveEvent(UID, 7803);
		end
	end
end

if (EVENT == 142) then 
	SelectMsg(UID, 4, 1277, 44127, NPC, 22, 143, 23, -1);
end

if (EVENT == 143) then
	QuestStatus = GetQuestStatus(UID, 1277)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7808);
	end
end

if (EVENT == 144) then
	QuestStatus = GetQuestStatus(UID, 1277)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	BADINTOKEN = HowmuchItem(UID, 900684000);
		if (BADINTOKEN < 10) then
			SelectMsg(UID, 2, 1277, 44127, NPC, 19, -1);
		else
			SaveEvent(UID, 7811);
		end
	end
end

if (EVENT == 145) then
	QuestStatus = GetQuestStatus(UID, 1277)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	BADINTOKEN = HowmuchItem(UID, 900684000);
		if (BADINTOKEN < 10) then
			SelectMsg(UID, 2, 1277, 44127, NPC, 19, -1);
		else
			SelectMsg(UID, 4, 1277, 44127, NPC, 22, 148, 23, -1);
		end
	end
end

if (EVENT == 147) then
	SaveEvent(UID, 7810);
end

if (EVENT == 148) then
	QuestStatus = GetQuestStatus(UID, 1277)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	BADINTOKEN = HowmuchItem(UID, 900684000);
		if (BADINTOKEN < 10) then
			SelectMsg(UID, 2, 1277, 44127, NPC, 19, -1);
		else
			RunQuestExchange(UID,6073);
			SaveEvent(UID, 7809);
		end
	end
end

if (EVENT == 152) then 
	SelectMsg(UID, 4, 1278, 44128, NPC, 22, 153, 23, -1);
end

if (EVENT == 153) then
	QuestStatus = GetQuestStatus(UID, 1278)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7814);
	end
end

if (EVENT == 157) then
	QuestStatus = GetQuestStatus(UID, 1278)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1278, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1278, 44128, NPC, 18, -1);
		else
			SaveEvent(UID, 7816);
		end
	end
end

if (EVENT == 155) then
	QuestStatus = GetQuestStatus(UID, 1278)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1278, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1278, 44128, NPC, 18, -1);
		else
			SelectMsg(UID, 4, 1278, 44128, NPC, 22, 158, 23, -1);
		end
	end
end

if (EVENT == 158) then
	QuestStatus = GetQuestStatus(UID, 1278)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1278, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1278, 44128, NPC, 18, -1);
		else
			RunQuestExchange(UID,6074); 
			SaveEvent(UID, 7815);
		end
	end
end

if (EVENT == 162) then 
	SelectMsg(UID, 4, 1279, 44129, NPC, 22, 163, 23, -1);
end

if (EVENT == 163) then
	QuestStatus = GetQuestStatus(UID, 1279)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7820);
	end
end

if (EVENT == 167) then
	QuestStatus = GetQuestStatus(UID, 1279)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	GARLONGTOKEN = HowmuchItem(UID, 900685000);
		if (GARLONGTOKEN < 10) then
			SelectMsg(UID, 2, 1279, 44129, NPC, 19, -1);
		else
			SaveEvent(UID, 7822);
		end
	end
end

if (EVENT == 165) then
	QuestStatus = GetQuestStatus(UID, 1279)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	GARLONGTOKEN = HowmuchItem(UID, 900685000);
		if (GARLONGTOKEN < 10) then
			SelectMsg(UID, 2, 1279, 44129, NPC, 19, -1);
		else
			SelectMsg(UID, 4, 1279, 44129, NPC, 22, 168, 23, -1);
		end
	end
end

if (EVENT == 168) then
	QuestStatus = GetQuestStatus(UID, 1279)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	GARLONGTOKEN = HowmuchItem(UID, 900685000);
		if (GARLONGTOKEN < 10) then
			SelectMsg(UID, 2, 1279, 44129, NPC, 19, -1);
		else
			RunQuestExchange(UID,6075);
			SaveEvent(UID, 7821);
		end
	end
end

if (EVENT == 172) then 
	SelectMsg(UID, 4, 1280, 44130, NPC, 22, 173, 23, -1);
end

if (EVENT == 173) then
	QuestStatus = GetQuestStatus(UID, 1280)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7826);
	end
end

if (EVENT == 177) then
	QuestStatus = GetQuestStatus(UID, 1280)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1280, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1280, 44130, NPC, 18, -1);
		else
			SaveEvent(UID, 7828);
		end
	end
end

if (EVENT == 175) then
	QuestStatus = GetQuestStatus(UID, 1280)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1280, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1280, 44130, NPC, 18, -1);
		else
			SelectMsg(UID, 4, 1280, 44130, NPC, 22, 178, 23, -1);
		end
	end
end

if (EVENT == 178) then
	QuestStatus = GetQuestStatus(UID, 1280)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1280, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, 1280, 44130, NPC, 18, -1);
		else
			RunQuestExchange(UID,6076);
			SaveEvent(UID, 7827);
		end
	end
end

if (EVENT == 182) then 
	SelectMsg(UID, 4, 1281, 44131, NPC, 22, 183, 23, -1);
end

if (EVENT == 183) then
	QuestStatus = GetQuestStatus(UID, 1281)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7832);
	end
end

if (EVENT == 187) then
	QuestStatus = GetQuestStatus(UID, 1281)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	Monster01 = CountMonsterQuestSub(UID, 1281, 1);
	Monster02 = CountMonsterQuestSub(UID, 1281, 2);
	Monster03 = CountMonsterQuestSub(UID, 1281, 3);
	    if (Monster01 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 188);
		elseif (Monster02 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 189);
		elseif (Monster03 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 190);
		else
			SaveEvent(UID, 7834);
		end
	end
end

if (EVENT == 185) then
	QuestStatus = GetQuestStatus(UID, 1281)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	Monster01 = CountMonsterQuestSub(UID, 1281, 1);
	Monster02 = CountMonsterQuestSub(UID, 1281, 2);
	Monster03 = CountMonsterQuestSub(UID, 1281, 3);
	    if (Monster01 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 188);
		elseif (Monster02 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 189);
		elseif (Monster03 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 190);
		else
			SelectMsg(UID, 4, 1281, 44131, NPC, 22,186,27, -1);
		end
	end
end

if (EVENT == 188) then
	ShowMap(UID, 737);
end

if (EVENT == 189) then
	ShowMap(UID, 966);
end

if (EVENT == 190) then
	ShowMap(UID, 739);
end

if (EVENT == 186) then
	QuestStatus = GetQuestStatus(UID, 1281)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	Monster01 = CountMonsterQuestSub(UID, 1281, 1);
	Monster02 = CountMonsterQuestSub(UID, 1281, 2);
	Monster03 = CountMonsterQuestSub(UID, 1281, 3);
	    if (Monster01 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 188);
		elseif (Monster02 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 189);
		elseif (Monster03 < 1) then
			SelectMsg(UID, 2, 1281, 44131, NPC, 18, 190);
		else
			RunQuestExchange(UID,6077);
			SaveEvent(UID, 7833);
		end
	end
end

if (EVENT == 192) then 
	SelectMsg(UID, 4, 1282, 44132, NPC, 22, 193, 23,-1);
end

if (EVENT == 193) then
	QuestStatus = GetQuestStatus(UID, 1282)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7838);
	end
end

if (EVENT == 197) then
	QuestStatus = GetQuestStatus(UID, 1282)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	ESC = HowmuchItem(UID, 900634000);
	JER = HowmuchItem(UID, 900635000);
	ESP = HowmuchItem(UID, 900633000);
		if (ESC < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		elseif (JER < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, 200);
		elseif (ESP < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		else
			SaveEvent(UID, 7840);
		end
	end
end

if (EVENT == 195) then
	QuestStatus = GetQuestStatus(UID, 1282)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	ESC = HowmuchItem(UID, 900634000);
	JER = HowmuchItem(UID, 900635000);
	ESP = HowmuchItem(UID, 900633000);
		if (ESC < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		elseif (JER < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, 200);
		elseif (ESP < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		else
			SelectMsg(UID, 4, 1282, 44132, NPC, 22, 198, 23, -1);
		end
	end
end

if (EVENT == 198) then
	QuestStatus = GetQuestStatus(UID, 1282)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	ESC = HowmuchItem(UID, 900634000);
	JER = HowmuchItem(UID, 900635000);
	ESP = HowmuchItem(UID, 900633000);
		if (ESC < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		elseif (JER < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, 200);
		elseif (ESP < 1) then
			SelectMsg(UID, 2, 1282, 44132, NPC, 18, -1);
		else
			RunQuestExchange(UID,6078);
			SaveEvent(UID, 7839);
		end
	end
end

if (EVENT == 202) then 
	SelectMsg(UID, 4, 1283, 44133, NPC, 22, 203, 23, -1);
end

if (EVENT == 203) then
	QuestStatus = GetQuestStatus(UID, 1283)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
			SaveEvent(UID, 7844);
	end
end

if (EVENT == 207) then
	QuestStatus = GetQuestStatus(UID, 1283)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1283, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1283, 44133, NPC, 18, 206);
		else
			SaveEvent(UID, 7846);
		end
	end
end

if (EVENT == 205) then
	QuestStatus = GetQuestStatus(UID, 1283)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1283, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1283, 44133, NPC, 18, 206);
		else
			SelectMsg(UID, 4, 1283, 44133, NPC, 22, 208, 23, -1);
		end
	end
end

if (EVENT == 206) then
	ShowMap(UID, 1243);
end

if (EVENT == 208) then
	QuestStatus = GetQuestStatus(UID, 1283)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 43943, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1283, 1);
		if (MonsterCount < 1) then
			SelectMsg(UID, 2, 1283, 44133, NPC, 18, 206);
		else
			RunQuestExchange(UID,6079);
			SaveEvent(UID, 7845);
		end
	end
end

if (EVENT == 212) then 
	SelectMsg(UID, 4, 1284, 44134, NPC, 22, 213, 23, -1);
end

if (EVENT == 213) then
RunQuestExchange(UID,6080);
SaveEvent(UID, 7850);
SaveEvent(UID, 7852);
SaveEvent(UID, 7851);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1274 status=255 n_index=7788
if (EVENT == 110) then
	SaveEvent(UID, 7789);
end

-- [AUTO-GEN] quest=1274 status=1 n_index=7790
if (EVENT == 117) then
	QuestStatusCheck = GetQuestStatus(UID, 1274)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6070);
		SaveEvent(UID, 7791);
	end
end

-- [AUTO-GEN] quest=1275 status=255 n_index=7794
if (EVENT == 120) then
	SaveEvent(UID, 7795);
end

-- [AUTO-GEN] quest=1276 status=255 n_index=7800
if (EVENT == 130) then
	SaveEvent(UID, 7801);
end

-- [AUTO-GEN] quest=1277 status=255 n_index=7806
if (EVENT == 140) then
	SaveEvent(UID, 7807);
end

-- [AUTO-GEN] quest=1278 status=255 n_index=7812
if (EVENT == 150) then
	SaveEvent(UID, 7813);
end

-- [AUTO-GEN] quest=1279 status=255 n_index=7818
if (EVENT == 160) then
	SaveEvent(UID, 7819);
end

-- [AUTO-GEN] quest=1280 status=255 n_index=7824
if (EVENT == 170) then
	SaveEvent(UID, 7825);
end

-- [AUTO-GEN] quest=1281 status=255 n_index=7830
if (EVENT == 180) then
	SaveEvent(UID, 7831);
end

-- [AUTO-GEN] quest=1283 status=255 n_index=7842
if (EVENT == 200) then
	SaveEvent(UID, 7843);
end

-- [AUTO-GEN] quest=1284 status=255 n_index=7848
if (EVENT == 210) then
	SaveEvent(UID, 7849);
end

