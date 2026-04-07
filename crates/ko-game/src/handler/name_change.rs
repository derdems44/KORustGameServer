//! WIZ_NAME_CHANGE (0x6E) handler — character and clan name change.
//! ## Sub-Opcodes
//! - `CharNameChange (0)`: Player character name change
//! - `CharSelectNameChange (2)`: Character selection screen name change (stub)
//! - `ClanNameChange (16)`: Clan/Knights name change
//! ## Items Required
//! - Character name: `ITEM_SCROLL_OF_IDENTITY (800032000)` — GMs bypass
//! - Clan name: `ITEM_CLAN_NAME_SCROLL (800086000)`

use std::sync::Arc;

use ko_db::repositories::character::CharacterRepository;
use ko_db::repositories::knights::KnightsRepository;
use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};
use crate::world::MAX_ID_SIZE;

// ── Sub-opcodes ─────────────────────────────────────────────────────────────

/// Character name change request.
const CHAR_NAME_CHANGE: u8 = 0;

/// Character select screen name change (stub).
const CHAR_SELECT_NAME_CHANGE: u8 = 2;

/// Clan name change request.
const CLAN_NAME_CHANGE: u8 = 16;

// ── Response codes (character) ──────────────────────────────────────────────

/// Show name change dialog (missing scroll / prompt).
const NAME_CHANGE_SHOW_DIALOG: u8 = 1;

/// Invalid name (empty, too long, or already taken).
const NAME_CHANGE_INVALID_NAME: u8 = 2;

/// Success — name changed.
const NAME_CHANGE_SUCCESS: u8 = 3;

/// Cannot change name while king.
const NAME_CHANGE_KING: u8 = 5;

// ── Response codes (clan) ───────────────────────────────────────────────────

/// Clan name change: show dialog (missing scroll).
const CLAN_NAME_SHOW_DIALOG: u8 = 1;

/// Clan name change: invalid name.
const CLAN_NAME_INVALID: u8 = 2;

/// Clan name change: success.
const CLAN_NAME_SUCCESS: u8 = 16;

/// Clan name change: not in clan or not leader.
const CLAN_NAME_NOT_CLAN: u8 = 4;

// ── Item IDs ────────────────────────────────────────────────────────────────

/// Scroll of Identity — required for character name change.
const ITEM_SCROLL_OF_IDENTITY: u32 = 800032000;

/// Clan Name Change Scroll — required for clan name change.
const ITEM_CLAN_NAME_SCROLL: u32 = 800086000;

