-- Restore original selling_group values (zeroed by sniffer sync migration 005)
-- selling_group determines NPC shop inventory, NOT sent in NPC_INFO wire format

UPDATE npc_template SET i_selling_group = 251000 WHERE s_sid = 501 AND is_monster = false; -- [Siege War Weapons]Serendil
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 503 AND is_monster = false; -- Froil[Armors]Froil
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 504 AND is_monster = false; -- [Arms]verofewen
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 505 AND is_monster = false; -- [Sundries]Nand
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 506 AND is_monster = false; -- [Potions]Uronia
UPDATE npc_template SET i_selling_group = 252000 WHERE s_sid = 522 AND is_monster = false; -- [Maintain]Captain Hugor
UPDATE npc_template SET i_selling_group = 251000 WHERE s_sid = 523 AND is_monster = false; -- [Siege War Weapons]Elrond
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 525 AND is_monster = false; -- [Armors]Pinrod
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 526 AND is_monster = false; -- [Arms]Rorendil
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 527 AND is_monster = false; -- [Sundries]Hures
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 528 AND is_monster = false; -- [Potions]Nion
UPDATE npc_template SET i_selling_group = 254000 WHERE s_sid = 7002 AND is_monster = false; -- Rodion[Scroll]
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 8111 AND is_monster = false; -- [Potion Merchant]Ruber 
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 8112 AND is_monster = false; -- [Sundries] Halber 
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 8161 AND is_monster = false; -- [Healing Potions]Shama 
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 8162 AND is_monster = false; -- [Sundries] Ardin
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 11045 AND is_monster = false; -- [sundries]Citizen
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 11710 AND is_monster = false; -- Sara
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 12100 AND is_monster = false; -- [Armor Merchant]Tyroon
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 12102 AND is_monster = false; -- [Armor]Ranggise
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 12117 AND is_monster = false; -- [Healing Potions]Ruber
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 12118 AND is_monster = false; -- [Healing potion]Raina
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 12120 AND is_monster = false; -- [sundries]Halber
UPDATE npc_template SET i_selling_group = 203002 WHERE s_sid = 12121 AND is_monster = false; -- [accessory]Drafe
UPDATE npc_template SET i_selling_group = 203072 WHERE s_sid = 12122 AND is_monster = false; -- [Magic Item]Nanggis
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 12200 AND is_monster = false; -- [Arms Merchant]Pallus
UPDATE npc_template SET i_selling_group = 201007 WHERE s_sid = 12202 AND is_monster = false; -- [Handmade Weapon Arms]Engram
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 12203 AND is_monster = false; -- Grudy
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 12208 AND is_monster = false; -- [Sundries]Yasval
UPDATE npc_template SET i_selling_group = 254000 WHERE s_sid = 12301 AND is_monster = false; -- [Scrolls]Charon
UPDATE npc_template SET i_selling_group = 203002 WHERE s_sid = 13001 AND is_monster = false; -- [Accessory Merchant]Hera
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 13002 AND is_monster = false; -- [Armor]Rospel
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 13003 AND is_monster = false; -- [Potion Merchant]Clepio
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 13004 AND is_monster = false; -- [Weapons]Nauke
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 13005 AND is_monster = false; -- [Healing Potion]Karpis
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 13006 AND is_monster = false; -- [Armor Merchant]Hesta
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 13007 AND is_monster = false; -- [Sundries]Zarta
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 13008 AND is_monster = false; -- [Weapon Merchant]Gargameth
UPDATE npc_template SET i_selling_group = 232000 WHERE s_sid = 13016 AND is_monster = false; -- [Familiar Trainer] Kate
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 13333 AND is_monster = false; -- [Sundries]Zarta
UPDATE npc_template SET i_selling_group = 285000 WHERE s_sid = 14000 AND is_monster = false; -- [Special Scroll NPC] AryaOnlineWorld
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 14401 AND is_monster = false; -- [Rental booth]Helard
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 14402 AND is_monster = false; -- [Novice Weapon Rental]Sodern
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 14403 AND is_monster = false; -- [Novice Weapon Rental]Soren
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 14415 AND is_monster = false; -- Sodun[Beginner Item Rental]
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 14416 AND is_monster = false; -- Soren[Beginner Item Rental]
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 16062 AND is_monster = false; -- [sundries]Halber
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 16089 AND is_monster = false; -- [Healing potion]Rona
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 17002 AND is_monster = false; -- [Weapon] Hyon
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 17003 AND is_monster = false; -- [Armor] Udian
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 17004 AND is_monster = false; -- [Sundries] Udian
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 17005 AND is_monster = false; -- [Portion] Kawani
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 18001 AND is_monster = false; -- [Sundries] Cobel
UPDATE npc_template SET i_selling_group = 231000 WHERE s_sid = 18003 AND is_monster = false; -- [Item Rental] Nernozan
UPDATE npc_template SET i_selling_group = 250000 WHERE s_sid = 18006 AND is_monster = false; -- [QuarterMaster]Valtor
UPDATE npc_template SET i_selling_group = 250000 WHERE s_sid = 18007 AND is_monster = false; -- [QuarterMaster]Hector
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 19021 AND is_monster = false; -- [Sundries]Rabiro
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 19022 AND is_monster = false; -- [Sundries]Lionel
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 19023 AND is_monster = false; -- [Potion] Kisiner
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 21047 AND is_monster = false; -- Citizen[sundries]
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 22100 AND is_monster = false; -- [Armor Merchant] Cubense
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 22106 AND is_monster = false; -- [Arms]Rian
UPDATE npc_template SET i_selling_group = 103002 WHERE s_sid = 22109 AND is_monster = false; -- [Accessory]Kamal
UPDATE npc_template SET i_selling_group = 103072 WHERE s_sid = 22110 AND is_monster = false; -- [Magic Item]Rawal
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 22120 AND is_monster = false; -- [sundries]Ardin
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 22200 AND is_monster = false; -- [Weapon]Mori
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 22202 AND is_monster = false; -- [Healing Potions]Shama
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 22203 AND is_monster = false; -- [Handmade Armor]Abul
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 22208 AND is_monster = false; -- [Sundries]Veron
UPDATE npc_template SET i_selling_group = 101001 WHERE s_sid = 22210 AND is_monster = false; -- Maren
UPDATE npc_template SET i_selling_group = 254000 WHERE s_sid = 22301 AND is_monster = false; -- [Scrolls]Charon
UPDATE npc_template SET i_selling_group = 282000 WHERE s_sid = 23400 AND is_monster = false; -- HSACSX MARKET
UPDATE npc_template SET i_selling_group = 279000 WHERE s_sid = 23518 AND is_monster = false; -- [Accessories] Officer
UPDATE npc_template SET i_selling_group = 280000 WHERE s_sid = 23519 AND is_monster = false; -- [Warrior] Officer
UPDATE npc_template SET i_selling_group = 273000 WHERE s_sid = 23520 AND is_monster = false; -- [Rogue] Officer
UPDATE npc_template SET i_selling_group = 274000 WHERE s_sid = 23521 AND is_monster = false; -- [Mage] Officer
UPDATE npc_template SET i_selling_group = 275000 WHERE s_sid = 23522 AND is_monster = false; -- [Priest] Officer
UPDATE npc_template SET i_selling_group = 250000 WHERE s_sid = 24301 AND is_monster = false; -- [Scroll]
UPDATE npc_template SET i_selling_group = 283000 WHERE s_sid = 25278 AND is_monster = false; -- HSACSX Beta Weapon
UPDATE npc_template SET i_selling_group = 265000 WHERE s_sid = 25279 AND is_monster = false; -- TheKnightCrownBetaScroll
UPDATE npc_template SET i_selling_group = 270000 WHERE s_sid = 25280 AND is_monster = false; -- TheKnightCrownBetaPotion
UPDATE npc_template SET i_selling_group = 271000 WHERE s_sid = 25281 AND is_monster = false; -- TheKnightCrownBetaKrowas
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 26062 AND is_monster = false; -- Ardin[sundries]
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 27002 AND is_monster = false; -- [Weapon Merchant] Kuus
UPDATE npc_template SET i_selling_group = 202001 WHERE s_sid = 27003 AND is_monster = false; -- [Armor Merchant] Hion
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 27004 AND is_monster = false; -- [Sundries] Udian
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 27005 AND is_monster = false; -- [Portion Merchant] Kawani
UPDATE npc_template SET i_selling_group = 267000 WHERE s_sid = 29057 AND is_monster = false; -- [DC Sundries] Diska
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 29206 AND is_monster = false; -- [Twilight] Repair Merchant
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 29207 AND is_monster = false; -- [Twilight] Potion Merchant
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 29210 AND is_monster = false; -- [Potion Merchant]Karpis
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 29505 AND is_monster = false; -- [Potion Merchant] Amore
UPDATE npc_template SET i_selling_group = 282000 WHERE s_sid = 29508 AND is_monster = false; -- [Warrior Armor Merchant]
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 30352 AND is_monster = false; -- [QuarterInstructor] Amy
UPDATE npc_template SET i_selling_group = 201001 WHERE s_sid = 30353 AND is_monster = false; -- [QuarterInstructor] Dneero
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 30354 AND is_monster = false; -- Yasbal[Sundries]
UPDATE npc_template SET i_selling_group = 255000 WHERE s_sid = 30357 AND is_monster = false; -- Beron[Sundries]
UPDATE npc_template SET i_selling_group = 277000 WHERE s_sid = 31537 AND is_monster = false; -- [Sundries]Ghost
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 32280 AND is_monster = false; -- [Portion] Sonyador
UPDATE npc_template SET i_selling_group = 253000 WHERE s_sid = 32281 AND is_monster = false; -- [Portion] Ninomaxi
UPDATE npc_template SET i_selling_group = 256000 WHERE s_sid = 32700 AND is_monster = false; -- [QuarterMaster] Kalson
UPDATE npc_template SET i_selling_group = 256000 WHERE s_sid = 32750 AND is_monster = false; -- [QuarterMaster] Walter
UPDATE npc_template SET i_selling_group = 283000 WHERE s_sid = 32756 AND is_monster = false; -- [PUS] SRGame

