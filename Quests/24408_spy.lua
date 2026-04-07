local NPC= 24408;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 4287, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 4288, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 125) then 
	SelectMsg(UID, 2, 216, 4292, NPC, 4170, 140, 4169, -1);
end

if (EVENT == 140) then 
	ItemA = HowmuchItem(UID, 910085000);  
	if (ItemA == 0) then 
		Check = isRoomForItem(UID, 910085000);
		if (Check == -1) then
			SelectMsg(UID, 2, -1, 1627, NPC, 27, -1);
		else
			GiveItem(UID, 910085000, 1);
			--SaveEvent(UID, 4191);
		end	
	else
		SelectMsg(UID, 2, 216, 4293, NPC, 10, -1);
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=216 status=1 n_index=4189
if (EVENT == 130) then
	ItemA = HowmuchItem(UID, 910085000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 216, 0, NPC, 18, 132);
	else
		SelectMsg(UID, 4, 216, 0, NPC, 41, 131, 27, -1);
	end
end

-- [AUTO-GEN] quest=216 status=1 n_index=4189
if (EVENT == 131) then
	QuestStatusCheck = GetQuestStatus(UID, 216)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 469);
		SaveEvent(UID, 4190);
	end
end

-- [AUTO-GEN] quest=216 status=1 n_index=4189
if (EVENT == 132) then
	ShowMap(UID, 61);
end

