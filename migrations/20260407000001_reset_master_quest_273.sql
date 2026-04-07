-- Reset quest 273 (class change / master quest) for characters whose class
-- was never actually changed to master tier.
--
-- Root cause: characters were GM-leveled to 83 and quest 273 was marked
-- completed (state=2), but RunQuestExchange never applied the class change.
-- The client sees quest 273 as done and hides the class change NPC dialog.
--
-- Master class_type values (class % 100): 6, 8, 10, 12, 15
-- This migration only deletes quest 273 for non-master characters.

DELETE FROM user_quest
WHERE quest_id = 273
  AND str_user_id IN (
    SELECT str_user_id FROM userdata
    WHERE (class % 100) NOT IN (6, 8, 10, 12, 15)
  );
