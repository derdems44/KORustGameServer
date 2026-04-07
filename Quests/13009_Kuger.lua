local NPC = 13009;

if EVENT == 3000 then
	QuestNum = SearchQuest(UID, 13009);
		if QuestNum == 0 then 
			 SelectMsg(UID, 2, -1, 1186, 13009, 10, -1);
		elseif QuestNum > 1 and  QuestNum < 100 then
          NpcMsg(UID, 1187,NPC)
      else 
          EVENT = QuestNum
	end
end

if EVENT == 3005 then
   SelectMsg(UID, 2, 84, 1190, 13009, 10, 3006);
end

if EVENT == 3006 then
   SelectMsg(UID, 4, 84, 1191, 13009, 22, 3007, 23, -1);
end

if EVENT == 3007 then
   ZoneChange(UID, 51, 150, 150)
   SaveEvent(UID, 3034);
end

if EVENT == 205 then
   SaveEvent(UID, 3036);
end

if EVENT == 3010 then
   ITEM_COUNT = HowmuchItem(UID, 910038000);
   if  ITEM_COUNT < 5 then
      SelectMsg(UID, 2, 84, 1194, 13009, 18, 3011);
   elseif ITEM_COUNT  > 4 then
      SelectMsg(UID, 5, 84, 1195, 13009, 4006, 3012, 23, -1);
   end
end

if EVENT == 3011 then
   ZoneChange(UID, 51, 150, 150)
end

if EVENT == 3012 then
	QuestStatusCheck = GetQuestStatus(UID, 84) 
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 4638, NPC, 10, -1);
	else
   ITEM_COUNT = HowmuchItem(UID, 910038000);
   if  ITEM_COUNT < 5 then
      SelectMsg(UID, 2, 84, 1194, 13009, 18, 3011);
	  else
        RunQuestExchange(UID, 304,STEP,1);
        SaveEvent(UID, 3035);
		RobItem(UID, 910038000, 5);
end
end
end

if (EVENT == 632) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 358, 4637, NPC, 22, 633, 23, -1);
	else
		SelectMsg(UID, 2, 358, 4638, NPC, 10, -1);
	end
end

if EVENT == 633 then
   Class = CheckClass (UID);
    if Class == 1 or Class == 5 or Class == 6 then
    SaveEvent(UID, 4378);
    elseif Class == 2 or Class == 7 or Class == 8 then
    SaveEvent(UID, 4383);
    elseif Class == 3 or Class == 9 or Class == 10 then
    SaveEvent(UID, 4388);
    elseif Class == 4 or Class == 11 or Class == 12 then
    SaveEvent(UID, 4393);
   end
end

if EVENT == 280 then
   Class = CheckClass (UID);
    if Class == 1 or Class == 5 or Class == 6 then
    SaveEvent(UID, 4380);
    EVENT = 281
    elseif Class == 2 or Class == 7 or Class == 8 then
    SaveEvent(UID, 4385);
    EVENT = 281
    elseif Class == 3 or Class == 9 or Class == 10 then
    SaveEvent(UID, 4390);
    EVENT = 281
   elseif Class == 4 or Class == 11 or Class == 12 then
    SaveEvent(UID, 4395);
     EVENT = 281
   end
end

if EVENT == 281 then
   NATION = CheckNation(UID);
   if NATION == 1 then 
    SelectMsg(UID, 1, 358, 4639, NPC, 4080, -1);
   else 
    SelectMsg(UID, 1, 358, 4640, NPC, 4080, -1);
   end
end

if (EVENT == 636) then
	MonsterCount = CountMonsterQuestSub(UID, 358, 1);
	if (MonsterCount < 1) then
		SelectMsg(UID, 2, 358, 4641, NPC, 10, -1);
	else
		SelectMsg(UID, 4, 358, 4642, NPC, 4161, 637, 4162, -1);
	end
end

if (EVENT == 637) then
	QuestStatusCheck = GetQuestStatus(UID, 220) 
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 4638, NPC, 10, -1);
	else
	MonsterCount = CountMonsterQuestSub(UID, 358, 1);
	if (MonsterCount < 1) then
		SelectMsg(UID, 2, 358, 4641, NPC, 10, -1);
	else
	Class = CheckClass(UID);
	if (Class == 1 or Class == 5 or Class == 6 or Class == 13 or Class == 14 or Class == 15) then
		RunQuestExchange(UID,501);
		SaveEvent(UID, 4379); 
	elseif (Class == 2 or Class == 7 or Class == 8) then
		RunQuestExchange(UID,502);
		SaveEvent(UID, 4384); 
	elseif (Class == 3 or Class == 9 or Class == 10) then
		RunQuestExchange(UID,503);
		SaveEvent(UID, 4389);
	elseif (Class == 4 or Class == 11 or Class == 12) then
		RunQuestExchange(UID,504);
		SaveEvent(UID, 4394);
