local NPC = 25164;

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

if(EVENT == 1112) then 
	SelectMsg(UID, 4, 1324, 43928, NPC, 22, 1113, 23, -1);
end

if(EVENT == 1113) then
	QuestStatus = GetQuestStatus(UID, 1324)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 3654);
	end
end

if(EVENT == 1117) then
	QuestStatus = GetQuestStatus(UID, 1324)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE1 = HowmuchItem(UID, 900654000)
		if( BRACE1 < 1) then
			SelectMsg(UID, 2, 1324, 43928, NPC, 18, 1116);
		else
			SaveEvent(UID, 3656);
		end
	end
end

if(EVENT == 1115) then
	QuestStatus = GetQuestStatus(UID, 1324)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE1 = HowmuchItem(UID, 900654000)
		if( BRACE1 < 1) then
			SelectMsg(UID, 2, 1324, 43928, NPC, 18, 1116);
		else
			SelectMsg(UID, 4, 1324, 43928, NPC, 10, 1118, 27, -1);
		end
	end
end

if(EVENT == 1116) then
	ShowMap(UID, 1336);
end

if(EVENT == 1118) then
	QuestStatus = GetQuestStatus(UID, 1324)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	BRACE1 = HowmuchItem(UID, 900654000)
		if( BRACE1 < 1) then
			SelectMsg(UID, 2, 1324, 43928, NPC, 18, 1116);
		else
			RunQuestExchange(UID,6118);
			SaveEvent(UID, 3655);
		end
	end
end

if(EVENT == 1152) then 
	SelectMsg(UID, 4, 1351, 44139, NPC, 22, 1156, 23, -1);
end

if(EVENT == 1156) then
RunQuestExchange(UID,6145);
SaveEvent(UID, 3816);
SaveEvent(UID, 3818);
SaveEvent(UID, 3817);
end

if(EVENT == 1182) then 
	SelectMsg(UID, 4, 1352, 44140, NPC, 22, 1183, 23, -1);
end

if(EVENT == 1183) then
	QuestStatus = GetQuestStatus(UID, 1352)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 3822);
	end
end

if(EVENT == 1187) then
	QuestStatus = GetQuestStatus(UID, 1352)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1352, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1352, 44140, NPC, 18, 1186);
		else
			SaveEvent(UID, 3824);
		end
	end
end

if (EVENT == 1185) then
	QuestStatus = GetQuestStatus(UID, 1352)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1352, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1352, 44140, NPC, 18, 1186);
		else
			SelectMsg(UID, 4, 1352, 44140, NPC, 22, 1188, 23, -1);
		end
	end
end

if(EVENT == 1186) then
	ShowMap(UID, 489);
end

if(EVENT == 1188) then
	QuestStatus = GetQuestStatus(UID, 1352)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	MonsterCount = CountMonsterQuestSub(UID, 1352, 1);
		if (MonsterCount < 50) then
			SelectMsg(UID, 2, 1352, 44140, NPC, 18, 1186);
		else
			RunQuestExchange(UID,6146);
			SaveEvent(UID, 3823);
		end
	end
end

if(EVENT == 1192) then 
	SelectMsg(UID, 4, 1353, 44141, NPC, 22, 1193, 23, -1);
end

if(EVENT == 1193) then
	QuestStatus = GetQuestStatus(UID, 1353)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
			SaveEvent(UID, 3828);
	end
end

if(EVENT == 1197) then
	QuestStatus = GetQuestStatus(UID, 1353)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1353, 44141, NPC, 18, 1196);
		else
			SaveEvent(UID, 3830);
		end
	end
end

if(EVENT == 1195) then
	QuestStatus = GetQuestStatus(UID, 1353)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1353, 44141, NPC, 18, 1196);
		else
			SelectMsg(UID, 4, 1353, 44141, NPC, 10, 1198, 27, -1);
		end
	end
end

if(EVENT == 1196) then
	ShowMap(UID, 1332);
end

if(EVENT == 1198) then
	QuestStatus = GetQuestStatus(UID, 1353)	
		if(QuestStatus == 2) then
			SelectMsg(UID, 2, -1, 44614, NPC, 10, -1);
		else
	GHOST = HowmuchItem(UID, 900638000)
		if( GHOST < 1) then
			SelectMsg(UID, 2, 1353, 44141, NPC, 18, 1196);
		else
			RunQuestExchange(UID,6147);
			SaveEvent(UID, 3829);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1324 status=255 n_index=3652
if (EVENT == 1110) then
	SaveEvent(UID, 3653);
end

-- [AUTO-GEN] quest=1351 status=255 n_index=3814
if (EVENT == 1150) then
	SaveEvent(UID, 3815);
end

-- [AUTO-GEN] quest=1351 status=1 n_index=3816
if (EVENT == 1157) then
	QuestStatusCheck = GetQuestStatus(UID, 1351)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6145);
		SaveEvent(UID, 3817);
	end
end

-- [AUTO-GEN] quest=1352 status=255 n_index=3820
if (EVENT == 1180) then
	SaveEvent(UID, 3821);
end

-- [AUTO-GEN] quest=1353 status=255 n_index=3826
if (EVENT == 1190) then
	SaveEvent(UID, 3827);
end

