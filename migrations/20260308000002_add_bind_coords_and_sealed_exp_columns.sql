-- Sprint 625: Add bind point coordinate columns + sealed_exp to userdata.
--
-- bind_px, bind_pz: Persist exact bind stone coordinates (object_event.rs).
-- Without these, bind resets to zone spawn on every login.
--
-- The existing `bind` column (SMALLINT) stores the bind zone ID.
-- These new columns store the precise X/Z coordinates within that zone.

ALTER TABLE userdata ADD COLUMN IF NOT EXISTS bind_px INTEGER NOT NULL DEFAULT 0;
ALTER TABLE userdata ADD COLUMN IF NOT EXISTS bind_pz INTEGER NOT NULL DEFAULT 0;
