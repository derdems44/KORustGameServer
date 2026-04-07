-- Fix column widths for account_info_save:
-- email: VARCHAR(50) → VARCHAR(250) (C++ validates up to 250 chars)
-- user_phone_number: VARCHAR(10) → VARCHAR(11) (C++ validates exactly 11 digits)
ALTER TABLE tb_user ALTER COLUMN email TYPE VARCHAR(250);
ALTER TABLE tb_user ALTER COLUMN user_phone_number TYPE VARCHAR(11);
