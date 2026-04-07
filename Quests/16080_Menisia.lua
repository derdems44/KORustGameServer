local NPC = 16080;

if (EVENT == 193) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 323, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 323, NPC)
	else
		EVENT = QuestNum
	end
end
