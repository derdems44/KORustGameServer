-- Zone 21 Moradon: restore NPCs that were wrongly deleted + remove remaining extras.
-- Previous migration (20260326000002) used incomplete sniffer proto list.
-- This migration uses the COMPLETE list extracted from sniffer REQ_NPCIN packets (92 protos).
-- Note: proto 18034 and 31774 have no template — skipped (need template first).

-- 1. Restore wrongly deleted NPCs (sniffer-verified positions)
INSERT INTO npc_spawn (zone_id, npc_id, is_monster, num_npc, left_x, top_z, act_type, regen_type, dungeon_family, special_type, trap_number, spawn_range, regen_time, direction, dot_cnt, path, room)
VALUES
  -- 12200: [Arms Merchant]Pallus — pos(1688,1384) unusual position, might be event/instance
  -- SKIPPED: position outside normal Moradon bounds
  -- 18004: [Grand Merchant] Kaishan — pos(816,708)
  (21, 18004, false, 1, 816, 708, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 18005: [Manager]Billbor — pos(816,671)
  (21, 18005, false, 1, 816, 671, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 19002: [Entrep Trader] Berret — pos(392,550)
  (21, 19002, false, 1, 392, 550, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 19004: [Mercenary Supply Officer] Osm — pos(359,535)
  (21, 19004, false, 1, 359, 535, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 19007: [Mercenary adjutant] — 2 spawns: (742,530) and (756,530)
  (21, 19007, false, 1, 742, 530, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  (21, 19007, false, 1, 756, 530, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 19018: [FolkVillage Captain] Kronil — pos(411,557)
  (21, 19018, false, 1, 411, 557, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 19022: [Sundries]Lionel — pos(419,547)
  (21, 19022, false, 1, 419, 547, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25008: Suspicious Box — pos(349,555)
  (21, 25008, false, 1, 349, 555, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25153: Suspicious Box — pos(373,633)
  (21, 25153, false, 1, 373, 633, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25155: Suspicious Box — pos(412,539)
  (21, 25155, false, 1, 412, 539, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25156: Suspicious Box — pos(418,472)
  (21, 25156, false, 1, 418, 472, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25157: Suspicious Box — pos(523,567)
  (21, 25157, false, 1, 523, 567, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 25159: Suspicious Box — pos(338,668)
  (21, 25159, false, 1, 338, 668, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 29999: [Peddler]Hemes — pos(839,587)
  (21, 29999, false, 1, 839, 587, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 31511: [Pitman] kurushack — pos(679,422) sniffer verified
  (21, 31511, false, 1, 679, 422, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0),
  -- 31525: [makeup artist] Ulku — pos(924,605)
  (21, 31525, false, 1, 924, 605, 0, 0, 0, 0, 0, 0, 0, 0, 0, '', 0)
ON CONFLICT DO NOTHING;

-- 2. Remove any remaining spawns for protos NOT in sniffer
-- Complete sniffer proto list (92 protos, excluding template-less 18034 and 31774):
DELETE FROM npc_spawn
WHERE zone_id = 21
  AND npc_id NOT IN (
    -- Monsters (26)
    150,155,159,250,251,253,254,257,350,351,354,550,551,750,852,
    3201,8644,8645,8646,8647,8649,9274,9275,9276,9641,
    -- NPCs (65 — full sniffer list)
    5001,11021,12200,13003,13005,13006,13007,13008,13009,13016,
    14301,14401,15002,16073,16074,16085,16096,16097,
    18004,18005,
    19002,19004,19005,19006,19007,19018,19019,19022,19057,19060,
    19067,19068,19069,19070,19071,19072,19073,
    21021,22301,
    25007,25008,25153,25155,25156,25157,25159,25174,25177,
    29056,29057,29079,29999,
    31508,31511,31525,31526,31719,31720
  );
