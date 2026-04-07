-- Fix game_version to 2599 (v2600 client Ghidra: FUN_007084A0 expects exactly 0x0A27).
-- 2599 = game version (exe hardcoded), NOT patch version (Server.ini Files=2602).
UPDATE server_settings SET game_version = 2599 WHERE server_no = 1;
