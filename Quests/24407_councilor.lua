local NPC= 24407;

if (EVENT == 100) then
	QuestStatusCheck = GetQuestStatus(UID, 789)	
	if(QuestStatusCheck == 3) then
		SelectMsg(UID, 2, 788, 23165, NPC, 10, 1908,4005,-1);
		else
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then 
		SelectMsg(UID, 2, -1, 4178, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then 
		NpcMsg(UID, 4174, NPC)
	else
		EVENT = QuestNum
	end
end
end

if(EVENT == 1501) then
	SelectMsg(UID, 4, 778, 23043, NPC, 3000, 1502,3005,-1);
end

if(EVENT == 1502) then
	SaveEvent(UID, 13687);
end

if(EVENT == 1503) then
	SelectMsg(UID, 4, 778, 23043, NPC, 10, 1504,4005,-1);
end

if(EVENT == 1504) then
	SaveEvent(UID, 13689);
	SaveEvent(UID, 13688);
	SaveEvent(UID, 13699);
end

if(EVENT == 1601) then
	SelectMsg(UID, 4, 779, 23044, NPC, 3000, 1602,3005,-1);
end

if(EVENT == 1602) then
	QuestStatus = GetQuestStatus(UID, 779)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13699);
	end
end

if(EVENT == 1606) then
	QuestStatus = GetQuestStatus(UID, 779)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 508214000)	
		if(COUNTA < 4) then
			SelectMsg(UID, 2, 779, 23044, NPC, 18, 1604);
		else
			SaveEvent(UID, 13701);
		end
	end
end

if(EVENT == 1605 ) then
	QuestStatus = GetQuestStatus(UID, 779)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 508214000)	
		if(COUNTA < 4) then
			SelectMsg(UID, 2, 779, 23044, NPC, 18, 1604);
		else
			SelectMsg(UID, 4, 779, 23044, NPC, 41, 1603, 27, -1);
		end
	end
end

if (EVENT == 1604) then
	ShowMap(UID, 514);
end

if(EVENT == 1603 ) then
	QuestStatus = GetQuestStatus(UID, 779)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 508214000)	
		if(COUNTA < 4) then
			SelectMsg(UID, 2, 779, 23044, NPC, 18, 1604);
		else
			RunQuestExchange(UID, 3227);
			SaveEvent(UID, 13700);
			SaveEvent(UID, 13711);
		end
	end
end

if (EVENT == 1301) then
	SelectMsg(UID, 4, 630, 21240, NPC, 22, 1302, 27, -1);
end

if (EVENT == 1302) then
	QuestStatus = GetQuestStatus(UID, 630)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 12360);
	end
end

if (EVENT == 1306) then
	QuestStatus = GetQuestStatus(UID, 630)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900197000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 630, 21240, NPC, 10, -1);
		else
			SaveEvent(UID, 12362);
		end
	end
end

if(EVENT == 1305 ) then
	QuestStatus = GetQuestStatus(UID, 630)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900197000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 630, 21240, NPC, 10, -1);
		else
			SelectMsg(UID, 4, 630, 21240, NPC, 41, 1307, 27, -1);
		end
	end
end

if(EVENT == 1307 ) then
	QuestStatus = GetQuestStatus(UID, 630)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900197000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 630, 21240, NPC, 10, -1);
		else
			RunQuestExchange(UID, 3115);
			SaveEvent(UID, 12361);
		end
	end
end

if (EVENT == 1401) then
	SelectMsg(UID, 4, 641, 21262, NPC, 22, 1402, 27, -1);
end

if (EVENT == 1402) then
	QuestStatus = GetQuestStatus(UID, 641)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 12492);
	end
end

if (EVENT == 1406) then
	QuestStatus = GetQuestStatus(UID, 641)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900192000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 641, 21262, NPC, 10, -1);
		else
			SaveEvent(UID, 12494);
		end
	end
end

if(EVENT == 1405 ) then
	QuestStatus = GetQuestStatus(UID, 641)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900192000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 641, 21262, NPC, 10, -1);
		else
			SelectMsg(UID, 4, 641, 21262, NPC, 41, 1407, 27, -1);
		end
	end
end

if(EVENT == 1407 ) then
	QuestStatus = GetQuestStatus(UID, 641)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	COUNTA = HowmuchItem(UID, 900192000)	
		if(COUNTA < 1) then
			SelectMsg(UID, 2, 641, 21262, NPC, 10, -1);
		else
			RunQuestExchange(UID, 3126);
			SaveEvent(UID, 12493);
		end
	end
end

if (EVENT == 1701) then
	SelectMsg(UID, 4, 782, 23012, NPC, 22, 1702, 27, -1);
end