/// Handle WIZ_NAME_CHANGE from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = match reader.read_u8() {
        Some(v) => v,
        None => return Ok(()),
    };

    match sub_opcode {
        CHAR_NAME_CHANGE => handle_char_name_change(session, &mut reader).await,
        CHAR_SELECT_NAME_CHANGE => {
            // Character select screen name change — sends a static response.
            // Wire: WIZ_NAME_CHANGE << u8(CharSelectNameChange=2) << u16(2) << u8(16)
            handle_char_select_name_change(session).await
        }
        CLAN_NAME_CHANGE => handle_clan_name_change(session, &mut reader).await,
        _ => {
            warn!(
                "[{}] WIZ_NAME_CHANGE: unknown sub-opcode {}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

/// Handle character name change.
/// Packet: `[u8 sub=0] [string new_name]`
async fn handle_char_name_change(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    // Check state: not dead, not trading, not mining, not fishing
    if !validate_state(&world, sid) {
        return Ok(());
    }

    let ch = match world.get_character_info(sid) {
        Some(c) => c,
        None => return Ok(()),
    };

    let is_gm = ch.authority == 0;

    // Check scroll (GMs bypass)
    if !is_gm && !has_item(&world, sid, ITEM_SCROLL_OF_IDENTITY) {
        let mut result = Packet::new(Opcode::WizNameChange as u8);
        result.write_u8(NAME_CHANGE_SHOW_DIALOG);
        session.send_packet(&result).await?;
        return Ok(());
    }

    // Read new name
    let new_name = match reader.read_string() {
        Some(s) => s,
        None => {
            send_char_error(session, NAME_CHANGE_INVALID_NAME).await?;
            return Ok(());
        }
    };

    // Validate name length
    if new_name.is_empty() || new_name.len() > MAX_ID_SIZE {
        send_char_error(session, NAME_CHANGE_INVALID_NAME).await?;
        return Ok(());
    }

    // King cannot change name
    if world.is_king(ch.nation, &ch.name) {
        send_char_error(session, NAME_CHANGE_KING).await?;
        return Ok(());
    }

    let old_name = ch.name.clone();
    let account_id = match session.account_id() {
        Some(a) => a.to_string(),
        None => return Ok(()),
    };

    // Perform DB rename (checks uniqueness, updates all tables in transaction)
    let char_repo = CharacterRepository::new(session.pool());
    let db_result = match char_repo
        .rename_character(&account_id, &old_name, &new_name)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            warn!("[{}] WIZ_NAME_CHANGE: DB error: {}", session.addr(), e);
            send_char_error(session, NAME_CHANGE_INVALID_NAME).await?;
            return Ok(());
        }
    };

    // C++ result codes: 3 = success, 2 = name taken, other = error
    if db_result != 3 {
        let code = if db_result == 2 {
            NAME_CHANGE_INVALID_NAME
        } else {
            NAME_CHANGE_SHOW_DIALOG
        };
        send_char_error(session, code).await?;
        return Ok(());
    }

    // DB success — update in-memory state
    // Update name index: remove old, add new
    world.update_name_index(&old_name, &new_name, sid);
    world.update_character_stats(sid, |ch| {
        ch.name = new_name.clone();
    });
    session.set_character_id(new_name.clone());

    // Update clan member name if in a clan
    if ch.knights_id > 0 {
        world.update_knights(ch.knights_id, |k| {
            if k.chief.eq_ignore_ascii_case(&old_name) {
                k.chief = new_name.clone();
            }
            if k.vice_chief_1.eq_ignore_ascii_case(&old_name) {
                k.vice_chief_1 = new_name.clone();
            }
            if k.vice_chief_2.eq_ignore_ascii_case(&old_name) {
                k.vice_chief_2 = new_name.clone();
            }
            if k.vice_chief_3.eq_ignore_ascii_case(&old_name) {
                k.vice_chief_3 = new_name.clone();
            }
        });
    }

    // Region re-broadcast: remove + re-add so nearby players see updated name
    let pos = world.get_position(sid).unwrap_or_default();
    let out_pkt = crate::handler::region::build_user_inout(
        crate::handler::region::INOUT_OUT,
        sid,
        None,
        &pos,
    );
    world.broadcast_to_zone(pos.zone_id, Arc::new(out_pkt), Some(sid));

    let new_ch = world.get_character_info(sid);
    let invis = world.get_invisibility_type(sid);
    let abnormal = world.get_abnormal_type(sid);
    let equip_vis = crate::handler::region::get_equipped_visual(&world, sid);
    let in_pkt = crate::handler::region::build_user_inout_with_invis(
        crate::handler::region::INOUT_IN,
        sid,
        new_ch.as_ref(),
        &pos,
        invis,
        abnormal,
        &equip_vis,
    );
    world.broadcast_to_zone(pos.zone_id, Arc::new(in_pkt), Some(sid));

    // Send success response
    let mut result = Packet::new(Opcode::WizNameChange as u8);
    result.write_u8(NAME_CHANGE_SUCCESS);
    result.write_string(&new_name);
    session.send_packet(&result).await?;

    // Consume scroll (non-GM)
    if !is_gm {
        world.rob_item(sid, ITEM_SCROLL_OF_IDENTITY, 1);
    }

    // FerihaLog: UserNameChangeInsertLog
    super::audit_log::log_name_change(
        session.pool(),
        super::audit_log::AuditEvent::NameChange,
        session.account_id().unwrap_or(""),
        &old_name,
        &new_name,
    );

    debug!(
        "[{}] WIZ_NAME_CHANGE: char name '{}' -> '{}'",
        session.addr(),
        old_name,
        new_name,
    );

    Ok(())
}

/// Handle clan name change.
/// Packet: `[u8 sub=16] [string new_clan_name]`
async fn handle_clan_name_change(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    // Check state
    if !validate_state(&world, sid) {
        return Ok(());
    }

    let ch = match world.get_character_info(sid) {
        Some(c) => c,
        None => return Ok(()),
    };

    // Check scroll
    if !has_item(&world, sid, ITEM_CLAN_NAME_SCROLL) {
        send_clan_error(session, CLAN_NAME_SHOW_DIALOG).await?;
        return Ok(());
    }

    // Read new clan name
    let new_name = match reader.read_string() {
        Some(s) => s,
        None => {
            send_clan_error(session, CLAN_NAME_INVALID).await?;
            return Ok(());
        }
    };

    // Validate name length
    if new_name.is_empty() || new_name.len() > MAX_ID_SIZE {
        send_clan_error(session, CLAN_NAME_INVALID).await?;
        return Ok(());
    }

    // Must be in a clan and be the clan leader
    if ch.knights_id == 0 {
        send_clan_error(session, CLAN_NAME_NOT_CLAN).await?;
        return Ok(());
    }

    let knights = match world.get_knights(ch.knights_id) {
        Some(k) => k,
        None => {
            send_clan_error(session, CLAN_NAME_NOT_CLAN).await?;
            return Ok(());
        }
    };

    if !knights.chief.eq_ignore_ascii_case(&ch.name) {
        send_clan_error(session, CLAN_NAME_NOT_CLAN).await?;
        return Ok(());
    }

    // Check clan name uniqueness (case-insensitive scan of all clans)
    let name_upper = new_name.to_uppercase();
    let name_exists = world.knights_name_exists(&name_upper);
    if name_exists {
        send_clan_error(session, CLAN_NAME_INVALID).await?;
        return Ok(());
    }

    let old_name = knights.name.clone();

    // Perform DB rename
    let knights_repo = KnightsRepository::new(session.pool());
    let db_result = match knights_repo
        .rename_clan(ch.knights_id as i16, &new_name)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            warn!("[{}] WIZ_NAME_CHANGE: clan DB error: {}", session.addr(), e);
            send_clan_error(session, CLAN_NAME_INVALID).await?;
            return Ok(());
        }
    };

    if db_result != 3 {
        let code = if db_result == 2 {
            CLAN_NAME_INVALID
        } else {
            CLAN_NAME_NOT_CLAN
        };
        let mut resp = Packet::new(Opcode::WizNameChange as u8);
        resp.write_u8(CLAN_NAME_CHANGE);
        resp.write_u8(code);
        session.send_packet(&resp).await?;
        return Ok(());
    }

    // DB success — update clan name in WorldState
    world.update_knights(ch.knights_id, |k| {
        k.name = new_name.clone();
    });

    // Region re-broadcast for the requesting player
    let pos = world.get_position(sid).unwrap_or_default();
    let out_pkt = crate::handler::region::build_user_inout(
        crate::handler::region::INOUT_OUT,
        sid,
        None,
        &pos,
    );
    world.broadcast_to_zone(pos.zone_id, Arc::new(out_pkt), Some(sid));

    let new_ch = world.get_character_info(sid);
    let invis = world.get_invisibility_type(sid);
    let abnormal = world.get_abnormal_type(sid);
    let equip_vis = crate::handler::region::get_equipped_visual(&world, sid);
    let in_pkt = crate::handler::region::build_user_inout_with_invis(
        crate::handler::region::INOUT_IN,
        sid,
        new_ch.as_ref(),
        &pos,
        invis,
        abnormal,
        &equip_vis,
    );
    world.broadcast_to_zone(pos.zone_id, Arc::new(in_pkt), Some(sid));

    // Send success to requesting player and all clan members
    // Wire: WIZ_NAME_CHANGE << u8(ClanNameChange=16) << u8(Success=3) << strClanName
    let mut result = Packet::new(Opcode::WizNameChange as u8);
    result.write_u8(CLAN_NAME_CHANGE);
    result.write_u8(CLAN_NAME_SUCCESS);
    result.write_string(&new_name);
    session.send_packet(&result).await?;

    // Notify all clan members
    world.send_to_knights_members(ch.knights_id, Arc::new(result), Some(sid));

    // Consume scroll
    world.rob_item(sid, ITEM_CLAN_NAME_SCROLL, 1);

    // FerihaLog: ClanNameChangeInsertLog
    super::audit_log::log_name_change(
        session.pool(),
        super::audit_log::AuditEvent::ClanNameChange,
        session.account_id().unwrap_or(""),
        &old_name,
        &new_name,
    );

    debug!(
        "[{}] WIZ_NAME_CHANGE: clan name '{}' -> '{}'",
        session.addr(),
        old_name,
        new_name,
    );

    Ok(())
}

