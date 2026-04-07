

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1321 status=2 n_index=3637
if (EVENT == 100) then
	SearchQuest(UID, 25171);
end

-- [AUTO-GEN] quest=1358 status=255 n_index=3856
if (EVENT == 1120) then
	SaveEvent(UID, 3857);
end

-- [AUTO-GEN] quest=1358 status=0 n_index=3857
if (EVENT == 1122) then
	SelectMsg(UID, 4, 1358, 44146, NPC, 774, 1123, 23, -1);
end

-- [AUTO-GEN] quest=1358 status=0 n_index=3857
if (EVENT == 1123) then
	SaveEvent(UID, 3858);
end

-- [AUTO-GEN] quest=1358 status=1 n_index=3858
if (EVENT == 1125) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1358, 44146, NPC, 22, 1127, 23, -1);
	else
		SelectMsg(UID, 2, 1358, 44146, NPC, 18, 1126);
	end
end

-- [AUTO-GEN] quest=1358 status=1 n_index=3858
if (EVENT == 1126) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1358 status=1 n_index=3858
if (EVENT == 1127) then
	QuestStatusCheck = GetQuestStatus(UID, 1358)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6152);
		SaveEvent(UID, 3859);
	end
end

-- [AUTO-GEN] quest=1321 status=255 n_index=3634
if (EVENT == 1210) then
	SaveEvent(UID, 3635);
end

-- [AUTO-GEN] quest=1321 status=0 n_index=3635
if (EVENT == 1212) then
	SelectMsg(UID, 4, 1321, 43860, NPC, 724, 1213, 23, -1);
end

-- [AUTO-GEN] quest=1321 status=0 n_index=3635
if (EVENT == 1213) then
	SaveEvent(UID, 3636);
end

-- [AUTO-GEN] quest=1321 status=1 n_index=3636
if (EVENT == 1215) then
	ItemA = HowmuchItem(UID, 900628000);
	if (ItemA < 1) then
		SelectMsg(UID, 2, 1321, 43860, NPC, 18, 1216);
	else
		SelectMsg(UID, 4, 1321, 43860, NPC, 41, 1217, 27, -1);
	end
end

-- [AUTO-GEN] quest=1321 status=1 n_index=3636
if (EVENT == 1216) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1321 status=1 n_index=3636
if (EVENT == 1217) then
	QuestStatusCheck = GetQuestStatus(UID, 1321)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6115);
		SaveEvent(UID, 3637);
	end
end

-- [AUTO-GEN] quest=1322 status=255 n_index=3640
if (EVENT == 1220) then
	SaveEvent(UID, 3641);
end

-- [AUTO-GEN] quest=1322 status=0 n_index=3641
if (EVENT == 1222) then
	SelectMsg(UID, 4, 1322, 43874, NPC, 725, 1223, 23, -1);
end

-- [AUTO-GEN] quest=1322 status=0 n_index=3641
if (EVENT == 1223) then
	SaveEvent(UID, 3642);
end

-- [AUTO-GEN] quest=1322 status=1 n_index=3642
if (EVENT == 1225) then
	ItemA = HowmuchItem(UID, 900616000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1322, 43874, NPC, 18, 1226);
	else
		SelectMsg(UID, 4, 1322, 43874, NPC, 41, 1227, 27, -1);
	end
end

-- [AUTO-GEN] quest=1322 status=1 n_index=3642
if (EVENT == 1226) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1322 status=1 n_index=3642
if (EVENT == 1227) then
	QuestStatusCheck = GetQuestStatus(UID, 1322)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6116);
		SaveEvent(UID, 3643);
	end
end

-- [AUTO-GEN] quest=1323 status=255 n_index=3646
if (EVENT == 1230) then
	SaveEvent(UID, 3647);
end

-- [AUTO-GEN] quest=1323 status=0 n_index=3647
if (EVENT == 1232) then
	SelectMsg(UID, 4, 1323, 43883, NPC, 726, 1233, 23, -1);
end

-- [AUTO-GEN] quest=1323 status=0 n_index=3647
if (EVENT == 1233) then
	SaveEvent(UID, 3648);
end

-- [AUTO-GEN] quest=1323 status=1 n_index=3648
if (EVENT == 1235) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1323, 43883, NPC, 22, 1237, 23, -1);
	else
		SelectMsg(UID, 2, 1323, 43883, NPC, 18, 1236);
	end
end

-- [AUTO-GEN] quest=1323 status=1 n_index=3648
if (EVENT == 1236) then
	ShowMap(UID, 1);
end

-- [AUTO-GEN] quest=1323 status=1 n_index=3648
if (EVENT == 1237) then
	QuestStatusCheck = GetQuestStatus(UID, 1323)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6117);
		SaveEvent(UID, 3649);
	end
end

