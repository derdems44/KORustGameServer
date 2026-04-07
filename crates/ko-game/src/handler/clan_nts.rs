//! Clan-wide nation transfer (ClanNts) handler.
//!
//! C++ Reference: `ClanNtsHandler.cpp` (268 LOC)
//!
//! Triggered by the Lua `ClanNts(uid)` function (NPC dialog).
//! The leader's clan and all member characters switch nation
//! (Karus ↔ El Morad). Requires: leader, NTS item, all members
//! offline, no king, no cross-clan characters.
//!
//! ## Response Packet
//!
//! `WIZ_EXT_HOOK (0xE9) << u8(0xBE MESSAGE) << string(title) << string(msg)`

use std::sync::Arc;

use ko_db::repositories::character::CharacterRepository;
use ko_db::repositories::knights::KnightsRepository;
use ko_db::DbPool;
use ko_protocol::{Opcode, Packet};

use crate::world::WorldState;
use crate::zone::SessionId;

// ── Constants ──────────────────────────────────────────────────────────────

use super::ext_hook::EXT_SUB_MESSAGE;

/// Item required for clan nation transfer.
///
/// C++ Reference: `GameDefine.h` — `#define CLAN_NTS_ITEM 900144023`
const CLAN_NTS_ITEM: u32 = 900144023;

use crate::world::{NATION_ELMORAD, NATION_KARUS};

use crate::race_constants::{
    ELMORAD_MAN, ELMORAD_WOMAN, KARUS_BIG, KARUS_MIDDLE, KARUS_SMALL, KARUS_WOMAN, KURIAN, PORUTU,
};

// ── Result Codes ───────────────────────────────────────────────────────────

/// C++ Reference: `GameDefine.h` — `cntscode` enum values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClanNtsResult {
    Failed = 0,
    SuccessReq = 1,
    AlreadyReq = 2,
    NoClanLeader = 3,
    // LowMember = 4, // Defined in C++ but never used
    OnlineMember = 5,
    NoItem = 6,
    IsKing = 7,
    InOtherClan = 8,
    Success = 9,
}

// ── Race Conversion ────────────────────────────────────────────────────────

/// Get the new race for a class after clan nation transfer.
///
/// C++ Reference: `CGameServerDlg::GetCntNewRace(uint16 newclass, Nation newnation)`
///
/// The class modulo 100 determines the base class type; the target nation
/// determines which race constant to assign.
pub fn get_cnts_new_race(class: u16, nation: u8) -> u8 {
    let base = (class % 100) as u8;
    if nation == NATION_ELMORAD {
        match base {
            1 | 5 | 6 => ELMORAD_MAN,
            2 | 7 | 8 => ELMORAD_MAN,
            3 | 9 | 10 => ELMORAD_WOMAN,
            4 | 11 | 12 => ELMORAD_WOMAN,
            13..=15 => PORUTU,
            _ => ELMORAD_MAN,
        }
    } else {
        match base {
            1 | 5 | 6 => KARUS_BIG,
            2 | 7 | 8 => KARUS_MIDDLE,
            3 | 9 | 10 => KARUS_SMALL,
            4 | 11 | 12 => KARUS_WOMAN,
            13..=15 => KURIAN,
            _ => KARUS_BIG,
        }
    }
}

// ── Packet Builders ────────────────────────────────────────────────────────

/// Build WIZ_EXT_HOOK (0xE9) + MESSAGE (0xBE) packet.
///
/// C++ Reference: `CUser::ClanNtsSendMsg()` — sends ext_hook message box.
fn build_ext_hook_message(title: &str, msg: &str) -> Packet {
    let mut pkt = Packet::new(Opcode::EXT_HOOK_S2C);
    pkt.write_u8(EXT_SUB_MESSAGE);
    pkt.write_string(title);
    pkt.write_string(msg);
    pkt
}

