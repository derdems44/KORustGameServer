local NPC = 25265;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44362, NPC, 65, 101, 13, -1);
end

if(EVENT == 101)then
	ZoneChange(UID, 95, 79, 214);
	DrakiRiftChange(UID, 5, 1);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1545 status=1 n_index=10400
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1545, 44397, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1545, 44397, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1545 status=1 n_index=10400
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1545)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6245);
		SaveEvent(UID, 10401);
	end
end

-- [AUTO-GEN] quest=1545 status=1 n_index=10400
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

