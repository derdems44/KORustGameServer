local NPC = 25258;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44346, NPC, 65, 101, 13, -1);
end

if(EVENT == 101)then
	ZoneChange(UID, 95, 78, 58);
	DrakiRiftChange(UID, 2, 1);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1539 status=1 n_index=10288
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1539, 44391, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1539, 44391, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1539 status=1 n_index=10288
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1539)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6239);
		SaveEvent(UID, 10289);
	end
end

-- [AUTO-GEN] quest=1539 status=1 n_index=10288
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

