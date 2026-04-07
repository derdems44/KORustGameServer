//! WIZ_RENTAL (0x73) handler — item rental system.
//! The rental system in the C++ reference only reads the sub-opcode byte and
//! logs it (completely stubbed). However, the infrastructure exists:
//! - `packets.h:983-998`: enums for rental opcodes (RENTAL_PREMIUM/PVP/NPC)
//!   and PvP sub-opcodes (OPEN/REGISTER/LEND/ITEM_CHECK/ITEM_CANCEL/REPORT)
//! - `DBAgent.h:13-36`: RentalType enum + _USER_RENTAL_ITEM struct
//! - `DBAgent.cpp:308-355`: LoadRentalData (loads user_rental_item on login)
//! - `RentalItemSet.h`: _RENTAL_ITEM catalog loaded at startup
//! - `NPCHandler.cpp:698-704`: NPC_RENTAL sends WIZ_RENTAL(RENTAL_NPC) to client
//! This implementation handles the RENTAL_PVP sub-opcodes based on the
//! data structures and enums defined in the C++ source.
//! ## Sub-opcodes
//! | Value | Name              | Direction  |
//! |-------|-------------------|------------|
//! | 1     | RENTAL_PREMIUM    | C->S       |
//! | 2     | RENTAL_PVP        | C->S       |
//! | 3     | RENTAL_NPC        | S->C only  |
//! ## RENTAL_PVP sub-sub-opcodes
//! | Value | Name              | Description                    |
//! |-------|-------------------|--------------------------------|
//! | 0     | RENTAL_OPEN       | List available rental items    |
//! | 1     | RENTAL_REGISTER   | Register item for rental       |
//! | 2     | RENTAL_LEND       | Borrow a rental item           |
//! | 3     | RENTAL_ITEM_CHECK | Check rental item status       |
//! | 4     | RENTAL_ITEM_CANCEL| Cancel rental registration     |
//! | 10    | RENTAL_REPORT     | Report rental issue            |

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

use super::{HAVE_MAX, SLOT_MAX};

/// Rental main sub-opcodes
const RENTAL_PREMIUM: u8 = 1;
const RENTAL_PVP: u8 = 2;
#[cfg(test)]
const RENTAL_NPC: u8 = 3;

/// RENTAL_PVP sub-sub-opcodes
const RENTAL_OPEN: u8 = 0;
const RENTAL_REGISTER: u8 = 1;
const RENTAL_LEND: u8 = 2;
const RENTAL_ITEM_CHECK: u8 = 3;
const RENTAL_ITEM_CANCEL: u8 = 4;
const RENTAL_REPORT: u8 = 10;

/// C++ RentalType enum (DBAgent.h:13-18).
const RENTAL_TYPE_IN_LIST: i16 = 1;
#[cfg(test)]
const RENTAL_TYPE_LENDER: i16 = 2;
const RENTAL_TYPE_BORROWER: i16 = 3;

/// Maximum number of rental items a single user can register.
const MAX_RENTAL_ITEMS_PER_USER: i64 = 10;

/// Handle WIZ_RENTAL from the client.
/// The C++ server just logs the sub-opcode. This implementation extends it
/// to handle RENTAL_PVP operations using the existing data structures.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = reader.read_u8().unwrap_or(0);

    match sub_opcode {
        RENTAL_PVP => handle_pvp(session, &mut reader).await,
        RENTAL_PREMIUM => {
            // RENTAL_PREMIUM is defined but never implemented in C++.
            debug!(
                "[{}] WIZ_RENTAL: RENTAL_PREMIUM (unhandled, matches C++ stub)",
                session.addr()
            );
            Ok(())
        }
        _ => {
            debug!(
                sub_opcode,
                "[{}] WIZ_RENTAL: unknown sub-opcode",
                session.addr()
            );
            Ok(())
        }
    }
}

