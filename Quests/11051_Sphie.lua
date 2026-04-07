local NPC = 11051;

if (EVENT == 215) then
	ITEM = HowmuchItem(UID, 910044000); 
		if (ITEM < 1 or ITEM == 0) then
			SelectMsg(UID, 2, 179, 677, NPC, 18, 191);
		else
			SelectMsg(UID, 4, 179, 678, NPC, 22, 218, 23, -1);
	end
end

if (EVENT == 191) then
	ShowMap(UID, 39);
end

if (EVENT == 218) then
	QuestStatusCheck = GetQuestStatus(UID, 179) 
		if(QuestStatusCheck == 2) then
			SelectMsg(UID, 2, -1, 8004, NPC, 10, -1);
		else
	Check = isRoomForItem(UID, 910041000);
	ITEM = HowmuchItem(UID, 910044000); 
		if (Check == -1) then
			SelectMsg(UID, 2, -1, 1626, NPC, 27, -1);
		elseif (ITEM < 1 or ITEM == 0) then
			SelectMsg(UID, 2, 179, 677, NPC, 18, 191);
		else
			SelectMsg(UID, 2, 179, 676, NPC, 10, -1);
			RunQuestExchange(UID,89);
			SaveEvent(UID, 446);
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=179 status=1 n_index=445
if (EVENT == 216) then
	ItemA = HowmuchItem(UID, 910044000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 179, 0, NPC, 18, 218);
	else
		SelectMsg(UID, 4, 179, 0, NPC, 41, 217, 27, -1);
	end
end

-- [AUTO-GEN] quest=179 status=1 n_index=445
if (EVENT == 217) then
	QuestStatusCheck = GetQuestStatus(UID, 179)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 89);
		SaveEvent(UID, 446);
	end
end

