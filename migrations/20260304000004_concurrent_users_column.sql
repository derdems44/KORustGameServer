-- Add concurrent_users column to game_server_list for Timer_UpdateConcurrent.
-- C++ Reference: CONCURRENT table (zone1_count) — simplified as a single column here.
ALTER TABLE game_server_list ADD COLUMN IF NOT EXISTS concurrent_users INTEGER NOT NULL DEFAULT 0;
