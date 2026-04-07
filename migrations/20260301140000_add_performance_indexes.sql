-- Sprint 382: Performance indexes for frequently queried columns
-- Only adds indexes that are MISSING — uses IF NOT EXISTS to be idempotent

-- ══════════════════════════════════════════════════════════════════════════
-- 1. letter: Queries filter on (recipient_name, b_status, b_deleted)
--    Existing: idx_letter_recipient ON (recipient_name, b_deleted)
--    Missing:  b_status in the index — queries at letter.rs:25,47 use all three
-- ══════════════════════════════════════════════════════════════════════════
CREATE INDEX IF NOT EXISTS idx_letter_status_recipient
    ON letter (recipient_name, b_status, b_deleted);

-- ══════════════════════════════════════════════════════════════════════════
-- 2. trash_item_list: Composite for repurchase lookup
--    Query: WHERE str_user_id = $1 AND delete_time > $2 (character.rs:601,668)
--    Existing: idx_trash_item_list_user ON (str_user_id) — no delete_time
-- ══════════════════════════════════════════════════════════════════════════
CREATE INDEX IF NOT EXISTS idx_trash_item_list_user_time
    ON trash_item_list (str_user_id, delete_time);

-- ══════════════════════════════════════════════════════════════════════════
-- 3. userdata: Knights member list with ordering
--    Query: WHERE knights = $1 ORDER BY fame DESC, level DESC (knights.rs:244)
--    Existing: idx_userdata_knights ON (knights) — no fame/level for sort
-- ══════════════════════════════════════════════════════════════════════════
CREATE INDEX IF NOT EXISTS idx_userdata_knights_fame_level
    ON userdata (knights, fame DESC, level DESC);

-- ══════════════════════════════════════════════════════════════════════════
-- 4. user_items: Serial number lookup (for item tracking / anti-dupe)
--    No existing index on serial_num
-- ══════════════════════════════════════════════════════════════════════════
CREATE INDEX IF NOT EXISTS idx_user_items_serial
    ON user_items (serial_num) WHERE serial_num > 0;

-- ══════════════════════════════════════════════════════════════════════════
-- 5. friend_list: Reverse lookup (who has me as friend)
--    Existing: idx_friend_list_user ON (user_id)
--    Missing:  friend_name for online status broadcast
-- ══════════════════════════════════════════════════════════════════════════
CREATE INDEX IF NOT EXISTS idx_friend_list_friend
    ON friend_list (friend_name);
