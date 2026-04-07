//! Sheriff (WIZ_REPORT) handler -- player reporting and voting system.
//!
//! C++ Reference: `SheriffHandler.cpp` -- `CUser::SheriffVote()`
//!
//! Sub-opcodes:
//! | Code | Name            | Description                                    |
//! |------|-----------------|------------------------------------------------|
//! | 9    | ReportSuccess   | King/GM files a report against a player        |
//! | 12   | ReportAgree     | King/GM casts a "yes" vote on existing report  |
//! | 13   | ReportDisagree  | King/GM casts a "no" vote on existing report   |
//! | 14   | ListOpen        | Opens paginated sheriff report list             |
//! | 18   | KingsInspector  | Opens the Kings Inspector UI                   |

use dashmap::DashMap;
use ko_protocol::{Opcode, Packet, PacketReader};
use std::sync::Arc;
use tracing::debug;

use crate::session::{ClientSession, SessionState};
use crate::world::WorldState;

/// Sub-opcodes for the sheriff system.
pub mod sub_opcode {
    pub const REPORT_FAILED: u8 = 0;
    pub const REPORT_SUCCESS: u8 = 9;
    pub const VOTING_FAILED: u8 = 10;
    pub const REPORT_AGREE: u8 = 12;
    pub const REPORT_DISAGREE: u8 = 13;
    pub const LIST_OPEN: u8 = 14;
    pub const KINGS_INSPECTOR: u8 = 18;
    pub const QUESTION_NOT_ANSWERED: u8 = 6;
}

/// Maximum length of a report reason string.
const MAX_ID_REPORT: usize = 512;
/// Maximum reports displayed per page.
const REPORTS_PER_PAGE: u8 = 8;

/// In-memory sheriff report entry (matches C++ `_SHERIFF_STUFF`).
#[derive(Debug, Clone)]
pub struct SheriffReport {
    /// Name of the reported player.
    pub reported_name: String,
    /// Name of the sheriff/king who filed the report.
    pub reporter_name: String,
    /// Reason for the report.
    pub crime: String,
    /// Number of "yes" votes.
    pub vote_yes: u8,
    /// Names of up to 3 "yes" voters (A = reporter, B/C = additional).
    pub yes_voters: [String; 3],
    /// Number of "no" votes.
    pub vote_no: u8,
    /// Names of up to 3 "no" voters.
    pub no_voters: [String; 3],
    /// Date/time string when the report was filed.
    pub date: String,
}

impl SheriffReport {
    /// Check if a voter name already exists in the yes or no voter lists.
    pub fn has_voted(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        self.yes_voters.iter().any(|v| v == name) || self.no_voters.iter().any(|v| v == name)
    }
}

/// Sheriff report storage (in-memory, keyed by reported player name).
pub type SheriffReportMap = DashMap<String, SheriffReport>;

/// Create a new empty sheriff report map.
pub fn new_sheriff_map() -> Arc<SheriffReportMap> {
    Arc::new(DashMap::new())
}

/// Check if a session belongs to a king of their nation.
fn is_session_king(world: &WorldState, sid: crate::zone::SessionId) -> bool {
    world
        .with_session(sid, |h| {
            h.character
                .as_ref()
                .map(|c| world.is_king(c.nation, &c.name))
        })
        .flatten()
        .unwrap_or(false)
}

/// Handle incoming WIZ_REPORT packet.
///
/// C++ Reference: `CUser::SheriffVote(Packet & pkt)` in `SheriffHandler.cpp`
pub async fn handle(session: &mut ClientSession, pkt: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&pkt.data);
    let sub_op = match reader.read_u8() {
        Some(v) => v,
        None => return Ok(()),
    };

    match sub_op {
        sub_opcode::REPORT_SUCCESS => handle_report_success(session, &mut reader).await,
        sub_opcode::REPORT_AGREE => handle_report_vote(session, &mut reader, true).await,
        sub_opcode::REPORT_DISAGREE => handle_report_vote(session, &mut reader, false).await,
        sub_opcode::LIST_OPEN => handle_list_open(session, &mut reader).await,
        sub_opcode::KINGS_INSPECTOR => handle_kings_inspector(session).await,
        _ => {
            debug!(
                "[{}] Unhandled WIZ_REPORT sub-opcode: 0x{:02X}",
                session.addr(),
                sub_op
            );
            Ok(())
        }
    }
}

