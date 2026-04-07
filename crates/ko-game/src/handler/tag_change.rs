//! Tag Change handler — player name tag (title) system.
//! `UserInOutTag()`, `SendTagNameChangePanel()`
//! ## Overview
//! Players can set a custom tag (title) displayed above their character name.
//! Requires consuming item `800099000` (tag change scroll). The tag includes
//! a custom string and RGB colour.
//! ## Wire Format
//! All packets use `WIZ_EXT_HOOK (0xE9)` with sub-opcode `TagInfo = 0xD1`.
//! **Client → Server (change tag request):**
//! ```text
//! [0xE9][0xD1][u8=2 (newtag)][string newtag][u8 r][u8 g][u8 b]
//! ```
//! **Server → Client responses:**
//! ```text
//! Open    = 0: [0xE9][0xD1][u8=0]
//! List    = 1: [0xE9][0xD1][u8=1][u16 count]([string name][string tag][u8 r][u8 g][u8 b])×N
//! success = 3: [0xE9][0xD1][u8=3][u8 who][string name][string tag][u8 r][u8 g][u8 b]
//! noitem  = 4: [0xE9][0xD1][u8=4]
//! already = 5: [0xE9][0xD1][u8=5]
//! error   = 6: [0xE9][0xD1][u8=6]
//! ```

use std::sync::Arc;

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::ClientSession;
use crate::zone::SessionId;

pub(crate) use super::ext_hook::EXT_SUB_TAG_INFO;

/// Tag change scroll item ID.
const TAG_SCROLL_ITEM_ID: u32 = 800_099_000;

/// Maximum tag name length.
const MAX_TAG_LENGTH: usize = 20;

// ── tagerror sub-opcodes ────────────────────────────────────────────────────

mod tagerror {
    /// Send the tag name change panel UI to client.
    pub const OPEN: u8 = 0;
    /// Bulk tag list (sent on region INOUT_IN).
    pub const LIST: u8 = 1;
    /// Client request: change my tag.
    pub const NEWTAG: u8 = 2;
    /// Tag changed successfully.
    pub const SUCCESS: u8 = 3;
    /// Missing required item (800099000).
    pub const NOITEM: u8 = 4;
    /// New tag is identical to current tag.
    pub const ALREADY: u8 = 5;
    /// Generic error (empty name, too long, RobItem failed).
    pub const ERROR: u8 = 6;
}

// ─────────────────────────────────────────────────────────────────────────────
// Packet Builders
// ─────────────────────────────────────────────────────────────────────────────

/// Build the tag name change panel open packet.
pub fn build_tag_panel_packet() -> Packet {
    let mut pkt = Packet::new(Opcode::EXT_HOOK_S2C);
    pkt.write_u8(EXT_SUB_TAG_INFO);
    pkt.write_u8(tagerror::OPEN);
    pkt
}

/// Build a simple error response packet.
fn build_tag_error_packet(error_code: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::EXT_HOOK_S2C);
    pkt.write_u8(EXT_SUB_TAG_INFO);
    pkt.write_u8(error_code);
    pkt
}

