local NPC = 29049;

if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 9464, 29049, 7176,1001,7177,1101,7178,1201);
end

if(EVENT == 1001) then 
SelectMsg(UID, 2, -1, 9465, NPC, 10,1002);
end

if(EVENT == 1002) then 
SelectMsg(UID, 2, -1, 9469, NPC, 7179,1003,7180,1004);
end

if(EVENT == 1003) then 
STAMP = HowmuchItem(UID, 508213000)
CUBE = HowmuchItem(UID, 508211000)
GOLDP = HowmuchItem(UID, 508212000)
if(STAMP > 29 and CUBE > 0 and GOLDP > 0) then
SelectMsg(UID, 5, 808, 9492, NPC,3000,1005,3005,-1)
else
SelectMsg(UID, 2, -1, 9492, NPC, 10,-1);
end
end


if(EVENT == 1004) then 
STAMP = HowmuchItem(UID, 508213000)
CUBE = HowmuchItem(UID, 508211000)
GOLDP = HowmuchItem(UID, 508212000)
if(STAMP > 29 and CUBE > 0 and GOLDP > 0) then
SelectMsg(UID, 5, 809, 9492, NPC,3000,1006,3005,-1)
else
SelectMsg(UID, 2, -1, 9492, NPC, 10,-1);
end
end

if(EVENT == 1005) then
RunQuestExchange(UID,1221,STEP,1);
RobItem(UID, 508213000, 30);
RobItem(UID, 508211000, 1);
RobItem(UID, 508212000, 1);
end

if(EVENT == 1006) then
RunQuestExchange(UID,1222,STEP,1);
RobItem(UID, 508213000, 30);
RobItem(UID, 508211000, 1);
RobItem(UID, 508212000, 1);
end

if(EVENT == 1101) then 
	QuestStatusCheck = GetQuestStatus(UID, 810)	
	if(QuestStatusCheck == 1) then
		EVENT = 1106
		else
	QuestStatusCheck = GetQuestStatus(UID, 810)	
	if(QuestStatusCheck == 3) then
		EVENT = 1108
		else
SelectMsg(UID, 2, 810, 9471, NPC, 10,1102);
end
end
end

if(EVENT == 1102) then 
SelectMsg(UID, 4, 810, 9493, NPC,3000,1107,3005,-1);
end

if(EVENT == 1103) then 
SaveEvent(UID, 2713);
end

if(EVENT == 1107) then 
SaveEvent(UID, 2711);
end

if(EVENT == 1106) then 
SelectMsg(UID, 2, 810, 9471, NPC, 18,1109);
end

if (EVENT == 1109) then
	ShowMap(UID, 1011);
end

if(EVENT == 1108) then 
SelectMsg(UID, 4, 810, 9493, NPC,3000,1110,3005,-1);
end

if (EVENT == 1110) then
	RunQuestExchange(UID,1223)
	SaveEvent(UID, 2714);
end

if(EVENT == 1201) then 
ZoneChange(UID, 35, 464, 116);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=808 status=2 n_index=2702
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 808)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 1221);
		SaveEvent(UID, 2704);
	end
end

-- [AUTO-GEN] quest=810 status=1 n_index=2711
if (EVENT == 1104) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 810, 9493, NPC, 22, 1103, 23, -1);
	else
		SelectMsg(UID, 2, 810, 9493, NPC, 18, 1105);
	end
end

-- [AUTO-GEN] quest=810 status=1 n_index=2711
if (EVENT == 1105) then
	ShowMap(UID, 35);
end

