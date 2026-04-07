-- Add zone status column for zone activation/deactivation.
-- C++ Reference: C3DMap::m_Status — zones with Status=0 are inactive.
-- Default 1 = active (all existing zones remain active).
ALTER TABLE zone_info ADD COLUMN IF NOT EXISTS status SMALLINT NOT NULL DEFAULT 1;
