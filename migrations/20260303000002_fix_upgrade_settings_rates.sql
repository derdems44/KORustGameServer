-- Sprint 470: Fix item upgrade rates — add missing scroll-only entries and correct 0-rates
--
-- Root cause (Bug 6): PVP server seed data has success_rate=0 for ALL non-blessed scroll entries
-- and scroll-only entries for Low/Middle Class scrolls are completely MISSING.
-- Only Trina+scroll combo entries exist, all with rate=0.
--
-- Fix: Add scroll-only entries for Low/Middle Class scrolls with standard KO rates,
-- and update existing Trina+scroll entries to have non-zero rates.
--
-- Scroll IDs:
--   379221000 = Low Class Scroll
--   379205000 = Middle Class Scroll
--   379021000 = Blessed Upgrade Scroll (High Class)
--   700002000 = Trina's Piece
--   353000000 = Trina's Piece (Low Class)
--   352900000 = Trina's Piece (Middle Class)
--
-- Standard KO upgrade rates (out of 10000):
--   Grade 0-2 (Low Class): 10000 (100%)
--   Grade 3 (Middle Class): 7500
--   Grade 4: 5500, Grade 5: 4000
--   Grade 6: 3000, Grade 7: 1000, Grade 8: 100
--   Trina bonus: +1000 (capped at 10000)

-- ═══════════════════════════════════════════════════════════
-- 1. Add MISSING scroll-only entries for Low Class Scroll
-- ═══════════════════════════════════════════════════════════
INSERT INTO item_upgrade_settings
  (req_item_id1, req_item_name1, req_item_id2, req_item_name2, upgrade_note, item_type, item_rate, item_grade, item_req_coins, success_rate)
VALUES
  (379221000, 'Upgrade Scroll (Low Class item)', 0, '', 'Low Class Item Upgrade', 4, 33, 0, 5000, 10000),
  (379221000, 'Upgrade Scroll (Low Class item)', 0, '', 'Low Class Item Upgrade', 4, 33, 1, 5000, 10000),
  (379221000, 'Upgrade Scroll (Low Class item)', 0, '', 'Low Class Item Upgrade', 4, 33, 2, 5000, 10000);

-- ═══════════════════════════════════════════════════════════
-- 2. Add MISSING scroll-only entries for Middle Class Scroll
-- ═══════════════════════════════════════════════════════════
INSERT INTO item_upgrade_settings
  (req_item_id1, req_item_name1, req_item_id2, req_item_name2, upgrade_note, item_type, item_rate, item_grade, item_req_coins, success_rate)
VALUES
  (379205000, 'Upgrade Scroll (Middle Class item)', 0, '', 'Middle Class Item Upgrade', 4, 33, 3, 10000, 7500),
  (379205000, 'Upgrade Scroll (Middle Class item)', 0, '', 'Middle Class Item Upgrade', 4, 33, 4, 10000, 5500),
  (379205000, 'Upgrade Scroll (Middle Class item)', 0, '', 'Middle Class Item Upgrade', 4, 33, 5, 10000, 4000);

-- ═══════════════════════════════════════════════════════════
-- 3. Fix existing Trina Low Class + Low Class Scroll rates (currently all 0)
-- ═══════════════════════════════════════════════════════════
UPDATE item_upgrade_settings
SET success_rate = 10000
WHERE req_item_id1 = 353000000
  AND req_item_id2 = 379221000
  AND item_type = 4
  AND item_rate = 33
  AND success_rate = 0;

-- ═══════════════════════════════════════════════════════════
-- 4. Fix existing Trina Middle Class + Middle Class Scroll rates (currently all 0)
--    Trina adds ~+1000 to base rate
-- ═══════════════════════════════════════════════════════════
UPDATE item_upgrade_settings
SET success_rate = CASE item_grade
    WHEN 1 THEN 10000
    WHEN 2 THEN 10000
    WHEN 3 THEN 8500
    WHEN 4 THEN 6500
    WHEN 5 THEN 5000
    WHEN 6 THEN 4000
    WHEN 7 THEN 2000
    WHEN 8 THEN 1000
    ELSE 0
  END
WHERE req_item_id1 = 352900000
  AND req_item_id2 = 379205000
  AND item_type = 4
  AND item_rate = 33
  AND success_rate = 0;

-- ═══════════════════════════════════════════════════════════
-- 5. Fix existing Blessed Item Upgrade Scroll rates (379152000, currently all 0)
--    These are the non-Trina blessed scroll entries
-- ═══════════════════════════════════════════════════════════
UPDATE item_upgrade_settings
SET success_rate = CASE item_grade
    WHEN 1 THEN 10000
    WHEN 2 THEN 10000
    WHEN 3 THEN 10000
    WHEN 4 THEN 7500
    WHEN 5 THEN 5500
    WHEN 6 THEN 3000
    WHEN 7 THEN 1000
    WHEN 8 THEN 100
    ELSE 0
  END
WHERE req_item_id1 = 379152000
  AND req_item_id2 = 0
  AND item_type = 4
  AND item_rate = 33
  AND success_rate = 0;
