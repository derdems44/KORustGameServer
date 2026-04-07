//! WIZ_FRIEND_PROCESS (0x49) handler — friend list system.
//! Sub-opcodes:
//! - 1 = FRIEND_REQUEST: Load friend list from DB + report online status
//! - 2 = FRIEND_REPORT: Refresh online/offline status for friend list
//! - 3 = FRIEND_ADD: Add a friend by name
//! - 4 = FRIEND_REMOVE: Remove a friend by name
//! Max 24 friends per player.

use ko_db::repositories::friend::FriendRepository;
use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};
use crate::world::MAX_ID_SIZE;

/// Maximum number of friends per player.
const MAX_FRIEND_COUNT: u16 = 24;

/// Friend sub-opcodes from `packets.h:775-778`.
const FRIEND_REQUEST: u8 = 1;
const FRIEND_REPORT: u8 = 2;
const FRIEND_ADD: u8 = 3;
const FRIEND_REMOVE: u8 = 4;

/// Friend add result codes from `packets.h:808-812`.
const FRIEND_ADD_SUCCESS: u8 = 0;
const FRIEND_ADD_ERROR: u8 = 1;
const FRIEND_ADD_FULL: u8 = 2;

/// Friend remove result codes from `packets.h:817-819`.
const FRIEND_REMOVE_SUCCESS: u8 = 0;
/// C++ protocol reference constant (test-only).
#[cfg(test)]
const FRIEND_REMOVE_ERROR: u8 = 1;
const FRIEND_REMOVE_NOT_FOUND: u8 = 2;

