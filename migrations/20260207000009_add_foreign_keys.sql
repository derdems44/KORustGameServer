-- Foreign key constraint'leri — tablolar arası ilişkiler
-- Idempotent: sadece yoksa ekle

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_account_char_user') THEN
        ALTER TABLE account_char
            ADD CONSTRAINT fk_account_char_user
            FOREIGN KEY (str_account_id) REFERENCES tb_user (str_account_id)
            ON DELETE CASCADE;
    END IF;
END $$;

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_currentuser_user') THEN
        ALTER TABLE currentuser
            ADD CONSTRAINT fk_currentuser_user
            FOREIGN KEY (str_account_id) REFERENCES tb_user (str_account_id)
            ON DELETE CASCADE;
    END IF;
END $$;

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_userdata_knights') THEN
        ALTER TABLE userdata
            ADD CONSTRAINT fk_userdata_knights
            FOREIGN KEY (knights) REFERENCES knights (id_num)
            ON DELETE SET DEFAULT;
    END IF;
END $$;

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_user_items_userdata') THEN
        ALTER TABLE user_items
            ADD CONSTRAINT fk_user_items_userdata
            FOREIGN KEY (str_user_id) REFERENCES userdata (str_user_id)
            ON DELETE CASCADE;
    END IF;
END $$;

-- user_items.item_id → item.num: FK koymuyoruz (item_id=0 boş slot)

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_user_deleted_items_userdata') THEN
        ALTER TABLE user_deleted_items
            ADD CONSTRAINT fk_user_deleted_items_userdata
            FOREIGN KEY (str_user_id) REFERENCES userdata (str_user_id)
            ON DELETE CASCADE;
    END IF;
END $$;