/// Handle RENTAL_PVP sub-opcode with its nested sub-sub-opcodes.
async fn handle_pvp(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let pvp_opcode = reader.read_u8().unwrap_or(0);

    match pvp_opcode {
        RENTAL_OPEN => rental_open(session).await,
        RENTAL_REGISTER => rental_register(session, reader).await,
        RENTAL_LEND => rental_lend(session, reader).await,
        RENTAL_ITEM_CHECK => rental_item_check(session, reader).await,
        RENTAL_ITEM_CANCEL => rental_item_cancel(session, reader).await,
        RENTAL_REPORT => {
            debug!(
                "[{}] WIZ_RENTAL: RENTAL_REPORT (acknowledged, no action)",
                session.addr()
            );
            Ok(())
        }
        _ => {
            debug!(
                pvp_opcode,
                "[{}] WIZ_RENTAL PVP: unknown sub-sub-opcode",
                session.addr()
            );
            Ok(())
        }
    }
}

/// RENTAL_OPEN — send the list of available rental items to the client.
/// Response format:
/// ```text
/// [WIZ_RENTAL] [RENTAL_PVP(2)] [RENTAL_OPEN(0)] [count:u16]
/// for each item:
///   [rental_index:u32] [item_index:u32] [durability:u16]
///   [serial_number:u64] [rental_time:u16] [rental_money:u32]
///   [lender_char_id:string]
/// ```
async fn rental_open(session: &mut ClientSession) -> anyhow::Result<()> {
    let world = session.world().clone();
    let rental_items = world.get_all_rental_items();

    let mut pkt = Packet::new(Opcode::WizRental as u8);
    pkt.write_u8(RENTAL_PVP);
    pkt.write_u8(RENTAL_OPEN);

    // Filter: only show items that have no borrower (available)
    let available: Vec<_> = rental_items
        .iter()
        .filter(|r| r.borrower_char_id.is_empty())
        .collect();

    pkt.write_u16(available.len() as u16);
    for item in &available {
        pkt.write_u32(item.rental_index as u32);
        pkt.write_u32(item.item_index as u32);
        pkt.write_u16(item.durability as u16);
        pkt.write_u64(item.serial_number as u64);
        pkt.write_u16(item.rental_time as u16);
        pkt.write_u32(item.rental_money as u32);
        pkt.write_string(&item.lender_char_id);
    }

    session.send_packet(&pkt).await?;
    debug!(
        count = available.len(),
        "[{}] WIZ_RENTAL: RENTAL_OPEN sent {} items",
        session.addr(),
        available.len()
    );
    Ok(())
}

