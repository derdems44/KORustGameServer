local NPC = 19001;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 6347, NPC, 4424, 101, 27, -1);
end   

if (EVENT == 101) then
	SelectMsg(UID, 2, -1, 1640, NPC, 4446, -1);
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=912 status=0 n_index=6645
if (EVENT == 1000) then
	SelectMsg(UID, 4, 912, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=912 status=0 n_index=6645
if (EVENT == 1001) then
	SaveEvent(UID, 6646);
end

-- [AUTO-GEN] quest=912 status=1 n_index=6646
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

