-- PPCard (product key / serial code) redemption table.
-- C++ Reference: DBAgent.cpp:5225-5277 — LoadPPCard()
--
-- Each row represents a single prepaid card code. When redeemed,
-- the card status is updated to 1 (used) and the account/character
-- info is recorded. Reward is delivered via GiveBalance (Knight Cash).

CREATE TABLE IF NOT EXISTS ppcard_list (
    card_key            VARCHAR(20)     NOT NULL PRIMARY KEY,
    knight_cash         INTEGER         NOT NULL DEFAULT 0,
    tl_balance          INTEGER         NOT NULL DEFAULT 0,
    cash_type           SMALLINT        NOT NULL DEFAULT 1,
    status              SMALLINT        NOT NULL DEFAULT 0,
    used_by_account     VARCHAR(50),
    used_by_character   VARCHAR(21),
    used_at             TIMESTAMP WITH TIME ZONE
);

-- Index for quick lookup by status (unused cards)
CREATE INDEX IF NOT EXISTS idx_ppcard_status ON ppcard_list (status);