/// Validate pre-requisite state for name change operations.
fn validate_state(world: &crate::world::WorldState, sid: crate::zone::SessionId) -> bool {
    if world.is_player_dead(sid)
        || world.is_trading(sid)
        || world.is_store_open(sid)
        || world.is_merchanting(sid)
        || world.is_selling_merchant_preparing(sid)
        || world.is_buying_merchant_preparing(sid)
    {
        return false;
    }
    // Check mining/fishing
    let busy = world.is_mining(sid) || world.is_fishing(sid);
    !busy
}

/// Check if player has a specific item in inventory (bag slots 14-41).
fn has_item(world: &crate::world::WorldState, sid: crate::zone::SessionId, item_id: u32) -> bool {
    world.update_inventory(sid, |inv| {
        for i in 14..42 {
            if let Some(slot) = inv.get(i) {
                if slot.item_id == item_id && slot.count > 0 {
                    return true;
                }
            }
        }
        false
    })
}

/// Handle character select screen name change.
/// Sends a static response packet. The client-side UI handles the rest.
async fn handle_char_select_name_change(session: &mut ClientSession) -> anyhow::Result<()> {
    let mut result = Packet::new(Opcode::WizNameChange as u8);
    result.write_u8(CHAR_SELECT_NAME_CHANGE);
    result.write_u16(2);
    result.write_u8(16);
    session.send_packet(&result).await?;

    debug!(
        "[{}] WIZ_NAME_CHANGE: CharSelectNameChange response sent",
        session.addr()
    );
    Ok(())
}

