-- Fix r_damage multiplier: C++ original default is 1.0, not 0.9.
-- C++ Reference: GameDefine.h:294 — rdamage = 1.0f
UPDATE damage_settings SET r_damage = 1.0 WHERE id = 1;
