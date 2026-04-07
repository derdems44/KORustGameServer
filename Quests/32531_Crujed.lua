local NPC = 32531;

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

if (EVENT == 9402) then
	SelectMsg(UID, 4, 318, 209, NPC, 22, 9403, 23, -1);
end

if (EVENT == 9403) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 9436);
	end
end

if (EVENT == 9405) then
	QuestStatus = GetQuestStatus(UID, 318)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount1 = CountMonsterQuestSub(UID, 318, 1);
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 2);
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1010);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1001);
		else
			SaveEvent(UID, 9438);
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
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1010);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1001);
		else
			SelectMsg(UID, 4, savenum, 8693, NPC, 41, 9409, 23, -1);
		end
	end
end

if (EVENT == 9408) then
	ShowMap(UID, 614);
end

if (EVENT == 1010) then
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
	MonsterCount2 = CountMonsterQuestSub(UID, 318, 3);
		if (MonsterCount1 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 9408);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1010);
		elseif (MonsterCount2 < 1) then
			SelectMsg(UID, 2, savenum, 8693, NPC, 18, 1001);
		else
			RunQuestExchange(UID, 1101);
			SaveEvent(UID, 9437);
		end
	end
end

if (EVENT == 1000) then
	SaveEvent(UID, 11780);
end

if (EVENT == 1002) then
	SelectMsg(UID, 4, 576, 20722, NPC, 22, 1003, 23, -1);
end

if (EVENT == 1003) then
	SaveEvent(UID, 11782);
	SaveEvent(UID, 11784);
end

if (EVENT == 1005) then
	SaveEvent(UID, 11783);
end

if (EVENT == 1102) then
	SelectMsg(UID, 4, 577, 20724, NPC, 22, 1103, 23, -1);
end

if (EVENT == 1103) then 
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 11794);
	end
end

if (EVENT == 1106) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20724, NPC, 18, 1108);
		else
			SaveEvent(UID, 11796);
		end
	end
end

if (EVENT == 1105) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20724, NPC, 18, 1108);
		else
			SelectMsg(UID, 4, 577, 20724, NPC, 41, 1107, 27, -1);
		end
	end
end

if (EVENT == 1108) then
	ShowMap(UID, 755);
end

if (EVENT == 1107) then
	QuestStatus = GetQuestStatus(UID, 577)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 577, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, -1, 20724, NPC, 18, 1108);
		else
			RunQuestExchange(UID,3067);
			SaveEvent(UID, 11795);
		end
	end
end

if (EVENT == 1202) then
	SelectMsg(UID, 4, 578, 20726, NPC, 22, 1203, 23, -1);
end

if (EVENT == 1203) then
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
			SaveEvent(UID, 11806);
	end
end

if (EVENT == 1204) then
	QuestStatus = GetQuestStatus(UID, 578)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
	MonsterCount = CountMonsterQuestSub(UID, 578, 1);
		if (MonsterCount < 40) then
			SelectMsg(UID, 2, -1, 20726, NPC, 18, 1206);
		else		
			SaveEvent(UID, 11808);
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
			SelectMsg(UID, 2, -1, 20726, NPC, 18, 1206);
		else
			SelectMsg(UID, 4, 578, 20726, NPC, 41, 1207, 27, -1);
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
			SelectMsg(UID, 2, -1, 20726, NPC, 18, 1206);
		else
			RunQuestExchange(UID,3068);
			SaveEvent(UID, 11807);
		end
	end
end

if (EVENT == 1302) then
	SelectMsg(UID, 4, 579, 20728, NPC, 22, 1303, 23, -1);
end

if (EVENT == 1303) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else  
			SaveEvent(UID, 11818);
	end
end

if (EVENT == 1304) then
	QuestStatus = GetQuestStatus(UID, 579)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 579, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, -1, 20728, NPC, 18, 1306);
		else		
			SaveEvent(UID, 11820);
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
			SelectMsg(UID, 2, -1, 20728, NPC, 18, 1306);
		else
			SelectMsg(UID, 4, 579, 20728, NPC, 41, 1307, 27, -1);
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
			SelectMsg(UID, 2, -1, 20728, NPC, 18, 1306);
		else
			RunQuestExchange(UID,3069);
			SaveEvent(UID, 11819);
		end
	end
end

if (EVENT == 111) then
	SelectMsg(UID, 4, 274, 8986, NPC, 22, 112, 23, -1);
end

if (EVENT == 112) then
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else 
			SaveEvent(UID, 1687);
	end
end

if (EVENT == 113) then 
	QuestStatus = GetQuestStatus(UID, 274)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 274, 1);
		if (MonsterCount < 20) then
			SelectMsg(UID, 2, -1, 8986, NPC, 18, 115);
		else
			SaveEvent(UID, 1689);
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
			SelectMsg(UID, 2, -1, 8986, NPC, 18, 115);
		else
			SelectMsg(UID, 4, 274, 8986, NPC, 22, 117, 23, -1);
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
			SelectMsg(UID, 2, -1, 8986, NPC, 18, 115);
		else
			RunQuestExchange(UID,3);
			SaveEvent(UID, 1688);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=190 status=2 n_index=1282
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 190)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 191);
		SaveEvent(UID, 1284);
	end
