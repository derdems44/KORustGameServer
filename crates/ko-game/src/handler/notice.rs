//! WIZ_NOTICE (0x25) — server notice packet builders and config loading.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/User.cpp:3170-3306`
//!
//! Four notice types are sent during game start:
//! - `SendNotice()` — type=2, notice board (title+message pairs, u16 string prefix)
//! - `TopSendNotice()` — type=1, top-right notices (message only, u8 SByte prefix)
//! - `SendCapeBonusNotice()` — type=2, cape bonus info (same format as board)
//! - `SendClanPremiumNotice()` — type=2, clan premium info (same format as board)
//!
//! ## Packet Format
//!
//! ### Type 2 (notice board / cape bonus / clan premium)
//! ```text
//! WIZ_NOTICE (0x25)
//!   u8(2)          — notice type (new-style board)
//!   u8(count)      — number of entries
//!   [for each entry]:
//!     string(title)   — u16 length-prefixed
//!     string(message) — u16 length-prefixed
//! ```
//!
//! ### Type 1 (TopSendNotice — top-right text)
//! ```text
//! WIZ_NOTICE (0x25)
//!   u8(1)          — notice type (old-style top-right)
//!   u8(count)      — number of entries
//!   [for each entry]:
//!     sbyte_string(message) — u8 length-prefixed (SByte mode)
//! ```
//!
//! ## Config Loading
//!
//! Notices are loaded from a TOML config file (`notices.toml`) at server startup.
//! C++ loads from flat text files (`Notice.txt`, `Notice_up.txt`, `CapeBonus.txt`,
//! `ClanPremiumNotice.txt`). Our Rust version uses structured TOML instead.

use std::path::Path;

use ko_protocol::{Opcode, Packet};

use crate::session::ClientSession;
use crate::world::WorldState;

/// Notice type for new-style notice board (title+message pairs).
///
/// C++ Reference: `User.cpp:3175` — `result << uint8(2);`
pub const NOTICE_TYPE_BOARD: u8 = 2;

/// Notice type for old-style top-right notices (message only, SByte).
///
/// C++ Reference: `User.cpp:3249` — `result << uint8(1);`
pub const NOTICE_TYPE_TOP: u8 = 1;

/// Maximum number of notice board entries (title+message pairs).
///
/// C++ Reference: `m_ppNotice[20][128]` — 10 lines paired = 5 entries.
pub const MAX_NOTICE_BOARD_ENTRIES: usize = 5;

/// Maximum number of top-right notice entries.
///
/// C++ Reference: `m_peNotice[20][128]` — up to 20 lines.
pub const MAX_TOP_NOTICE_ENTRIES: usize = 20;

/// Build a WIZ_NOTICE type=2 packet (notice board).
///
/// C++ Reference: `User.cpp:3170-3187` — `CUser::SendNotice()`
///
/// Each entry is a (title, message) pair written as u16-prefixed strings.
/// When `entries` is empty, sends count=0 (empty notice board).
pub fn build_notice_board_packet(entries: &[(&str, &str)]) -> Packet {
    let mut pkt = Packet::new(Opcode::WizNotice as u8);
    pkt.write_u8(NOTICE_TYPE_BOARD); // type=2
    pkt.write_u8(entries.len() as u8); // count

    // C++ Reference: User.cpp:3269-3276 — AppendNoticeEntry writes title then message
    for &(title, message) in entries {
        pkt.write_string(title);
        pkt.write_string(message);
    }

    pkt
}

/// Build a WIZ_NOTICE type=2 packet from owned String pairs.
///
/// Variant of [`build_notice_board_packet`] that accepts owned strings,
/// used when reading from WorldState storage.
pub fn build_notice_board_packet_owned(entries: &[(String, String)]) -> Packet {
    let refs: Vec<(&str, &str)> = entries
        .iter()
        .map(|(t, m)| (t.as_str(), m.as_str()))
        .collect();
    build_notice_board_packet(&refs)
}

