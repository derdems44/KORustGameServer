//! WIZ_MARKET_BBS (0x50) handler -- Market bulletin board system.
//! The market bulletin board allows players to post buy/sell listings
//! visible to all players on the server. Other players can browse,
//! search, and remotely purchase listed items.
//! ## Sub-opcodes
//! | Code | Name                    | Direction | Description                      |
//! |------|-------------------------|-----------|----------------------------------|
//! | 0x01 | MARKET_BBS_REGISTER     | C2S       | Post a new listing               |
//! | 0x02 | MARKET_BBS_DELETE       | C2S       | Remove own listing               |
//! | 0x03 | MARKET_BBS_REPORT       | C2S/S2C   | Report/list existing listings    |
//! | 0x04 | MARKET_BBS_OPEN         | C2S       | Open market BBS UI               |
//! | 0x05 | MARKET_BBS_REMOTE_PURCHASE | C2S    | Buy from remote listing          |
//! | 0x06 | MARKET_BBS_MESSAGE      | S2C       | Status/error message to client   |
//! ## Trade types
//! | Code | Name             | Description    |
//! |------|------------------|----------------|
//! | 0x01 | MARKET_BBS_BUY   | Buying listing |
//! | 0x02 | MARKET_BBS_SELL  | Selling listing|
//! IDA analysis (sub_A6B080): Client sends `[opcode=0x97] [sub=1] [sub=1] [u32 npc_item_id]`
//! when interacting with the market BBS NPC. The 0x50 dispatch on client side
//! triggers the C2S send, and S2C responses update the local listing cache.

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};

/// Market BBS sub-opcode constants.
const MARKET_BBS_REGISTER: u8 = 0x01;
const MARKET_BBS_DELETE: u8 = 0x02;
const MARKET_BBS_REPORT: u8 = 0x03;
const MARKET_BBS_OPEN: u8 = 0x04;
const MARKET_BBS_REMOTE_PURCHASE: u8 = 0x05;
const MARKET_BBS_MESSAGE: u8 = 0x06;

/// Market BBS trade types.
#[allow(dead_code)]
const MARKET_BBS_BUY: u8 = 0x01;
#[allow(dead_code)]
const MARKET_BBS_SELL: u8 = 0x02;

/// Result codes for market BBS operations.
const RESULT_SUCCESS: u8 = 1;
#[allow(dead_code)]
const RESULT_FAIL: u8 = 0;
#[allow(dead_code)]
const RESULT_FULL: u8 = 2;
const RESULT_NOT_FOUND: u8 = 3;
#[allow(dead_code)]
const RESULT_NO_GOLD: u8 = 4;
#[allow(dead_code)]
const RESULT_NO_SLOT: u8 = 5;

/// Maximum listings per player.
#[allow(dead_code)]
const MAX_LISTINGS_PER_PLAYER: u8 = 5;
/// Maximum listings returned in a report/search.
#[allow(dead_code)]
const MAX_REPORT_LISTINGS: u8 = 20;