end

-- [AUTO-GEN] quest=274 status=255 n_index=1685
if (EVENT == 110) then
	SaveEvent(UID, 1686);
end

-- [AUTO-GEN] quest=318 status=2 n_index=9437
if (EVENT == 190) then
	SearchQuest(UID, 32531);
end

-- [AUTO-GEN] quest=190 status=0 n_index=1280
if (EVENT == 200) then
	SelectMsg(UID, 4, 190, 8879, NPC, 171, 201, 23, -1);
end

-- [AUTO-GEN] quest=190 status=0 n_index=1280
if (EVENT == 201) then
	SaveEvent(UID, 1281);
end

-- [AUTO-GEN] quest=190 status=1 n_index=1281
if (EVENT == 220) then
	QuestStatusCheck = GetQuestStatus(UID, 190)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 191);
		SaveEvent(UID, 1282);
	end
end

-- [AUTO-GEN] quest=577 status=255 n_index=11792
if (EVENT == 1100) then
	SaveEvent(UID, 11793);
end

-- [AUTO-GEN] quest=578 status=255 n_index=11804
if (EVENT == 1200) then
	SaveEvent(UID, 11805);
end

-- [AUTO-GEN] quest=579 status=255 n_index=11816
if (EVENT == 1300) then
	SaveEvent(UID, 11817);
end

-- [AUTO-GEN] quest=588 status=255 n_index=11924
if (EVENT == 1400) then
	SaveEvent(UID, 11925);
end

-- [AUTO-GEN] quest=588 status=0 n_index=11925
if (EVENT == 1402) then
	SelectMsg(UID, 4, 588, 20746, NPC, 3144, 1403, 23, -1);
end

-- [AUTO-GEN] quest=588 status=1 n_index=11926
if (EVENT == 1403) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 588, 20746, NPC, 18, 1405);
	else
		SelectMsg(UID, 4, 588, 20746, NPC, 41, 1404, 27, -1);
	end
end

-- [AUTO-GEN] quest=588 status=1 n_index=11926
if (EVENT == 1404) then
	QuestStatusCheck = GetQuestStatus(UID, 588)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3078);
		SaveEvent(UID, 11927);
	end
end

-- [AUTO-GEN] quest=588 status=3 n_index=11928
if (EVENT == 1405) then
	SelectMsg(UID, 2, 588, 20746, NPC, 10, -1);
end

-- [AUTO-GEN] quest=589 status=255 n_index=11936
if (EVENT == 1500) then
	SaveEvent(UID, 11937);
end

-- [AUTO-GEN] quest=589 status=0 n_index=11937
if (EVENT == 1502) then
	SelectMsg(UID, 4, 589, 20748, NPC, 3146, 1503, 23, -1);
end

-- [AUTO-GEN] quest=589 status=0 n_index=11937
if (EVENT == 1503) then
	SaveEvent(UID, 11938);
end

-- [AUTO-GEN] quest=589 status=1 n_index=11938
if (EVENT == 1505) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 589, 20748, NPC, 22, 1506, 23, -1);
	else
		SelectMsg(UID, 2, 589, 20748, NPC, 18, 1506);
	end
end

-- [AUTO-GEN] quest=589 status=1 n_index=11938
if (EVENT == 1506) then
	QuestStatusCheck = GetQuestStatus(UID, 589)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3079);
		SaveEvent(UID, 11939);
	end
end

-- [AUTO-GEN] quest=590 status=255 n_index=11948
if (EVENT == 1600) then
	SaveEvent(UID, 11949);
end

-- [AUTO-GEN] quest=590 status=0 n_index=11949
if (EVENT == 1602) then
	SelectMsg(UID, 4, 590, 20750, NPC, 3148, 1603, 23, -1);
end

-- [AUTO-GEN] quest=590 status=0 n_index=11949
if (EVENT == 1603) then
	SaveEvent(UID, 11950);
end

-- [AUTO-GEN] quest=590 status=1 n_index=11950
if (EVENT == 1605) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 590, 20750, NPC, 22, 1606, 23, -1);
	else
		SelectMsg(UID, 2, 590, 20750, NPC, 18, 1606);
	end
end

-- [AUTO-GEN] quest=590 status=1 n_index=11950
if (EVENT == 1606) then
	QuestStatusCheck = GetQuestStatus(UID, 590)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3080);
		SaveEvent(UID, 11951);
	end
end

-- [AUTO-GEN] quest=591 status=255 n_index=11960
if (EVENT == 1700) then
	SaveEvent(UID, 11961);
end

-- [AUTO-GEN] quest=591 status=0 n_index=11961
if (EVENT == 1702) then
	SelectMsg(UID, 4, 591, 20752, NPC, 3150, 1703, 23, -1);
