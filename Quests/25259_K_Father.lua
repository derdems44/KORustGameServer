local NPC = 25259;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44350, NPC, 4170, 101,4162,-1);
end

if(EVENT == 101)then
	DrakiRiftChange(UID, 2, 4);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1540 status=1 n_index=10290
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1540, 44392, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1540, 44392, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1540 status=1 n_index=10290
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1540)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6240);
		SaveEvent(UID, 10291);
	end
end

-- [AUTO-GEN] quest=1540 status=1 n_index=10290
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

