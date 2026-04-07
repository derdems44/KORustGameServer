

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=0 status=0 n_index=5699
if (EVENT == 100) then
	SearchQuest(UID, 25146);
end

-- [AUTO-GEN] quest=1298 status=255 n_index=7980
if (EVENT == 120) then
	SaveEvent(UID, 7981);
end

-- [AUTO-GEN] quest=1298 status=0 n_index=7981
if (EVENT == 122) then
	SelectMsg(UID, 4, 1298, 44146, NPC, 774, 123, 23, -1);
end

-- [AUTO-GEN] quest=1298 status=0 n_index=7981
if (EVENT == 123) then
	SaveEvent(UID, 7982);
end

-- [AUTO-GEN] quest=1298 status=1 n_index=7982
if (EVENT == 125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1298, 44146, NPC, 22, 127, 23, -1);
	else
		SelectMsg(UID, 2, 1298, 44146, NPC, 18, 126);
	end
end

-- [AUTO-GEN] quest=1298 status=1 n_index=7982
if (EVENT == 126) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1298 status=1 n_index=7982
if (EVENT == 127) then
	QuestStatusCheck = GetQuestStatus(UID, 1298)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6097);
		SaveEvent(UID, 7983);
	end
end

-- [AUTO-GEN] quest=1252 status=255 n_index=7618
if (EVENT == 210) then
	SaveEvent(UID, 7619);
end

-- [AUTO-GEN] quest=1252 status=0 n_index=7619
if (EVENT == 212) then
	SelectMsg(UID, 4, 1252, 43860, NPC, 724, 213, 23, -1);
end

-- [AUTO-GEN] quest=1252 status=0 n_index=7619
if (EVENT == 213) then
	SaveEvent(UID, 7620);
end

-- [AUTO-GEN] quest=1252 status=1 n_index=7620
if (EVENT == 215) then
	ItemA = HowmuchItem(UID, 900628000);
	if (ItemA < 1) then
		SelectMsg(UID, 2, 1252, 43860, NPC, 18, 216);
	else
		SelectMsg(UID, 4, 1252, 43860, NPC, 41, 217, 27, -1);
	end
end

-- [AUTO-GEN] quest=1252 status=1 n_index=7620
if (EVENT == 216) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1252 status=1 n_index=7620
if (EVENT == 217) then
	QuestStatusCheck = GetQuestStatus(UID, 1252)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6048);
		SaveEvent(UID, 7621);
	end
end

-- [AUTO-GEN] quest=1253 status=255 n_index=7624
if (EVENT == 220) then
	SaveEvent(UID, 7625);
end

-- [AUTO-GEN] quest=1253 status=0 n_index=7625
if (EVENT == 222) then
	SelectMsg(UID, 4, 1253, 43874, NPC, 725, 223, 23, -1);
end

-- [AUTO-GEN] quest=1253 status=0 n_index=7625
if (EVENT == 223) then
	SaveEvent(UID, 7626);
end

-- [AUTO-GEN] quest=1253 status=1 n_index=7626
if (EVENT == 225) then
	ItemA = HowmuchItem(UID, 900616000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1253, 43874, NPC, 18, 226);
	else
		SelectMsg(UID, 4, 1253, 43874, NPC, 41, 227, 27, -1);
	end
end

-- [AUTO-GEN] quest=1253 status=1 n_index=7626
if (EVENT == 226) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1253 status=1 n_index=7626
if (EVENT == 227) then
	QuestStatusCheck = GetQuestStatus(UID, 1253)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6049);
		SaveEvent(UID, 7627);
	end
end

-- [AUTO-GEN] quest=1254 status=255 n_index=7630
if (EVENT == 230) then
	SaveEvent(UID, 7631);
end

-- [AUTO-GEN] quest=1254 status=0 n_index=7631
if (EVENT == 232) then
	SelectMsg(UID, 4, 1254, 43883, NPC, 726, 233, 23, -1);
end

-- [AUTO-GEN] quest=1254 status=0 n_index=7631
if (EVENT == 233) then
	SaveEvent(UID, 7632);
end

-- [AUTO-GEN] quest=1254 status=1 n_index=7632
if (EVENT == 235) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1254, 43883, NPC, 22, 237, 23, -1);
	else
		SelectMsg(UID, 2, 1254, 43883, NPC, 18, 236);
	end
end

-- [AUTO-GEN] quest=1254 status=1 n_index=7632
if (EVENT == 236) then
	ShowMap(UID, 2);
end

-- [AUTO-GEN] quest=1254 status=1 n_index=7632
if (EVENT == 237) then
	QuestStatusCheck = GetQuestStatus(UID, 1254)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6050);
		SaveEvent(UID, 7633);
	end
end

