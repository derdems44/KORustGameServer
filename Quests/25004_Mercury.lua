local NPC = 25004;

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

-- [AUTO-GEN] quest=1264 status=255 n_index=7690
if (EVENT == 110) then
	SaveEvent(UID, 7691);
end

-- [AUTO-GEN] quest=1264 status=0 n_index=7691
if (EVENT == 112) then
	SelectMsg(UID, 4, 1264, 44114, NPC, 734, 113, 23, -1);
end

-- [AUTO-GEN] quest=1264 status=0 n_index=7691
if (EVENT == 113) then
	SaveEvent(UID, 7692);
end

-- [AUTO-GEN] quest=1264 status=1 n_index=7692
if (EVENT == 115) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1264, 44114, NPC, 18, 116);
	else
		SelectMsg(UID, 4, 1264, 44114, NPC, 41, 117, 27, -1);
	end
end

-- [AUTO-GEN] quest=1264 status=1 n_index=7692
if (EVENT == 116) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1264 status=1 n_index=7692
if (EVENT == 117) then
	QuestStatusCheck = GetQuestStatus(UID, 1264)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6060);
		SaveEvent(UID, 7693);
	end
end

-- [AUTO-GEN] quest=1265 status=255 n_index=7696
if (EVENT == 120) then
	SaveEvent(UID, 7697);
end

-- [AUTO-GEN] quest=1265 status=0 n_index=7697
if (EVENT == 122) then
	SelectMsg(UID, 4, 1265, 44115, NPC, 735, 123, 23, -1);
end

-- [AUTO-GEN] quest=1265 status=0 n_index=7697
if (EVENT == 123) then
	SaveEvent(UID, 7698);
end

-- [AUTO-GEN] quest=1265 status=1 n_index=7698
if (EVENT == 125) then
	ItemA = HowmuchItem(UID, 900679000);
	if (ItemA < 1) then
		SelectMsg(UID, 2, 1265, 44115, NPC, 18, 126);
	else
		SelectMsg(UID, 4, 1265, 44115, NPC, 41, 127, 27, -1);
	end
end

-- [AUTO-GEN] quest=1265 status=1 n_index=7698
if (EVENT == 126) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1265 status=1 n_index=7698
if (EVENT == 127) then
	QuestStatusCheck = GetQuestStatus(UID, 1265)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6061);
		SaveEvent(UID, 7699);
	end
end

-- [AUTO-GEN] quest=1266 status=255 n_index=7702
if (EVENT == 130) then
	SaveEvent(UID, 7703);
end

-- [AUTO-GEN] quest=1266 status=0 n_index=7703
if (EVENT == 132) then
	SelectMsg(UID, 4, 1266, 44116, NPC, 736, 133, 23, -1);
end

-- [AUTO-GEN] quest=1266 status=0 n_index=7703
if (EVENT == 133) then
	SaveEvent(UID, 7704);
end

-- [AUTO-GEN] quest=1266 status=1 n_index=7704
if (EVENT == 135) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1266, 44116, NPC, 22, 137, 23, -1);
	else
		SelectMsg(UID, 2, 1266, 44116, NPC, 18, 136);
	end
end

-- [AUTO-GEN] quest=1266 status=1 n_index=7704
if (EVENT == 136) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1266 status=1 n_index=7704
if (EVENT == 137) then
	QuestStatusCheck = GetQuestStatus(UID, 1266)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6062);
		SaveEvent(UID, 7705);
	end
end

-- [AUTO-GEN] quest=1267 status=255 n_index=7708
if (EVENT == 140) then
	SaveEvent(UID, 7709);
end

-- [AUTO-GEN] quest=1267 status=0 n_index=7709
if (EVENT == 142) then
	SelectMsg(UID, 4, 1267, 44117, NPC, 737, 143, 23, -1);
end

-- [AUTO-GEN] quest=1267 status=0 n_index=7709
if (EVENT == 143) then
	SaveEvent(UID, 7710);
end

-- [AUTO-GEN] quest=1267 status=1 n_index=7710
if (EVENT == 145) then
	ItemA = HowmuchItem(UID, 900691000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1267, 44117, NPC, 18, 146);
	else
		SelectMsg(UID, 4, 1267, 44117, NPC, 41, 147, 27, -1);
	end
end

-- [AUTO-GEN] quest=1267 status=1 n_index=7710
if (EVENT == 146) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1267 status=1 n_index=7710
if (EVENT == 147) then
	QuestStatusCheck = GetQuestStatus(UID, 1267)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6063);
		SaveEvent(UID, 7711);
	end
end

-- [AUTO-GEN] quest=1268 status=255 n_index=7714
if (EVENT == 150) then
	SaveEvent(UID, 7715);
end

-- [AUTO-GEN] quest=1268 status=0 n_index=7715
if (EVENT == 152) then
	SelectMsg(UID, 4, 1268, 44118, NPC, 738, 153, 23, -1);
end

-- [AUTO-GEN] quest=1268 status=0 n_index=7715
if (EVENT == 153) then
	SaveEvent(UID, 7716);
end

-- [AUTO-GEN] quest=1268 status=1 n_index=7716
if (EVENT == 155) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1268, 44118, NPC, 22, 157, 23, -1);
	else
		SelectMsg(UID, 2, 1268, 44118, NPC, 18, 156);
	end
end

-- [AUTO-GEN] quest=1268 status=1 n_index=7716
if (EVENT == 156) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1268 status=1 n_index=7716
if (EVENT == 157) then
	QuestStatusCheck = GetQuestStatus(UID, 1268)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6064);
		SaveEvent(UID, 7717);
	end
end

-- [AUTO-GEN] quest=1269 status=255 n_index=7720
if (EVENT == 160) then
	SaveEvent(UID, 7721);
