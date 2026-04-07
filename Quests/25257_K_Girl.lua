local NPC = 25257;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 12401, NPC, 65, 101);
end

if(EVENT == 101)then
	DrakiRiftChange(UID, 1, 4);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1538 status=1 n_index=10286
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1538, 44390, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1538, 44390, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1538 status=1 n_index=10286
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1538)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6238);
		SaveEvent(UID, 10287);
	end
end

-- [AUTO-GEN] quest=1538 status=1 n_index=10286
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

