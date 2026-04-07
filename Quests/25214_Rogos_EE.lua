local NPC = 25214;

if (EVENT == 100) then
		SaveEvent(UID, 855); 
		SelectMsg(UID, 3, -1, 3017, NPC, 8737, 200, 8738, 300, 8739, 400,8740, 500);	
end

if (EVENT == 200) then 
	Cast = CastSkill(UID, 500034);
	
  KillNpcEvent(UID, 25214)
	if (Cast) then
	CastSkill(UID, 500034)

	else
		NpcMsg(UID, 9137);
	end
end
	
if (EVENT == 300) then 
	Cast = CastSkill(UID, 492018);
		
  KillNpcEvent(UID, 25214)
	if (Cast) then
		CastSkill(UID, 492018)
		
		
	else
		NpcMsg(UID, 9137);
	end
end
if (EVENT == 400) then 
	Cast = CastSkill(UID, 510533)
	  KillNpcEvent(UID, 25214)
	if (Cast) then
		CastSkill(UID, 510533)
		
		
	else
		NpcMsg(UID, 9137);
	end
end

if (EVENT == 500) then 
	Cast = CastSkill(UID, 504001);
	  KillNpcEvent(UID, 25214);
	if (Cast) then
		CastSkill(UID, 504001)
		
	else
		NpcMsg(UID, 9137);
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1431 status=255 n_index=4856
if (EVENT == 240) then
	SaveEvent(UID, 4857);
end

-- [AUTO-GEN] quest=1431 status=0 n_index=4857
if (EVENT == 242) then
	SelectMsg(UID, 4, 1431, 0, NPC, 793, 243, 23, -1);
end

-- [AUTO-GEN] quest=1431 status=0 n_index=4857
if (EVENT == 243) then
	SaveEvent(UID, 4858);
end

-- [AUTO-GEN] quest=1431 status=1 n_index=4858
if (EVENT == 245) then
	SelectMsg(UID, 2, 1431, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=1431 status=1 n_index=4858
if (EVENT == 246) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=1431 status=1 n_index=4858
if (EVENT == 247) then
	QuestStatusCheck = GetQuestStatus(UID, 1431)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 4859);
	end
end

