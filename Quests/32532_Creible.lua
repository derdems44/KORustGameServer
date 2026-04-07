local NPC = 32532;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8996, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8996, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 9402) then
	SelectMsg(UID, 4, 318, 996, NPC, 22, 9403, 23, -1);
end

if (EVENT == 9403) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 9442);
	end
end

if (EVENT == 9404) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount1 = CountMonsterQuestSub(UID, 318, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 2);
	MonsterCount3 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1000);
		elseif (MonsterCount3 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1001);
		else
			SaveEvent(UID, 9445);
		end
	end
end

if (EVENT == 9407) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount1 = CountMonsterQuestSub(UID, 318, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 2);
	MonsterCount3 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1000);
		elseif (MonsterCount3 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1001);
		else
			SelectMsg(UID, 4, 318, 996, NPC, 41, 9409, 23, -1);
		end
	end
end

if (EVENT == 9408) then
	ShowMap(UID, 614);
end

if (EVENT == 1000) then
	ShowMap(UID, 613);
end

if (EVENT == 1001) then
	ShowMap(UID, 612);
end

if (EVENT == 9409) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount1 = CountMonsterQuestSub(UID, 318, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 2);
	MonsterCount3 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1000);
		elseif (MonsterCount3 < 1) then
			SelectMsg(UID, 2, -1, 996, NPC, 18, 1001);
		else
			RunQuestExchange(UID, 1101);
			SaveEvent(UID, 9443);
		end
	end
end

if (EVENT == 1002) then
	SelectMsg(UID, 4, 576, 20723, NPC, 22, 1003, 23, -1);
end

if (EVENT == 1003) then
	SaveEvent(UID, 11788);
	SaveEvent(UID, 11790);
end

if (EVENT == 1005) then
	SaveEvent(UID, 11789);
end

if (EVENT == 1102) then
	SelectMsg(UID, 4, 577, 20725, NPC, 22, 1103, 23, -1);
end

if (EVENT == 1103) then 
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 11800);
		end
end

if (EVENT == 1106) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20725, NPC, 18, 1108);
		else
			SaveEvent(UID, 11802);
		end
	end
end

if (EVENT == 1108) then
	ShowMap(UID, 755);
end

if (EVENT == 1105) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20725, NPC, 18, 1108);
		else
			SelectMsg(UID, 4, 577, 20725, NPC, 41, 1107, 27, -1);
		end
	end
end

if (EVENT == 1107) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20725, NPC, 18, 1108);
		else
			RunQuestExchange(UID,3067);
			SaveEvent(UID, 11801);
		end
	end
end

if (EVENT == 1202) then
	SelectMsg(UID, 4, 578, 20727, NPC, 22, 1203, 23, -1);
end

if (EVENT == 1203) then
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
			SaveEvent(UID, 11812);
	end
end

if (EVENT == 1204) then 
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 578, 1);
		if (MonsterCount < 40) then
			SelectMsg(UID, 2, -1, 20727, NPC, 18, 1206);
		else
			SaveEvent(UID, 11814);
		end
	end
end

if (EVENT == 1205) then
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 578, 1);
		if (MonsterCount < 40) then
			SelectMsg(UID, 2, -1, 20727, NPC, 18, 1206);
		else
			SelectMsg(UID, 4, 578, 20727, NPC, 41, 1207, 27, -1);
		end
	end
end

if (EVENT == 1206) then
	ShowMap(UID, 756);
end

if (EVENT == 1207) then
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 578, 1);
		if (MonsterCount < 40) then
			SelectMsg(UID, 2, -1, 20727, NPC, 18, 1206);
		else
			RunQuestExchange(UID,3068);
			SaveEvent(UID, 11813);
		end
	end
end

if (EVENT == 1302) then
	SelectMsg(UID, 4, 579, 20729, NPC, 22, 1303, 23, -1);
end

if (EVENT == 1303) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else  
	SaveEvent(UID, 11824);
	end
end

if (EVENT == 1304) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 579, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, -1, 20729, NPC, 18, 1306);
		else		
			SaveEvent(UID, 11826);
		end
	end
end

if (EVENT == 1305) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 579, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, -1, 20729, NPC, 18, 1306);
		else
			SelectMsg(UID, 4, 579, 20729, NPC, 41, 1307, 27, -1);
		end
	end
end

if (EVENT == 1306) then
	ShowMap(UID, 757);
end

if (EVENT == 1307) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 579, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, -1, 20729, NPC, 18, 1306);
		else
			RunQuestExchange(UID,3069);
			SaveEvent(UID, 11825);
		end
	end
end

if (EVENT == 111) then
	SelectMsg(UID, 4, 274, 8996, NPC, 22, 112, 23, -1);
end

if (EVENT == 112) then
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else  
			SaveEvent(UID, 1693);
	end
end

if (EVENT == 113) then 
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 274, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, -1, 8996, NPC, 18, 115);
		else
			SaveEvent(UID, 1695);
		end
	end
end

if (EVENT == 116) then
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 274, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, -1, 8996, NPC, 18, 115);
		else
			SelectMsg(UID, 4, 274, 8996, NPC, 22, 117, 23, -1);
		end
	end
