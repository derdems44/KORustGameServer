local NPC = 31571;

if (EVENT == 0) then
	QuestNum = SearchQuest(UID, NPC);
	if (QuestNum == 0) then
		SelectMsg(UID, 2, -1, 22146, NPC, 10, -1);
	elseif (QuestNum > 1 and QuestNum < 100) then
		NpcMsg(UID, 22146, NPC)
	else
		EVENT = QuestNum
	end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=0 status=0 n_index=5027
if (EVENT == 100) then
	SearchQuest(UID, 31571);
end

-- [AUTO-GEN] quest=686 status=2 n_index=12914
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 686)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3161);
		SaveEvent(UID, 12916);
	end
end

-- [AUTO-GEN] quest=686 status=0 n_index=12912
if (EVENT == 1001) then
	SelectMsg(UID, 4, 686, 21332, NPC, 3308, 1002, 23, -1);
end

-- [AUTO-GEN] quest=686 status=0 n_index=12912
if (EVENT == 1002) then
	SaveEvent(UID, 12913);
end

-- [AUTO-GEN] quest=686 status=1 n_index=12913
if (EVENT == 1003) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 686, 21332, NPC, 18, 1004);
	else
		SelectMsg(UID, 4, 686, 21332, NPC, 41, 1006, 27, -1);
	end
end

-- [AUTO-GEN] quest=686 status=1 n_index=12913
if (EVENT == 1004) then
	ShowMap(UID, 12);
end

-- [AUTO-GEN] quest=686 status=1 n_index=12913
if (EVENT == 1006) then
	QuestStatusCheck = GetQuestStatus(UID, 686)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3161);
		SaveEvent(UID, 12914);
	end
end

-- [AUTO-GEN] quest=687 status=0 n_index=12924
if (EVENT == 1101) then
	SelectMsg(UID, 4, 687, 21334, NPC, 3310, 1102, 23, -1);
end

-- [AUTO-GEN] quest=687 status=0 n_index=12924
if (EVENT == 1102) then
	SaveEvent(UID, 12925);
end

-- [AUTO-GEN] quest=687 status=1 n_index=12925
if (EVENT == 1105) then
	ItemA = HowmuchItem(UID, 379061000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 687, 21334, NPC, 18, 1106);
	else
		SelectMsg(UID, 4, 687, 21334, NPC, 41, 1106, 27, -1);
	end
end

-- [AUTO-GEN] quest=687 status=1 n_index=12925
if (EVENT == 1106) then
	QuestStatusCheck = GetQuestStatus(UID, 687)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3162);
		SaveEvent(UID, 12926);
	end
end

-- [AUTO-GEN] quest=688 status=0 n_index=12936
if (EVENT == 1201) then
	SelectMsg(UID, 4, 688, 21336, NPC, 3312, 1202, 23, -1);
end

-- [AUTO-GEN] quest=688 status=0 n_index=12936
if (EVENT == 1202) then
	SaveEvent(UID, 12937);
end

-- [AUTO-GEN] quest=688 status=1 n_index=12937
if (EVENT == 1205) then
	ItemA = HowmuchItem(UID, 389480000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 688, 21336, NPC, 18, 1206);
	else
		SelectMsg(UID, 4, 688, 21336, NPC, 41, 1206, 27, -1);
	end
end

-- [AUTO-GEN] quest=688 status=1 n_index=12937
if (EVENT == 1206) then
	QuestStatusCheck = GetQuestStatus(UID, 688)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3163);
		SaveEvent(UID, 12938);
	end
end

-- [AUTO-GEN] quest=689 status=0 n_index=12948
if (EVENT == 1301) then
	SelectMsg(UID, 4, 689, 21338, NPC, 3314, 1302, 23, -1);
end

-- [AUTO-GEN] quest=689 status=0 n_index=12948
if (EVENT == 1302) then
	SaveEvent(UID, 12949);
end

-- [AUTO-GEN] quest=689 status=1 n_index=12949
if (EVENT == 1305) then
	ItemA = HowmuchItem(UID, 389470000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 689, 21338, NPC, 18, 1306);
	else
		SelectMsg(UID, 4, 689, 21338, NPC, 41, 1306, 27, -1);
	end
end

-- [AUTO-GEN] quest=689 status=1 n_index=12949
if (EVENT == 1306) then
	QuestStatusCheck = GetQuestStatus(UID, 689)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3164);
		SaveEvent(UID, 12950);
	end
end

-- [AUTO-GEN] quest=690 status=0 n_index=12960
if (EVENT == 1401) then
	SelectMsg(UID, 4, 690, 21340, NPC, 3316, 1402, 23, -1);
end

-- [AUTO-GEN] quest=690 status=0 n_index=12960
if (EVENT == 1402) then
	SaveEvent(UID, 12961);
end

-- [AUTO-GEN] quest=690 status=1 n_index=12961
if (EVENT == 1405) then
	ItemA = HowmuchItem(UID, 389750000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 690, 21340, NPC, 18, 1406);
	else
		SelectMsg(UID, 4, 690, 21340, NPC, 41, 1406, 27, -1);
	end
end

-- [AUTO-GEN] quest=690 status=1 n_index=12961
if (EVENT == 1406) then
	QuestStatusCheck = GetQuestStatus(UID, 690)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3165);
		SaveEvent(UID, 12962);
	end
end

-- [AUTO-GEN] quest=691 status=0 n_index=12972
if (EVENT == 1501) then
	SelectMsg(UID, 4, 691, 21342, NPC, 3318, 1502, 23, -1);
end

-- [AUTO-GEN] quest=691 status=0 n_index=12972
if (EVENT == 1502) then
	SaveEvent(UID, 12973);
end

-- [AUTO-GEN] quest=691 status=1 n_index=12973
if (EVENT == 1505) then
	ItemA = HowmuchItem(UID, 389560000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 691, 21342, NPC, 18, 1506);
	else
		SelectMsg(UID, 4, 691, 21342, NPC, 41, 1506, 27, -1);
	end
end

-- [AUTO-GEN] quest=691 status=1 n_index=12973
if (EVENT == 1506) then
	QuestStatusCheck = GetQuestStatus(UID, 691)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 3166);
		SaveEvent(UID, 12974);
	end
end

