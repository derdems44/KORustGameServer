local NPC = 31556;

if (EVENT == 100) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then 
		SelectMsg(UID, 2, -1, 4703, NPC, 10, -1);
	elseif (QuestNum > 1 and  QuestNum < 100) then 
		NpcMsg(UID, 6501, NPC)
	else
		EVENT = QuestNum
	end
end

if (EVENT == 1001) then
	SelectMsg(UID, 2, 642, 21617, NPC, 10, 1002);
end

if (EVENT == 1002) then
	SelectMsg(UID, 2, 642, 21618, NPC, 3000, 1003,3005,-1);
	SaveEvent(UID, 12504);
end

if (EVENT == 1003) then
	SelectMsg(UID, 4, 642, 21619, NPC, 3000, 1004,3005,-1);
	SaveEvent(UID, 12506);
end

if (EVENT == 1004) then
	SelectMsg(UID, 2, 642, 21620, NPC, 10,-1);
	SaveEvent(UID, 12505);
	SaveEvent(UID, 12516);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=642 status=2 n_index=12505
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 642)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3127);
		SaveEvent(UID, 12507);
	end
end