end

if (EVENT == 115) then
	ShowMap(UID, 757);
end

if (EVENT == 117) then
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 274, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, -1, 8996, NPC, 18, 115);
		else
			RunQuestExchange(UID,18);
			SaveEvent(UID, 1694);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=190 status=2 n_index=1287
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 190)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 192);
		SaveEvent(UID, 1289);
	end
end

-- [AUTO-GEN] quest=274 status=255 n_index=1691
if (EVENT == 110) then
	SaveEvent(UID, 1692);
end

-- [AUTO-GEN] quest=318 status=2 n_index=9443
if (EVENT == 190) then
	SearchQuest(UID, 32532);
end

-- [AUTO-GEN] quest=190 status=0 n_index=1285
if (EVENT == 200) then
	SelectMsg(UID, 4, 190, 8880, NPC, 172, 201, 23, -1);
end

-- [AUTO-GEN] quest=190 status=0 n_index=1285
if (EVENT == 201) then
	SaveEvent(UID, 1286);
end

-- [AUTO-GEN] quest=190 status=1 n_index=1286
if (EVENT == 220) then
	QuestStatusCheck = GetQuestStatus(UID, 190)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 192);
		SaveEvent(UID, 1287);
	end
end

-- [AUTO-GEN] quest=577 status=255 n_index=11798
if (EVENT == 1100) then
	SaveEvent(UID, 11799);
end

-- [AUTO-GEN] quest=578 status=255 n_index=11810
if (EVENT == 1200) then
	SaveEvent(UID, 11811);
end

-- [AUTO-GEN] quest=579 status=255 n_index=11822
if (EVENT == 1300) then
	SaveEvent(UID, 11823);
end

-- [AUTO-GEN] quest=588 status=255 n_index=11930
if (EVENT == 1400) then
	SaveEvent(UID, 11931);
end

-- [AUTO-GEN] quest=588 status=0 n_index=11931
if (EVENT == 1402) then
	SelectMsg(UID, 4, 588, 20747, NPC, 3145, 1403, 23, -1);
end

-- [AUTO-GEN] quest=588 status=1 n_index=11932
if (EVENT == 1403) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 588, 20747, NPC, 18, 1405);
	else
		SelectMsg(UID, 4, 588, 20747, NPC, 41, 1404, 27, -1);
	end
end

-- [AUTO-GEN] quest=588 status=1 n_index=11932
if (EVENT == 1404) then
	QuestStatusCheck = GetQuestStatus(UID, 588)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3078);
		SaveEvent(UID, 11933);
	end
end

-- [AUTO-GEN] quest=588 status=3 n_index=11934
if (EVENT == 1405) then
	SelectMsg(UID, 2, 588, 20747, NPC, 10, -1);
end

-- [AUTO-GEN] quest=589 status=255 n_index=11942
if (EVENT == 1500) then
	SaveEvent(UID, 11943);
end

-- [AUTO-GEN] quest=589 status=0 n_index=11943
if (EVENT == 1502) then
	SelectMsg(UID, 4, 589, 20749, NPC, 3147, 1503, 23, -1);
end

-- [AUTO-GEN] quest=589 status=0 n_index=11943
if (EVENT == 1503) then
	SaveEvent(UID, 11944);
end

-- [AUTO-GEN] quest=589 status=1 n_index=11944
if (EVENT == 1505) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 589, 20749, NPC, 22, 1506, 23, -1);
	else
		SelectMsg(UID, 2, 589, 20749, NPC, 18, 1506);
	end
end

-- [AUTO-GEN] quest=589 status=1 n_index=11944
if (EVENT == 1506) then
	QuestStatusCheck = GetQuestStatus(UID, 589)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3079);
		SaveEvent(UID, 11945);
	end
end

-- [AUTO-GEN] quest=590 status=255 n_index=11954
if (EVENT == 1600) then
	SaveEvent(UID, 11955);
end

-- [AUTO-GEN] quest=590 status=0 n_index=11955
if (EVENT == 1602) then
	SelectMsg(UID, 4, 590, 20751, NPC, 3149, 1603, 23, -1);
end

-- [AUTO-GEN] quest=590 status=0 n_index=11955
if (EVENT == 1603) then
	SaveEvent(UID, 11956);
end

-- [AUTO-GEN] quest=590 status=1 n_index=11956
if (EVENT == 1605) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 590, 20751, NPC, 22, 1606, 23, -1);
	else
		SelectMsg(UID, 2, 590, 20751, NPC, 18, 1606);
	end
end

-- [AUTO-GEN] quest=590 status=1 n_index=11956
if (EVENT == 1606) then
	QuestStatusCheck = GetQuestStatus(UID, 590)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3080);
		SaveEvent(UID, 11957);
	end
end

-- [AUTO-GEN] quest=592 status=255 n_index=11966
if (EVENT == 1700) then
	SaveEvent(UID, 11967);
end

-- [AUTO-GEN] quest=592 status=0 n_index=11967
if (EVENT == 1702) then
	SelectMsg(UID, 4, 592, 20753, NPC, 3151, 1703, 23, -1);
end