/// Handle WIZ_MARKET_BBS (0x50) -- market bulletin board operations.
/// Routes to sub-handlers based on the sub-opcode byte.
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_opcode = match reader.read_u8() {
        Some(op) => op,
        None => return Ok(()),
    };

    match sub_opcode {
        MARKET_BBS_REGISTER => handle_register(session, &mut reader).await,
        MARKET_BBS_DELETE => handle_delete(session, &mut reader).await,
        MARKET_BBS_REPORT => handle_report(session, &mut reader).await,
        MARKET_BBS_OPEN => handle_open(session).await,
        MARKET_BBS_REMOTE_PURCHASE => handle_purchase(session, &mut reader).await,
        _ => {
            debug!(
                "[{}] WIZ_MARKET_BBS: unknown sub-opcode 0x{:02X}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

/// Handle MARKET_BBS_REGISTER (0x01) -- post a new listing.
/// C2S: `[u8 sub=1] [u8 trade_type] [u32 item_id] [u16 count] [u32 price] [sbyte_string title]`
/// S2C: `[u8 sub=1] [u8 result]`
async fn handle_register(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let trade_type = reader.read_u8().unwrap_or(0);
    let item_id = reader.read_u32().unwrap_or(0);
    let count = reader.read_u16().unwrap_or(0);
    let price = reader.read_u32().unwrap_or(0);
    let title = reader.read_sbyte_string().unwrap_or_default();

    debug!(
        "[{}] MARKET_BBS_REGISTER: type={}, item={}, count={}, price={}, title='{}'",
        session.addr(),
        trade_type,
        item_id,
        count,
        price,
        title
    );

    // Send success stub response -- real implementation requires DB table
    let response = build_result_packet(MARKET_BBS_REGISTER, RESULT_SUCCESS);
    session.send_packet(&response).await?;
    Ok(())
}

/// Handle MARKET_BBS_DELETE (0x02) -- remove own listing.
/// C2S: `[u8 sub=2] [u32 listing_id]`
/// S2C: `[u8 sub=2] [u8 result]`
async fn handle_delete(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let listing_id = reader.read_u32().unwrap_or(0);

    debug!(
        "[{}] MARKET_BBS_DELETE: listing_id={}",
        session.addr(),
        listing_id
    );

    let response = build_result_packet(MARKET_BBS_DELETE, RESULT_SUCCESS);
    session.send_packet(&response).await?;
    Ok(())
}

/// Handle MARKET_BBS_REPORT (0x03) -- list/search existing listings.
/// C2S: `[u8 sub=3] [u8 trade_type] [u16 page] [sbyte_string search_text]`
/// S2C: `[u8 sub=3] [u8 count] [per_listing: [u32 id] [u8 trade_type]
///        [u32 item_id] [u16 count] [u32 price] [sbyte_string seller] [sbyte_string title]]`
async fn handle_report(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let trade_type = reader.read_u8().unwrap_or(0);
    let page = reader.read_u16().unwrap_or(0);
    let search = reader.read_sbyte_string().unwrap_or_default();

    debug!(
        "[{}] MARKET_BBS_REPORT: type={}, page={}, search='{}'",
        session.addr(),
        trade_type,
        page,
        search
    );

    // Send empty listing -- no DB backend yet
    let response = build_empty_report_packet();
    session.send_packet(&response).await?;
    Ok(())
}

/// Handle MARKET_BBS_OPEN (0x04) -- open market BBS UI.
/// C2S: `[u8 sub=4]`
/// S2C: `[u8 sub=4] [u8 result(1=ok)]`
async fn handle_open(session: &mut ClientSession) -> anyhow::Result<()> {
    debug!("[{}] MARKET_BBS_OPEN", session.addr());

    let response = build_result_packet(MARKET_BBS_OPEN, RESULT_SUCCESS);
    session.send_packet(&response).await?;
    Ok(())
}

/// Handle MARKET_BBS_REMOTE_PURCHASE (0x05) -- buy from remote listing.
/// C2S: `[u8 sub=5] [u32 listing_id]`
/// S2C: `[u8 sub=5] [u8 result]`
async fn handle_purchase(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let listing_id = reader.read_u32().unwrap_or(0);

    debug!(
        "[{}] MARKET_BBS_REMOTE_PURCHASE: listing_id={}",
        session.addr(),
        listing_id
    );

    // For now, report not found since there's no listing DB
    let response = build_result_packet(MARKET_BBS_REMOTE_PURCHASE, RESULT_NOT_FOUND);
    session.send_packet(&response).await?;
    Ok(())
}

/// Build a simple result packet for market BBS operations.
/// Format: `[u8 sub_opcode] [u8 result]`
fn build_result_packet(sub_opcode: u8, result: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizMarketBbs as u8);
    pkt.write_u8(sub_opcode);
    pkt.write_u8(result);
    pkt
}

/// Build an empty report (no listings found).
/// Format: `[u8 sub=3] [u8 count=0]`
fn build_empty_report_packet() -> Packet {
    let mut pkt = Packet::new(Opcode::WizMarketBbs as u8);
    pkt.write_u8(MARKET_BBS_REPORT);
    pkt.write_u8(0); // count = 0
    pkt
}

/// Build a MARKET_BBS_MESSAGE S2C packet (status/error notification).
/// Format: `[u8 sub=6] [u8 message_type] [sbyte_string message]`
pub fn build_message_packet(message_type: u8, message: &str) -> Packet {
    let mut pkt = Packet::new(Opcode::WizMarketBbs as u8);
    pkt.write_u8(MARKET_BBS_MESSAGE);
    pkt.write_u8(message_type);
    pkt.write_sbyte_string(message);
    pkt
}

#[cfg(test)]
mod tests {
    use ko_protocol::{Opcode, PacketReader};

    use super::*;

    #[test]
    fn test_market_bbs_opcode_value() {
        assert_eq!(Opcode::WizMarketBbs as u8, 0x50);
    }

    #[test]
    fn test_market_bbs_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x50), Some(Opcode::WizMarketBbs));
    }

    #[test]
    fn test_market_bbs_sub_opcodes() {
        assert_eq!(MARKET_BBS_REGISTER, 0x01);
        assert_eq!(MARKET_BBS_DELETE, 0x02);
        assert_eq!(MARKET_BBS_REPORT, 0x03);
        assert_eq!(MARKET_BBS_OPEN, 0x04);
        assert_eq!(MARKET_BBS_REMOTE_PURCHASE, 0x05);
        assert_eq!(MARKET_BBS_MESSAGE, 0x06);
    }

    #[test]
    fn test_market_bbs_trade_types() {
        assert_eq!(MARKET_BBS_BUY, 0x01);
        assert_eq!(MARKET_BBS_SELL, 0x02);
    }

    #[test]
    fn test_result_packet_format() {
        let pkt = build_result_packet(MARKET_BBS_REGISTER, RESULT_SUCCESS);
        assert_eq!(pkt.opcode, Opcode::WizMarketBbs as u8);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(MARKET_BBS_REGISTER));
        assert_eq!(reader.read_u8(), Some(RESULT_SUCCESS));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_empty_report_packet_format() {
        let pkt = build_empty_report_packet();
        assert_eq!(pkt.opcode, Opcode::WizMarketBbs as u8);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(MARKET_BBS_REPORT));
        assert_eq!(reader.read_u8(), Some(0)); // count=0
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_message_packet_format() {
        let pkt = build_message_packet(1, "Listing posted!");
        assert_eq!(pkt.opcode, Opcode::WizMarketBbs as u8);
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(MARKET_BBS_MESSAGE));
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(
            reader.read_sbyte_string().as_deref(),
            Some("Listing posted!")
        );
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_result_packet_data_length() {
        let pkt = build_result_packet(0x01, 0x01);
        assert_eq!(pkt.data.len(), 2); // sub(1) + result(1)
    }

    #[test]
    fn test_result_codes_distinct() {
        let codes = [
            RESULT_SUCCESS,
            RESULT_FAIL,
            RESULT_FULL,
            RESULT_NOT_FOUND,
            RESULT_NO_GOLD,
            RESULT_NO_SLOT,
        ];
        for i in 0..codes.len() {
            for j in (i + 1)..codes.len() {
                assert_ne!(codes[i], codes[j], "Result codes must be distinct");
            }
        }
    }

    #[test]
    fn test_result_packet_all_sub_opcodes() {
        for sub in [
            MARKET_BBS_REGISTER,
            MARKET_BBS_DELETE,
            MARKET_BBS_REPORT,
            MARKET_BBS_OPEN,
            MARKET_BBS_REMOTE_PURCHASE,
        ] {
            let pkt = build_result_packet(sub, RESULT_SUCCESS);
            let mut r = PacketReader::new(&pkt.data);
            assert_eq!(r.read_u8(), Some(sub));
            assert_eq!(r.read_u8(), Some(RESULT_SUCCESS));
        }
    }

    #[test]
    fn test_message_packet_empty_message() {
        let pkt = build_message_packet(0, "");
        let mut reader = PacketReader::new(&pkt.data);
        assert_eq!(reader.read_u8(), Some(MARKET_BBS_MESSAGE));
        assert_eq!(reader.read_u8(), Some(0));
        assert_eq!(reader.read_sbyte_string().as_deref(), Some(""));
    }

    #[test]
    fn test_max_listings_constant() {
        assert!(MAX_LISTINGS_PER_PLAYER > 0);
        assert!(MAX_REPORT_LISTINGS > 0);
        assert!(MAX_REPORT_LISTINGS >= MAX_LISTINGS_PER_PLAYER);
    }
}
