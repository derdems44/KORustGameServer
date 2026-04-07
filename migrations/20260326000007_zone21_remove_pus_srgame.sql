-- Remove [PUS] SRGame (proto 32756) — user confirmed not on original server.
-- Zone 21 data will be fully re-synced after clean sniffer capture.
DELETE FROM npc_spawn WHERE zone_id = 21 AND npc_id = 32756;
