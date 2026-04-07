--SelectMsg(UID,3,-1,101,NPC,40154,202,4262,-1,4554,-1,4475,712,8209,-1,7591,-1,7746,-1,7785,-1,7799,-1,3019,-1);

local NPC = 18005;

if (EVENT == 100) then
Quest1 = GetQuestStatus(UID,1203); --25000
Quest2 = GetQuestStatus(UID,1207); --18005
	if (Quest1 == 1 and Quest2 ~= 2) then
		SelectMsg(UID,3,-1,101,NPC,40154,202,4554,910,8209,920,7591,930,7746,940,7785,950,7799,960,3019,-1);  --,4262,900
	else
		SelectMsg(UID,3,-1,101,NPC,4554,910,8209,920,7591,930,7746,940,7785,950,7799,960,3019,-1); --,4262,900
	end
end

if (EVENT == 202) then
THRUST = GetQuestStatus(UID,1207);
BOOK = HowmuchItem(UID, 900605000);
	if (THRUST == 1 and BOOK == 0) then
		SelectMsg(UID, 2, -1, 43637, NPC,10,-1);
	elseif (THRUST == 3 and BOOK > 0) then
		EVENT = 205
	else
		SelectMsg(UID, 4, 1207, 43636, NPC, 22,203,23,-1);
	end
end

if (EVENT == 203) then
	SaveEvent(UID,7351);
end

if (EVENT == 205) then
THRUST = GetQuestStatus(UID,1207);
BOOK = HowmuchItem(UID, 900605000);
	if (BOOK == 0 and THRUST == 1) then
		SelectMsg(UID, 2, -1, 43637, NPC,10,-1);
	else
		SelectMsg(UID, 4, 1207, 43636, NPC, 40143,206,23,-1);
	end
end

if (EVENT == 206) then
THRUST = GetQuestStatus(UID,1207);
BOOK = HowmuchItem(UID, 900605000);
	if (BOOK == 0 and THRUST == 1) then
		SelectMsg(UID, 2, -1, 43637, NPC,10,-1);
	else 
		RunQuestExchange(UID,6004);
		SaveEvent(UID,7352);
	end
end

if (EVENT == 207) then
	SaveEvent(UID,7353);
end

if (EVENT == 900) then
SelectMsg(UID, 10, -1, -1, NPC);
end
if (EVENT == 910) then
	KingsInspectorList(UID);
end

if (EVENT == 920) then
YEARCA = HowmuchItem(UID, 900580000);
	if (YEARCA == 0) then
		SelectMsg(UID,2,-1,11638,NPC,10,-1)
	else
SlotCheck = CheckGiveSlot(UID, 1)
    if SlotCheck == false then
       
    else	
		RobItem(UID,900580000,1);
		GiveItem(UID,900758945,1,7);
		end
	end
end

if (EVENT == 930) then
WIPCARD = HowmuchItem(UID, 000000);
	if (WIPCARD == 0) then
		SelectMsg(UID,2,-1,10510,NPC,10,-1)
	else
SlotCheck = CheckGiveSlot(UID, 1)
    if SlotCheck == false then
       
    else	
		RobItem(UID,000000,1);
		GiveItem(UID,000000,1);
		end
	end
end

if (EVENT == 940) then
	SelectMsg(UID,2,-1,10650,NPC,10,941)
end

if (EVENT == 941) then
	SelectMsg(UID,2,-1,10651,NPC,3014,942)
end

if (EVENT == 942) then
	SelectMsg(UID,2,-1,10652,NPC,7732,943,13,-1)
end

if (EVENT == 943) then
NOAH = HowmuchItem(UID, 900000000);
	if (NOAH > 9999 or NOAH == 10000) then
		SelectMsg(UID,2,-1,10653,NPC,4170,944)
	else
		SelectMsg(UID,2,-1,10651,NPC,10,-1)
	end
end

if (EVENT == 944) then
	SlotCheck = CheckGiveSlot(UID, 1)
	if SlotCheck then
		GoldLose(UID,10000);
		GiveItem(UID,810181843,1,1);
	end
end

if (EVENT == 950) then
	SelectMsg(UID,2,-1,10841,NPC,10,-1)
end

if (EVENT == 960) then
	SelectMsg(UID,2,-1,10871,NPC,10,-1)
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=954 status=2 n_index=6857
if (EVENT == 101) then
	QuestStatusCheck = GetQuestStatus(UID, 954)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		SaveEvent(UID, 6859);
	end
end

-- [AUTO-GEN] quest=1207 status=255 n_index=7349
if (EVENT == 200) then
	SaveEvent(UID, 7350);
end

-- [AUTO-GEN] quest=124 status=255 n_index=431
if (EVENT == 710) then
	SaveEvent(UID, 434);
end

-- [AUTO-GEN] quest=124 status=255 n_index=432
if (EVENT == 711) then
	SaveEvent(UID, 434);
end

-- [AUTO-GEN] quest=124 status=2 n_index=434
if (EVENT == 712) then
	QuestStatusCheck = GetQuestStatus(UID, 124)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 2);
		SaveEvent(UID, 433);
	end
end

-- [AUTO-GEN] quest=954 status=0 n_index=6855
if (EVENT == 1000) then
	SelectMsg(UID, 4, 954, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=954 status=0 n_index=6855
if (EVENT == 1001) then
	SaveEvent(UID, 6856);
end

-- [AUTO-GEN] quest=954 status=1 n_index=6856
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