/// Build the tag success packet.
/// `who`: 0 = self, 1 = region broadcast
fn build_tag_success_packet(
    who: u8,
    char_name: &str,
    tag_name: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Packet {
    let mut pkt = Packet::new(Opcode::EXT_HOOK_S2C);
    pkt.write_u8(EXT_SUB_TAG_INFO);
    pkt.write_u8(tagerror::SUCCESS);
    pkt.write_u8(who);
    pkt.write_string(char_name);
    pkt.write_string(tag_name);
    pkt.write_u8(r);
    pkt.write_u8(g);
    pkt.write_u8(b);
    pkt
}

/// Build the tag list packet for region INOUT_IN.
/// Each entry: `[string char_name][string tag_name][u8 r][u8 g][u8 b]`
pub fn build_tag_list_packet(entries: &[(String, String, u8, u8, u8)]) -> Packet {
    let mut pkt = Packet::new(Opcode::EXT_HOOK_S2C);
    pkt.write_u8(EXT_SUB_TAG_INFO);
    pkt.write_u8(tagerror::LIST);
    pkt.write_u16(entries.len() as u16);
    for (name, tag, r, g, b) in entries {
        pkt.write_string(name);
        pkt.write_string(tag);
        pkt.write_u8(*r);
        pkt.write_u8(*g);
        pkt.write_u8(*b);
    }
    pkt
}

// ─────────────────────────────────────────────────────────────────────────────
// Colour helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Pack RGB into a single i32 value (C++ COLORREF = RGB(r,g,b) = r | g<<8 | b<<16).
fn pack_rgb(r: u8, g: u8, b: u8) -> i32 {
    (r as i32) | ((g as i32) << 8) | ((b as i32) << 16)
}

/// Unpack a COLORREF i32 into (r, g, b).
pub fn unpack_rgb(rgb: i32) -> (u8, u8, u8) {
    let r = (rgb & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = ((rgb >> 16) & 0xFF) as u8;
    (r, g, b)
}

// ─────────────────────────────────────────────────────────────────────────────
// Handle Tag Change Request
// ─────────────────────────────────────────────────────────────────────────────

/// Handle `WIZ_EXT_HOOK (0xE9)` sub-opcode `TagInfo (0xD1)` from client.
/// Only `tagerror::newtag (2)` is handled from client side.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    // pkt.data has the sub-opcode byte already stripped by handle_ext_hook.
    // First byte is the tagerror sub-opcode.
    let mut reader = PacketReader::new(&pkt.data);
    let sub = reader.read_u8().unwrap_or(0xFF);

    match sub {
        tagerror::NEWTAG => handle_tag_change(session, &mut reader).await,
        _ => {
            debug!(
                "[{}] TagChange: unhandled sub-opcode {}",
                session.addr(),
                sub
            );
            Ok(())
        }
    }
}

/// Process a tag name change request.
async fn handle_tag_change(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let new_tag = reader.read_string().unwrap_or_default();
    let r = reader.read_u8().unwrap_or(255);
    let g = reader.read_u8().unwrap_or(255);
    let b = reader.read_u8().unwrap_or(255);

    let sid = session.session_id();
    let world = session.world().clone();

    // Validate: empty tag
    if new_tag.is_empty() {
        let pkt = build_tag_error_packet(tagerror::ERROR);
        session.send_packet(&pkt).await?;
        return Ok(());
    }

    // Validate: too long
    if new_tag.len() > MAX_TAG_LENGTH {
        let pkt = build_tag_error_packet(tagerror::ERROR);
        session.send_packet(&pkt).await?;
        return Ok(());
    }

    // Validate: same as current tag
    let current_tag = world
        .with_session(sid, |h| h.tagname.clone())
        .unwrap_or_default();
    if new_tag == current_tag {
        let pkt = build_tag_error_packet(tagerror::ALREADY);
        session.send_packet(&pkt).await?;
        return Ok(());
    }

    // Check required item (tag change scroll)
    if !world.check_exist_item(sid, TAG_SCROLL_ITEM_ID, 1) {
        let pkt = build_tag_error_packet(tagerror::NOITEM);
        session.send_packet(&pkt).await?;
        return Ok(());
    }

    // Rob item
    if !world.rob_item(sid, TAG_SCROLL_ITEM_ID, 1) {
        let pkt = build_tag_error_packet(tagerror::ERROR);
        session.send_packet(&pkt).await?;
        return Ok(());
    }

    // Get character name
    let char_name = world
        .get_character_info(sid)
        .map(|c| c.name.clone())
        .unwrap_or_default();

    // Update in-memory tag state
    let rgb_packed = pack_rgb(r, g, b);
    world.update_session(sid, |h| {
        h.tagname = new_tag.clone();
        h.tagname_rgb = rgb_packed;
    });

    // Persist to DB
    let pool = session.pool().clone();
    let char_name_db = char_name.clone();
    let tag_db = new_tag.clone();
    tokio::spawn(async move {
        let repo = ko_db::repositories::character::CharacterRepository::new(&pool);
        if let Err(e) = repo
            .update_tagname(&char_name_db, &tag_db, rgb_packed)
            .await
        {
            warn!("Failed to persist tag change for {}: {}", char_name_db, e);
        }
    });

    // Send success to self (who=0)
    let self_pkt = build_tag_success_packet(0, &char_name, &new_tag, r, g, b);
    session.send_packet(&self_pkt).await?;

    // Region broadcast (who=1)
    let region_info = world.with_session(sid, |h| {
        (
            h.position.zone_id,
            h.position.region_x,
            h.position.region_z,
            h.event_room,
        )
    });
    if let Some((zone_id, rx, rz, event_room)) = region_info {
        let broadcast = build_tag_success_packet(1, &char_name, &new_tag, r, g, b);
        world.broadcast_to_region_sync(zone_id, rx, rz, Arc::new(broadcast), Some(sid), event_room);
    }

    debug!(
        "[{}] TagChange: '{}' → tag='{}' rgb=({},{},{})",
        session.addr(),
        char_name,
        new_tag,
        r,
        g,
        b
    );
    Ok(())
}

/// Collect tag entries for visible users in a region (for INOUT_IN).
/// who have non-empty, non-"-" tag names.
/// This function is called from req_userin after GetUserInfo to send tag data
/// for nearby players. It uses `world.collect_session_tags()` which iterates
/// sessions internally (since the sessions field is private).
pub fn collect_region_tags(
    world: &crate::world::WorldState,
    zone_id: u16,
    rx: u16,
    rz: u16,
    except: Option<SessionId>,
) -> Vec<(String, String, u8, u8, u8)> {
    world.collect_session_tags(zone_id, rx, rz, except)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_rgb() {
        let (r, g, b) = (255, 128, 64);
        let packed = pack_rgb(r, g, b);
        let (ur, ug, ub) = unpack_rgb(packed);
        assert_eq!((ur, ug, ub), (r, g, b));
    }

    #[test]
    fn test_pack_rgb_black() {
        let packed = pack_rgb(0, 0, 0);
        assert_eq!(packed, 0);
    }

    #[test]
    fn test_pack_rgb_white() {
        let packed = pack_rgb(255, 255, 255);
        assert_eq!(packed, 0x00FFFFFF);
    }

    #[test]
    fn test_build_tag_panel_packet() {
        let pkt = build_tag_panel_packet();
        assert_eq!(pkt.opcode, Opcode::EXT_HOOK_S2C);
        assert_eq!(pkt.data[0], EXT_SUB_TAG_INFO);
        assert_eq!(pkt.data[1], tagerror::OPEN);
    }

    #[test]
    fn test_build_tag_error_packet() {
        let pkt = build_tag_error_packet(tagerror::NOITEM);
        assert_eq!(pkt.data[0], EXT_SUB_TAG_INFO);
        assert_eq!(pkt.data[1], tagerror::NOITEM);
    }

    #[test]
    fn test_build_tag_success_packet_self() {
        let pkt = build_tag_success_packet(0, "TestPlayer", "MyTag", 255, 0, 0);
        assert_eq!(pkt.data[0], EXT_SUB_TAG_INFO);
        assert_eq!(pkt.data[1], tagerror::SUCCESS);
        assert_eq!(pkt.data[2], 0); // who = self
    }

    #[test]
    fn test_build_tag_success_packet_broadcast() {
        let pkt = build_tag_success_packet(1, "TestPlayer", "MyTag", 0, 255, 0);
        assert_eq!(pkt.data[2], 1); // who = broadcast
    }

    #[test]
    fn test_build_tag_list_packet_empty() {
        let pkt = build_tag_list_packet(&[]);
        assert_eq!(pkt.data[0], EXT_SUB_TAG_INFO);
        assert_eq!(pkt.data[1], tagerror::LIST);
        assert_eq!(pkt.data[2], 0); // count lo
        assert_eq!(pkt.data[3], 0); // count hi
    }

    #[test]
    fn test_build_tag_list_packet_entries() {
        let entries = vec![
            ("Player1".to_string(), "Tag1".to_string(), 255u8, 0u8, 0u8),
            ("Player2".to_string(), "Tag2".to_string(), 0, 255, 0),
        ];
        let pkt = build_tag_list_packet(&entries);
        assert_eq!(pkt.data[0], EXT_SUB_TAG_INFO);
        assert_eq!(pkt.data[1], tagerror::LIST);
        // count = 2
        let count = u16::from_le_bytes([pkt.data[2], pkt.data[3]]);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_tag_scroll_item_id() {
        assert_eq!(TAG_SCROLL_ITEM_ID, 800_099_000);
    }

    #[test]
    fn test_max_tag_length() {
        assert_eq!(MAX_TAG_LENGTH, 20);
    }
}