/// Handle ReportSuccess (sub-opcode 9) -- file a new report.
///
/// C++ Reference: `SheriffHandler.cpp:19-66`
async fn handle_report_success(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    let reported_name = reader.read_string().unwrap_or_default();
    let reason = reader.read_string().unwrap_or_default();

    // Validate inputs
    if reported_name.is_empty() || reason.is_empty() || reason.len() > MAX_ID_REPORT {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_SUCCESS);
        resp.write_u8(sub_opcode::QUESTION_NOT_ANSWERED);
        session.send_packet(&resp).await?;
        return Ok(());
    }

    // Caller must be King or GM
    let caller_info = match world.get_character_info(sid) {
        Some(info) => info,
        None => return Ok(()),
    };
    let is_gm = caller_info.authority == 0;
    // King check: use king system
    let is_king = is_session_king(&world, sid);

    if !is_gm && !is_king {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_SUCCESS);
        resp.write_u8(sub_opcode::REPORT_FAILED);
        session.send_packet(&resp).await?;
        return Ok(());
    }

    // Target must exist, be in-game, and not be a king or GM
    let Some(target_sid) = world.find_session_by_name(&reported_name) else {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_SUCCESS);
        resp.write_u8(sub_opcode::REPORT_FAILED);
        session.send_packet(&resp).await?;
        return Ok(());
    };
    let target_info = match world.get_character_info(target_sid) {
        Some(info) => info,
        None => {
            let mut resp = Packet::new(Opcode::WizReport as u8);
            resp.write_u8(sub_opcode::REPORT_SUCCESS);
            resp.write_u8(sub_opcode::REPORT_FAILED);
            session.send_packet(&resp).await?;
            return Ok(());
        }
    };

    if target_info.authority == 0 || is_session_king(&world, target_sid) {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_SUCCESS);
        resp.write_u8(sub_opcode::REPORT_FAILED);
        session.send_packet(&resp).await?;
        return Ok(());
    }

    // Add to sheriff map
    let sheriff_map = world.sheriff_reports();
    let reporter_name = caller_info.name.clone();

    // Only insert if not already reported
    if !sheriff_map.contains_key(&reported_name) {
        let now = chrono::Local::now();
        let date_str = now.format("%y/%m/%d %H:%M:%S").to_string();

        let report = SheriffReport {
            reported_name: reported_name.clone(),
            reporter_name: reporter_name.clone(),
            crime: reason,
            vote_yes: 1,
            yes_voters: [reporter_name, String::new(), String::new()],
            vote_no: 0,
            no_voters: [String::new(), String::new(), String::new()],
            date: date_str,
        };
        sheriff_map.insert(reported_name, report);
    }

    let mut resp = Packet::new(Opcode::WizReport as u8);
    resp.write_u8(sub_opcode::REPORT_SUCCESS);
    resp.write_u8(1); // success
    session.send_packet(&resp).await?;

    debug!(
        "[{}] WIZ_REPORT: report filed by {}",
        session.addr(),
        caller_info.name
    );
    Ok(())
}

/// Handle ReportAgree/ReportDisagree (sub-opcode 12/13) -- vote on a report.
///
/// C++ Reference: `SheriffHandler.cpp:68-207`
async fn handle_report_vote(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
    is_agree: bool,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    let reported_name = reader.read_string().unwrap_or_default();

    let caller_info = match world.get_character_info(sid) {
        Some(info) => info,
        None => return Ok(()),
    };

    let is_gm = caller_info.authority == 0;
    let is_king = is_session_king(&world, sid);

    if !is_gm && !is_king {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_AGREE);
        resp.write_u8(sub_opcode::VOTING_FAILED);
        session.send_packet(&resp).await?;
        return Ok(());
    }

    let sheriff_map = world.sheriff_reports();
    let voter_name = caller_info.name.clone();

    let mut vote_result = false;
    let mut should_remove = false;

    if let Some(mut entry) = sheriff_map.get_mut(&reported_name) {
        let report = entry.value_mut();

        // Check if already voted
        if report.has_voted(&voter_name) {
            let mut resp = Packet::new(Opcode::WizReport as u8);
            resp.write_u8(sub_opcode::REPORT_AGREE);
            resp.write_u8(sub_opcode::VOTING_FAILED);
            session.send_packet(&resp).await?;
            return Ok(());
        }

        if is_agree {
            // Add yes vote
            report.vote_yes += 1;
            if report.yes_voters[1].is_empty() {
                report.yes_voters[1] = voter_name;
            } else if report.yes_voters[2].is_empty() {
                report.yes_voters[2] = voter_name;
            } else {
                let mut resp = Packet::new(Opcode::WizReport as u8);
                resp.write_u8(sub_opcode::REPORT_AGREE);
                resp.write_u8(sub_opcode::VOTING_FAILED);
                session.send_packet(&resp).await?;
                return Ok(());
            }
            // If 3+ yes votes, send to prison and remove
            if report.vote_yes > 2 {
                should_remove = true;
            }
        } else {
            // Add no vote
            report.vote_no += 1;
            if report.no_voters[0].is_empty() {
                report.no_voters[0] = voter_name;
            } else if report.no_voters[1].is_empty() {
                report.no_voters[1] = voter_name;
            } else if report.no_voters[2].is_empty() {
                report.no_voters[2] = voter_name;
            } else {
                let mut resp = Packet::new(Opcode::WizReport as u8);
                resp.write_u8(sub_opcode::REPORT_AGREE);
                resp.write_u8(sub_opcode::VOTING_FAILED);
                session.send_packet(&resp).await?;
                return Ok(());
            }
            // If 2+ no votes, dismiss and remove
            if report.vote_no >= 2 {
                should_remove = true;
            }
        }
        vote_result = true;
    }

    if should_remove {
        sheriff_map.remove(&reported_name);
    }

    if vote_result {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_AGREE);
        resp.write_u8(1); // success
        session.send_packet(&resp).await?;
    }

    debug!(
        "[{}] WIZ_REPORT: {} vote on '{}' by {}",
        session.addr(),
        if is_agree { "agree" } else { "disagree" },
        reported_name,
        caller_info.name
    );
    Ok(())
}