if (EVENT == 1702) then
	QuestStatus = GetQuestStatus(UID, 782)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13735);
	end
end

if (EVENT == 1706) then
	QuestStatus = GetQuestStatus(UID, 782)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 508215000)	
		if(ITEMA < 4) then
			SelectMsg(UID, 2, 782, 23012, NPC, 18, 1707);
		else
			SaveEvent(UID, 13737);
		end
	end
end

if(EVENT == 1705) then
	QuestStatus = GetQuestStatus(UID, 782)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 508215000)	
		if(ITEMA < 4) then
			SelectMsg(UID, 2, 782, 23012, NPC, 18, 1707);
		else
			SelectMsg(UID, 4, 782, 23012, NPC, 22, 1708,23,-1);
		end
	end
end

if (EVENT == 1707 ) then
	ShowMap(UID, 58)
end

if(EVENT == 1708 ) then
	QuestStatus = GetQuestStatus(UID, 782)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 508215000)	
		if(ITEMA < 4) then
			SelectMsg(UID, 2, 782, 23012, NPC, 18, 1707);
		else
			RunQuestExchange(UID, 3230);
			SaveEvent(UID, 13736);
			SaveEvent(UID, 13747);
		end
	end
end

if (EVENT == 1801) then
    SelectMsg(UID, 2, 785, 20772, NPC, 22, 1802);
end

if (EVENT == 1802) then
	QuestStatus = GetQuestStatus(UID, 785)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13771);
	end
end

if (EVENT == 1803) then
	SelectMsg(UID, 4, 785, 20772, NPC, 22, 1804, 27, -1);
	SaveEvent(UID, 13773);
end

if (EVENT == 1804) then
	QuestStatus = GetQuestStatus(UID, 785)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SelectMsg(UID, 2, 785, 23127, NPC, 10, -1);
			RunQuestExchange(UID, 3233);
			SaveEvent(UID, 13772);
			SaveEvent(UID, 13783);
	end
end

if (EVENT == 1901) then
	SelectMsg(UID, 4, 788, 209, NPC, 22, 1902, 27, -1);
end

if (EVENT == 1902) then
	QuestStatus = GetQuestStatus(UID, 788)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13807);
	end
end

if (EVENT == 1906) then
	QuestStatus = GetQuestStatus(UID, 788)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900326000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 788, 209, NPC, 18, -1);
		else
			SaveEvent(UID, 13809);
		end
	end
end

if(EVENT == 1905) then
	QuestStatus = GetQuestStatus(UID, 788)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900326000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 788, 209, NPC, 18, -1);
		else
			SelectMsg(UID, 4, 788, 209, NPC, 22, 1907,23,-1);
		end
	end
end

if (EVENT == 1907) then
	QuestStatus = GetQuestStatus(UID, 788)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900326000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 788, 209, NPC, 18, -1);
		else
			SelectMsg(UID, 2, 788, 23165, NPC, 10, 1908,4005,-1);
			RunQuestExchange(UID, 3236);
			SaveEvent(UID, 13808);
			SaveEvent(UID, 13820);
		end
	end
end

if (EVENT == 1908)then
MonsterStoneQuestJoin(UID,790);
end

if (EVENT == 2001) then
	SelectMsg(UID, 4, 792, 23008, NPC, 22, 2002, 27, -1);
end

if (EVENT == 2002) then
	QuestStatus = GetQuestStatus(UID, 792)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13845);
	end
end

if (EVENT == 2006) then
	QuestStatus = GetQuestStatus(UID, 792)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900330000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 792, 209, NPC, 18, -1);
		else
			SaveEvent(UID, 13847);
		end
	end
end

if(EVENT == 2005) then
	QuestStatus = GetQuestStatus(UID, 792)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900330000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 792, 209, NPC, 18, -1);
		else
			SelectMsg(UID, 4, 792, 209, NPC, 22, 2007,23,-1);
		end
	end
end

if (EVENT == 2007) then
	QuestStatus = GetQuestStatus(UID, 792)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900330000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 792, 209, NPC, 18, -1);
		else
			SelectMsg(UID, 2, 792, 23196, NPC, 10, -1);
			if CheckGiveSlot(UID, 1) then
				RunQuestExchange(UID, 3240);
				GiveItem(UID, 900335523);
				SaveEvent(UID, 13846);
				SaveEvent(UID, 13857);
			end
		end
	end
end

if (EVENT == 2101) then
	SelectMsg(UID, 4, 793, 23012, NPC, 22, 2102, 27, -1);
end

if (EVENT == 2102) then
	QuestStatus = GetQuestStatus(UID, 793)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13857);
	end
end

if (EVENT == 2106) then
	QuestStatus = GetQuestStatus(UID, 793)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 793, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 793, 23012, NPC, 18, 2107);
		else
			SaveEvent(UID, 13859);
		end
	end