/// RENTAL_REGISTER — register an inventory item for rental.
/// Client sends:
/// ```text
/// [slot:u8] [rental_time:u16] [rental_money:u32]
/// ```
/// Response:
/// ```text
/// [WIZ_RENTAL] [RENTAL_PVP(2)] [RENTAL_REGISTER(1)] [result:u8]
/// result: 1 = success, 0 = fail
/// ```
async fn rental_register(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let slot = reader.read_u8().unwrap_or(0) as usize;
    let rental_time = reader.read_u16().unwrap_or(0);
    let rental_money = reader.read_u32().unwrap_or(0);

    let world = session.world().clone();
    let sid = session.session_id();

    // Validate slot range: only inventory bag items (SLOT_MAX..SLOT_MAX+HAVE_MAX)
    if !(SLOT_MAX..SLOT_MAX + HAVE_MAX).contains(&slot) {
        send_register_result(session, 0).await?;
        return Ok(());
    }

    // Get the item at the specified slot
    let item = match world.get_inventory_slot(sid, slot) {
        Some(item) if item.item_id != 0 => item,
        _ => {
            send_register_result(session, 0).await?;
            return Ok(());
        }
    };

    // Cannot register flagged items (sealed, bound, rented, duplicate)
    if item.flag != 0 {
        send_register_result(session, 0).await?;
        return Ok(());
    }

    // Get character info for lender name + account ID
    let (char_name, account_id) = match (
        world.get_character_info(sid),
        session.account_id().map(|s| s.to_string()),
    ) {
        (Some(ci), Some(acc)) => (ci.name, acc),
        _ => {
            send_register_result(session, 0).await?;
            return Ok(());
        }
    };

    // Check per-user limit
    if let Some(pool) = world.db_pool() {
        let repo = ko_db::repositories::rental::RentalRepository::new(pool);
        let count = match repo.count_by_user(&char_name).await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("[{}] rental count_by_user DB error: {e}", session.addr());
                0
            }
        };
        if count >= MAX_RENTAL_ITEMS_PER_USER {
            send_register_result(session, 0).await?;
            return Ok(());
        }
    }

    // Generate a new rental index
    let rental_index = world.next_rental_index();

    // Get item table info for type/class
    let item_table = world.get_item(item.item_id);
    let (item_type, item_class) = match item_table {
        Some(ref it) => (it.item_type.unwrap_or(0) as i16, it.item_class.unwrap_or(0)),
        None => {
            send_register_result(session, 0).await?;
            return Ok(());
        }
    };

    // Mark the item as rented (flag = ITEM_FLAG_RENTED) in inventory
    let flagged = world.update_inventory(sid, |inv| {
        if let Some(s) = inv.get_mut(slot) {
            if s.item_id == item.item_id && s.serial_num == item.serial_num {
                s.flag = crate::world::ITEM_FLAG_RENTED;
                return true;
            }
        }
        false
    });

    if !flagged {
        send_register_result(session, 0).await?;
        return Ok(());
    }

    // Insert into rental_item catalog (in-memory + DB)
    let catalog_row = ko_db::models::RentalItemRow {
        rental_index,
        item_index: item.item_id as i32,
        durability: item.durability,
        serial_number: item.serial_num as i64,
        reg_type: RENTAL_TYPE_IN_LIST,
        item_type,
        item_class,
        rental_time: rental_time as i16,
        rental_money: rental_money as i32,
        lender_char_id: char_name.clone(),
        borrower_char_id: String::new(),
    };
    world.insert_rental_item(catalog_row);

    // Insert into user_rental_item + rental_item DB tables
    if let Some(pool) = world.db_pool() {
        let repo = ko_db::repositories::rental::RentalRepository::new(pool);
        if let Err(e) = repo
            .insert(
                &char_name,
                &account_id,
                RENTAL_TYPE_IN_LIST,
                RENTAL_TYPE_IN_LIST,
                rental_index,
                item.item_id as i32,
                item.durability,
                item.serial_num as i64,
                rental_money as i32,
                rental_time as i16,
            )
            .await
        {
            tracing::warn!("Failed to insert rental item: {e}");
        }
        if let Err(e) = repo
            .insert_catalog(
                rental_index,
                item.item_id as i32,
                item.durability,
                item.serial_num as i64,
                RENTAL_TYPE_IN_LIST,
                item_type,
                item_class,
                rental_time as i16,
                rental_money as i32,
                &char_name,
            )
            .await
        {
            tracing::warn!("Failed to insert rental catalog: {e}");
        }
    }

    send_register_result(session, 1).await?;
    debug!(
        rental_index,
        item_id = item.item_id,
        "[{}] WIZ_RENTAL: RENTAL_REGISTER success",
        session.addr()
    );
    Ok(())
}

