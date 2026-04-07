-- BUG-1 fix: Add UNIQUE constraint on (class, rank_id) so ON CONFLICT works.
-- The original ON CONFLICT (s_index) targeted the auto-increment PK which never
-- conflicts on INSERT. The logical key is (class, rank_id) per C++ stored procedure.

CREATE UNIQUE INDEX IF NOT EXISTS uq_draki_rift_class_rank
    ON draki_tower_rift_rank (class, rank_id);