/// Handle ListOpen (sub-opcode 14) -- paginated report list.
///
/// C++ Reference: `SheriffHandler.cpp:209-254`
async fn handle_list_open(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let world = session.world().clone();
    let current_page = reader.read_u8().unwrap_or(1);

    let sheriff_map = world.sheriff_reports();

    if sheriff_map.is_empty() {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::LIST_OPEN);
        resp.write_u8(0); // counter placeholder
        resp.write_u8(1); // current page
        resp.write_u8(1); // total pages
        session.send_packet(&resp).await?;
        return Ok(());
    }

    let show_start = (current_page as usize).saturating_sub(1) * REPORTS_PER_PAGE as usize;
    let mut counter: u8 = 0;

    let mut resp = Packet::new(Opcode::WizReport as u8);
    resp.write_u8(sub_opcode::LIST_OPEN);
    let counter_pos = resp.data.len();
    resp.write_u8(0); // placeholder for count

    let mut idx: usize = 0;
    for entry in sheriff_map.iter() {
        if idx < show_start {
            idx += 1;
            continue;
        }

        let report = entry.value();
        resp.write_string(&report.reported_name);
        resp.write_string(&report.reporter_name);
        resp.write_string(&report.crime);
        resp.write_u8(0); // padding byte
        resp.write_u8(report.vote_yes);
        resp.write_string(&report.yes_voters[0]);
        resp.write_string(&report.yes_voters[1]);
        resp.write_string(&report.yes_voters[2]);
        resp.write_u8(report.vote_no);
        resp.write_string(&report.no_voters[0]);
        resp.write_string(&report.no_voters[1]);
        resp.write_string(&report.no_voters[2]);
        resp.write_string(&report.date);

        counter += 1;
        idx += 1;

        if counter >= REPORTS_PER_PAGE {
            break;
        }
    }

    let total_reports = sheriff_map.len();
    let mut page_count = (total_reports / REPORTS_PER_PAGE as usize) as u8;
    if !total_reports.is_multiple_of(REPORTS_PER_PAGE as usize) {
        page_count += 1;
    }

    resp.write_u8(current_page);
    resp.write_u8(page_count);

    // Patch counter at the reserved position
    resp.data[counter_pos] = counter;

    session.send_packet(&resp).await?;
    debug!(
        "[{}] WIZ_REPORT LIST_OPEN: page={}, showing={}, total={}",
        session.addr(),
        current_page,
        counter,
        total_reports
    );
    Ok(())
}

