-- Fix FT_SUMMON_LIST: match MSSQL casing for "Stone golem" (b_index 162-165).
-- MSSQL has "Stone golem" (lowercase g), PG migration had "Stone Golem" (uppercase G).
-- The summon_name column is cosmetic (C++ never reads it), but we keep data fidelity.

UPDATE ft_summon_list
SET summon_name = 'Stone golem'
WHERE b_index IN (162, 163, 164, 165)
  AND event_type = 1
  AND stage = 28
  AND summon_name = 'Stone Golem';