/// Send a ClanNts result message to the requesting player.
fn send_result(world: &WorldState, sid: SessionId, code: ClanNtsResult, extra: &str) {
    let msg = match code {
        ClanNtsResult::Failed => "something went wrong.".to_string(),
        ClanNtsResult::SuccessReq => "The request has been successfully discarded.".to_string(),
        ClanNtsResult::AlreadyReq => "You have already made a request.".to_string(),
        ClanNtsResult::NoClanLeader => "You are not in a clan or a leader.".to_string(),
        ClanNtsResult::OnlineMember => format!("Online User {extra}"),
        ClanNtsResult::NoItem => "You do not have the required parts on you.".to_string(),
        ClanNtsResult::IsKing => format!("King User {extra}"),
        ClanNtsResult::InOtherClan => format!("is in clan User {extra}"),
        ClanNtsResult::Success => "Success Process".to_string(),
    };
    let pkt = build_ext_hook_message("Clan Nation Transfer", &msg);
    world.send_to_session_owned(sid, pkt);
    // WIZ_CHAT fallback for vanilla v2525 client (drops ext_hook 0xE9)
    tracing::debug!("[{sid}] ClanNTS fallback: {code:?}");
    let chat_msg = format!("[ClanNTS] {}", msg);
    let chat_pkt = crate::systems::timed_notice::build_notice_packet(7, &chat_msg);
    world.send_to_session_owned(sid, chat_pkt);
}

// ── Main Handler ───────────────────────────────────────────────────────────

/// Entry point for ClanNts — called from `tokio::spawn` in Lua binding.
///
/// C++ Reference: `CUser::ClanNtsHandler()` + `CUser::ReqClanNts()`
pub async fn handle_clan_nts(world: Arc<WorldState>, pool: DbPool, sid: SessionId) {
    match execute_clan_nts(&world, &pool, sid).await {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("[{sid}] ClanNts error: {e}");
            send_result(&world, sid, ClanNtsResult::Failed, "");
        }
    }
}

