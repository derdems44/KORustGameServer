-- Widen password column to 34 chars to support client-hashed passwords.
ALTER TABLE tb_user ALTER COLUMN str_passwd TYPE VARCHAR(34);

-- Test accounts removed for production safety.
-- Create accounts via web portal registration: https://portal.example.com/register
