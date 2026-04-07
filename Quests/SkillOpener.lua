--local Ret = 0;
local NPC = 20035;

if (EVENT == 100) then
	Class = CheckClass (UID);
	-- Warrior
	if (Class == 1 or Class == 5 or Class == 6) then
		if (Class == 5) then
			SelectMsg(UID, 2, -1, 47005, NPC, 47008,60, 47000,70, 47004,75, 47007,80);
		else
			SelectMsg(UID, 2, -1, 47005, NPC, 47000,70, 47004,75, 47007,80);
		end
	end
	-- Rogue
	if (Class == 2 or Class == 7 or Class == 8) then
		if (Class == 7) then
			SelectMsg(UID, 2, -1, 47005, NPC, 47008,60, 47000,70, 47002,72, 47004,75, 47007,80);
		else
			SelectMsg(UID, 2, -1, 47005, NPC, 47000,70, 47002,72, 47004,75, 47007,80);
		end
	end
	--Mage
	if (Class == 3 or Class == 9 or Class == 10) then
		if (Class == 9) then
			SelectMsg(UID, 2, -1, 47005, NPC, 47008,60, 47000,70, 47002,72, 47004,75, 47007,80);
		else
			SelectMsg(UID, 2, -1, 47005, NPC, 47000,70, 47002,72, 47004,75, 47007,80);
		end
	end
	-- Priest
	if (Class == 4 or Class == 11 or Class == 12) then
		if (Class == 11) then
			SelectMsg(UID, 2, -1, 47005, NPC, 47008,60, 47000,70, 47002,72, 47003,74 ,47004,75 ,47005,76 ,47006,78 ,47007,80);
		else
			SelectMsg(UID, 2, -1, 47005, NPC, 47000,70, 47002,72, 47003,74 ,47004,75 ,47005,76 ,47006,78 ,47007,80);
		end
	end
end

if (EVENT == 60) then
	OpenSkill(UID,60);
end
if (EVENT == 70) then
	OpenSkill(UID,70);
end
if (EVENT == 72) then
	OpenSkill(UID,72);
end
if (EVENT == 74) then
	OpenSkill(UID,74);
end
if (EVENT == 75) then
	OpenSkill(UID,75);
end
if (EVENT == 76) then
	OpenSkill(UID,76);
end
if (EVENT == 78) then
	OpenSkill(UID,78);
end
if (EVENT == 80) then
	OpenSkill(UID,80);
end
--SpellOfStoneSystem -- 47005