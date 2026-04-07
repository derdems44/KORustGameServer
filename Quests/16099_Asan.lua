local NPC = 16099;

if (EVENT == 500) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 8070, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 8070, NPC)
	else
		EVENT = QuestNum
	end
end