end

if (EVENT == 2105) then
	QuestStatus = GetQuestStatus(UID, 793)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 793, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 793, 23012, NPC, 18, 2107);
		else
			SelectMsg(UID, 4, 793, 23012, NPC, 41, 2108, 27, -1);
		end
	end
end

if (EVENT == 2107 ) then
	ShowMap(UID, 344)
end

if (EVENT == 2108) then
	QuestStatus = GetQuestStatus(UID, 793)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 793, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 793, 23012, NPC, 18, 2107);
		else
			SelectMsg(UID, 2, 793, 23226, NPC, 10, -1);
			if CheckGiveSlot(UID, 1) then
				RunQuestExchange(UID, 3241);
				GiveItem(UID, 900336524);
				RobItem(UID, 900335523);
				SaveEvent(UID, 13858);
				SaveEvent(UID, 13869);
			end
		end
	end
end

if (EVENT == 2201) then
	SelectMsg(UID, 4, 795, 23012, NPC, 22, 2202, 27, -1);
end

if (EVENT == 2202) then
	QuestStatus = GetQuestStatus(UID, 795)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13869);
	end
end

if (EVENT == 2206) then
	QuestStatus = GetQuestStatus(UID, 795)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13871);
	end
end

if (EVENT == 2205) then
	QuestStatus = GetQuestStatus(UID, 795)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 795, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 795, 23012, NPC, 18, 2207);
		else
			SelectMsg(UID, 4, 795, 23012, NPC, 41, 2208, 27, -1);
		end
	end
end

if (EVENT == 2207 ) then
	ShowMap(UID, 111)
end

if (EVENT == 2208) then
	QuestStatus = GetQuestStatus(UID, 795)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 795, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 795, 23012, NPC, 18, 2207);
		else
			SelectMsg(UID, 2, 795, 23012, NPC, 10, -1);
			RunQuestExchange(UID, 3242);
			SaveEvent(UID, 13870);
			SaveEvent(UID, 13881);
			RobItem(UID, 900336524);
		end
	end
end

if (EVENT == 2301) then
	SelectMsg(UID, 4, 798, 23016, NPC, 22, 2302, 27, -1);
end

if (EVENT == 2302) then
	QuestStatus = GetQuestStatus(UID, 798)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13893);
	end
end

if (EVENT == 2306) then
	QuestStatus = GetQuestStatus(UID, 798)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 798, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 798, 23016, NPC, 18, 2307);
		else
			SaveEvent(UID, 13895);
		end
	end
end

if (EVENT == 2305) then
	QuestStatus = GetQuestStatus(UID, 798)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 798, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 798, 23016, NPC, 18, 2307);
		else
			SelectMsg(UID, 4, 798, 23016, NPC, 41, 2308, 27, -1);
		end
	end
end

if (EVENT == 2307 ) then
	ShowMap(UID, 37)
end

if (EVENT == 2308) then
	QuestStatus = GetQuestStatus(UID, 798)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 798, 1);
		if (MonsterCount < 30) then
			SelectMsg(UID, 2, 798, 23016, NPC, 18, 2307);
		else
			ShowMap(UID, 1332);
			RunQuestExchange(UID, 3244);
			SaveEvent(UID, 13894);
			SaveEvent(UID, 13905);
			RobItem(UID, 900337525);
		end
	end
end

if (EVENT == 2401)then
	SelectMsg(UID, 2, 800, 23018, NPC, 3000, 2403);
	SaveEvent(UID, 13905);
end


if (EVENT == 2403)then
	SelectMsg(UID, 4, 800, 23018, NPC, 3000, 2404,3005,-1);
	SaveEvent(UID, 13907);
end

if (EVENT == 2404)then
	SelectMsg(UID, 2, 800, 23018, NPC, 10, -1);
	SaveEvent(UID, 13906);
	SaveEvent(UID, 13917);
end

if (EVENT == 2501) then
	SelectMsg(UID, 4, 803, 23024, NPC, 22, 2502, 27, -1);
end

if (EVENT == 2502) then
	QuestStatus = GetQuestStatus(UID, 803)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 13941);
	end
end

if (EVENT == 2506) then
	QuestStatus = GetQuestStatus(UID, 803)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900333000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 803, 23024, NPC, 18,-1);
		else
			SaveEvent(UID, 13943);
		end
	end
end

if(EVENT == 2505) then
	QuestStatus = GetQuestStatus(UID, 803)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900333000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 803, 23024, NPC, 18,-1);
		else
			SelectMsg(UID, 4, 803, 23024, NPC, 22, 2507,23,-1);
		end
	end
end