-- Restored 103 selling_group values-- Walking NPC'leri C++ import'a geri al

DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 19019;
DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 21021;
DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 11021;
DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 19056;

-- Restore walking NPCs from C++ import
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room) VALUES
(21,11021,FALSE,100,0,0,0,0,800,491,1,0,30,0,0,'',0),
(21,11021,FALSE,100,0,0,0,0,795,491,1,0,30,0,0,'',0),
(21,11021,FALSE,101,0,0,0,0,866,753,1,0,120,0,0,'',0),
(21,11021,FALSE,101,0,0,0,0,837,541,1,0,120,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,725,538,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,700,538,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,725,518,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,900,476,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,900,483,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,898,473,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,911,459,1,0,30,0,0,'',0),
(21,19019,FALSE,101,0,0,0,0,926,488,1,0,30,0,0,'',0),
(21,19056,FALSE,100,0,0,0,0,822,775,1,0,30,0,0,'',0),
(21,21021,FALSE,101,0,0,0,0,786,545,1,0,120,0,0,'',0),
(21,21021,FALSE,100,0,0,0,0,840,491,1,0,30,0,0,'',0),
(21,21021,FALSE,100,0,0,0,0,835,491,1,0,30,0,0,'',0),
(21,21021,FALSE,101,0,0,0,0,753,753,1,0,120,0,0,'',0);
