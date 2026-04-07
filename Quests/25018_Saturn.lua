local NPC = 25018;

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

-- [AUTO-GEN] quest=1326 status=255 n_index=3664
if (EVENT == 1110) then
	SaveEvent(UID, 3665);
end

-- [AUTO-GEN] quest=1326 status=0 n_index=3665
if (EVENT == 1112) then
	SelectMsg(UID, 4, 1326, 44114, NPC, 734, 1113, 23, -1);
end

-- [AUTO-GEN] quest=1326 status=0 n_index=3665
if (EVENT == 1113) then
	SaveEvent(UID, 3666);
end

-- [AUTO-GEN] quest=1326 status=1 n_index=3666
if (EVENT == 1115) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1326, 44114, NPC, 18, 1116);
	else
		SelectMsg(UID, 4, 1326, 44114, NPC, 41, 1117, 27, -1);
	end
end

-- [AUTO-GEN] quest=1326 status=1 n_index=3666
if (EVENT == 1116) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1326 status=1 n_index=3666
if (EVENT == 1117) then
	QuestStatusCheck = GetQuestStatus(UID, 1326)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6120);
		SaveEvent(UID, 3667);
	end
end

-- [AUTO-GEN] quest=1327 status=255 n_index=3670
if (EVENT == 1120) then
	SaveEvent(UID, 3671);
end

-- [AUTO-GEN] quest=1327 status=0 n_index=3671
if (EVENT == 1122) then
	SelectMsg(UID, 4, 1327, 44115, NPC, 735, 1123, 23, -1);
end

-- [AUTO-GEN] quest=1327 status=0 n_index=3671
if (EVENT == 1123) then
	SaveEvent(UID, 3672);
end

-- [AUTO-GEN] quest=1327 status=1 n_index=3672
if (EVENT == 1125) then
	ItemA = HowmuchItem(UID, 900679000);
	if (ItemA < 1) then
		SelectMsg(UID, 2, 1327, 44115, NPC, 18, 1126);
	else
		SelectMsg(UID, 4, 1327, 44115, NPC, 41, 1127, 27, -1);
	end
end

-- [AUTO-GEN] quest=1327 status=1 n_index=3672
if (EVENT == 1126) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1327 status=1 n_index=3672
if (EVENT == 1127) then
	QuestStatusCheck = GetQuestStatus(UID, 1327)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6121);
		SaveEvent(UID, 3673);
	end
end

-- [AUTO-GEN] quest=1328 status=255 n_index=3676
if (EVENT == 1130) then
	SaveEvent(UID, 3677);
end

-- [AUTO-GEN] quest=1328 status=0 n_index=3677
if (EVENT == 1132) then
	SelectMsg(UID, 4, 1328, 44116, NPC, 736, 1133, 23, -1);
end

-- [AUTO-GEN] quest=1328 status=0 n_index=3677
if (EVENT == 1133) then
	SaveEvent(UID, 3678);
end

-- [AUTO-GEN] quest=1328 status=1 n_index=3678
if (EVENT == 1135) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1328, 44116, NPC, 22, 1137, 23, -1);
	else
		SelectMsg(UID, 2, 1328, 44116, NPC, 18, 1136);
	end
end

-- [AUTO-GEN] quest=1328 status=1 n_index=3678
if (EVENT == 1136) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1328 status=1 n_index=3678
if (EVENT == 1137) then
	QuestStatusCheck = GetQuestStatus(UID, 1328)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6122);
		SaveEvent(UID, 3679);
	end
end

-- [AUTO-GEN] quest=1329 status=255 n_index=3682
if (EVENT == 1140) then
	SaveEvent(UID, 3683);
end

-- [AUTO-GEN] quest=1329 status=0 n_index=3683
if (EVENT == 1142) then
	SelectMsg(UID, 4, 1329, 44117, NPC, 737, 1143, 23, -1);
end

-- [AUTO-GEN] quest=1329 status=0 n_index=3683
if (EVENT == 1143) then
	SaveEvent(UID, 3684);
end

-- [AUTO-GEN] quest=1329 status=1 n_index=3684
if (EVENT == 1145) then
	ItemA = HowmuchItem(UID, 900691000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1329, 44117, NPC, 18, 1146);
	else
		SelectMsg(UID, 4, 1329, 44117, NPC, 41, 1147, 27, -1);
	end
end

-- [AUTO-GEN] quest=1329 status=1 n_index=3684
if (EVENT == 1146) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1329 status=1 n_index=3684
if (EVENT == 1147) then
	QuestStatusCheck = GetQuestStatus(UID, 1329)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6123);
		SaveEvent(UID, 3685);
	end
end

-- [AUTO-GEN] quest=1330 status=255 n_index=3688
if (EVENT == 1150) then
	SaveEvent(UID, 3689);
end

-- [AUTO-GEN] quest=1330 status=0 n_index=3689
if (EVENT == 1152) then
	SelectMsg(UID, 4, 1330, 44118, NPC, 738, 1153, 23, -1);
end

-- [AUTO-GEN] quest=1330 status=0 n_index=3689
if (EVENT == 1153) then
	SaveEvent(UID, 3690);
end

-- [AUTO-GEN] quest=1330 status=1 n_index=3690
if (EVENT == 1155) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1330, 44118, NPC, 22, 1157, 23, -1);
	else
		SelectMsg(UID, 2, 1330, 44118, NPC, 18, 1156);
	end
end

-- [AUTO-GEN] quest=1330 status=1 n_index=3690
if (EVENT == 1156) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1330 status=1 n_index=3690
if (EVENT == 1157) then
	QuestStatusCheck = GetQuestStatus(UID, 1330)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6124);
		SaveEvent(UID, 3691);
	end