-- [AUTO-GEN] quest=592 status=0 n_index=11967
if (EVENT == 1703) then
	SaveEvent(UID, 11968);
end

-- [AUTO-GEN] quest=592 status=1 n_index=11968
if (EVENT == 1705) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 592, 20753, NPC, 22, 1706, 23, -1);
	else
		SelectMsg(UID, 2, 592, 20753, NPC, 18, 1706);
	end
end

-- [AUTO-GEN] quest=592 status=1 n_index=11968
if (EVENT == 1706) then
	QuestStatusCheck = GetQuestStatus(UID, 592)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 13081);
		SaveEvent(UID, 11969);
	end
end

-- [AUTO-GEN] quest=593 status=255 n_index=11978
if (EVENT == 1800) then
	SaveEvent(UID, 11979);
end

-- [AUTO-GEN] quest=593 status=0 n_index=11979
if (EVENT == 1802) then
	SelectMsg(UID, 4, 593, 20755, NPC, 3153, 1803, 23, -1);
end

-- [AUTO-GEN] quest=593 status=0 n_index=11979
if (EVENT == 1803) then
	SaveEvent(UID, 11980);
end

-- [AUTO-GEN] quest=593 status=1 n_index=11980
if (EVENT == 1805) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 593, 20755, NPC, 22, 1806, 23, -1);
	else
		SelectMsg(UID, 2, 593, 20755, NPC, 18, 1806);
	end
end

-- [AUTO-GEN] quest=593 status=1 n_index=11980
if (EVENT == 1806) then
	QuestStatusCheck = GetQuestStatus(UID, 593)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3082);
		SaveEvent(UID, 11981);
	end
end

-- [AUTO-GEN] quest=595 status=255 n_index=12002
if (EVENT == 1900) then
	SaveEvent(UID, 12003);
end

-- [AUTO-GEN] quest=595 status=0 n_index=12003
if (EVENT == 1902) then
	SelectMsg(UID, 4, 595, 20759, NPC, 3157, 1903, 23, -1);
end

-- [AUTO-GEN] quest=595 status=0 n_index=12003
if (EVENT == 1903) then
	SaveEvent(UID, 12004);
end

-- [AUTO-GEN] quest=595 status=1 n_index=12004
if (EVENT == 1905) then
	ItemA = HowmuchItem(UID, 910238000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 595, 20759, NPC, 18, 1906);
	else
		SelectMsg(UID, 4, 595, 20759, NPC, 41, 1906, 27, -1);
	end
end

-- [AUTO-GEN] quest=595 status=1 n_index=12004
if (EVENT == 1906) then
	QuestStatusCheck = GetQuestStatus(UID, 595)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3084);
		SaveEvent(UID, 12005);
	end
end

-- [AUTO-GEN] quest=599 status=255 n_index=12050
if (EVENT == 2000) then
	SaveEvent(UID, 12051);
end

-- [AUTO-GEN] quest=599 status=0 n_index=12051
if (EVENT == 2002) then
	SelectMsg(UID, 4, 599, 20767, NPC, 3165, 2003, 23, -1);
end

-- [AUTO-GEN] quest=599 status=1 n_index=12052
if (EVENT == 2003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 599, 20767, NPC, 18, 2005);
	else
		SelectMsg(UID, 4, 599, 20767, NPC, 41, 2004, 27, -1);
	end
end

-- [AUTO-GEN] quest=599 status=1 n_index=12052
if (EVENT == 2004) then
	QuestStatusCheck = GetQuestStatus(UID, 599)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3088);
		SaveEvent(UID, 12053);
	end
end

-- [AUTO-GEN] quest=599 status=3 n_index=12054
if (EVENT == 2005) then
	SelectMsg(UID, 2, 599, 20767, NPC, 10, -1);
end

-- [AUTO-GEN] quest=600 status=255 n_index=12062
if (EVENT == 2100) then
	SaveEvent(UID, 12063);
end

-- [AUTO-GEN] quest=600 status=0 n_index=12063
if (EVENT == 2102) then
	SelectMsg(UID, 4, 600, 20769, NPC, 3167, 2103, 23, -1);
end

-- [AUTO-GEN] quest=600 status=0 n_index=12063
if (EVENT == 2103) then
	SaveEvent(UID, 12064);
end

-- [AUTO-GEN] quest=600 status=1 n_index=12064
if (EVENT == 2105) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 600, 20769, NPC, 22, 2106, 23, -1);
	else
		SelectMsg(UID, 2, 600, 20769, NPC, 18, 2106);
	end
end

-- [AUTO-GEN] quest=600 status=1 n_index=12064
if (EVENT == 2106) then
	QuestStatusCheck = GetQuestStatus(UID, 600)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3089);
		SaveEvent(UID, 12065);
	end
end

-- [AUTO-GEN] quest=318 status=255 n_index=9440
if (EVENT == 9400) then
	SaveEvent(UID, 9441);
end

-- [AUTO-GEN] quest=318 status=1 n_index=9442
if (EVENT == 9405) then
	QuestStatusCheck = GetQuestStatus(UID, 318)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1101);
		SaveEvent(UID, 9443);
	end
end

