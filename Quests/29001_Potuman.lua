local NPC = 29001;

if (EVENT == 100) then
	SelectMsg(UID, 2, -1, 1399, NPC, 7087, 101, 3005, -1);
end   

if (EVENT == 101) then
	ZoneChange(UID, 21, 780, 52)
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=911 status=0 n_index=6640
if (EVENT == 1000) then
	SelectMsg(UID, 4, 911, 0, NPC, 22, 1001, 23, -1);
end

-- [AUTO-GEN] quest=911 status=0 n_index=6640
if (EVENT == 1001) then
	SaveEvent(UID, 6641);
end

-- [AUTO-GEN] quest=911 status=1 n_index=6641
if (EVENT == 1002) then
	ShowMap(UID, 21);
end