/// Handle KingsInspector (sub-opcode 18) -- open inspector UI.
///
/// C++ Reference: `SheriffHandler.cpp:7-11` -- `CUser::KingsInspectorList()`
async fn handle_kings_inspector(session: &mut ClientSession) -> anyhow::Result<()> {
    let mut resp = Packet::new(Opcode::WizReport as u8);
    resp.write_u8(sub_opcode::KINGS_INSPECTOR);
    session.send_packet(&resp).await?;
    debug!("[{}] WIZ_REPORT: KingsInspector UI opened", session.addr());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_opcode_values() {
        assert_eq!(sub_opcode::REPORT_FAILED, 0);
        assert_eq!(sub_opcode::REPORT_SUCCESS, 9);
        assert_eq!(sub_opcode::VOTING_FAILED, 10);
        assert_eq!(sub_opcode::REPORT_AGREE, 12);
        assert_eq!(sub_opcode::REPORT_DISAGREE, 13);
        assert_eq!(sub_opcode::LIST_OPEN, 14);
        assert_eq!(sub_opcode::KINGS_INSPECTOR, 18);
        assert_eq!(sub_opcode::QUESTION_NOT_ANSWERED, 6);
    }

    #[test]
    fn test_sheriff_report_creation() {
        let report = SheriffReport {
            reported_name: "BadPlayer".to_string(),
            reporter_name: "TheKing".to_string(),
            crime: "Speed hacking".to_string(),
            vote_yes: 1,
            yes_voters: ["TheKing".to_string(), String::new(), String::new()],
            vote_no: 0,
            no_voters: [String::new(), String::new(), String::new()],
            date: "26/2/10 12:00:00".to_string(),
        };
        assert_eq!(report.reported_name, "BadPlayer");
        assert_eq!(report.vote_yes, 1);
        assert_eq!(report.vote_no, 0);
    }

    #[test]
    fn test_sheriff_report_has_voted() {
        let report = SheriffReport {
            reported_name: "Target".to_string(),
            reporter_name: "King".to_string(),
            crime: "Cheating".to_string(),
            vote_yes: 2,
            yes_voters: ["King".to_string(), "GM1".to_string(), String::new()],
            vote_no: 1,
            no_voters: ["GM2".to_string(), String::new(), String::new()],
            date: "26/2/10 12:00:00".to_string(),
        };

        assert!(report.has_voted("King"));
        assert!(report.has_voted("GM1"));
        assert!(report.has_voted("GM2"));
        assert!(!report.has_voted("RandomPlayer"));
        assert!(!report.has_voted(""));
    }

    #[test]
    fn test_sheriff_report_map_operations() {
        let map = new_sheriff_map();
        assert!(map.is_empty());

        map.insert(
            "Player1".to_string(),
            SheriffReport {
                reported_name: "Player1".to_string(),
                reporter_name: "King".to_string(),
                crime: "Bot usage".to_string(),
                vote_yes: 1,
                yes_voters: ["King".to_string(), String::new(), String::new()],
                vote_no: 0,
                no_voters: [String::new(), String::new(), String::new()],
                date: "26/2/10 15:30:00".to_string(),
            },
        );

        assert_eq!(map.len(), 1);
        assert!(map.contains_key("Player1"));
        assert!(!map.contains_key("Player2"));

        let entry = map.get("Player1").unwrap();
        assert_eq!(entry.crime, "Bot usage");
        assert_eq!(entry.reporter_name, "King");
    }

    #[test]
    fn test_sheriff_vote_yes_progression() {
        let map = new_sheriff_map();
        map.insert(
            "Target".to_string(),
            SheriffReport {
                reported_name: "Target".to_string(),
                reporter_name: "King".to_string(),
                crime: "Spam".to_string(),
                vote_yes: 1,
                yes_voters: ["King".to_string(), String::new(), String::new()],
                vote_no: 0,
                no_voters: [String::new(), String::new(), String::new()],
                date: "26/2/10 10:00:00".to_string(),
            },
        );

        // Add second yes vote
        if let Some(mut entry) = map.get_mut("Target") {
            entry.vote_yes += 1;
            entry.yes_voters[1] = "GM1".to_string();
        }

        {
            let entry = map.get("Target").unwrap();
            assert_eq!(entry.vote_yes, 2);
            assert_eq!(entry.yes_voters[1], "GM1");
        }

        // Add third yes vote -> should trigger prison
        if let Some(mut entry) = map.get_mut("Target") {
            entry.vote_yes += 1;
            entry.yes_voters[2] = "GM2".to_string();
        }

        {
            let entry = map.get("Target").unwrap();
            assert_eq!(entry.vote_yes, 3);
            assert!(entry.vote_yes > 2); // would trigger prison in C++
        }
    }

    #[test]
    fn test_sheriff_vote_no_dismissal() {
        let map = new_sheriff_map();
        map.insert(
            "Innocent".to_string(),
            SheriffReport {
                reported_name: "Innocent".to_string(),
                reporter_name: "King".to_string(),
                crime: "Alleged cheating".to_string(),
                vote_yes: 1,
                yes_voters: ["King".to_string(), String::new(), String::new()],
                vote_no: 0,
                no_voters: [String::new(), String::new(), String::new()],
                date: "26/2/10 10:00:00".to_string(),
            },
        );

        // Two no votes -> dismissed
        if let Some(mut entry) = map.get_mut("Innocent") {
            entry.vote_no += 1;
            entry.no_voters[0] = "GM1".to_string();
        }
        if let Some(mut entry) = map.get_mut("Innocent") {
            entry.vote_no += 1;
            entry.no_voters[1] = "GM2".to_string();
        }

        {
            let entry = map.get("Innocent").unwrap();
            assert_eq!(entry.vote_no, 2);
            assert!(entry.vote_no >= 2); // would trigger dismissal in C++
        }

        // Remove after dismissal
        map.remove("Innocent");
        assert!(map.is_empty());
    }

    #[test]
    fn test_sheriff_report_duplicate_vote_check() {
        let report = SheriffReport {
            reported_name: "Target".to_string(),
            reporter_name: "King".to_string(),
            crime: "Test".to_string(),
            vote_yes: 1,
            yes_voters: ["King".to_string(), String::new(), String::new()],
            vote_no: 0,
            no_voters: [String::new(), String::new(), String::new()],
            date: "26/2/10 10:00:00".to_string(),
        };

        // The King (reporter) has already voted
        assert!(report.has_voted("King"));
        // Unknown player has not voted
        assert!(!report.has_voted("NewGM"));
    }

    #[test]
    fn test_kings_inspector_packet() {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::KINGS_INSPECTOR);

        assert_eq!(resp.opcode, 0x7C);
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0], 18);
    }

    #[test]
    fn test_report_success_fail_packet() {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::REPORT_SUCCESS);
        resp.write_u8(sub_opcode::REPORT_FAILED);

        assert_eq!(resp.opcode, 0x7C);
        let mut reader = PacketReader::new(&resp.data);
        assert_eq!(reader.read_u8(), Some(sub_opcode::REPORT_SUCCESS));
        assert_eq!(reader.read_u8(), Some(sub_opcode::REPORT_FAILED));
    }

    #[test]
    fn test_empty_list_packet() {
        let mut resp = Packet::new(Opcode::WizReport as u8);
        resp.write_u8(sub_opcode::LIST_OPEN);
        resp.write_u8(0); // count
        resp.write_u8(1); // current page
        resp.write_u8(1); // total pages

        assert_eq!(resp.opcode, 0x7C);
        let mut reader = PacketReader::new(&resp.data);
        assert_eq!(reader.read_u8(), Some(sub_opcode::LIST_OPEN));
        assert_eq!(reader.read_u8(), Some(0)); // empty
        assert_eq!(reader.read_u8(), Some(1));
        assert_eq!(reader.read_u8(), Some(1));
    }

    #[test]
    fn test_page_calculation() {
        // 0 reports = 1 page (handled as special case)
        // 1-8 reports = 1 page
        // 9 reports = 2 pages
        // 16 reports = 2 pages
        // 17 reports = 3 pages

        let calc_pages = |total: usize| -> u8 {
            let mut pages = (total / REPORTS_PER_PAGE as usize) as u8;
            if !total.is_multiple_of(REPORTS_PER_PAGE as usize) {
                pages += 1;
            }
            pages
        };

        assert_eq!(calc_pages(0), 0);
        assert_eq!(calc_pages(1), 1);
        assert_eq!(calc_pages(8), 1);
        assert_eq!(calc_pages(9), 2);
        assert_eq!(calc_pages(16), 2);
        assert_eq!(calc_pages(17), 3);
    }

    #[test]
    fn test_sheriff_map_multiple_reports() {
        let map = new_sheriff_map();
        for i in 0..20 {
            map.insert(
                format!("Player{}", i),
                SheriffReport {
                    reported_name: format!("Player{}", i),
                    reporter_name: "King".to_string(),
                    crime: format!("Crime {}", i),
                    vote_yes: 1,
                    yes_voters: ["King".to_string(), String::new(), String::new()],
                    vote_no: 0,
                    no_voters: [String::new(), String::new(), String::new()],
                    date: "26/2/10 10:00:00".to_string(),
                },
            );
        }

        assert_eq!(map.len(), 20);

        // Remove one
        map.remove("Player5");
        assert_eq!(map.len(), 19);
        assert!(!map.contains_key("Player5"));
    }

    #[test]
    fn test_max_reason_length() {
        assert_eq!(MAX_ID_REPORT, 512);
        let long_reason = "a".repeat(513);
        assert!(long_reason.len() > MAX_ID_REPORT);
        let valid_reason = "a".repeat(512);
        assert!(valid_reason.len() <= MAX_ID_REPORT);
    }

    #[test]
    fn test_reports_per_page_constant() {
        assert_eq!(REPORTS_PER_PAGE, 8);
    }
}
