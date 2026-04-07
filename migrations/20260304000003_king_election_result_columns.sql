-- Add election result persistence columns to king_system.
-- These are used during the TERM_ENDED phase (5-minute window after voting ends)
-- to store the new king name and vote counts, preventing data loss on restart.

ALTER TABLE king_system ADD COLUMN IF NOT EXISTS str_new_king_name VARCHAR(21) NOT NULL DEFAULT '';
ALTER TABLE king_system ADD COLUMN IF NOT EXISTS king_votes INTEGER NOT NULL DEFAULT 0;
ALTER TABLE king_system ADD COLUMN IF NOT EXISTS total_votes INTEGER NOT NULL DEFAULT 0;