end

-- [AUTO-GEN] quest=1331 status=255 n_index=3694
if (EVENT == 1160) then
	SaveEvent(UID, 3695);
end

-- [AUTO-GEN] quest=1331 status=0 n_index=3695
if (EVENT == 1162) then
	SelectMsg(UID, 4, 1331, 44119, NPC, 739, 1163, 23, -1);
end

-- [AUTO-GEN] quest=1331 status=0 n_index=3695
if (EVENT == 1163) then
	SaveEvent(UID, 3696);
end

-- [AUTO-GEN] quest=1331 status=1 n_index=3696
if (EVENT == 1165) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1331, 44119, NPC, 22, 1167, 23, -1);
	else
		SelectMsg(UID, 2, 1331, 44119, NPC, 18, 1166);
	end
end

-- [AUTO-GEN] quest=1331 status=1 n_index=3696
if (EVENT == 1166) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1331 status=1 n_index=3696
if (EVENT == 1167) then
	QuestStatusCheck = GetQuestStatus(UID, 1331)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6125);
		SaveEvent(UID, 3697);
	end
end

-- [AUTO-GEN] quest=1332 status=255 n_index=3700
if (EVENT == 1170) then
	SaveEvent(UID, 3701);
end

-- [AUTO-GEN] quest=1332 status=0 n_index=3701
if (EVENT == 1172) then
	SelectMsg(UID, 4, 1332, 44120, NPC, 740, 1173, 23, -1);
end

-- [AUTO-GEN] quest=1332 status=0 n_index=3701
if (EVENT == 1173) then
	SaveEvent(UID, 3702);
end

-- [AUTO-GEN] quest=1332 status=1 n_index=3702
if (EVENT == 1175) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1332, 44120, NPC, 22, 1177, 23, -1);
	else
		SelectMsg(UID, 2, 1332, 44120, NPC, 18, 1176);
	end
end

-- [AUTO-GEN] quest=1332 status=1 n_index=3702
if (EVENT == 1176) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1332 status=1 n_index=3702
if (EVENT == 1177) then
	QuestStatusCheck = GetQuestStatus(UID, 1332)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6126);
		SaveEvent(UID, 3703);
	end
end

-- [AUTO-GEN] quest=1333 status=255 n_index=3706
if (EVENT == 1180) then
	SaveEvent(UID, 3707);
end

-- [AUTO-GEN] quest=1333 status=0 n_index=3707
if (EVENT == 1182) then
	SelectMsg(UID, 4, 1333, 44121, NPC, 741, 1183, 23, -1);
end

-- [AUTO-GEN] quest=1333 status=0 n_index=3707
if (EVENT == 1183) then
	SaveEvent(UID, 3708);
end

-- [AUTO-GEN] quest=1333 status=1 n_index=3708
if (EVENT == 1185) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1333, 44121, NPC, 22, 1187, 23, -1);
	else
		SelectMsg(UID, 2, 1333, 44121, NPC, 18, 1186);
	end
end

-- [AUTO-GEN] quest=1333 status=1 n_index=3708
if (EVENT == 1186) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1333 status=1 n_index=3708
if (EVENT == 1187) then
	QuestStatusCheck = GetQuestStatus(UID, 1333)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6127);
		SaveEvent(UID, 3709);
	end
end

-- [AUTO-GEN] quest=1334 status=255 n_index=3712
if (EVENT == 1190) then
	SaveEvent(UID, 3713);
end

-- [AUTO-GEN] quest=1334 status=0 n_index=3713
if (EVENT == 1192) then
	SelectMsg(UID, 4, 1334, 44122, NPC, 742, 1193, 23, -1);
end

-- [AUTO-GEN] quest=1334 status=0 n_index=3713
if (EVENT == 1193) then
	SaveEvent(UID, 3714);
end

-- [AUTO-GEN] quest=1334 status=1 n_index=3714
if (EVENT == 1195) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1334, 44122, NPC, 22, 1197, 23, -1);
	else
		SelectMsg(UID, 2, 1334, 44122, NPC, 18, 1196);
	end
end

-- [AUTO-GEN] quest=1334 status=1 n_index=3714
if (EVENT == 1196) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1334 status=1 n_index=3714
if (EVENT == 1197) then
	QuestStatusCheck = GetQuestStatus(UID, 1334)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6128);
		SaveEvent(UID, 3715);
	end
end

-- [AUTO-GEN] quest=1335 status=255 n_index=3718
if (EVENT == 1200) then
	SaveEvent(UID, 3719);
end

-- [AUTO-GEN] quest=1335 status=0 n_index=3719
if (EVENT == 1202) then
	SelectMsg(UID, 4, 1335, 44123, NPC, 743, 1203, 23, -1);
end

-- [AUTO-GEN] quest=1335 status=0 n_index=3719
if (EVENT == 1203) then
	SaveEvent(UID, 3720);
end

-- [AUTO-GEN] quest=1335 status=1 n_index=3720
if (EVENT == 1205) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1335, 44123, NPC, 18, 1206);
	else
		SelectMsg(UID, 4, 1335, 44123, NPC, 41, 1207, 27, -1);
	end
end

-- [AUTO-GEN] quest=1335 status=1 n_index=3720
if (EVENT == 1206) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1335 status=1 n_index=3720
if (EVENT == 1207) then
	QuestStatusCheck = GetQuestStatus(UID, 1335)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6129);
		SaveEvent(UID, 3721);
	end
end

