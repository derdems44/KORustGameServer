local NPC = 31700;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 4174, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 4174, NPC)
	else
		EVENT = QuestNum
	end
end
--if EVENT == 100 then
	--QuestStatusCheck = GetQuestStatus(UID, 182)	
	--if(QuestStatusCheck == 3) then
	--	EVENT = 107
	--	else
 --  SelectMsg(UID, 2, -1, 4174, NPC, 7015, 101, 7017, 300);
--end
--end

if EVENT == 201 then 
   SelectMsg(UID, 19, -1, 9184, NPC, 10,-1);
end

if EVENT == 202 then 
   SelectMsg(UID, 19, -1, 1637, NPC, 10,-1);
end

if EVENT == 203 then 
   SelectMsg(UID, 2, -1, 1564, NPC, 3000,-1,3005,-1);
end

if EVENT == 204 then
   SelectMsg(UID, 4, 182, 1565, NPC, 3000,-1,3005,-1);
end

if(EVENT == 205) then
IsTakeToday = GetUserDailyOp(UID,10);
if (IsTakeToday == 1) then
SelectMsg(UID, 19, -1, 1556, NPC, 10,106);
SaveEvent(UID, 1206)
	else
	SelectMsg(UID, 2, -1, 11584, NPC, 10, -1);
	end
end

if(EVENT == 106) then
	SelectMsg(UID, 2, -1, 1561, NPC, 10,-1);
	if CheckGiveSlot(UID, 2) then
		GiveItem(UID, 900074000, 1);
		GiveItem(UID, 900075000, 1);
		SaveEvent(UID, 1209)
	end
end

if EVENT == 107 then 
   SelectMsg(UID, 4, 182, 1565, NPC, 3000,108,3005,-1);
end

if EVENT == 108 then
	RunQuestExchange(UID, 188);
	SaveEvent(UID, 1209);
end

if(EVENT == 216) then
SaveEvent(UID, 1208)
end

if (EVENT == 125) then
	SelectMsg(UID, 4, 216, 6333, NPC, 22, 121, 27, -1);
end

if (EVENT == 121) then
	SaveEvent(UID,4167);
end

if (EVENT == 127) then
	SaveEvent(UID,4169);
end

if (EVENT == 130) then
	ITEM1_COUNT = HowmuchItem(UID, 910085000);   
	if (ITEM1_COUNT < 1) then
		SelectMsg(UID, 2, 216, 4186, NPC, 18,123);
	else
		SelectMsg(UID, 4, 216, 6377, NPC, 4146, 138,4005, -1);
end
end	

if (EVENT == 123 ) then
	ShowMap(UID, 425);
end

if (EVENT == 138)then
	QuestStatusCheck = GetQuestStatus(UID, 216) 
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 216, NPC, 10, -1);
	else
	ITEM1_COUNT = HowmuchItem(UID, 910085000);   
	if (ITEM1_COUNT < 1) then
		SelectMsg(UID, 2, 216, 4186, NPC, 18,123);
	else
	RunQuestExchange(UID,469);
	SaveEvent(UID,4168);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=182 status=2 n_index=1202
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 182)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 187);
		SaveEvent(UID, 1204);
	end
end

-- [AUTO-GEN] quest=187 status=0 n_index=1250
if (EVENT == 206) then
	SelectMsg(UID, 4, 187, 8875, NPC, 167, 207, 23, -1);
end

-- [AUTO-GEN] quest=187 status=0 n_index=1250
if (EVENT == 207) then
	SaveEvent(UID, 1251);
end

-- [AUTO-GEN] quest=188 status=0 n_index=1260
if (EVENT == 208) then
	SaveEvent(UID, 1261);
end

-- [AUTO-GEN] quest=182 status=1 n_index=1201
if (EVENT == 209) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 182, 8875, NPC, 18, 210);
	else
		SelectMsg(UID, 4, 182, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=182 status=1 n_index=1201
if (EVENT == 210) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=183 status=1 n_index=1211
if (EVENT == 309) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 183, 8875, NPC, 18, 310);
	else
		SelectMsg(UID, 4, 183, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=183 status=1 n_index=1211
if (EVENT == 310) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=184 status=1 n_index=1221
if (EVENT == 409) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 184, 8875, NPC, 18, 410);
	else
		SelectMsg(UID, 4, 184, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=184 status=1 n_index=1221
if (EVENT == 410) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=185 status=1 n_index=1231
if (EVENT == 509) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 185, 8875, NPC, 18, 510);
	else
		SelectMsg(UID, 4, 185, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=185 status=1 n_index=1231
if (EVENT == 510) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=186 status=1 n_index=1241
if (EVENT == 609) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 186, 8875, NPC, 18, 610);
	else
		SelectMsg(UID, 4, 186, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=186 status=1 n_index=1241
if (EVENT == 610) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=187 status=1 n_index=1251
if (EVENT == 709) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 187, 8875, NPC, 18, 710);
	else
		SelectMsg(UID, 4, 187, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=187 status=1 n_index=1251
if (EVENT == 710) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=188 status=1 n_index=1261
if (EVENT == 809) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 188, 8875, NPC, 18, 810);
	else
		SelectMsg(UID, 4, 188, 8875, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=188 status=1 n_index=1261
if (EVENT == 810) then
	ShowMap(UID, 1);
end

