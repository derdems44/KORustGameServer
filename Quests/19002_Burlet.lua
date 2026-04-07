local Ret = 0;
local NPC = 19002;

-- [Entrep Trader] Berret
-- Auto-generated from sniffer capture (dialog_builder v4)
-- 1 menus, 0 mapped, 1 inferred, 0 unknown [actions=SHOP]

-- header=4947 flag=1
if (EVENT == 165) then
	SelectMsg(UID, 1, 95, 4947, NPC, 28, 5000);
end

-- ═══ Action handlers (sniffer-verified) ═══

-- SHOP action (btn_text=28)
if (EVENT == 5000) then
	SelectMsg(UID, 21, -1, -1, NPC, -1, -1); -- selling_group=0, fallback to flag 21
end

-- Close dialog
if (EVENT == 3001) then
	Ret = 1;
end