if (EVENT == 2507 ) then
	QuestStatus = GetQuestStatus(UID, 803)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	ITEMA = HowmuchItem(UID, 900333000)	
		if(ITEMA < 1) then
			SelectMsg(UID, 2, 803, 23024, NPC, 18,-1);
		else
			SelectMsg(UID, 2, -1, 23303, NPC, 10, -1);
			RunQuestExchange(UID,3248);
			SaveEvent(UID, 13942);
			SaveEvent(UID, 13953);
		end
	end
end

if (EVENT == 2601)then
	SelectMsg(UID, 2, 804, 3249, NPC, 3000, 2603);
	SaveEvent(UID, 13953);
end

if (EVENT == 2603)then
	SelectMsg(UID, 4, 804, 3249, NPC, 3000, 2604,3005,-1);
	SaveEvent(UID, 13955);
end

if (EVENT == 2604)then
	QuestStatus = GetQuestStatus(UID, 804)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			RunQuestExchange(UID,3249);
			SaveEvent(UID, 13954);
			SaveEvent(UID, 13965);
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=564 status=2 n_index=11639
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 564)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3054);
		SaveEvent(UID, 11641);
	end
end

-- [AUTO-GEN] quest=602 status=2 n_index=12083
if (EVENT == 193) then
	QuestStatusCheck = GetQuestStatus(UID, 602)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3091);
		SaveEvent(UID, 12085);
	end
end

-- [AUTO-GEN] quest=564 status=255 n_index=11636
if (EVENT == 1000) then
	SaveEvent(UID, 11637);
end

-- [AUTO-GEN] quest=564 status=0 n_index=11637
if (EVENT == 1002) then
	SelectMsg(UID, 4, 564, 20096, NPC, 3096, 1003, 23, -1);
end

-- [AUTO-GEN] quest=564 status=0 n_index=11637
if (EVENT == 1003) then
	SaveEvent(UID, 11638);
end

-- [AUTO-GEN] quest=564 status=1 n_index=11638
if (EVENT == 1005) then
	ItemA = HowmuchItem(UID, 910232000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 564, 20096, NPC, 18, 1006);
	else
		SelectMsg(UID, 4, 564, 20096, NPC, 41, 1008, 27, -1);
	end
end

-- [AUTO-GEN] quest=564 status=1 n_index=11638
if (EVENT == 1006) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=564 status=1 n_index=11638
if (EVENT == 1008) then
	QuestStatusCheck = GetQuestStatus(UID, 564)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3054);
		SaveEvent(UID, 11639);
	end
end

-- [AUTO-GEN] quest=573 status=255 n_index=11743
if (EVENT == 1100) then
	SaveEvent(UID, 11744);
end

-- [AUTO-GEN] quest=573 status=0 n_index=11744
if (EVENT == 1102) then
	SelectMsg(UID, 4, 573, 20114, NPC, 3114, 1103, 23, -1);
end

-- [AUTO-GEN] quest=573 status=0 n_index=11744
if (EVENT == 1103) then
	SaveEvent(UID, 11745);
end

-- [AUTO-GEN] quest=573 status=1 n_index=11745
if (EVENT == 1105) then
	ItemA = HowmuchItem(UID, 910234000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 573, 20114, NPC, 18, 1106);
	else
		SelectMsg(UID, 4, 573, 20114, NPC, 41, 1108, 27, -1);
	end
end

-- [AUTO-GEN] quest=573 status=1 n_index=11745
if (EVENT == 1106) then
	ShowMap(UID, 11);
end

-- [AUTO-GEN] quest=573 status=1 n_index=11745
if (EVENT == 1108) then
	QuestStatusCheck = GetQuestStatus(UID, 573)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3063);
		SaveEvent(UID, 11746);
	end
end

-- [AUTO-GEN] quest=602 status=255 n_index=12080
if (EVENT == 1200) then
	SaveEvent(UID, 12081);
end

-- [AUTO-GEN] quest=602 status=0 n_index=12081
if (EVENT == 1202) then
	SelectMsg(UID, 4, 602, 20772, NPC, 3170, 1203, 23, -1);
end

-- [AUTO-GEN] quest=602 status=1 n_index=12082
if (EVENT == 1203) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 602, 20772, NPC, 18, 1205);
	else
		SelectMsg(UID, 4, 602, 20772, NPC, 41, 1204, 27, -1);
	end
end

-- [AUTO-GEN] quest=602 status=1 n_index=12082
if (EVENT == 1204) then
	QuestStatusCheck = GetQuestStatus(UID, 602)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3091);
		SaveEvent(UID, 12083);
	end
end

-- [AUTO-GEN] quest=602 status=3 n_index=12084
if (EVENT == 1205) then
	SelectMsg(UID, 2, 602, 20772, NPC, 10, -1);
end

