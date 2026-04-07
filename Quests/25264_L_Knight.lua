local NPC = 25264;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44366, NPC, 3000, 101,13,-1);
end

if(EVENT == 101)then
	DrakiRiftChange(UID, 4, 4);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1544 status=1 n_index=10298
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1544, 44396, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1544, 44396, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1544 status=1 n_index=10298
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1544)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6244);
		SaveEvent(UID, 10299);
	end
end

-- [AUTO-GEN] quest=1544 status=1 n_index=10298
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

