-- Sprint 161: Add missing indexes identified by repository WHERE clause analysis
-- These indexes improve query performance for frequently used lookups.

-- King system queries filter by nation and type
CREATE INDEX IF NOT EXISTS idx_king_election_nation_type
    ON king_election_list (by_nation, by_type);

CREATE INDEX IF NOT EXISTS idx_king_nomination_nation
    ON king_nomination_list (by_nation);

CREATE INDEX IF NOT EXISTS idx_king_candidacy_nation
    ON king_candidacy_notice_board (by_nation);

CREATE INDEX IF NOT EXISTS idx_king_election_votes_nation
    ON king_election_votes (by_nation);

-- Trash item list lookups filter by user and sort by delete_time
CREATE INDEX IF NOT EXISTS idx_trash_item_user_time
    ON trash_item_list (str_user_id, delete_time);