end

-- [AUTO-GEN] quest=1269 status=0 n_index=7721
if (EVENT == 162) then
	SelectMsg(UID, 4, 1269, 44119, NPC, 739, 163, 23, -1);
end

-- [AUTO-GEN] quest=1269 status=0 n_index=7721
if (EVENT == 163) then
	SaveEvent(UID, 7722);
end

-- [AUTO-GEN] quest=1269 status=1 n_index=7722
if (EVENT == 165) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1269, 44119, NPC, 22, 167, 23, -1);
	else
		SelectMsg(UID, 2, 1269, 44119, NPC, 18, 166);
	end
end

-- [AUTO-GEN] quest=1269 status=1 n_index=7722
if (EVENT == 166) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1269 status=1 n_index=7722
if (EVENT == 167) then
	QuestStatusCheck = GetQuestStatus(UID, 1269)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6065);
		SaveEvent(UID, 7723);
	end
end

-- [AUTO-GEN] quest=1270 status=255 n_index=7726
if (EVENT == 170) then
	SaveEvent(UID, 7727);
end

-- [AUTO-GEN] quest=1270 status=0 n_index=7727
if (EVENT == 172) then
	SelectMsg(UID, 4, 1270, 44120, NPC, 740, 173, 23, -1);
end

-- [AUTO-GEN] quest=1270 status=0 n_index=7727
if (EVENT == 173) then
	SaveEvent(UID, 7728);
end

-- [AUTO-GEN] quest=1270 status=1 n_index=7728
if (EVENT == 175) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1270, 44120, NPC, 22, 177, 23, -1);
	else
		SelectMsg(UID, 2, 1270, 44120, NPC, 18, 176);
	end
end

-- [AUTO-GEN] quest=1270 status=1 n_index=7728
if (EVENT == 176) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1270 status=1 n_index=7728
if (EVENT == 177) then
	QuestStatusCheck = GetQuestStatus(UID, 1270)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6066);
		SaveEvent(UID, 7729);
	end
end

-- [AUTO-GEN] quest=1271 status=255 n_index=7770
if (EVENT == 180) then
	SaveEvent(UID, 7771);
end

-- [AUTO-GEN] quest=1271 status=0 n_index=7771
if (EVENT == 182) then
	SelectMsg(UID, 4, 1271, 44121, NPC, 741, 183, 23, -1);
end

-- [AUTO-GEN] quest=1271 status=0 n_index=7771
if (EVENT == 183) then
	SaveEvent(UID, 7772);
end

-- [AUTO-GEN] quest=1271 status=1 n_index=7772
if (EVENT == 185) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1271, 44121, NPC, 22, 187, 23, -1);
	else
		SelectMsg(UID, 2, 1271, 44121, NPC, 18, 186);
	end
end

-- [AUTO-GEN] quest=1271 status=1 n_index=7772
if (EVENT == 186) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1271 status=1 n_index=7772
if (EVENT == 187) then
	QuestStatusCheck = GetQuestStatus(UID, 1271)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6067);
		SaveEvent(UID, 7773);
	end
end

-- [AUTO-GEN] quest=1272 status=255 n_index=7776
if (EVENT == 190) then
	SearchQuest(UID, 25004);
end

-- [AUTO-GEN] quest=1272 status=0 n_index=7777
if (EVENT == 192) then
	SelectMsg(UID, 4, 1272, 44122, NPC, 742, 193, 23, -1);
end

-- [AUTO-GEN] quest=1272 status=0 n_index=7777
if (EVENT == 193) then
	SaveEvent(UID, 7778);
end

-- [AUTO-GEN] quest=1272 status=1 n_index=7778
if (EVENT == 195) then
	MonsterSub = ExistMonsterQuestSub(UID);
	if (MonsterSub == 0) then
		SelectMsg(UID, 4, 1272, 44122, NPC, 22, 197, 23, -1);
	else
		SelectMsg(UID, 2, 1272, 44122, NPC, 18, 196);
	end
end

-- [AUTO-GEN] quest=1272 status=1 n_index=7778
if (EVENT == 196) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1272 status=1 n_index=7778
if (EVENT == 197) then
	QuestStatusCheck = GetQuestStatus(UID, 1272)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6068);
		SaveEvent(UID, 7779);
	end
end

-- [AUTO-GEN] quest=1273 status=255 n_index=7782
if (EVENT == 200) then
	SaveEvent(UID, 7783);
end

-- [AUTO-GEN] quest=1273 status=0 n_index=7783
if (EVENT == 202) then
	SelectMsg(UID, 4, 1273, 44123, NPC, 743, 203, 23, -1);
end

-- [AUTO-GEN] quest=1273 status=0 n_index=7783
if (EVENT == 203) then
	SaveEvent(UID, 7784);
end

-- [AUTO-GEN] quest=1273 status=1 n_index=7784
if (EVENT == 205) then
	ItemA = HowmuchItem(UID, 900012000);
	if (ItemA < 0) then
		SelectMsg(UID, 2, 1273, 44123, NPC, 18, 206);
	else
		SelectMsg(UID, 4, 1273, 44123, NPC, 41, 207, 27, -1);
	end
end

-- [AUTO-GEN] quest=1273 status=1 n_index=7784
if (EVENT == 206) then
	ShowMap(UID, 72);
end

-- [AUTO-GEN] quest=1273 status=1 n_index=7784
if (EVENT == 207) then
	QuestStatusCheck = GetQuestStatus(UID, 1273)
	if(QuestStatusCheck == 2) then
		SelectMsg(UID, 2, -1, 8779, NPC, 10, -1);
	else
		RunQuestExchange(UID, 6069);
		SaveEvent(UID, 7785);
	end
end

