local NPC = 31762;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 187, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 187, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=1641 status=255 n_index=10875
if (EVENT == 240) then
	SaveEvent(UID, 10876);
end

-- [AUTO-GEN] quest=1641 status=0 n_index=10876
if (EVENT == 242) then
	SelectMsg(UID, 4, 1641, 0, NPC, 793, 243, 23, -1);
end

-- [AUTO-GEN] quest=1641 status=0 n_index=10876
if (EVENT == 243) then
	SaveEvent(UID, 10877);
end

-- [AUTO-GEN] quest=1641 status=1 n_index=10877
if (EVENT == 245) then
	SelectMsg(UID, 2, 1641, 0, NPC, 10, -1);
end

-- [AUTO-GEN] quest=1641 status=1 n_index=10877
if (EVENT == 246) then
	ShowMap(UID, 36);
end

-- [AUTO-GEN] quest=1641 status=1 n_index=10877
if (EVENT == 247) then
	QuestStatusCheck = GetQuestStatus(UID, 1641)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 10878);
	end
end

