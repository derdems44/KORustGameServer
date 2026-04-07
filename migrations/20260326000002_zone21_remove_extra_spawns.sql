-- Zone 21 Moradon: remove 105 spawns NOT present in original server (sniffer session 38).
-- Sniffer verified 265 unique NPC/Monster (151 MON + 114 NPC).
-- These extra spawns were imported from C++ reference data but don't exist on the v2600 original server.

DELETE FROM npc_spawn
WHERE zone_id = 21
  AND npc_id NOT IN (
    -- === MONSTERS (sniffer verified, 26 protos) ===
    150,   -- Kecoon
    155,   -- Kecoon warrior
    159,   -- Kecoon captain
    250,   -- Bulcan
    251,   -- Wild bulcan
    253,   -- Giant bulcan
    254,   -- Bulture
    257,   -- Silan
    350,   -- Gavolt
    351,   -- Giant gavolt
    354,   -- Gloomwing
    550,   -- Werewolf
    551,   -- Lycan
    750,   -- Worm
    852,   -- Scavenger Bandicoot
    3201,  -- Football
    8644,  -- [Captine] Kekurikang
    8645,  -- [Captine] Bandiking
    8646,  -- [Captine] Gaboltin
    8647,  -- [Captine] Keshark
    8649,  -- [Captine] Wolfraiger
    9274,  -- Mana Predator
    9275,  -- Shadow Spectre
    9276,  -- [Leader] Shadow of Krowaz
    9641,  -- Looter
    -- === NPCs (sniffer verified, 38 protos) ===
    5001,  -- Magic anvil
    11021, -- Guard (Elmorad)
    13003, -- [Healing Potion]Karpis
    13005, -- [Healing Potion]Karpis
    13006, -- [Armor Merchant]Hesta
    13007, -- [Sundries]Zarta
    13008, -- [Weapon Merchant]Gargameth
    13009, -- [Mercenary Captain]Kugar
    13016, -- [Familiar Trainer] Kate
    14301, -- [Blacksmith] Heppa
    14401, -- [Rental booth]Helard
    15002, -- [Coliseum] Artes
    16073, -- Karus Hero Statue
    16074, -- Elmorad Hero Statue
    16085, -- [Vendor] Hemes
    16096, -- [InnHostess] Neria
    16097, -- [InnHostess] Nia
    19005, -- [Tarot Reader] Mekin
    19006, -- [Arena Manager] Dualer
    19019, -- Kaul
    19057, -- Diary lost man
    19060, -- Psssimist
    19067, -- Leather Scarecrow
    19068, -- Chain Scarecrow
    19069, -- Iron Scarecrow
    19070, -- Leather Scarecrow (small)
    19071, -- Chain Scarecrow (small)
    19072, -- Iron Scarecrow (small)
    19073, -- unknown (NPC near inn)
    21021, -- Guard (Karus)
    22301, -- [Scrolls]Charon
    25007, -- Quest NPC
    25174, -- Pulchino
    25177, -- [Vendor] Iruta
    29056, -- [Vendor] Kaira
    29057, -- [DC Sundries] Diska
    29079, -- [Stadium Manager] Burak
    31508, -- [National Enchanter]
    31526, -- [White Shadow Captin] Sirin
    31719, -- [CSW Manager] Aaron
    31720  -- (JAPKO) LvL Jump
  );