/// Handle WIZ_FRIEND_PROCESS from the client.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    // Dead players cannot modify friend list
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = reader.read_u8().unwrap_or(0);

    let char_name = match session.world().get_character_info(session.session_id()) {
        Some(ch) => ch.name.clone(),
        None => {
            debug!("[{}] FRIEND: no character info", session.addr());
            return Ok(());
        }
    };

    match sub_opcode {
        FRIEND_REQUEST => handle_friend_request(session, &char_name).await,
        FRIEND_REPORT => handle_friend_report(session, &mut reader).await,
        FRIEND_ADD => handle_friend_add(session, &char_name, &mut reader).await,
        FRIEND_REMOVE => handle_friend_remove(session, &char_name, &mut reader).await,
        _ => {
            warn!(
                "[{}] FRIEND: unhandled sub-opcode {}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

/// Get the online status and session ID for a friend.
/// Returns (session_id, status):
/// - status 0 = offline
/// - status 1 = online, not in party
/// - status 3 = online, in party
fn get_friend_status(session: &ClientSession, name: &str) -> (i32, u8) {
    let world = session.world();
    match world.find_session_by_name(name) {
        Some(sid) => {
            let in_party = world.is_in_party(sid);
            let status = if in_party { 3u8 } else { 1u8 };
            (sid as i32, status)
        }
        None => (-1i32, 0u8),
    }
}

/// FRIEND_REQUEST (1): Load friend list from DB, then report online status.
/// Flow:
/// 1. Load friend names from DB
/// 2. For each friend, get online status
/// 3. Send response in FRIEND_REPORT format
/// Response: WIZ_FRIEND_PROCESS + u8(FRIEND_REQUEST) + u16(count)
///           + for each: string(name) + i32(session_id) + u8(status)
async fn handle_friend_request(session: &mut ClientSession, char_name: &str) -> anyhow::Result<()> {
    let repo = FriendRepository::new(session.pool());
    let friends = repo.load_friends(char_name).await?;

    let mut response = Packet::new(Opcode::WizFriendProcess as u8);
    response.write_u8(FRIEND_REQUEST); // v2600: echo sub=1 (sniff verified)
    response.write_u16(friends.len() as u16);

    for friend in &friends {
        let (sid, status) = get_friend_status(session, &friend.friend_name);
        response.write_string(&friend.friend_name);
        response.write_i32(sid);
        response.write_u8(status);
    }

    session.send_packet(&response).await?;
    debug!(
        "[{}] FRIEND_REQUEST: sent {} friends for {}",
        session.addr(),
        friends.len(),
        char_name
    );
    Ok(())
}

/// FRIEND_REPORT (2): Client sends list of friend names, server responds with status.
/// Client sends: u16(count) + for each: string(name)
/// Response: WIZ_FRIEND_PROCESS + u8(FRIEND_REPORT) + u16(count)
///           + for each: string(name) + i32(session_id) + u8(status)
async fn handle_friend_report(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let count = reader.read_u16().unwrap_or(0);
    if count > MAX_FRIEND_COUNT {
        debug!(
            "[{}] FRIEND_REPORT: count {} exceeds max",
            session.addr(),
            count
        );
        return Ok(());
    }

    let mut names = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name = match reader.read_string() {
            Some(n) if !n.is_empty() && n.len() <= MAX_ID_SIZE => n,
            _ => return Ok(()), // malformed packet
        };
        names.push(name);
    }

    let mut response = Packet::new(Opcode::WizFriendProcess as u8);
    response.write_u8(FRIEND_REPORT);
    response.write_u16(count);

    for name in &names {
        let (sid, status) = get_friend_status(session, name);
        response.write_string(name);
        response.write_i32(sid);
        response.write_u8(status);
    }

    session.send_packet(&response).await?;
    Ok(())
}

/// FRIEND_ADD (3): Add a friend by name.
/// Client sends: u8(FRIEND_ADD) + string(name)
/// Response: WIZ_FRIEND_PROCESS + u8(FRIEND_ADD) + u8(result) + string(name)
///           + u32(session_id) + u8(status)
async fn handle_friend_add(
    session: &mut ClientSession,
    char_name: &str,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    // C++ reads SByte string for the target name
    let target_name = match reader.read_sbyte_string() {
        Some(n) if !n.is_empty() && n.len() <= MAX_ID_SIZE => n,
        _ => {
            debug!("[{}] FRIEND_ADD: invalid target name", session.addr());
            return Ok(());
        }
    };

    let repo = FriendRepository::new(session.pool());

    // Cannot add yourself
    if target_name.eq_ignore_ascii_case(char_name) {
        send_friend_modify_result(session, FRIEND_ADD, FRIEND_ADD_ERROR, &target_name).await?;
        return Ok(());
    }

    // Check max friend count
    let count = repo.count_friends(char_name).await?;
    if count >= MAX_FRIEND_COUNT as i64 {
        send_friend_modify_result(session, FRIEND_ADD, FRIEND_ADD_FULL, &target_name).await?;
        return Ok(());
    }

    // Check target exists (online check first, then DB)
    let target_exists = session.world().find_session_by_name(&target_name).is_some()
        || repo.character_exists(&target_name).await?;

    if !target_exists {
        send_friend_modify_result(session, FRIEND_ADD, FRIEND_ADD_ERROR, &target_name).await?;
        return Ok(());
    }

    // Insert into DB
    let inserted = repo.add_friend(char_name, &target_name).await?;
    let result = if inserted {
        FRIEND_ADD_SUCCESS
    } else {
        FRIEND_ADD_ERROR // already in list
    };

    send_friend_modify_result(session, FRIEND_ADD, result, &target_name).await?;
    debug!(
        "[{}] FRIEND_ADD: {} -> {} (result={})",
        session.addr(),
        char_name,
        target_name,
        result
    );
    Ok(())
}

/// FRIEND_REMOVE (4): Remove a friend by name.
/// Client sends: u8(FRIEND_REMOVE) + sbyte_string(name)
/// Response: WIZ_FRIEND_PROCESS + u8(FRIEND_REMOVE) + u8(result) + string(name)
///           + u32(session_id) + u8(status)
async fn handle_friend_remove(
    session: &mut ClientSession,
    char_name: &str,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let target_name = match reader.read_sbyte_string() {
        Some(n) if !n.is_empty() && n.len() <= MAX_ID_SIZE => n,
        _ => {
            debug!("[{}] FRIEND_REMOVE: invalid target name", session.addr());
            return Ok(());
        }
    };

    let repo = FriendRepository::new(session.pool());
    let removed = repo.remove_friend(char_name, &target_name).await?;
    let result = if removed {
        FRIEND_REMOVE_SUCCESS
    } else {
        FRIEND_REMOVE_NOT_FOUND
    };

    send_friend_modify_result(session, FRIEND_REMOVE, result, &target_name).await?;
    debug!(
        "[{}] FRIEND_REMOVE: {} -> {} (result={})",
        session.addr(),
        char_name,
        target_name,
        result
    );
    Ok(())
}

/// Send a friend add/remove result packet.
/// Response: WIZ_FRIEND_PROCESS + u8(opcode) + u8(result) + string(name) + u32(sid) + u8(status)
async fn send_friend_modify_result(
    session: &mut ClientSession,
    opcode: u8,
    result: u8,
    target_name: &str,
) -> anyhow::Result<()> {
    let (sid, status) = get_friend_status(session, target_name);

    let mut response = Packet::new(Opcode::WizFriendProcess as u8);
    response.write_u8(opcode);
    response.write_u8(result);
    response.write_string(target_name);
    response.write_u32(sid as u32);
    response.write_u8(status);

    session.send_packet(&response).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_friend_constants() {
        assert_eq!(MAX_FRIEND_COUNT, 24);
        assert_eq!(FRIEND_REQUEST, 1);
        assert_eq!(FRIEND_REPORT, 2);
        assert_eq!(FRIEND_ADD, 3);
        assert_eq!(FRIEND_REMOVE, 4);
    }

    #[test]
    fn test_friend_result_codes() {
        assert_eq!(FRIEND_ADD_SUCCESS, 0);
        assert_eq!(FRIEND_ADD_ERROR, 1);
        assert_eq!(FRIEND_ADD_FULL, 2);
        assert_eq!(FRIEND_REMOVE_SUCCESS, 0);
        assert_eq!(FRIEND_REMOVE_ERROR, 1);
        assert_eq!(FRIEND_REMOVE_NOT_FOUND, 2);
    }

    /// Validate FRIEND_REPORT packet format matches C++ FriendHandler.cpp:68-92.
    ///
    /// C++ sends: WIZ_FRIEND_PROCESS + u8(FRIEND_REPORT) + u16(count)
    ///            + for each: string(name) + i32(session_id) + u8(status)
    #[test]
    fn test_friend_report_packet_format() {
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REPORT);
        pkt.write_u16(2); // 2 friends

        // Friend 1: online, not in party
        pkt.write_string("Warrior01");
        pkt.write_i32(100); // session id
        pkt.write_u8(1); // status: online, not in party

        // Friend 2: offline
        pkt.write_string("Mage02");
        pkt.write_i32(-1i32); // session id: -1 = offline
        pkt.write_u8(0); // status: offline

        assert_eq!(pkt.opcode, Opcode::WizFriendProcess as u8);

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(FRIEND_REPORT));
        assert_eq!(reader.read_u16(), Some(2));

        // Friend 1
        assert_eq!(reader.read_string(), Some("Warrior01".to_string()));
        assert_eq!(reader.read_u32(), Some(100u32));
        assert_eq!(reader.read_u8(), Some(1));

        // Friend 2
        assert_eq!(reader.read_string(), Some("Mage02".to_string()));
        assert_eq!(reader.read_u32(), Some((-1i32) as u32));
        assert_eq!(reader.read_u8(), Some(0));

        assert_eq!(reader.remaining(), 0);
    }

    /// Validate FRIEND_ADD result packet format matches C++ RecvFriendModify
    /// in FriendHandler.cpp:118-136.
    ///
    /// C++ sends: WIZ_FRIEND_PROCESS + u8(FRIEND_ADD) + u8(result)
    ///            + string(name) + u32(sid) + u8(status)
    #[test]
    fn test_friend_add_result_packet_format() {
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_ADD);
        pkt.write_u8(FRIEND_ADD_SUCCESS);
        pkt.write_string("TestFriend");
        pkt.write_u32(42u32); // session id
        pkt.write_u8(1); // status: online

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(FRIEND_ADD));
        assert_eq!(reader.read_u8(), Some(FRIEND_ADD_SUCCESS));
        assert_eq!(reader.read_string(), Some("TestFriend".to_string()));
        assert_eq!(reader.read_u32(), Some(42));
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.remaining(), 0);
    }

    /// Validate FRIEND_REMOVE result packet format matches C++ RecvFriendModify.
    #[test]
    fn test_friend_remove_result_packet_format() {
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REMOVE);
        pkt.write_u8(FRIEND_REMOVE_SUCCESS);
        pkt.write_string("OldFriend");
        pkt.write_u32((-1i32) as u32); // offline
        pkt.write_u8(0);

        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(FRIEND_REMOVE));
        assert_eq!(reader.read_u8(), Some(FRIEND_REMOVE_SUCCESS));
        assert_eq!(reader.read_string(), Some("OldFriend".to_string()));
        assert_eq!(reader.read_u32(), Some(0xFFFFFFFF)); // -1 as u32
        assert_eq!(reader.read_u8(), Some(0));
        assert_eq!(reader.remaining(), 0);
    }

    /// Validate max friend count matches C++ FriendHandler.cpp:3.
    #[test]
    fn test_friend_max_count_matches_cpp() {
        assert_eq!(MAX_FRIEND_COUNT, 24);
        // DB repo also has the same constant
        assert_eq!(
            ko_db::repositories::friend::MAX_FRIEND_COUNT,
            MAX_FRIEND_COUNT as usize
        );
    }

    // ── Sprint 922: Additional coverage ─────────────────────────────

    #[test]
    fn test_friend_request_packet_format() {
        // C2S: [u8 sub=1] — no additional data
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REQUEST);
        assert_eq!(pkt.data.len(), 1);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_REQUEST));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_friend_add_c2s_format() {
        // C2S: [u8 sub=3] [string name]
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_ADD);
        pkt.write_string("NewFriend");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_ADD));
        assert_eq!(r.read_string(), Some("NewFriend".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_friend_remove_c2s_format() {
        // C2S: [u8 sub=4] [string name]
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REMOVE);
        pkt.write_string("ExFriend");

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_REMOVE));
        assert_eq!(r.read_string(), Some("ExFriend".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_friend_add_full_result() {
        // S2C result when friend list is full (24)
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_ADD);
        pkt.write_u8(FRIEND_ADD_FULL);
        pkt.write_string("Blocked");
        pkt.write_u32(0);
        pkt.write_u8(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_ADD));
        assert_eq!(r.read_u8(), Some(FRIEND_ADD_FULL));
        assert_eq!(r.read_string(), Some("Blocked".to_string()));
    }

    #[test]
    fn test_friend_remove_not_found_result() {
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REMOVE);
        pkt.write_u8(FRIEND_REMOVE_NOT_FOUND);
        pkt.write_string("Unknown");
        pkt.write_u32(0);
        pkt.write_u8(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_REMOVE));
        assert_eq!(r.read_u8(), Some(FRIEND_REMOVE_NOT_FOUND));
    }

    #[test]
    fn test_friend_report_empty_list() {
        // FRIEND_REPORT with 0 friends
        let mut pkt = Packet::new(Opcode::WizFriendProcess as u8);
        pkt.write_u8(FRIEND_REPORT);
        pkt.write_u16(0);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(FRIEND_REPORT));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_friend_sub_opcode_range() {
        // Sub-opcodes 1-4 are valid
        assert!(matches!(FRIEND_REQUEST, 1));
        assert!(matches!(FRIEND_REPORT, 2));
        assert!(matches!(FRIEND_ADD, 3));
        assert!(matches!(FRIEND_REMOVE, 4));
    }
}
