local NPC = 25066;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 43928, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then
		NpcMsg(UID, 43928, NPC)
	else
		EVENT = QuestNum
	end
end

if(EVENT == 112) then 
	SelectMsg(UID, 4, 1262, 43928, NPC, 22, 113, 23, -1);
end

if(EVENT == 113) then
	QuestStatus = GetQuestStatus(UID, 1262)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7680);
	end
end

if(EVENT == 117) then
	QuestStatus = GetQuestStatus(UID, 1262)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
		if( BRACE < 1) then
			SelectMsg(UID, 2, 1262, 43928, NPC, 18, 116);
		else
			SaveEvent(UID, 7682);
		end
	end
end

if(EVENT == 115) then
	QuestStatus = GetQuestStatus(UID, 1262)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
		if( BRACE < 1) then
			SelectMsg(UID, 2, 1262, 43928, NPC, 18, 116);
		else
			SelectMsg(UID, 4, 1262, 43928, NPC, 10, 118, 27, -1);
		end
	end
end

if(EVENT == 116) then
	ShowMap(UID, 1337);
end

if(EVENT == 118) then
	QuestStatus = GetQuestStatus(UID, 1262)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE = HowmuchItem(UID, 900654000)
		if( BRACE < 1) then
			SelectMsg(UID, 2, 1262, 43928, NPC, 18, 116);
		else
			RunQuestExchange(UID,6058);
			SaveEvent(UID, 7681);
		end
	end
end

if (EVENT == 152) then
	SelectMsg(UID, 4, 1289, 44139, NPC, 10, 153, 23, -1);
end

if(EVENT == 153) then
    RunQuestExchange(UID,6085);
    SaveEvent(UID, 7880);
	SaveEvent(UID, 7882);
	SaveEvent(UID, 7881);
end

if (EVENT == 182) then
	SelectMsg(UID, 4, 1292, 44140, NPC, 10, 183, 23, -1);
end

if(EVENT == 183) then
	QuestStatus = GetQuestStatus(UID, 1292)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7898);
	end
end

if(EVENT == 187) then
	QuestStatus = GetQuestStatus(UID, 1292)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1292, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1292, 44140, NPC, 10, 186);
		else
			SaveEvent(UID, 7900);
		end
	end
end

if (EVENT == 185) then
	QuestStatus = GetQuestStatus(UID, 1292)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1292, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1292, 44140, NPC, 10, 186);
		else
			SelectMsg(UID, 4, 1292, 44140, NPC, 10, 188, 27, -1);
		end
	end
end

if(EVENT == 186) then
ShowMap(UID, 488);
end

if(EVENT == 188) then
	QuestStatus = GetQuestStatus(UID, 1292)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1292, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1292, 44140, NPC, 10, 186);
		else
			RunQuestExchange(UID,6088);
			SaveEvent(UID, 7899);
		end
	end
end

if (EVENT == 192) then
	SelectMsg(UID, 4, 1293, 44141, NPC, 10, 193, 23, -1);
end

if(EVENT == 193) then
	QuestStatus = GetQuestStatus(UID, 1293)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 7904);
	end
end

if(EVENT == 197) then
	QuestStatus = GetQuestStatus(UID, 1293)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1293, 44141, NPC, 18, 196);
		else
			SaveEvent(UID, 7906);
		end
	end
end

if(EVENT == 195) then
	QuestStatus = GetQuestStatus(UID, 1293)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1293, 44141, NPC, 18, 196);
		else
			SelectMsg(UID, 4, 1293, 44141, NPC, 10, 198, 27, -1);
		end
	end
end

if(EVENT == 196) then
ShowMap(UID, 1284);
end

if(EVENT == 198) then
	QuestStatus = GetQuestStatus(UID, 1293)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1293, 44141, NPC, 18, 196);
		else
			RunQuestExchange(UID,6089);
			SaveEvent(UID, 7905);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1262 status=255 n_index=7678
if (EVENT == 110) then
	SaveEvent(UID, 7679);
end

-- [AUTO-GEN] quest=1289 status=255 n_index=7878
if (EVENT == 150) then
	SaveEvent(UID, 7879);
end

-- [AUTO-GEN] quest=1289 status=1 n_index=7880
if (EVENT == 157) then
	QuestStatusCheck = GetQuestStatus(UID, 1289)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6085);
		SaveEvent(UID, 7881);
	end
end

-- [AUTO-GEN] quest=1292 status=255 n_index=7896
if (EVENT == 180) then
	SaveEvent(UID, 7897);
end

-- [AUTO-GEN] quest=1293 status=255 n_index=7902
if (EVENT == 190) then
	SearchQuest(UID, 25066);
end