/// Build a WIZ_NOTICE type=1 packet (top-right notices).
///
/// C++ Reference: `User.cpp:3244-3260` — `CUser::TopSendNotice()`
///
/// Each entry is a message string written with u8 (SByte) length prefix.
/// When `entries` is empty, sends count=0 (no top notices).
pub fn build_top_notice_packet(entries: &[&str]) -> Packet {
    let mut pkt = Packet::new(Opcode::WizNotice as u8);
    pkt.write_u8(NOTICE_TYPE_TOP); // type=1
    pkt.write_u8(entries.len() as u8); // count

    // C++ Reference: User.cpp:3251 — pkt.SByte() switches to u8 length prefix
    // C++ Reference: User.cpp:3261-3268 — AppendNoticeEntryOld writes message as SByte string
    for &message in entries {
        pkt.write_sbyte_string(message);
    }

    pkt
}

/// Build a WIZ_NOTICE type=1 packet from owned String entries.
///
/// Variant of [`build_top_notice_packet`] that accepts owned strings.
pub fn build_top_notice_packet_owned(entries: &[String]) -> Packet {
    let refs: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
    build_top_notice_packet(&refs)
}

/// Send notice board (type=2) to client during game start.
///
/// C++ Reference: `CharacterSelectionHandler.cpp:1045` — `SendNotice();`
///
/// Reads entries from `WorldState::get_notice_board()`. Sends count=0 if
/// no entries are configured.
pub async fn send_notice(session: &mut ClientSession) -> anyhow::Result<()> {
    let entries = session.world().get_notice_board();
    let pkt = build_notice_board_packet_owned(&entries);
    session.send_packet(&pkt).await?;

    tracing::debug!(
        "[{}] Sent notice board (type=2, count={})",
        session.addr(),
        entries.len(),
    );
    Ok(())
}

/// Send top-right notices (type=1) to client during game start.
///
/// C++ Reference: `CharacterSelectionHandler.cpp:1046` — `TopSendNotice();`
///
/// Reads entries from `WorldState::get_top_notices()`. Sends count=0 if
/// no entries are configured.
pub async fn send_top_notice(session: &mut ClientSession) -> anyhow::Result<()> {
    let entries = session.world().get_top_notices();
    let pkt = build_top_notice_packet_owned(&entries);
    session.send_packet(&pkt).await?;

    tracing::debug!(
        "[{}] Sent top notice (type=1, count={})",
        session.addr(),
        entries.len(),
    );
    Ok(())
}

/// Build a cape bonus notice packet (type=2) from WorldState entries.
///
/// C++ Reference: `User.cpp:3189-3223` — `CUser::SendCapeBonusNotice()`
///
/// In C++, this is only sent to players in a clan with a castellan cape
/// that has `BonusType > 0`. The packet format is identical to the notice
/// board (type=2 with title+message pairs).
///
/// Callers (e.g. gamestart.rs) should check clan/cape conditions before
/// calling this.
pub fn build_cape_bonus_notice_packet(world: &WorldState) -> Packet {
    let entries = world.get_cape_bonus_entries();
    build_notice_board_packet_owned(&entries)
}

/// Build a clan premium notice packet (type=2) from WorldState entries.
///
/// C++ Reference: `User.cpp:3226-3242` — `CUser::SendClanPremiumNotice()`
///
/// In C++, this is only sent to players whose clan `isInPremium()`.
/// The packet format is identical to the notice board (type=2 with
/// title+message pairs).
///
/// Callers (e.g. gamestart.rs) should check clan premium status before
/// calling this.
pub fn build_clan_premium_notice_packet(world: &WorldState) -> Packet {
    let entries = world.get_clan_premium_entries();
    build_notice_board_packet_owned(&entries)
}

// ── Config Loading ─────────────────────────────────────────────────────

