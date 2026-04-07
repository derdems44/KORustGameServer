local NPC = 32000;

if (EVENT == 100) then
	SelectMsg(UID, 3, -1, 45540, NPC, 1719, 101);
end

if (EVENT == 101) then --Voucher of Infinite Arrow	800605000
	CHEST = HowmuchItem(UID, 700079000);
	if (CHEST < 1) then
		SelectMsg(UID, 2, -1, 45544, NPC, 27, -1);
	else
	SlotCheck = CheckGiveSlot(UID, 1)
     if SlotCheck == false then	
       
	    else
		RobItem(UID, 700079000,1);
		GiveItem(UID, 810665000,1);
		SelectMsg(UID, 2, -1, 45545, NPC, 27, -1);	
    	end
    end
end

-- ═══════════════════════════════════════════════════════════════════
-- AUTO-GENERATED EVENT HANDLERS (ko-quest-gen)
-- ═══════════════════════════════════════════════════════════════════

-- [AUTO-GEN] quest=7094 status=4 n_index=14187
if (EVENT == 186) then
	SelectMsg(UID, 2, -1, 331, NPC, 10, -1);
end