end

-- [AUTO-GEN] quest=591 status=0 n_index=11961
if (EVENT == 1703) then
	SaveEvent(UID, 11962);
end

-- [AUTO-GEN] quest=591 status=1 n_index=11962
if (EVENT == 1705) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 591, 20752, NPC, 22, 1706, 23, -1);
	else
		SelectMsg(UID, 2, 591, 20752, NPC, 18, 1706);
	end
end

-- [AUTO-GEN] quest=591 status=1 n_index=11962
if (EVENT == 1706) then
	QuestStatusCheck = GetQuestStatus(UID, 591)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3081);
		SaveEvent(UID, 11963);
	end
end

-- [AUTO-GEN] quest=593 status=255 n_index=11972
if (EVENT == 1800) then
	SaveEvent(UID, 11973);
end

-- [AUTO-GEN] quest=593 status=0 n_index=11973
if (EVENT == 1802) then
	SelectMsg(UID, 4, 593, 20754, NPC, 3152, 1803, 23, -1);
end

-- [AUTO-GEN] quest=593 status=0 n_index=11973
if (EVENT == 1803) then
	SaveEvent(UID, 11974);
end

-- [AUTO-GEN] quest=593 status=1 n_index=11974
if (EVENT == 1805) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 593, 20754, NPC, 22, 1806, 23, -1);
	else
		SelectMsg(UID, 2, 593, 20754, NPC, 18, 1806);
	end
end

-- [AUTO-GEN] quest=593 status=1 n_index=11974
if (EVENT == 1806) then
	QuestStatusCheck = GetQuestStatus(UID, 593)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3082);
		SaveEvent(UID, 11975);
	end
end

-- [AUTO-GEN] quest=595 status=255 n_index=11996
if (EVENT == 1900) then
	SaveEvent(UID, 11997);
end

-- [AUTO-GEN] quest=595 status=0 n_index=11997
if (EVENT == 1902) then
	SelectMsg(UID, 4, 595, 20758, NPC, 3156, 1903, 23, -1);
end

-- [AUTO-GEN] quest=595 status=0 n_index=11997
if (EVENT == 1903) then
	SaveEvent(UID, 11998);
end

-- [AUTO-GEN] quest=595 status=1 n_index=11998
if (EVENT == 1905) then
	ItemA = HowmuchItem(UID, 910238000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 595, 20758, NPC, 18, 1906);
	else
		SelectMsg(UID, 4, 595, 20758, NPC, 41, 1906, 27, -1);
	end
end

-- [AUTO-GEN] quest=595 status=1 n_index=11998
if (EVENT == 1906) then
	QuestStatusCheck = GetQuestStatus(UID, 595)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3084);
		SaveEvent(UID, 11999);
	end
end

-- [AUTO-GEN] quest=599 status=255 n_index=12044
if (EVENT == 2000) then
	SaveEvent(UID, 12045);
end

-- [AUTO-GEN] quest=599 status=0 n_index=12045
if (EVENT == 2002) then
	SelectMsg(UID, 4, 599, 20766, NPC, 3164, 2003, 23, -1);
end

-- [AUTO-GEN] quest=599 status=1 n_index=12046
if (EVENT == 2003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 599, 20766, NPC, 18, 2005);
	else
		SelectMsg(UID, 4, 599, 20766, NPC, 41, 2004, 27, -1);
	end
end

-- [AUTO-GEN] quest=599 status=1 n_index=12046
if (EVENT == 2004) then
	QuestStatusCheck = GetQuestStatus(UID, 599)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3088);
		SaveEvent(UID, 12047);
	end
end

-- [AUTO-GEN] quest=599 status=3 n_index=12048
if (EVENT == 2005) then
	SelectMsg(UID, 2, 599, 20766, NPC, 10, -1);
end

-- [AUTO-GEN] quest=600 status=255 n_index=12056
if (EVENT == 2100) then
	SaveEvent(UID, 12057);
end

-- [AUTO-GEN] quest=600 status=0 n_index=12057
if (EVENT == 2102) then
	SelectMsg(UID, 4, 600, 20768, NPC, 3166, 2103, 23, -1);
end

-- [AUTO-GEN] quest=600 status=0 n_index=12057
if (EVENT == 2103) then
	SaveEvent(UID, 12058);
end

-- [AUTO-GEN] quest=600 status=1 n_index=12058
if (EVENT == 2105) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 600, 20768, NPC, 22, 2106, 23, -1);
	else
		SelectMsg(UID, 2, 600, 20768, NPC, 18, 2106);
	end
end

-- [AUTO-GEN] quest=600 status=1 n_index=12058
if (EVENT == 2106) then
	QuestStatusCheck = GetQuestStatus(UID, 600)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3089);
		SaveEvent(UID, 12059);
	end
end

-- [AUTO-GEN] quest=318 status=255 n_index=9434
if (EVENT == 9400) then
	SaveEvent(UID, 9435);
end