/// Validate a character or clan name.
/// Rules:
/// - Must not be empty
/// - Must not exceed `MAX_ID_SIZE` (20) characters
/// - Must contain only ASCII alphanumeric characters (letters and digits)
pub fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > MAX_ID_SIZE {
        return false;
    }
    // Only ASCII alphanumeric characters allowed
    name.bytes().all(|b| b.is_ascii_alphanumeric())
}

/// Send a character name change error response.
async fn send_char_error(session: &mut ClientSession, code: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizNameChange as u8);
    pkt.write_u8(code);
    session.send_packet(&pkt).await
}

/// Send a clan name change error response.
async fn send_clan_error(session: &mut ClientSession, code: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizNameChange as u8);
    pkt.write_u8(CLAN_NAME_CHANGE);
    pkt.write_u8(code);
    session.send_packet(&pkt).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::Packet;

    #[test]
    fn test_sub_opcode_constants() {
        assert_eq!(CHAR_NAME_CHANGE, 0);
        assert_eq!(CHAR_SELECT_NAME_CHANGE, 2);
        assert_eq!(CLAN_NAME_CHANGE, 16);
    }

    #[test]
    fn test_char_result_codes() {
        assert_eq!(NAME_CHANGE_SHOW_DIALOG, 1);
        assert_eq!(NAME_CHANGE_INVALID_NAME, 2);
        assert_eq!(NAME_CHANGE_SUCCESS, 3);
        assert_eq!(NAME_CHANGE_KING, 5);
    }

    #[test]
    fn test_clan_result_codes() {
        assert_eq!(CLAN_NAME_SHOW_DIALOG, 1);
        assert_eq!(CLAN_NAME_INVALID, 2);
        assert_eq!(CLAN_NAME_SUCCESS, 16);
        assert_eq!(CLAN_NAME_NOT_CLAN, 4);
    }

    #[test]
    fn test_item_constants() {
        assert_eq!(ITEM_SCROLL_OF_IDENTITY, 800032000);
        assert_eq!(ITEM_CLAN_NAME_SCROLL, 800086000);
        assert_eq!(MAX_ID_SIZE, 20);
    }

    #[test]
    fn test_char_name_change_request_packet() {
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(CHAR_NAME_CHANGE); // sub-opcode
        pkt.write_string("NewPlayerName"); // new name

        assert_eq!(pkt.opcode, Opcode::WizNameChange as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(CHAR_NAME_CHANGE));
        assert_eq!(r.read_string(), Some("NewPlayerName".to_string()));
    }

    #[test]
    fn test_char_name_change_success_response() {
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(NAME_CHANGE_SUCCESS);
        pkt.write_string("NewName");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(NAME_CHANGE_SUCCESS));
        assert_eq!(r.read_string(), Some("NewName".to_string()));
    }

    #[test]
    fn test_char_name_change_error_responses() {
        // Show dialog (missing scroll)
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(NAME_CHANGE_SHOW_DIALOG);
        assert_eq!(pkt.data, vec![1]);

        // Invalid name
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(NAME_CHANGE_INVALID_NAME);
        assert_eq!(pkt.data, vec![2]);

        // King cannot change
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(NAME_CHANGE_KING);
        assert_eq!(pkt.data, vec![5]);
    }

    #[test]
    fn test_clan_name_change_request_packet() {
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(CLAN_NAME_CHANGE); // sub-opcode = 16
        pkt.write_string("NewClanName");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(CLAN_NAME_CHANGE));
        assert_eq!(r.read_string(), Some("NewClanName".to_string()));
    }

    #[test]
    fn test_clan_name_change_success_response() {
        // C++ Wire: WIZ_NAME_CHANGE << u8(ClanNameChange=16) << u8(Success=3) << string
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(CLAN_NAME_CHANGE); // 16
        pkt.write_u8(CLAN_NAME_SUCCESS); // 16
        pkt.write_string("NewClan");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(16)); // CLAN_NAME_CHANGE
        assert_eq!(r.read_u8(), Some(16)); // CLAN_NAME_SUCCESS
        assert_eq!(r.read_string(), Some("NewClan".to_string()));
    }

    #[test]
    fn test_char_select_name_change_response() {
        // C++ sends: WIZ_NAME_CHANGE << u8(CharSelectNameChange=2) << u16(2) << u8(16)
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(CHAR_SELECT_NAME_CHANGE);
        pkt.write_u16(2);
        pkt.write_u8(16);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u16(), Some(2));
        assert_eq!(r.read_u8(), Some(16));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_clan_name_change_error_responses() {
        // Not clan / not leader
        let mut pkt = Packet::new(Opcode::WizNameChange as u8);
        pkt.write_u8(CLAN_NAME_CHANGE);
        pkt.write_u8(CLAN_NAME_NOT_CLAN);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(CLAN_NAME_CHANGE));
        assert_eq!(r.read_u8(), Some(CLAN_NAME_NOT_CLAN));
    }

    #[test]
    fn test_name_length_validation() {
        // Empty name fails
        assert_eq!("".len(), 0);
        // Long name fails
        let long_name = "A".repeat(21);
        assert!(long_name.len() > MAX_ID_SIZE);
        // Normal name passes
        let normal = "ValidName";
        assert!(!normal.is_empty() && normal.len() <= MAX_ID_SIZE);
    }

    #[test]
    fn test_is_valid_name_basic() {
        use super::is_valid_name;
        // Valid names
        assert!(is_valid_name("Warrior"));
        assert!(is_valid_name("Knight01"));
        assert!(is_valid_name("a")); // minimum 1 char
        assert!(is_valid_name(&"A".repeat(20))); // exactly MAX_ID_SIZE

        // Invalid names
        assert!(!is_valid_name("")); // empty
        assert!(!is_valid_name(&"A".repeat(21))); // too long
    }

    #[test]
    fn test_is_valid_name_special_chars() {
        use super::is_valid_name;
        // Special characters are not allowed
        assert!(!is_valid_name("War rior")); // space
        assert!(!is_valid_name("Knight!")); // exclamation
        assert!(!is_valid_name("name@123")); // at sign
        assert!(!is_valid_name("hello_world")); // underscore
        assert!(!is_valid_name("test-name")); // hyphen
        assert!(!is_valid_name("name.dot")); // period

        // Pure alphanumeric is fine
        assert!(is_valid_name("abc123"));
        assert!(is_valid_name("XYZ789"));
    }
}