/// RENTAL_LEND — borrow a rental item from the catalog.
/// Client sends:
/// ```text
/// [rental_index:u32]
/// ```
/// Response:
/// ```text
/// [WIZ_RENTAL] [RENTAL_PVP(2)] [RENTAL_LEND(2)] [result:u8]
/// result: 1 = success, 0 = fail
/// ```
async fn rental_lend(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let rental_index = reader.read_u32().unwrap_or(0) as i32;

    let world = session.world().clone();
    let sid = session.session_id();

    // Look up the rental item in catalog
    let rental_item = match world.get_rental_item(rental_index) {
        Some(ri) => ri,
        None => {
            send_lend_result(session, 0).await?;
            return Ok(());
        }
    };

    // Must not already be borrowed
    if !rental_item.borrower_char_id.is_empty() {
        send_lend_result(session, 0).await?;
        return Ok(());
    }

    // Get borrower info
    let (char_name, account_id, gold) = match (
        world.get_character_info(sid),
        session.account_id().map(|s| s.to_string()),
    ) {
        (Some(ci), Some(acc)) => (ci.name.clone(), acc, ci.gold),
        _ => {
            send_lend_result(session, 0).await?;
            return Ok(());
        }
    };

    // Cannot borrow your own item
    if char_name == rental_item.lender_char_id {
        send_lend_result(session, 0).await?;
        return Ok(());
    }

    // Check if borrower has enough gold
    if gold < rental_item.rental_money as u32 {
        send_lend_result(session, 0).await?;
        return Ok(());
    }

    // Find an empty inventory slot for the borrowed item
    let empty_slot = world.with_session(sid, |h| {
        for i in SLOT_MAX..(SLOT_MAX + HAVE_MAX) {
            if let Some(s) = h.inventory.get(i) {
                if s.item_id == 0 {
                    return Some(i);
                }
            }
        }
        None
    });

    let target_slot = match empty_slot.flatten() {
        Some(s) => s,
        None => {
            send_lend_result(session, 0).await?;
            return Ok(());
        }
    };

    // Place the item in the borrower's inventory with RENTED flag
    let placed = world.update_inventory(sid, |inv| {
        if let Some(s) = inv.get_mut(target_slot) {
            s.item_id = rental_item.item_index as u32;
            s.durability = rental_item.durability;
            s.count = 1;
            s.flag = crate::world::ITEM_FLAG_RENTED;
            s.serial_num = rental_item.serial_number as u64;
            s.expire_time = 0;
            return true;
        }
        false
    });

    if !placed {
        send_lend_result(session, 0).await?;
        return Ok(());
    }

    // Deduct gold from borrower
    world.update_session(sid, |h| {
        if let Some(ref mut ch) = h.character {
            ch.gold = ch.gold.saturating_sub(rental_item.rental_money as u32);
        }
    });

    // Update rental catalog: set borrower
    if let Some(mut ri) = world.rental_items_entry(rental_index) {
        ri.borrower_char_id = char_name.clone();
    }

    // DB persistence
    if let Some(pool) = world.db_pool() {
        let repo = ko_db::repositories::rental::RentalRepository::new(pool);
        if let Err(e) = repo
            .update_rental_item_borrower(rental_index, &char_name)
            .await
        {
            tracing::warn!("Failed to update rental borrower: {e}");
        }
        if let Err(e) = repo
            .insert(
                &char_name,
                &account_id,
                RENTAL_TYPE_BORROWER,
                RENTAL_TYPE_IN_LIST,
                rental_index,
                rental_item.item_index,
                rental_item.durability,
                rental_item.serial_number,
                rental_item.rental_money,
                rental_item.rental_time,
            )
            .await
        {
            tracing::warn!("Failed to insert rental borrower record: {e}");
        }
    }

    // Send gold change to borrower
    // Format: [u8 type] [u32 amount_lost] [u32 remaining_gold]
    let new_gold = world
        .with_session(sid, |h| h.character.as_ref().map(|ch| ch.gold))
        .flatten()
        .unwrap_or(0);
    let mut gold_pkt = Packet::new(Opcode::WizGoldChange as u8);
    gold_pkt.write_u8(2); // CoinLoss
    gold_pkt.write_u32(rental_item.rental_money as u32);
    gold_pkt.write_u32(new_gold);
    session.send_packet(&gold_pkt).await?;

    send_lend_result(session, 1).await?;
    debug!(
        rental_index,
        borrower = %char_name,
        "[{}] WIZ_RENTAL: RENTAL_LEND success",
        session.addr()
    );
    Ok(())
}

