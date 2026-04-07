local NPC = 29197;

if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 10486, NPC, 7572, 101, 7582, 110,7589,-1,7590,130,7636,-1);
end

if (EVENT == 110) then
	TROPHYFLAME = HowmuchItem(UID, 800149000);
		if (TROPHYFLAME < 1 or TROPHYFLAME == 0) then
			SelectMsg(UID, 2, -1, 10370, NPC, 10, -1);
		else
	SlotCheck = CheckGiveSlot(UID, 1)
		if SlotCheck == false then
		else
	Check = CheckExchange(UID, 2626)
		if  Check == true then   
			Roll = RollDice(UID, 25) 
			found = Roll + 2601
			RunRandomExchange(UID, found);
			end
		end
	end
end

if (EVENT == 130) then
	TROPHYBLOODY = HowmuchItem(UID, 508165000);
		if (TROPHYBLOODY < 1 or TROPHYBLOODY == 0) then
			SelectMsg(UID, 2, -1, 10370, NPC, 10, -1);
		else
	SlotCheck = CheckGiveSlot(UID, 1)
		if SlotCheck == false then
		else
	Check = CheckExchange(UID, 2734)
		if  Check == true then   
			Roll = RollDice(UID, 33) 
			found = Roll + 2701
			RunRandomExchange(UID, found);
			end
		end
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=952 status=2 n_index=6847
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 952)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6849);
	end
end

-- [AUTO-GEN] quest=952 status=0 n_index=6845
if (EVENT == 1000) then
	SelectMsg(UID, 4, 952, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=952 status=0 n_index=6845
if (EVENT == 1001) then
	SaveEvent(UID, 6846);
end

-- [AUTO-GEN] quest=952 status=1 n_index=6846
if (EVENT == 1002) then
	ShowMap(UID, 86);
end

