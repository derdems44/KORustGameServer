local NPC = 31701;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 4274, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 4274, NPC)
	else
		EVENT = QuestNum
	end
end
--if EVENT == 100 then
	--QuestStatusCheck = GetQuestStatus(UID, 182)	
	--if(QuestStatusCheck == 3) then
	--	EVENT = 107
	--	else
 --  SelectMsg(UID, 2, -1, 4274, NPC, 7015, 101, 7017, 125);
--end
--end

if EVENT == 201 then 
   SelectMsg(UID, 19, -1, 9185, NPC, 10,202);
end

if EVENT == 202 then 
   SelectMsg(UID, 19, -1, 1640, NPC, 10,203);
end

if EVENT == 203 then 
   SelectMsg(UID, 2, -1, 1564, NPC, 3000,204,3005,-1);
end

if EVENT == 204 then 
   SelectMsg(UID, 4, 182, 1565, NPC, 3000,205,3005,-1);
end

if(EVENT == 205) then
IsTakeToday = GetUserDailyOp(UID,10);
if (IsTakeToday == 1) then
SelectMsg(UID, 19, -1, 1566, NPC, 10,-1);
SaveEvent(UID, 1206)
	else
	SelectMsg(UID, 2, -1, 11584, NPC, 10, -1);
	end
end

if(EVENT == 206) then
	SelectMsg(UID, 2, -1, 1571, NPC, 10,-1);
	if CheckGiveSlot(UID, 2) then
		GiveItem(UID, 900074000, 1);
		GiveItem(UID, 900075000, 1);
		SaveEvent(UID, 1209)
	end
end

if EVENT == 207 then
   SelectMsg(UID, 4, 182, 1565, NPC, 3000,108,3005,-1);
end

if EVENT == 108 then
	RunQuestExchange(UID, 188);
	SaveEvent(UID, 1209);
end

if EVENT == 209 then
	ITEM1_COUNT = HowmuchItem(UID, 900035000);   
	if (ITEM1_COUNT < 1) then
		SelectMsg(UID, 2, 182, 4279, NPC, 18,-1);
	else
		SelectMsg(UID, 4, 182, 6378, NPC, 4006, 210,4005, -1);
	end
end

if EVENT == 210 then
	ITEM1_COUNT = HowmuchItem(UID, 900035000);   
	if (ITEM1_COUNT < 1) then
		SelectMsg(UID, 2, 182, 4279, NPC, 18,123);
	else
	RunQuestExchange(UID, 188);
	SaveEvent(UID, 1207);
	end
end

if(EVENT == 216) then
SaveEvent(UID, 1208)
end


if (EVENT == 125) then
	SelectMsg(UID, 4, 216, 6349, NPC, 22, 121, 27, -1);
end

if (EVENT == 121) then
	SaveEvent(UID,4178);
end

if (EVENT == 127) then
	SaveEvent(UID,4180);
end

if (EVENT == 130) then
	ITEM1_COUNT = HowmuchItem(UID, 910085000);   
	if (ITEM1_COUNT < 1) then
		SelectMsg(UID, 2, 216, 4279, NPC, 18,123);
	else
		SelectMsg(UID, 4, 216, 6378, NPC, 4006, 138,4005, -1);
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
		SelectMsg(UID, 2, 216, 4279, NPC, 18,123);
	else
	RunQuestExchange(UID,469);
	SaveEvent(UID,4179);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=182 status=2 n_index=1207
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 182)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 188);
		SaveEvent(UID, 1209);
	end
end

-- [AUTO-GEN] quest=183 status=1 n_index=1216
if (EVENT == 309) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 183, 8876, NPC, 18, 310);
	else
		SelectMsg(UID, 4, 183, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=183 status=1 n_index=1216
if (EVENT == 310) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=184 status=1 n_index=1226
if (EVENT == 409) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 184, 8876, NPC, 18, 410);
	else
		SelectMsg(UID, 4, 184, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=184 status=1 n_index=1226
if (EVENT == 410) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=185 status=1 n_index=1236
if (EVENT == 509) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 185, 8876, NPC, 18, 510);
	else
		SelectMsg(UID, 4, 185, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=185 status=1 n_index=1236
if (EVENT == 510) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=186 status=1 n_index=1246
if (EVENT == 609) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 186, 8876, NPC, 18, 610);
	else
		SelectMsg(UID, 4, 186, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=186 status=1 n_index=1246
if (EVENT == 610) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=187 status=1 n_index=1256
if (EVENT == 709) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 187, 8876, NPC, 18, 710);
	else
		SelectMsg(UID, 4, 187, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=187 status=1 n_index=1256
if (EVENT == 710) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=188 status=1 n_index=1266
if (EVENT == 809) then
	ItemA = HowmuchItem(UID, 900035000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 188, 8876, NPC, 18, 810);
	else
		SelectMsg(UID, 4, 188, 8876, NPC, 41, 216, 27, -1);
	end
end

-- [AUTO-GEN] quest=188 status=1 n_index=1266
if (EVENT == 810) then
	ShowMap(UID, 2);
end

