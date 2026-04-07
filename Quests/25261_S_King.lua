local NPC = 25261;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44358, NPC, 40483, 101);
end

if(EVENT == 101)then
	DrakiRiftChange(UID, 3, 4);
	DrakiTowerNpcOut(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1542 status=1 n_index=10294
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1542, 44394, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1542, 44394, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1542 status=1 n_index=10294
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1542)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6242);
		SaveEvent(UID, 10295);
	end
end

-- [AUTO-GEN] quest=1542 status=1 n_index=10294
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