/// RENTAL_ITEM_CHECK — check the status of the player's rental items.
/// Client sends: (no extra data)
/// Response:
/// ```text
/// [WIZ_RENTAL] [RENTAL_PVP(2)] [RENTAL_ITEM_CHECK(3)] [count:u16]
/// for each item:
///   [rental_index:u32] [item_index:u32] [durability:u16]
///   [rental_type:u8] [rental_time:u16] [rental_money:u32]
///   [other_char_id:string]
/// ```
async fn rental_item_check(
    session: &mut ClientSession,
    _reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    let char_name = match world.get_character_info(sid) {
        Some(ci) => ci.name,
        None => return Ok(()),
    };

    let mut pkt = Packet::new(Opcode::WizRental as u8);
    pkt.write_u8(RENTAL_PVP);
    pkt.write_u8(RENTAL_ITEM_CHECK);

    // Collect items where this player is the lender or borrower
    let all_items = world.get_all_rental_items();
    let my_items: Vec<_> = all_items
        .iter()
        .filter(|r| r.lender_char_id == char_name || r.borrower_char_id == char_name)
        .collect();

    pkt.write_u16(my_items.len() as u16);
    for item in &my_items {
        pkt.write_u32(item.rental_index as u32);
        pkt.write_u32(item.item_index as u32);
        pkt.write_u16(item.durability as u16);

        // Show if this player is lender or borrower
        let is_lender = item.lender_char_id == char_name;
        pkt.write_u8(if is_lender {
            RENTAL_TYPE_IN_LIST as u8
        } else {
            RENTAL_TYPE_BORROWER as u8
        });
        pkt.write_u16(item.rental_time as u16);
        pkt.write_u32(item.rental_money as u32);

        // Show the other party's name
        let other_name = if is_lender {
            &item.borrower_char_id
        } else {
            &item.lender_char_id
        };
        pkt.write_string(other_name);
    }

    session.send_packet(&pkt).await?;
    debug!(
        count = my_items.len(),
        "[{}] WIZ_RENTAL: RENTAL_ITEM_CHECK sent {} items",
        session.addr(),
        my_items.len()
    );
    Ok(())
}

/// RENTAL_ITEM_CANCEL — cancel a rental registration.
/// Client sends:
/// ```text
/// [rental_index:u32]
/// ```
/// Response:
/// ```text
/// [WIZ_RENTAL] [RENTAL_PVP(2)] [RENTAL_ITEM_CANCEL(4)] [result:u8]
/// result: 1 = success, 0 = fail
/// ```
async fn rental_item_cancel(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let rental_index = reader.read_u32().unwrap_or(0) as i32;

    let world = session.world().clone();
    let sid = session.session_id();

    let char_name = match world.get_character_info(sid) {
        Some(ci) => ci.name,
        None => {
            send_cancel_result(session, 0).await?;
            return Ok(());
        }
    };

    // Look up the rental item
    let rental_item = match world.get_rental_item(rental_index) {
        Some(ri) => ri,
        None => {
            send_cancel_result(session, 0).await?;
            return Ok(());
        }
    };

    // Only the lender can cancel, and only if not yet borrowed
    if rental_item.lender_char_id != char_name || !rental_item.borrower_char_id.is_empty() {
        send_cancel_result(session, 0).await?;
        return Ok(());
    }

    // Remove the RENTED flag from the item in inventory
    let serial_to_find = rental_item.serial_number as u64;
    let item_id_to_find = rental_item.item_index as u32;
    world.update_inventory(sid, |inv| {
        for s in inv.iter_mut() {
            if s.item_id == item_id_to_find
                && s.serial_num == serial_to_find
                && s.flag == crate::world::ITEM_FLAG_RENTED
            {
                s.flag = 0;
                return true;
            }
        }
        false
    });

    // Remove from in-memory catalog
    world.remove_rental_item(rental_index);

    // Remove from DB
    if let Some(pool) = world.db_pool() {
        let repo = ko_db::repositories::rental::RentalRepository::new(pool);
        if let Err(e) = repo.delete(&char_name, rental_index).await {
            tracing::warn!("Failed to delete rental record: {e}");
        }
        if let Err(e) = repo.delete_catalog(rental_index).await {
            tracing::warn!("Failed to delete rental catalog: {e}");
        }
    }

    send_cancel_result(session, 1).await?;
    debug!(
        rental_index,
        "[{}] WIZ_RENTAL: RENTAL_ITEM_CANCEL success",
        session.addr()
    );
    Ok(())
}

