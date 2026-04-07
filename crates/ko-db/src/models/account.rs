//! Account-related models: TB_USER, ACCOUNT_CHAR, CURRENTUSER.

use chrono::{DateTime, Utc};

/// A user account (maps to `tb_user` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TbUser {
    pub id: i64,
    pub str_account_id: String,
    pub str_passwd: String,
    pub str_seal_passwd: String,
    pub otp_password: String,
    pub str_client_ip: Option<String>,
    pub b_premium_type: i16,
    pub dt_premium_time: Option<DateTime<Utc>>,
    pub s_hours: i16,
    pub dt_create_time: DateTime<Utc>,
    pub cash_point: i32,
    pub email: Option<String>,
    pub security_answer: Option<String>,
    pub security_question: Option<String>,
    pub str_authority: i16,
    pub user_name: Option<String>,
    pub user_surname: Option<String>,
    pub bonus_cash_point: Option<i32>,
    pub user_phone_number: Option<String>,
    pub punishment_date: Option<DateTime<Utc>>,
    pub punishment_period: Option<i32>,
    pub account_check: i16,
    pub promotion_check: i16,
    pub str_genie_time: i32,
}

/// Account-character mapping (maps to `account_char` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountChar {
    pub str_account_id: String,
    pub b_nation: i16,
    pub b_char_num: i16,
    pub str_char_id1: Option<String>,
    pub str_char_id2: Option<String>,
    pub str_char_id3: Option<String>,
    pub str_char_id4: Option<String>,
}

/// Currently online user (maps to `currentuser` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CurrentUser {
    pub str_account_id: String,
    pub str_char_id: String,
    pub n_server_no: i16,
    pub str_server_ip: String,
    pub str_client_ip: String,
}
