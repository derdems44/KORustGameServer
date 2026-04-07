local NPC = 25178;

if (EVENT == 100) then
		SaveEvent(UID, 855); 
		SelectMsg(UID, 3, -1, 3017, NPC, 8737, 200, 8738, 300, 8739, 400,8740, 500);	
end

if (EVENT == 200) then 
	Cast = CastSkill(UID, 500034);
	
  KillNpcEvent(UID, 25178)
	if (Cast) then
	CastSkill(UID, 500034)

	else
		NpcMsg(UID, 9137);
	end
end
	
if (EVENT == 300) then 
	Cast = CastSkill(UID, 492018);
		
  KillNpcEvent(UID, 25178)
	if (Cast) then
		CastSkill(UID, 492018)
		
		
	else
		NpcMsg(UID, 9137);
	end
end
if (EVENT == 400) then 
	Cast = CastSkill(UID, 510533)
	  KillNpcEvent(UID, 25178)
	if (Cast) then
		CastSkill(UID, 510533)
		
		
	else
		NpcMsg(UID, 9137);
	end
end

if (EVENT == 500) then 
	Cast = CastSkill(UID, 504001);
	  KillNpcEvent(UID, 25178);
	if (Cast) then
		CastSkill(UID, 504001)
		
	else
		NpcMsg(UID, 9137);
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1395 status=255 n_index=4404
if (EVENT == 240) then
	SaveEvent(UID, 4405);
end

-- [AUTO-GEN] quest=1395 status=0 n_index=4405
if (EVENT == 242) then
	SelectMsg(UID, 4, 1395, 0, NPC, 793, 243, 23, -1);
end

-- [AUTO-GEN] quest=1395 status=0 n_index=4405
if (EVENT == 243) then
	SaveEvent(UID, 4406);
end

-- [AUTO-GEN] quest=1395 status=1 n_index=4406
if (EVENT == 245) then
	SelectMsg(UID, 2, 1395, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=1395 status=1 n_index=4406
if (EVENT == 246) then
	ShowMap(UID, 21);
end

-- [AUTO-GEN] quest=1395 status=1 n_index=4406
if (EVENT == 247) then
	QuestStatusCheck = GetQuestStatus(UID, 1395)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 4407);
	end
end

