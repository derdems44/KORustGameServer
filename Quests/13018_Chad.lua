local NPC = 13018;

if (EVENT == 250) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 9124, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 9124, NPC)
	else
		EVENT = QuestNum
	end
end