/// TOML config structure for server notices.
///
/// C++ loads from separate text files:
/// - `Notice.txt` (20 lines, 128 chars each) — notice board, paired as title+message
/// - `Notice_up.txt` (20 lines, 128 chars each) — top-right notices
/// - `CapeBonus.txt` (20 lines, 256 chars each) — cape bonus, paired
/// - `ClanPremiumNotice.txt` (20 lines, 128 chars each) — clan premium, paired
///
/// Our TOML format is more structured and readable.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct NoticeConfig {
    /// Notice board entries (title+message pairs), max 5.
    #[serde(default)]
    pub notice_board: Vec<NoticeEntry>,
    /// Top-right notice messages, max 20.
    #[serde(default)]
    pub top_notices: Vec<String>,
    /// Cape bonus notice entries (title+message pairs), max 5.
    #[serde(default)]
    pub cape_bonus: Vec<NoticeEntry>,
    /// Clan premium notice entries (title+message pairs), max 5.
    #[serde(default)]
    pub clan_premium: Vec<NoticeEntry>,
}

/// A single notice board entry with title and message.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct NoticeEntry {
    pub title: String,
    pub message: String,
}

/// Load notice configuration from a TOML file and populate WorldState.
///
/// C++ Reference: `CGameServerDlg::LoadNoticeData()` + `LoadNoticeUpData()`
///                + `LoadCapeBonusNotice()` + `LoadClanPremiumNotice()`
///
/// If the file does not exist or cannot be parsed, logs a warning and
/// leaves the WorldState with empty notice lists (matches C++ behavior
/// when the text files are missing).
pub fn load_notices_from_config(world: &WorldState, path: &Path) {
    match std::fs::read_to_string(path) {
        Ok(contents) => match toml::from_str::<NoticeConfig>(&contents) {
            Ok(config) => {
                let board: Vec<(String, String)> = config
                    .notice_board
                    .into_iter()
                    .take(MAX_NOTICE_BOARD_ENTRIES)
                    .filter(|e| !e.title.is_empty() && !e.message.is_empty())
                    .map(|e| (e.title, e.message))
                    .collect();
                let board_count = board.len();
                world.set_notice_board(board);

                let top: Vec<String> = config
                    .top_notices
                    .into_iter()
                    .take(MAX_TOP_NOTICE_ENTRIES)
                    .filter(|s| !s.is_empty())
                    .collect();
                let top_count = top.len();
                world.set_top_notices(top);

                let cape: Vec<(String, String)> = config
                    .cape_bonus
                    .into_iter()
                    .take(MAX_NOTICE_BOARD_ENTRIES)
                    .filter(|e| !e.title.is_empty() && !e.message.is_empty())
                    .map(|e| (e.title, e.message))
                    .collect();
                let cape_count = cape.len();
                world.set_cape_bonus_entries(cape);

                let clan: Vec<(String, String)> = config
                    .clan_premium
                    .into_iter()
                    .take(MAX_NOTICE_BOARD_ENTRIES)
                    .filter(|e| !e.title.is_empty() && !e.message.is_empty())
                    .map(|e| (e.title, e.message))
                    .collect();
                let clan_count = clan.len();
                world.set_clan_premium_entries(clan);

                tracing::info!(
                    board = board_count,
                    top = top_count,
                    cape = cape_count,
                    clan = clan_count,
                    "notice config loaded from {}",
                    path.display()
                );
            }
            Err(e) => {
                tracing::warn!("Failed to parse notice config {}: {e}", path.display());
            }
        },
        Err(_) => {
            tracing::info!(
                "Notice config {} not found, using empty notices",
                path.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ---- Type 2 (notice board) tests ----

    #[test]
    fn test_notice_board_empty() {
        let pkt = build_notice_board_packet(&[]);
        assert_eq!(pkt.opcode, Opcode::WizNotice as u8);
        assert_eq!(pkt.opcode, 0x2E);
        assert_eq!(pkt.data.len(), 2); // type(1) + count(1)

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(NOTICE_TYPE_BOARD)); // type=2
        assert_eq!(r.read_u8(), Some(0)); // count=0
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_notice_board_single_entry() {
        let pkt = build_notice_board_packet(&[("Welcome", "Hello players!")]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type
        assert_eq!(r.read_u8(), Some(1)); // count

        // Title: u16 length-prefixed string
        assert_eq!(r.read_string(), Some("Welcome".to_string()));
        // Message: u16 length-prefixed string
        assert_eq!(r.read_string(), Some("Hello players!".to_string()));

        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_notice_board_multiple_entries() {
        let entries = [
            ("Title1", "Message1"),
            ("Title2", "Message2"),
            ("Title3", "Message3"),
        ];
        let pkt = build_notice_board_packet(&entries);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type
        assert_eq!(r.read_u8(), Some(3)); // count

        for (title, message) in &entries {
            assert_eq!(r.read_string(), Some(title.to_string()));
            assert_eq!(r.read_string(), Some(message.to_string()));
        }
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_notice_board_opcode_value() {
        let pkt = build_notice_board_packet(&[]);
        assert_eq!(pkt.opcode, 0x2E);
    }

    #[test]
    fn test_notice_board_type_byte() {
        let pkt = build_notice_board_packet(&[]);
        assert_eq!(pkt.data[0], 2); // type=2
    }

    // ---- Type 1 (top notice) tests ----

    #[test]
    fn test_top_notice_empty() {
        let pkt = build_top_notice_packet(&[]);
        assert_eq!(pkt.opcode, Opcode::WizNotice as u8);
        assert_eq!(pkt.data.len(), 2); // type(1) + count(1)

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(NOTICE_TYPE_TOP)); // type=1
        assert_eq!(r.read_u8(), Some(0)); // count=0
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_top_notice_single_entry() {
        let pkt = build_top_notice_packet(&["Server restarting soon"]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type
        assert_eq!(r.read_u8(), Some(1)); // count

        // SByte string: u8 length prefix + bytes
        assert_eq!(
            r.read_sbyte_string(),
            Some("Server restarting soon".to_string())
        );

        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_top_notice_multiple_entries() {
        let entries = ["Notice1", "Notice2", "Notice3"];
        let pkt = build_top_notice_packet(&entries);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type
        assert_eq!(r.read_u8(), Some(3)); // count

        for msg in &entries {
            assert_eq!(r.read_sbyte_string(), Some(msg.to_string()));
        }
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_top_notice_opcode_value() {
        let pkt = build_top_notice_packet(&[]);
        assert_eq!(pkt.opcode, 0x2E);
    }

    #[test]
    fn test_top_notice_type_byte() {
        let pkt = build_top_notice_packet(&[]);
        assert_eq!(pkt.data[0], 1); // type=1
    }

    // ---- Cross-type tests ----

    #[test]
    fn test_notice_types_differ() {
        let board = build_notice_board_packet(&[]);
        let top = build_top_notice_packet(&[]);
        // Same opcode
        assert_eq!(board.opcode, top.opcode);
        // Different type byte
        assert_ne!(board.data[0], top.data[0]);
        assert_eq!(board.data[0], 2);
        assert_eq!(top.data[0], 1);
    }

    #[test]
    fn test_notice_board_string_uses_u16_prefix() {
        // Verify u16 length prefix (not u8 SByte)
        let pkt = build_notice_board_packet(&[("AB", "CD")]);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // type
        r.read_u8(); // count
                     // Title "AB": read via u16-prefixed read_string
        assert_eq!(r.read_string(), Some("AB".to_string()));
        // Message "CD": read via u16-prefixed read_string
        assert_eq!(r.read_string(), Some("CD".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_top_notice_string_uses_u8_prefix() {
        // Verify u8 SByte length prefix (not u16)
        let pkt = build_top_notice_packet(&["AB"]);
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8(); // type
        r.read_u8(); // count
                     // Message "AB": read via u8-prefixed read_sbyte_string
        assert_eq!(r.read_sbyte_string(), Some("AB".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_notice_board_empty_strings() {
        let pkt = build_notice_board_packet(&[("", "")]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type
        assert_eq!(r.read_u8(), Some(1)); // count=1
        assert_eq!(r.read_u16(), Some(0)); // empty title
        assert_eq!(r.read_u16(), Some(0)); // empty message
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_top_notice_empty_string() {
        let pkt = build_top_notice_packet(&[""]);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1)); // type
        assert_eq!(r.read_u8(), Some(1)); // count=1
        assert_eq!(r.read_u8(), Some(0)); // empty SByte string
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_notice_board_byte_layout() {
        // Verify exact byte layout: type(2) + count(1) + title_len(2) + "T" + msg_len(2) + "M"
        let pkt = build_notice_board_packet(&[("T", "M")]);
        assert_eq!(
            pkt.data,
            vec![
                2, // type
                1, // count
                1, 0,    // title length (u16 LE) = 1
                b'T', // title
                1, 0,    // message length (u16 LE) = 1
                b'M', // message
            ]
        );
    }

    #[test]
    fn test_top_notice_byte_layout() {
        // Verify exact byte layout: type(1) + count(1) + msg_len(1) + "X"
        let pkt = build_top_notice_packet(&["X"]);
        assert_eq!(
            pkt.data,
            vec![
                1,    // type
                1,    // count
                1,    // message length (u8 SByte) = 1
                b'X', // message
            ]
        );
    }

    // ---- WorldState integration tests ----

    #[test]
    fn test_world_notice_board_default_empty() {
        let world = WorldState::new();
        assert!(world.get_notice_board().is_empty());
        assert!(world.get_top_notices().is_empty());
        assert!(world.get_cape_bonus_entries().is_empty());
        assert!(world.get_clan_premium_entries().is_empty());
    }

    #[test]
    fn test_world_set_get_notice_board() {
        let world = WorldState::new();
        let entries = vec![
            ("Welcome".to_string(), "Hello World".to_string()),
            ("Rules".to_string(), "Be nice".to_string()),
        ];
        world.set_notice_board(entries.clone());
        let got = world.get_notice_board();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].0, "Welcome");
        assert_eq!(got[0].1, "Hello World");
        assert_eq!(got[1].0, "Rules");
        assert_eq!(got[1].1, "Be nice");
    }

    #[test]
    fn test_world_notice_board_truncates_to_5() {
        let world = WorldState::new();
        let entries: Vec<(String, String)> = (0..10)
            .map(|i| (format!("Title{i}"), format!("Msg{i}")))
            .collect();
        world.set_notice_board(entries);
        assert_eq!(world.get_notice_board().len(), MAX_NOTICE_BOARD_ENTRIES);
    }

    #[test]
    fn test_world_set_get_top_notices() {
        let world = WorldState::new();
        let entries = vec!["Notice A".to_string(), "Notice B".to_string()];
        world.set_top_notices(entries);
        let got = world.get_top_notices();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], "Notice A");
        assert_eq!(got[1], "Notice B");
    }

    #[test]
    fn test_world_top_notices_truncates_to_20() {
        let world = WorldState::new();
        let entries: Vec<String> = (0..30).map(|i| format!("Notice{i}")).collect();
        world.set_top_notices(entries);
        assert_eq!(world.get_top_notices().len(), MAX_TOP_NOTICE_ENTRIES);
    }

    #[test]
    fn test_world_set_get_cape_bonus() {
        let world = WorldState::new();
        let entries = vec![("Cape Bonus".to_string(), "+10% XP".to_string())];
        world.set_cape_bonus_entries(entries);
        let got = world.get_cape_bonus_entries();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, "Cape Bonus");
        assert_eq!(got[0].1, "+10% XP");
    }

    #[test]
    fn test_world_set_get_clan_premium() {
        let world = WorldState::new();
        let entries = vec![("Premium".to_string(), "Active until 2026".to_string())];
        world.set_clan_premium_entries(entries);
        let got = world.get_clan_premium_entries();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, "Premium");
        assert_eq!(got[0].1, "Active until 2026");
    }

    #[test]
    fn test_build_notice_board_owned() {
        let entries = vec![("Title".to_string(), "Message".to_string())];
        let pkt = build_notice_board_packet_owned(&entries);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_string(), Some("Title".to_string()));
        assert_eq!(r.read_string(), Some("Message".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_top_notice_owned() {
        let entries = vec!["Hello".to_string()];
        let pkt = build_top_notice_packet_owned(&entries);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_sbyte_string(), Some("Hello".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_cape_bonus_packet_uses_world() {
        let world = WorldState::new();
        world.set_cape_bonus_entries(vec![("Bonus".to_string(), "Attack +5".to_string())]);
        let pkt = build_cape_bonus_notice_packet(&world);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(NOTICE_TYPE_BOARD));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_string(), Some("Bonus".to_string()));
        assert_eq!(r.read_string(), Some("Attack +5".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_clan_premium_packet_uses_world() {
        let world = WorldState::new();
        world.set_clan_premium_entries(vec![("Premium".to_string(), "XP +50%".to_string())]);
        let pkt = build_clan_premium_notice_packet(&world);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(NOTICE_TYPE_BOARD));
        assert_eq!(r.read_u8(), Some(1));
        assert_eq!(r.read_string(), Some("Premium".to_string()));
        assert_eq!(r.read_string(), Some("XP +50%".to_string()));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_cape_bonus_empty_world() {
        let world = WorldState::new();
        let pkt = build_cape_bonus_notice_packet(&world);
        assert_eq!(pkt.data, vec![2, 0]); // type=2, count=0
    }

    #[test]
    fn test_build_clan_premium_empty_world() {
        let world = WorldState::new();
        let pkt = build_clan_premium_notice_packet(&world);
        assert_eq!(pkt.data, vec![2, 0]); // type=2, count=0
    }

    #[test]
    fn test_load_notices_missing_file() {
        let world = WorldState::new();
        load_notices_from_config(&world, Path::new("/nonexistent/notices.toml"));
        // Should leave everything empty without panicking
        assert!(world.get_notice_board().is_empty());
        assert!(world.get_top_notices().is_empty());
    }

    #[test]
    fn test_load_notices_from_toml() {
        let world = WorldState::new();
        let toml_content = r#"
top_notices = ["Server v2.0", "Event this weekend"]

[[notice_board]]
title = "Welcome"
message = "Hello players!"

[[notice_board]]
title = "Rules"
message = "No cheating"

[[cape_bonus]]
title = "Cape Bonus"
message = "+10% Attack"

[[clan_premium]]
title = "Premium Status"
message = "Active"
"#;
        // Parse TOML directly to avoid filesystem issues in tests
        let config: NoticeConfig = toml::from_str(toml_content).unwrap();

        let board: Vec<(String, String)> = config
            .notice_board
            .into_iter()
            .filter(|e| !e.title.is_empty() && !e.message.is_empty())
            .map(|e| (e.title, e.message))
            .collect();
        world.set_notice_board(board);

        let top: Vec<String> = config
            .top_notices
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        world.set_top_notices(top);

        let cape: Vec<(String, String)> = config
            .cape_bonus
            .into_iter()
            .filter(|e| !e.title.is_empty() && !e.message.is_empty())
            .map(|e| (e.title, e.message))
            .collect();
        world.set_cape_bonus_entries(cape);

        let clan: Vec<(String, String)> = config
            .clan_premium
            .into_iter()
            .filter(|e| !e.title.is_empty() && !e.message.is_empty())
            .map(|e| (e.title, e.message))
            .collect();
        world.set_clan_premium_entries(clan);

        let board = world.get_notice_board();
        assert_eq!(board.len(), 2);
        assert_eq!(board[0].0, "Welcome");
        assert_eq!(board[1].1, "No cheating");

        let top = world.get_top_notices();
        assert_eq!(top.len(), 2);
        assert_eq!(top[0], "Server v2.0");

        let cape = world.get_cape_bonus_entries();
        assert_eq!(cape.len(), 1);
        assert_eq!(cape[0].1, "+10% Attack");

        let clan = world.get_clan_premium_entries();
        assert_eq!(clan.len(), 1);
        assert_eq!(clan[0].0, "Premium Status");
    }

    #[test]
    fn test_load_notices_invalid_toml() {
        let world = WorldState::new();
        let tmp = std::env::temp_dir();
        let path = tmp.join("ko_notice_bad.toml");
        std::fs::write(&path, "this is not valid [[[toml").unwrap();

        load_notices_from_config(&world, &path);
        // Should leave empty without panicking
        assert!(world.get_notice_board().is_empty());

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_notices_from_file() {
        let world = WorldState::new();
        let tmp = std::env::temp_dir();
        let path = tmp.join("ko_notice_file_test.toml");
        let content = r#"
top_notices = ["TopMsg"]

[[notice_board]]
title = "Test"
message = "FileLoad"
"#;
        std::fs::write(&path, content).unwrap();

        load_notices_from_config(&world, &path);

        let board = world.get_notice_board();
        assert_eq!(
            board.len(),
            1,
            "board should have 1 entry, got {}",
            board.len()
        );
        assert_eq!(board[0].0, "Test");
        assert_eq!(board[0].1, "FileLoad");

        let top = world.get_top_notices();
        assert_eq!(top.len(), 1);
        assert_eq!(top[0], "TopMsg");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_notices_empty_entries_filtered() {
        let world = WorldState::new();
        let toml_content = r#"
top_notices = ["", "Valid notice", ""]

[[notice_board]]
title = ""
message = "Should be filtered"

[[notice_board]]
title = "Valid"
message = ""

[[notice_board]]
title = "Good"
message = "Entry"
"#;
        // Parse TOML directly to test filtering logic
        let config: NoticeConfig = toml::from_str(toml_content).unwrap();

        let board: Vec<(String, String)> = config
            .notice_board
            .into_iter()
            .take(MAX_NOTICE_BOARD_ENTRIES)
            .filter(|e| !e.title.is_empty() && !e.message.is_empty())
            .map(|e| (e.title, e.message))
            .collect();
        world.set_notice_board(board);

        let top: Vec<String> = config
            .top_notices
            .into_iter()
            .take(MAX_TOP_NOTICE_ENTRIES)
            .filter(|s| !s.is_empty())
            .collect();
        world.set_top_notices(top);

        // Empty title or message entries should be filtered out
        let board = world.get_notice_board();
        assert_eq!(board.len(), 1);
        assert_eq!(board[0].0, "Good");

        // Empty string top notices should be filtered out
        let top = world.get_top_notices();
        assert_eq!(top.len(), 1);
        assert_eq!(top[0], "Valid notice");
    }

    #[test]
    fn test_notice_config_deserialize_defaults() {
        let config: NoticeConfig = toml::from_str("").unwrap();
        assert!(config.notice_board.is_empty());
        assert!(config.top_notices.is_empty());
        assert!(config.cape_bonus.is_empty());
        assert!(config.clan_premium.is_empty());
    }

    #[test]
    fn test_notice_board_max_5_from_world() {
        let world = WorldState::new();
        let entries: Vec<(String, String)> =
            (0..8).map(|i| (format!("T{i}"), format!("M{i}"))).collect();
        world.set_notice_board(entries);
        let pkt = build_notice_board_packet_owned(&world.get_notice_board());
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(2)); // type
        assert_eq!(r.read_u8(), Some(5)); // count capped at 5
    }
}