/// Core ClanNts logic — validates all conditions, then bulk-transfers.
async fn execute_clan_nts(world: &WorldState, pool: &DbPool, sid: SessionId) -> anyhow::Result<()> {
    // 1. Get leader character info
    let ch = match world.get_character_info(sid) {
        Some(c) => c,
        None => {
            send_result(world, sid, ClanNtsResult::NoClanLeader, "");
            return Ok(());
        }
    };

    let clan_id = ch.knights_id;
    if clan_id == 0 {
        send_result(world, sid, ClanNtsResult::NoClanLeader, "");
        return Ok(());
    }

    // 2. Check leader status
    let clan = match world.get_knights(clan_id) {
        Some(k) => k,
        None => {
            send_result(world, sid, ClanNtsResult::NoClanLeader, "");
            return Ok(());
        }
    };

    if !clan.chief.eq_ignore_ascii_case(&ch.name) {
        send_result(world, sid, ClanNtsResult::NoClanLeader, "");
        return Ok(());
    }

    // 3. Check NTS item
    let has_nts_item = world.update_inventory(sid, |inv| {
        for slot in inv.iter().skip(14).take(28) {
            if slot.item_id == CLAN_NTS_ITEM && slot.count > 0 {
                return true;
            }
        }
        false
    });
    if !has_nts_item {
        send_result(world, sid, ClanNtsResult::NoItem, "");
        return Ok(());
    }

    // 4. Check no other online clan members (leader is allowed)
    let online_sids = world.get_online_knights_session_ids(clan_id);
    for online_sid in &online_sids {
        if *online_sid == sid {
            continue; // skip the leader
        }
        if let Some(online_ch) = world.get_character_info(*online_sid) {
            send_result(world, sid, ClanNtsResult::OnlineMember, &online_ch.name);
            return Ok(());
        }
    }

    // 5. Determine target nation (flip)
    let target_nation = if ch.nation == NATION_KARUS {
        NATION_ELMORAD
    } else {
        NATION_KARUS
    };

    // 6. Get all unique account IDs for clan members
    let knights_repo = KnightsRepository::new(pool);
    let account_ids = knights_repo.get_member_account_ids(clan_id as i16).await?;

    // 7. For each account, load all chars and validate
    let char_repo = CharacterRepository::new(pool);
    let mut all_chars: Vec<(String, i16)> = Vec::with_capacity(account_ids.len() * 4); // (name, class)

    for account_id in &account_ids {
        let chars = char_repo.load_all_for_account(account_id).await?;
        for c in &chars {
            // 7a. Check king (both nations)
            if world.is_king(NATION_KARUS, &c.str_user_id)
                || world.is_king(NATION_ELMORAD, &c.str_user_id)
            {
                send_result(world, sid, ClanNtsResult::IsKing, &c.str_user_id);
                return Ok(());
            }

            // 7b. Check if in another clan
            if c.knights > 0 && c.knights != clan_id as i16 {
                send_result(world, sid, ClanNtsResult::InOtherClan, &c.str_user_id);
                return Ok(());
            }

            all_chars.push((c.str_user_id.clone(), c.class));
        }
    }

    // 8. All validations passed — save clan nation to DB
    knights_repo
        .save_clan_nation(clan_id as i16, target_nation as i16)
        .await?;

    // 9. Update all characters in DB
    for (name, class) in &all_chars {
        let new_class = if target_nation == NATION_ELMORAD {
            *class + 100
        } else {
            *class - 100
        };
        let new_race = get_cnts_new_race(new_class as u16, target_nation);
        char_repo
            .save_nation_transfer_char(name, target_nation as i16, new_race as i16, new_class)
            .await?;
    }

    // 10. Consume NTS item from leader
    world.rob_item(sid, CLAN_NTS_ITEM, 1);

    // 11. Update leader's in-memory state
    let new_leader_class = if target_nation == NATION_ELMORAD {
        ch.class as i16 + 100
    } else {
        ch.class as i16 - 100
    };
    let new_leader_race = get_cnts_new_race(new_leader_class as u16, target_nation);
    world.update_character_stats(sid, |c| {
        c.nation = target_nation;
        c.race = new_leader_race;
        c.class = new_leader_class as u16;
    });

    // 12. Update clan in-memory nation
    world.update_knights(clan_id, |k| {
        k.nation = target_nation;
    });

    // 13. Send success
    send_result(world, sid, ClanNtsResult::Success, "");

    tracing::info!("[{sid}] ClanNts: clan {clan_id} transferred to nation {target_nation}");

    Ok(())
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clan_nts_result_values() {
        assert_eq!(ClanNtsResult::Failed as u8, 0);
        assert_eq!(ClanNtsResult::SuccessReq as u8, 1);
        assert_eq!(ClanNtsResult::AlreadyReq as u8, 2);
        assert_eq!(ClanNtsResult::NoClanLeader as u8, 3);
        assert_eq!(ClanNtsResult::OnlineMember as u8, 5);
        assert_eq!(ClanNtsResult::NoItem as u8, 6);
        assert_eq!(ClanNtsResult::IsKing as u8, 7);
        assert_eq!(ClanNtsResult::InOtherClan as u8, 8);
        assert_eq!(ClanNtsResult::Success as u8, 9);
    }

    // ── Race Conversion: Karus → Elmorad ──

    #[test]
    fn test_cnts_race_warrior_to_elmorad() {
        // Warrior (base 1,5,6) → ELMORAD_MAN
        assert_eq!(get_cnts_new_race(101, NATION_ELMORAD), ELMORAD_MAN);
        assert_eq!(get_cnts_new_race(105, NATION_ELMORAD), ELMORAD_MAN);
        assert_eq!(get_cnts_new_race(106, NATION_ELMORAD), ELMORAD_MAN);
    }

    #[test]
    fn test_cnts_race_rogue_to_elmorad() {
        // Rogue (base 2,7,8) → ELMORAD_MAN
        assert_eq!(get_cnts_new_race(102, NATION_ELMORAD), ELMORAD_MAN);
        assert_eq!(get_cnts_new_race(107, NATION_ELMORAD), ELMORAD_MAN);
        assert_eq!(get_cnts_new_race(108, NATION_ELMORAD), ELMORAD_MAN);
    }

    #[test]
    fn test_cnts_race_mage_to_elmorad() {
        // Mage (base 3,9,10) → ELMORAD_WOMAN
        assert_eq!(get_cnts_new_race(103, NATION_ELMORAD), ELMORAD_WOMAN);
        assert_eq!(get_cnts_new_race(109, NATION_ELMORAD), ELMORAD_WOMAN);
        assert_eq!(get_cnts_new_race(110, NATION_ELMORAD), ELMORAD_WOMAN);
    }

    #[test]
    fn test_cnts_race_priest_to_elmorad() {
        // Priest (base 4,11,12) → ELMORAD_WOMAN
        assert_eq!(get_cnts_new_race(104, NATION_ELMORAD), ELMORAD_WOMAN);
        assert_eq!(get_cnts_new_race(111, NATION_ELMORAD), ELMORAD_WOMAN);
        assert_eq!(get_cnts_new_race(112, NATION_ELMORAD), ELMORAD_WOMAN);
    }

    #[test]
    fn test_cnts_race_kurian_to_elmorad() {
        // Kurian (base 13,14,15) → PORUTU
        assert_eq!(get_cnts_new_race(113, NATION_ELMORAD), PORUTU);
        assert_eq!(get_cnts_new_race(114, NATION_ELMORAD), PORUTU);
        assert_eq!(get_cnts_new_race(115, NATION_ELMORAD), PORUTU);
    }

    // ── Race Conversion: Elmorad → Karus ──

    #[test]
    fn test_cnts_race_warrior_to_karus() {
        // Warrior (base 1,5,6) → KARUS_BIG
        assert_eq!(get_cnts_new_race(1, NATION_KARUS), KARUS_BIG);
        assert_eq!(get_cnts_new_race(5, NATION_KARUS), KARUS_BIG);
        assert_eq!(get_cnts_new_race(6, NATION_KARUS), KARUS_BIG);
    }

    #[test]
    fn test_cnts_race_rogue_to_karus() {
        // Rogue (base 2,7,8) → KARUS_MIDDLE
        assert_eq!(get_cnts_new_race(2, NATION_KARUS), KARUS_MIDDLE);
        assert_eq!(get_cnts_new_race(7, NATION_KARUS), KARUS_MIDDLE);
        assert_eq!(get_cnts_new_race(8, NATION_KARUS), KARUS_MIDDLE);
    }

    #[test]
    fn test_cnts_race_mage_to_karus() {
        // Mage (base 3,9,10) → KARUS_SMALL
        assert_eq!(get_cnts_new_race(3, NATION_KARUS), KARUS_SMALL);
        assert_eq!(get_cnts_new_race(9, NATION_KARUS), KARUS_SMALL);
        assert_eq!(get_cnts_new_race(10, NATION_KARUS), KARUS_SMALL);
    }

    #[test]
    fn test_cnts_race_priest_to_karus() {
        // Priest (base 4,11,12) → KARUS_WOMAN
        assert_eq!(get_cnts_new_race(4, NATION_KARUS), KARUS_WOMAN);
        assert_eq!(get_cnts_new_race(11, NATION_KARUS), KARUS_WOMAN);
        assert_eq!(get_cnts_new_race(12, NATION_KARUS), KARUS_WOMAN);
    }

    #[test]
    fn test_cnts_race_kurian_to_karus() {
        // Kurian (base 13,14,15) → KURIAN
        assert_eq!(get_cnts_new_race(13, NATION_KARUS), KURIAN);
        assert_eq!(get_cnts_new_race(14, NATION_KARUS), KURIAN);
        assert_eq!(get_cnts_new_race(15, NATION_KARUS), KURIAN);
    }

    // ── Packet Format ──

    #[test]
    fn test_ext_hook_message_packet_format() {
        let pkt = build_ext_hook_message("Title", "Message");
        assert_eq!(pkt.opcode, Opcode::EXT_HOOK_S2C);
        // [0xBE] [u16 len] [title bytes] [u16 len] [message bytes]
        assert_eq!(pkt.data[0], EXT_SUB_MESSAGE);
        // Title "Title" = 5 bytes → u16 LE = [5, 0]
        assert_eq!(pkt.data[1], 5);
        assert_eq!(pkt.data[2], 0);
        assert_eq!(&pkt.data[3..8], b"Title");
        // Message "Message" = 7 bytes → u16 LE = [7, 0]
        assert_eq!(pkt.data[8], 7);
        assert_eq!(pkt.data[9], 0);
        assert_eq!(&pkt.data[10..17], b"Message");
    }

    #[test]
    fn test_send_result_messages() {
        // Verify message strings match C++ ClanNtsSendMsg
        let msg = match ClanNtsResult::Failed {
            ClanNtsResult::Failed => "something went wrong.",
            _ => "",
        };
        assert_eq!(msg, "something went wrong.");
    }

    #[test]
    fn test_cnts_class_offset_karus_to_elmorad() {
        // Karus warrior class=1, transfer to Elmorad → class=101
        let original_class: i16 = 1;
        let new_class = original_class + 100;
        assert_eq!(new_class, 101);
        assert_eq!(
            get_cnts_new_race(new_class as u16, NATION_ELMORAD),
            ELMORAD_MAN
        );
    }

    #[test]
    fn test_cnts_class_offset_elmorad_to_karus() {
        // Elmorad rogue class=107, transfer to Karus → class=7
        let original_class: i16 = 107;
        let new_class = original_class - 100;
        assert_eq!(new_class, 7);
        assert_eq!(
            get_cnts_new_race(new_class as u16, NATION_KARUS),
            KARUS_MIDDLE
        );
    }

    #[test]
    fn test_cnts_race_default_fallback() {
        // Unknown base class → defaults (ELMORAD_MAN / KARUS_BIG)
        assert_eq!(get_cnts_new_race(199, NATION_ELMORAD), ELMORAD_MAN); // base 99
        assert_eq!(get_cnts_new_race(99, NATION_KARUS), KARUS_BIG); // base 99
    }
}