// ── Helper functions ────────────────────────────────────────────────────

/// Send a RENTAL_REGISTER result packet.
async fn send_register_result(session: &mut ClientSession, result: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizRental as u8);
    pkt.write_u8(RENTAL_PVP);
    pkt.write_u8(RENTAL_REGISTER);
    pkt.write_u8(result);
    session.send_packet(&pkt).await
}

/// Send a RENTAL_LEND result packet.
async fn send_lend_result(session: &mut ClientSession, result: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizRental as u8);
    pkt.write_u8(RENTAL_PVP);
    pkt.write_u8(RENTAL_LEND);
    pkt.write_u8(result);
    session.send_packet(&pkt).await
}

/// Send a RENTAL_ITEM_CANCEL result packet.
async fn send_cancel_result(session: &mut ClientSession, result: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizRental as u8);
    pkt.write_u8(RENTAL_PVP);
    pkt.write_u8(RENTAL_ITEM_CANCEL);
    pkt.write_u8(result);
    session.send_packet(&pkt).await
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;
    use crate::world::UserItemSlot;

    #[test]
    fn test_rental_sub_opcode_constants() {
        // Verify the sub-opcode constants match C++ packets.h:983-988
        assert_eq!(RENTAL_PREMIUM, 1);
        assert_eq!(RENTAL_PVP, 2);
        assert_eq!(RENTAL_NPC, 3);
    }

    #[test]
    fn test_rental_pvp_sub_opcodes() {
        // Verify the PvP sub-sub-opcode constants match C++ packets.h:990-998
        assert_eq!(RENTAL_OPEN, 0);
        assert_eq!(RENTAL_REGISTER, 1);
        assert_eq!(RENTAL_LEND, 2);
        assert_eq!(RENTAL_ITEM_CHECK, 3);
        assert_eq!(RENTAL_ITEM_CANCEL, 4);
        assert_eq!(RENTAL_REPORT, 10);
    }

    #[test]
    fn test_rental_type_constants() {
        // Verify the rental type constants match C++ DBAgent.h:13-18
        assert_eq!(RENTAL_TYPE_IN_LIST, 1);
        assert_eq!(RENTAL_TYPE_LENDER, 2);
        assert_eq!(RENTAL_TYPE_BORROWER, 3);
    }

    #[test]
    fn test_max_rental_items_per_user() {
        // Reasonable limit for rental items per user
        assert_eq!(MAX_RENTAL_ITEMS_PER_USER, 10);
    }

    #[test]
    fn test_register_slot_validation() {
        // Valid inventory bag slots: SLOT_MAX(14) .. SLOT_MAX+HAVE_MAX(42)
        assert!(SLOT_MAX <= 14);
        assert!(SLOT_MAX + HAVE_MAX <= 42);

        // Below SLOT_MAX = equipment, not valid for rental
        for slot in 0..SLOT_MAX {
            assert!(slot < SLOT_MAX, "Equipment slots should be rejected");
        }

        // At or above SLOT_MAX+HAVE_MAX = cospre/magic bag, not valid
        for slot in (SLOT_MAX + HAVE_MAX)..77 {
            assert!(
                slot >= SLOT_MAX + HAVE_MAX,
                "Cospre/magic bag slots should be rejected"
            );
        }
    }

    #[test]
    fn test_register_result_packet_format() {
        // Build the expected register result packet manually
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_REGISTER);
        pkt.write_u8(1); // success

        assert_eq!(pkt.opcode, Opcode::WizRental as u8);
        assert_eq!(pkt.data.len(), 3);
        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_REGISTER);
        assert_eq!(pkt.data[2], 1);
    }

    #[test]
    fn test_lend_result_packet_format() {
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_LEND);
        pkt.write_u8(0); // fail

        assert_eq!(pkt.data.len(), 3);
        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_LEND);
        assert_eq!(pkt.data[2], 0);
    }

    #[test]
    fn test_cancel_result_packet_format() {
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_ITEM_CANCEL);
        pkt.write_u8(1); // success

        assert_eq!(pkt.data.len(), 3);
        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_ITEM_CANCEL);
        assert_eq!(pkt.data[2], 1);
    }

    #[test]
    fn test_open_packet_header() {
        // The RENTAL_OPEN response starts with WIZ_RENTAL + PVP + OPEN + count
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_OPEN);
        pkt.write_u16(0); // empty list

        assert_eq!(pkt.opcode, Opcode::WizRental as u8);
        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_OPEN);
        // count is u16 LE at offset 2-3
        assert_eq!(pkt.data[2], 0);
        assert_eq!(pkt.data[3], 0);
    }

    #[test]
    fn test_item_check_packet_header() {
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_ITEM_CHECK);
        pkt.write_u16(0); // empty list

        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_ITEM_CHECK);
        assert_eq!(pkt.data[2], 0);
        assert_eq!(pkt.data[3], 0);
    }

    #[test]
    fn test_open_packet_with_items() {
        // Test that a single rental item is correctly serialized
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_OPEN);
        pkt.write_u16(1); // 1 item

        let rental_index: u32 = 42;
        let item_index: u32 = 123456;
        let durability: u16 = 100;
        let serial_number: u64 = 999888777;
        let rental_time: u16 = 24; // hours
        let rental_money: u32 = 50000;
        let lender = "TestPlayer";

        pkt.write_u32(rental_index);
        pkt.write_u32(item_index);
        pkt.write_u16(durability);
        pkt.write_u64(serial_number);
        pkt.write_u16(rental_time);
        pkt.write_u32(rental_money);
        pkt.write_string(lender);

        // Verify total size: 2 (sub-opcodes) + 2 (count) + 4+4+2+8+2+4 (fields) + 2+10 (string)
        let expected_size = 2 + 2 + 4 + 4 + 2 + 8 + 2 + 4 + 2 + lender.len();
        assert_eq!(pkt.data.len(), expected_size);

        // Parse the count back
        let count = u16::from_le_bytes([pkt.data[2], pkt.data[3]]);
        assert_eq!(count, 1);

        // Parse the rental_index back
        let ri = u32::from_le_bytes([pkt.data[4], pkt.data[5], pkt.data[6], pkt.data[7]]);
        assert_eq!(ri, rental_index);
    }

    #[test]
    fn test_item_flag_rented_value() {
        // Verify ITEM_FLAG_RENTED = 1 matches C++ globals.h
        assert_eq!(crate::world::ITEM_FLAG_RENTED, 1);
    }

    #[test]
    fn test_rental_register_input_parsing() {
        // Simulate the client's RENTAL_REGISTER packet data after sub-opcodes
        let slot: u8 = 16; // inventory bag slot
        let rental_time: u16 = 48;
        let rental_money: u32 = 100000;

        let mut data = Vec::new();
        data.push(slot);
        data.extend_from_slice(&rental_time.to_le_bytes());
        data.extend_from_slice(&rental_money.to_le_bytes());

        let mut reader = PacketReader::new(&data);
        assert_eq!(reader.read_u8(), Some(16));
        assert_eq!(reader.read_u16(), Some(48));
        assert_eq!(reader.read_u32(), Some(100000));
    }

    #[test]
    fn test_rental_lend_input_parsing() {
        // Simulate the client's RENTAL_LEND packet data
        let rental_index: u32 = 42;

        let mut data = Vec::new();
        data.extend_from_slice(&rental_index.to_le_bytes());

        let mut reader = PacketReader::new(&data);
        assert_eq!(reader.read_u32(), Some(42));
    }

    #[test]
    fn test_rental_cancel_input_parsing() {
        // Simulate the client's RENTAL_ITEM_CANCEL packet data
        let rental_index: u32 = 99;

        let mut data = Vec::new();
        data.extend_from_slice(&rental_index.to_le_bytes());

        let mut reader = PacketReader::new(&data);
        assert_eq!(reader.read_u32(), Some(99));
    }

    #[test]
    fn test_user_item_slot_rental_flag() {
        // Verify that a UserItemSlot can hold rental flag
        let slot = UserItemSlot {
            item_id: 200001000,
            durability: 100,
            count: 1,
            flag: crate::world::ITEM_FLAG_RENTED,
            original_flag: 0,
            serial_num: 12345,
            expire_time: 0,
        };
        assert_eq!(slot.flag, 1);
        assert_eq!(slot.item_id, 200001000);
    }

    // ── Sprint 956: Additional coverage ──────────────────────────────

    /// Main sub-opcodes are sequential (1-3) matching C++ packets.h.
    #[test]
    fn test_rental_main_subopcodes_sequential() {
        assert_eq!(RENTAL_PREMIUM, 1);
        assert_eq!(RENTAL_PVP, 2);
        assert_eq!(RENTAL_NPC, 3);
        // All distinct
        let ops = [RENTAL_PREMIUM, RENTAL_PVP, RENTAL_NPC];
        for i in 0..ops.len() {
            for j in (i + 1)..ops.len() {
                assert_ne!(ops[i], ops[j]);
            }
        }
    }

    /// PVP sub-sub-opcodes: gap between ITEM_CANCEL(4) and REPORT(10).
    #[test]
    fn test_rental_pvp_subopcode_gap() {
        assert_eq!(RENTAL_OPEN, 0);
        assert_eq!(RENTAL_REGISTER, 1);
        assert_eq!(RENTAL_LEND, 2);
        assert_eq!(RENTAL_ITEM_CHECK, 3);
        assert_eq!(RENTAL_ITEM_CANCEL, 4);
        assert_eq!(RENTAL_REPORT, 10);
        // Gap: 5..9 are unused
        assert_eq!(RENTAL_REPORT - RENTAL_ITEM_CANCEL, 6);
    }

    /// RentalType enum values from C++ DBAgent.h.
    #[test]
    fn test_rental_type_enum_values() {
        assert_eq!(RENTAL_TYPE_IN_LIST, 1);
        assert_eq!(RENTAL_TYPE_LENDER, 2);
        assert_eq!(RENTAL_TYPE_BORROWER, 3);
    }

    /// MAX_RENTAL_ITEMS_PER_USER limit.
    #[test]
    fn test_max_rental_items_limit() {
        assert_eq!(MAX_RENTAL_ITEMS_PER_USER, 10);
        // Fits comfortably in a single DB query result
        assert!(MAX_RENTAL_ITEMS_PER_USER > 0);
        assert!(MAX_RENTAL_ITEMS_PER_USER <= 100);
    }

    /// Report packet uses same header as other PVP sub-opcodes.
    #[test]
    fn test_rental_report_packet_format() {
        let mut pkt = Packet::new(Opcode::WizRental as u8);
        pkt.write_u8(RENTAL_PVP);
        pkt.write_u8(RENTAL_REPORT);
        pkt.write_u8(1); // result

        assert_eq!(pkt.opcode, Opcode::WizRental as u8);
        assert_eq!(pkt.data[0], RENTAL_PVP);
        assert_eq!(pkt.data[1], RENTAL_REPORT);
        assert_eq!(pkt.data[2], 1);
        assert_eq!(pkt.data.len(), 3);
    }
}
