local NPC = 25266;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 44398, NPC, 40497, 101);
end

if(EVENT == 101)then
	DrakiOutZone(UID);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1546 status=1 n_index=10402
if (EVENT == 5000) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1546, 44398, NPC, 18, 5002);
	else
		SelectMsg(UID, 4, 1546, 44398, NPC, 41, 5001, 27, -1);
	end
end

-- [AUTO-GEN] quest=1546 status=1 n_index=10402
if (EVENT == 5001) then
	QuestStatusCheck = GetQuestStatus(UID, 1546)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6246);
		SaveEvent(UID, 10403);
	end
end

-- [AUTO-GEN] quest=1546 status=1 n_index=10402
if (EVENT == 5002) then
	ShowMap(UID, 95);
end