end
end
end
end

if (EVENT == 6014) then
	SelectMsg(UID, 4, 64, 6013, NPC, 22, -1, 23, -1);
end

if (EVENT == 6063) then
	SelectMsg(UID, 2, 88, 6016, NPC, 10, 6064);
end

if (EVENT == 6064) then
	SelectMsg(UID, 4, 88, 6017, NPC, 22, 6065, 23, -1);
end

if (EVENT == 6065) then
	ZoneChange(UID, 52, 150, 150)
	SaveEvent(UID, 6024);
end

if (EVENT == 6066) then
	SaveEvent(UID, 6026);
end

if (EVENT == 6067) then
	ShowMap(UID, 6);
end

if (EVENT == 6068) then
	ITEM_COUNT = HowmuchItem(UID, 910039000);
	if (ITEM_COUNT < 9) then
		SelectMsg(UID, 2, 88, 6021, NPC, 18, 6069);
	else
		SelectMsg(UID, 5, 88, 6022, NPC, 22, 6070, 23, -1);
	end
end

if (EVENT == 6070) then
	QuestStatusCheck = GetQuestStatus(UID, 88) 
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 4638, NPC, 10, -1);
	else
	ITEM_COUNT = HowmuchItem(UID, 910039000);
	if (ITEM_COUNT < 9) then
		SelectMsg(UID, 2, 88, 6021, NPC, 18, 6069);
	else
	RunQuestExchange(UID,600,STEP,1);
	RobItem(UID,910039000,10);
	SaveEvent(UID, 6025);
end
end
end

if (EVENT == 6069) then
	ZoneChange(UID, 52, 150, 150)
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=358 status=2 n_index=4379
if (EVENT == 190) then
	SearchQuest(UID, 13009);
end

-- [AUTO-GEN] quest=437 status=2 n_index=7102
if (EVENT == 240) then
	QuestStatusCheck = GetQuestStatus(UID, 437)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 7104);
	end
end

-- [AUTO-GEN] quest=437 status=255 n_index=7099
if (EVENT == 400) then
	SaveEvent(UID, 7100);
end

-- [AUTO-GEN] quest=437 status=0 n_index=7100
if (EVENT == 401) then
	SelectMsg(UID, 4, 437, 4985, NPC, 963, 402, 23, -1);
end

-- [AUTO-GEN] quest=437 status=0 n_index=7100
if (EVENT == 402) then
	SaveEvent(UID, 7101);
end

-- [AUTO-GEN] quest=437 status=1 n_index=7101
if (EVENT == 403) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=59 status=255 n_index=11
if (EVENT == 420) then
	SaveEvent(UID, 12);
end

-- [AUTO-GEN] quest=59 status=0 n_index=12
if (EVENT == 422) then
	SelectMsg(UID, 4, 59, 8138, NPC, 6, 423, 23, -1);
end

-- [AUTO-GEN] quest=59 status=0 n_index=12
if (EVENT == 423) then
	SaveEvent(UID, 12);
end

-- [AUTO-GEN] quest=358 status=255 n_index=4376
if (EVENT == 630) then
	SaveEvent(UID, 4377);
end

-- [AUTO-GEN] quest=84 status=255 n_index=3031
if (EVENT == 3002) then
	SaveEvent(UID, 3033);
end

-- [AUTO-GEN] quest=84 status=255 n_index=3032
if (EVENT == 3004) then
	SaveEvent(UID, 3033);
end

-- [AUTO-GEN] quest=64 status=255 n_index=6000
if (EVENT == 6000) then
	SaveEvent(UID, 6002);
end

-- [AUTO-GEN] quest=64 status=1 n_index=6003
if (EVENT == 6003) then
	ItemA = HowmuchItem(UID, 910117000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 64, 6013, NPC, 18, 6004);
	else
		SelectMsg(UID, 4, 64, 6013, NPC, 41, 6009, 27, -1);
	end
end

-- [AUTO-GEN] quest=64 status=1 n_index=6003
if (EVENT == 6004) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=64 status=1 n_index=6003
if (EVENT == 6009) then
	QuestStatusCheck = GetQuestStatus(UID, 64)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 86);
		SaveEvent(UID, 6004);
	end
end

-- [AUTO-GEN] quest=88 status=255 n_index=6021
if (EVENT == 6060) then
	SaveEvent(UID, 6023);
end

-- [AUTO-GEN] quest=88 status=255 n_index=6022
if (EVENT == 6062) then
	SaveEvent(UID, 6023);
end

