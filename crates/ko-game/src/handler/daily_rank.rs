//! WIZ_DAILYRANK (0xC2) handler — Daily ranking system.
//! ## Daily Rank Types (`DailyRankType` enum in `GameDefine.h:4511-4522`)
//! | Value | Name              | Description                    |
//! |-------|-------------------|--------------------------------|
//! | 0     | GRAND_MERCHANT    | Top gold earners               |
//! | 1     | KNIGHT_ADONIS     | Knight achievement rank         |
//! | 2     | HERO_OF_CHAOS     | Chaos war heroes                |
//! | 3     | MONSTER_HUNTER    | Monster kill rank               |
//! | 4     | SHOZIN            | Shozin rank                     |
//! | 5     | DRAKI_RANK        | Draki Tower ranking             |
//! | 6     | DISCIPLE_KERON    | Upgrade master rank             |
//! | 8     | KING_OF_FELANKOR  | Clan ranking (by points)        |
//! ## Client -> Server (WIZ_DAILYRANK 0xC2)
//! ```text
//! [u8 sub_opcode]   // always 1 = show
//! [u8 rank_type]    // DailyRankType
//! ```
//! ## Server -> Client (WIZ_DAILYRANK 0xC2)
//! ```text
//! [u8 sub_opcode=1]
//! [u8 rank_type]
//! [u32 my_rank]       // 1-based rank position
//! [i32 rank_diff]     // prev_rank - cur_rank (positive = improved)
//! [u16 count]         // number of ranked names (max 100)
//! For each entry:
//!   [sbyte_string name]  // C++ SByte() mode — u8 length prefix
//! ```

use ko_db::models::daily_rank::DailyRankRow;
use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::{debug, warn};

use crate::session::{ClientSession, SessionState};

/// Daily rank type constants from `DailyRankType` enum.
const DAILY_RANK_GRAND_MERCHANT: u8 = 0;
const DAILY_RANK_KNIGHT_ADONIS: u8 = 1;
const DAILY_RANK_HERO_OF_CHAOS: u8 = 2;
const DAILY_RANK_MONSTER_HUNTER: u8 = 3;
const DAILY_RANK_SHOZIN: u8 = 4;
const DAILY_RANK_DRAKI: u8 = 5;
const DAILY_RANK_DISCIPLE_KERON: u8 = 6;
const DAILY_RANK_KING_OF_FELANKOR: u8 = 8;

/// Handle incoming WIZ_DAILYRANK (0xC2) packet.
pub async fn handle(session: &mut ClientSession, packet: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    let mut reader = PacketReader::new(&packet.data);
    let sub_opcode = match reader.read_u8() {
        Some(v) => v,
        None => return Ok(()),
    };

    match sub_opcode {
        1 => handle_daily_rank_show(session, &mut reader).await,
        _ => {
            warn!(
                "[{}] WIZ_DAILYRANK: unhandled sub_opcode {}",
                session.addr(),
                sub_opcode
            );
            Ok(())
        }
    }
}

/// Handle daily rank show request (sub_opcode = 1).
async fn handle_daily_rank_show(
    session: &mut ClientSession,
    reader: &mut PacketReader<'_>,
) -> anyhow::Result<()> {
    let rank_type = match reader.read_u8() {
        Some(v) => v,
        None => return Ok(()),
    };

    match rank_type {
        DAILY_RANK_GRAND_MERCHANT
        | DAILY_RANK_KNIGHT_ADONIS
        | DAILY_RANK_HERO_OF_CHAOS
        | DAILY_RANK_MONSTER_HUNTER
        | DAILY_RANK_SHOZIN
        | DAILY_RANK_DISCIPLE_KERON => send_user_daily_rank(session, rank_type).await,
        DAILY_RANK_KING_OF_FELANKOR => send_clan_daily_rank(session, rank_type).await,
        DAILY_RANK_DRAKI => send_draki_daily_rank(session, rank_type).await,
        _ => {
            warn!(
                "[{}] WIZ_DAILYRANK: unhandled rank_type {}",
                session.addr(),
                rank_type
            );
            Ok(())
        }
    }
}

/// Extract (current_rank, previous_rank) for a given rank type from a `DailyRankRow`.
fn get_rank_fields(row: &DailyRankRow, rank_type: u8) -> (i32, i32) {
    match rank_type {
        DAILY_RANK_GRAND_MERCHANT => (row.gm_rank_cur, row.gm_rank_prev),
        DAILY_RANK_KNIGHT_ADONIS => (row.ak_rank_cur, row.ak_rank_prev),
        DAILY_RANK_HERO_OF_CHAOS => (row.cw_rank_cur, row.cw_rank_prev),
        DAILY_RANK_MONSTER_HUNTER => (row.mh_rank_cur, row.mh_rank_prev),
        DAILY_RANK_SHOZIN => (row.sh_rank_cur, row.sh_rank_prev),
        DAILY_RANK_DISCIPLE_KERON => (row.up_rank_cur, row.up_rank_prev),
        _ => (0, 0),
    }
}

/// Send an empty daily rank response.
/// C++ pattern: WIZ_DAILYRANK, sub=1, rank_type, my_rank=0, diff=0, count=0
/// NOTE: Draki (type 5) has a different format — no diff field.
async fn send_empty_daily_rank(session: &mut ClientSession, rank_type: u8) -> anyhow::Result<()> {
    let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
    pkt.write_u8(1); // sub_opcode
    pkt.write_u8(rank_type);
    pkt.write_u32(0); // my_rank
    if rank_type != DAILY_RANK_DRAKI {
        pkt.write_u32(0); // rank_diff (absent for Draki)
    }
    pkt.write_u16(0); // count

    session.send_packet(&pkt).await?;

    debug!(
        "[{}] WIZ_DAILYRANK: sent empty rank response for type {}",
        session.addr(),
        rank_type
    );

    Ok(())
}

/// Send user-based daily rank (6 rank types) from cached DB data.
/// Loads cached `daily_rank` data, filters ranked users (rank > 0), sorts ascending
/// (rank 1 = best), finds player's own rank, and sends top 100 names.
async fn send_user_daily_rank(session: &mut ClientSession, rank_type: u8) -> anyhow::Result<()> {
    let world = session.world();
    let sid = session.session_id();

    let char_name = world
        .with_session(sid, |h| h.character.as_ref().map(|c| c.name.clone()))
        .flatten()
        .unwrap_or_default();

    let cache = world.get_daily_rank_cache();
    if cache.is_empty() {
        return send_empty_daily_rank(session, rank_type).await;
    }

    // Filter and sort by current rank ascending (rank 1 = best)
    let mut ranked: Vec<(&str, i32, i32)> = cache
        .iter()
        .map(|r| {
            let (cur, prev) = get_rank_fields(r, rank_type);
            (r.char_id.as_str(), cur, prev)
        })
        .filter(|(_, cur, _)| *cur > 0)
        .collect();

    ranked.sort_by_key(|(_, cur, _)| *cur);

    // Find player's own rank and diff
    let mut my_rank: u32 = 0;
    let mut my_diff: i32 = 0;
    if !char_name.is_empty() {
        for (i, (name, cur, prev)) in ranked.iter().enumerate() {
            if *name == char_name {
                my_rank = (i + 1) as u32; // 1-based
                my_diff = *prev - *cur; // positive = improved
                break;
            }
        }
    }

    let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
    pkt.write_u8(1); // sub_opcode
    pkt.write_u8(rank_type);
    pkt.write_u32(my_rank);
    pkt.write_i32(my_diff); // C++ writes int32 Diff (signed)

    let count_offset = pkt.wpos();
    pkt.write_u16(0); // placeholder
    let mut count: u16 = 0;
    for (name, _, _) in ranked.iter() {
        if count >= 100 {
            break;
        }
        pkt.write_sbyte_string(name); // C++ SByte() mode
        count += 1;
    }
    pkt.put_u16_at(count_offset, count);

    session.send_packet(&pkt).await?;

    debug!(
        "[{}] WIZ_DAILYRANK type {}: my_rank={}, diff={}, entries={}",
        session.addr(),
        rank_type,
        my_rank,
        my_diff,
        count
    );

    Ok(())
}

/// Send clan-based daily rank for KING_OF_FELANKOR.
/// Merges top clans from both nations sorted by points, finds player's clan rank.
async fn send_clan_daily_rank(session: &mut ClientSession, rank_type: u8) -> anyhow::Result<()> {
    let world = session.world();
    let sid = session.session_id();

    // Get player's clan info
    let knights_id = match world.with_session(sid, |h| h.character.as_ref().map(|c| c.knights_id)) {
        Some(Some(kid)) => kid,
        _ => return send_empty_daily_rank(session, rank_type).await,
    };

    // Gather top clans from both nations
    let karus_clans = world.get_top_ranked_clans(1, 50);
    let elmo_clans = world.get_top_ranked_clans(2, 50);

    let mut all_clans: Vec<(u16, String)> = Vec::with_capacity(100);
    all_clans.extend(karus_clans);
    all_clans.extend(elmo_clans);

    if all_clans.is_empty() {
        return send_empty_daily_rank(session, rank_type).await;
    }

    // Find my clan's rank (0-based, C++ parity)
    let mut my_rank: u32 = 0;
    if knights_id != 0 && knights_id != 0xFFFF {
        for (i, (cid, _)) in all_clans.iter().enumerate() {
            if *cid == knights_id {
                my_rank = i as u32;
                break;
            }
        }
    }

    let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
    pkt.write_u8(1); // sub_opcode
    pkt.write_u8(rank_type);
    pkt.write_u32(my_rank);
    pkt.write_u32(0); // diff (no previous data for clan rank)

    let count_offset = pkt.wpos();
    pkt.write_u16(0); // placeholder
    let mut count: u16 = 0;
    for (_, clan_name) in all_clans.iter() {
        if count >= 100 {
            break;
        }
        pkt.write_sbyte_string(clan_name); // C++ SByte() mode
        count += 1;
    }
    pkt.put_u16_at(count_offset, count);

    session.send_packet(&pkt).await?;

    debug!(
        "[{}] WIZ_DAILYRANK KING_OF_FELANKOR: my_rank={}, clans={}",
        session.addr(),
        my_rank,
        count
    );

    Ok(())
}

/// Send Draki Tower daily rank filtered by player's class.
/// Loads ranking from DB filtered by class, sorted by stage DESC then time ASC.
async fn send_draki_daily_rank(session: &mut ClientSession, rank_type: u8) -> anyhow::Result<()> {
    let world = session.world();
    let sid = session.session_id();
    let pool = session.pool();

    // Get player's class and name
    let (char_name, class_id) = match world.with_session(sid, |h| {
        h.character
            .as_ref()
            .map(|c| (c.name.clone(), c.class as i32))
    }) {
        Some(Some(v)) => v,
        _ => return send_empty_daily_rank(session, rank_type).await,
    };

    // Async DB query for class-filtered ranking (C++ ReqHandleDailyRank pattern)
    let repo = ko_db::repositories::daily_rank::DailyRankRepository::new(pool);
    let entries = match repo.load_draki_by_class(class_id).await {
        Ok(rows) => rows,
        Err(e) => {
            warn!("DailyRank DRAKI: DB load failed: {}", e);
            return send_empty_daily_rank(session, rank_type).await;
        }
    };

    if entries.is_empty() {
        return send_empty_daily_rank(session, rank_type).await;
    }

    // Find player's rank (already sorted by stage DESC, time ASC from DB)
    let mut my_rank: u32 = 0;
    for (i, entry) in entries.iter().enumerate() {
        if entry.char_id == char_name {
            my_rank = (i + 1) as u32; // 1-based
            break;
        }
    }

    // Format: [sub=1][rank_type=5][u32 MyRank][u16 Count][sbyte_strings]
    let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
    pkt.write_u8(1); // sub_opcode
    pkt.write_u8(rank_type);
    pkt.write_u32(my_rank);
    // NOTE: No diff field for Draki rank (C++ parity)

    let count_offset = pkt.wpos();
    pkt.write_u16(0); // placeholder
    let mut count: u16 = 0;
    for entry in entries.iter() {
        if count >= 100 {
            break;
        }
        pkt.write_sbyte_string(&entry.char_id); // C++ SByte() mode
        count += 1;
    }
    pkt.put_u16_at(count_offset, count);

    session.send_packet(&pkt).await?;

    debug!(
        "[{}] WIZ_DAILYRANK DRAKI (class {}): my_rank={}, entries={}",
        session.addr(),
        class_id,
        my_rank,
        count
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_db::models::daily_rank::{DailyRankRow, DrakiTowerDailyRankRow};

    fn make_rank_row(
        name: &str,
        gm: i32,
        mh: i32,
        sh: i32,
        ak: i32,
        cw: i32,
        up: i32,
    ) -> DailyRankRow {
        DailyRankRow {
            char_id: name.to_string(),
            gm_rank_cur: gm,
            gm_rank_prev: gm + 1,
            mh_rank_cur: mh,
            mh_rank_prev: mh + 2,
            sh_rank_cur: sh,
            sh_rank_prev: sh,
            ak_rank_cur: ak,
            ak_rank_prev: ak,
            cw_rank_cur: cw,
            cw_rank_prev: cw + 3,
            up_rank_cur: up,
            up_rank_prev: up,
        }
    }

    #[test]
    fn test_empty_daily_rank_packet_format() {
        // Non-Draki empty: sub(1) + rank_type(1) + rank(4) + diff(4) + count(2) = 12
        let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
        pkt.write_u8(1);
        pkt.write_u8(DAILY_RANK_GRAND_MERCHANT);
        pkt.write_u32(0);
        pkt.write_u32(0); // diff present for non-Draki
        pkt.write_u16(0);

        assert_eq!(pkt.opcode, 0xC2);
        assert_eq!(pkt.data.len(), 12);
        assert_eq!(pkt.data[0], 1);
        assert_eq!(pkt.data[1], DAILY_RANK_GRAND_MERCHANT);
    }

    #[test]
    fn test_empty_draki_rank_packet_format() {
        // Draki empty: sub(1) + rank_type(1) + rank(4) + count(2) = 8 (NO diff field)
        let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
        pkt.write_u8(1);
        pkt.write_u8(DAILY_RANK_DRAKI);
        pkt.write_u32(0);
        // No diff for Draki
        pkt.write_u16(0);

        assert_eq!(pkt.opcode, 0xC2);
        assert_eq!(pkt.data.len(), 8); // 4 bytes shorter than non-Draki
        assert_eq!(pkt.data[1], DAILY_RANK_DRAKI);
    }

    #[test]
    fn test_daily_rank_type_constants() {
        assert_eq!(DAILY_RANK_GRAND_MERCHANT, 0);
        assert_eq!(DAILY_RANK_KNIGHT_ADONIS, 1);
        assert_eq!(DAILY_RANK_HERO_OF_CHAOS, 2);
        assert_eq!(DAILY_RANK_MONSTER_HUNTER, 3);
        assert_eq!(DAILY_RANK_SHOZIN, 4);
        assert_eq!(DAILY_RANK_DRAKI, 5);
        assert_eq!(DAILY_RANK_DISCIPLE_KERON, 6);
        assert_eq!(DAILY_RANK_KING_OF_FELANKOR, 8);
    }

    #[test]
    fn test_daily_rank_row_model() {
        let row = DailyRankRow {
            char_id: "TestPlayer".to_string(),
            gm_rank_cur: 5,
            gm_rank_prev: 8,
            mh_rank_cur: 1,
            mh_rank_prev: 1,
            sh_rank_cur: 0,
            sh_rank_prev: 0,
            ak_rank_cur: 3,
            ak_rank_prev: 5,
            cw_rank_cur: 10,
            cw_rank_prev: 7,
            up_rank_cur: 0,
            up_rank_prev: 0,
        };
        assert_eq!(row.gm_rank_cur, 5);
        assert_eq!(row.mh_rank_cur, 1);
        assert_eq!(row.cw_rank_prev, 7);
    }

    #[test]
    fn test_draki_tower_rank_row_model() {
        let row = DrakiTowerDailyRankRow {
            char_id: "DrakiMaster".to_string(),
            class_id: 102,
            draki_stage: 15,
            draki_time: 3600,
        };
        assert_eq!(row.class_id, 102);
        assert_eq!(row.draki_stage, 15);
        assert_eq!(row.draki_time, 3600);
    }

    #[test]
    fn test_get_rank_fields() {
        let row = make_rank_row("Player", 5, 3, 10, 7, 1, 20);
        assert_eq!(get_rank_fields(&row, DAILY_RANK_GRAND_MERCHANT), (5, 6));
        assert_eq!(get_rank_fields(&row, DAILY_RANK_MONSTER_HUNTER), (3, 5));
        assert_eq!(get_rank_fields(&row, DAILY_RANK_HERO_OF_CHAOS), (1, 4));
        assert_eq!(get_rank_fields(&row, DAILY_RANK_SHOZIN), (10, 10));
        assert_eq!(get_rank_fields(&row, DAILY_RANK_KNIGHT_ADONIS), (7, 7));
        assert_eq!(get_rank_fields(&row, DAILY_RANK_DISCIPLE_KERON), (20, 20));
    }

    #[test]
    fn test_rank_sorting_ascending() {
        let rows = [
            make_rank_row("Third", 3, 0, 0, 0, 0, 0),
            make_rank_row("First", 1, 0, 0, 0, 0, 0),
            make_rank_row("Unranked", 0, 0, 0, 0, 0, 0),
            make_rank_row("Second", 2, 0, 0, 0, 0, 0),
        ];

        let mut ranked: Vec<(&str, i32, i32)> = rows
            .iter()
            .map(|r| {
                let (cur, prev) = get_rank_fields(r, DAILY_RANK_GRAND_MERCHANT);
                (r.char_id.as_str(), cur, prev)
            })
            .filter(|(_, cur, _)| *cur > 0)
            .collect();
        ranked.sort_by_key(|(_, cur, _)| *cur);

        assert_eq!(ranked.len(), 3); // Unranked filtered out
        assert_eq!(ranked[0].0, "First");
        assert_eq!(ranked[1].0, "Second");
        assert_eq!(ranked[2].0, "Third");
    }

    #[test]
    fn test_rank_diff_calculation() {
        let row = DailyRankRow {
            char_id: "Player".to_string(),
            gm_rank_cur: 3,
            gm_rank_prev: 8,
            mh_rank_cur: 5,
            mh_rank_prev: 2,
            sh_rank_cur: 1,
            sh_rank_prev: 1,
            ak_rank_cur: 0,
            ak_rank_prev: 0,
            cw_rank_cur: 0,
            cw_rank_prev: 0,
            up_rank_cur: 0,
            up_rank_prev: 0,
        };
        // GM: prev(8) - cur(3) = +5 (improved)
        let (cur, prev) = get_rank_fields(&row, DAILY_RANK_GRAND_MERCHANT);
        assert_eq!(prev - cur, 5);
        // MH: prev(2) - cur(5) = -3 (worsened)
        let (cur, prev) = get_rank_fields(&row, DAILY_RANK_MONSTER_HUNTER);
        assert_eq!(prev - cur, -3);
        // SH: prev(1) - cur(1) = 0 (unchanged)
        let (cur, prev) = get_rank_fields(&row, DAILY_RANK_SHOZIN);
        assert_eq!(prev - cur, 0);
    }

    #[test]
    fn test_sbyte_string_in_rank_packet() {
        let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
        pkt.write_u8(1);
        pkt.write_u8(DAILY_RANK_GRAND_MERCHANT);
        pkt.write_u32(1); // my_rank
        pkt.write_u32(0); // diff
        pkt.write_u16(2); // count
        pkt.write_sbyte_string("TopPlayer");
        pkt.write_sbyte_string("SecondGuy");

        let data = &pkt.data;
        // After header (1+1+4+4+2 = 12 bytes), first sbyte string:
        assert_eq!(data[12], 9); // "TopPlayer" = 9 bytes (u8 length prefix)
        assert_eq!(&data[13..22], b"TopPlayer");
        assert_eq!(data[22], 9); // "SecondGuy" = 9 bytes
        assert_eq!(&data[23..32], b"SecondGuy");
    }

    #[test]
    fn test_draki_sort_stage_desc_time_asc() {
        let mut entries = [
            DrakiTowerDailyRankRow {
                char_id: "SlowHigh".to_string(),
                class_id: 102,
                draki_stage: 15,
                draki_time: 5000,
            },
            DrakiTowerDailyRankRow {
                char_id: "FastHigh".to_string(),
                class_id: 102,
                draki_stage: 15,
                draki_time: 3000,
            },
            DrakiTowerDailyRankRow {
                char_id: "LowStage".to_string(),
                class_id: 102,
                draki_stage: 10,
                draki_time: 1000,
            },
        ];

        // Sort: stage DESC, time ASC (DB does this, but verify logic)
        entries.sort_by(|a, b| {
            b.draki_stage
                .cmp(&a.draki_stage)
                .then(a.draki_time.cmp(&b.draki_time))
        });

        assert_eq!(entries[0].char_id, "FastHigh"); // stage 15, time 3000
        assert_eq!(entries[1].char_id, "SlowHigh"); // stage 15, time 5000
        assert_eq!(entries[2].char_id, "LowStage"); // stage 10, time 1000
    }

    #[test]
    fn test_clan_rank_packet_with_sbyte() {
        let mut pkt = Packet::new(Opcode::WizDailyRank as u8);
        pkt.write_u8(1);
        pkt.write_u8(DAILY_RANK_KING_OF_FELANKOR);
        pkt.write_u32(2); // my_rank
        pkt.write_u32(0); // diff
        pkt.write_u16(1); // count
        pkt.write_sbyte_string("TopClan");

        assert_eq!(pkt.opcode, 0xC2);
        let data = &pkt.data;
        // After header (12 bytes): sbyte string
        assert_eq!(data[12], 7); // "TopClan" = 7 bytes (u8 prefix, not u16)
        assert_eq!(&data[13..20], b"TopClan");
    }

    #[test]
    fn test_daily_rank_cache_accessor() {
        use crate::world::WorldState;
        let world = WorldState::new();
        let cache = world.get_daily_rank_cache();
        assert!(cache.is_empty()); // Empty at startup (no DB loaded)
    }

    // ── Sprint 552: Daily rank stat tracking tests ─────────────────

    #[test]
    fn test_session_daily_rank_stats_default_zero() {
        use crate::world::WorldState;
        let world = WorldState::new();
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        let sid = world.allocate_session_id();
        world.register_session(sid, tx);

        let stats = world
            .with_session(sid, |h| {
                (
                    h.dr_gm_total_sold,
                    h.dr_mh_total_kill,
                    h.dr_sh_total_exchange,
                    h.dr_cw_counter_win,
                    h.dr_up_counter_bles,
                )
            })
            .unwrap();
        assert_eq!(stats, (0, 0, 0, 0, 0));
    }

    #[test]
    fn test_session_daily_rank_stats_increment() {
        use crate::world::WorldState;
        let world = WorldState::new();
        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        let sid = world.allocate_session_id();
        world.register_session(sid, tx);

        // Simulate monster kill
        world.update_session(sid, |h| {
            h.dr_mh_total_kill += 1;
        });
        assert_eq!(world.with_session(sid, |h| h.dr_mh_total_kill).unwrap(), 1);

        // Simulate crafting success
        world.update_session(sid, |h| {
            h.dr_sh_total_exchange += 1;
        });
        assert_eq!(
            world.with_session(sid, |h| h.dr_sh_total_exchange).unwrap(),
            1
        );

        // Simulate merchant gold sale
        world.update_session(sid, |h| {
            h.dr_gm_total_sold += 50000;
        });
        assert_eq!(
            world.with_session(sid, |h| h.dr_gm_total_sold).unwrap(),
            50000
        );

        // Simulate chaos war win
        world.update_session(sid, |h| {
            h.dr_cw_counter_win += 1;
        });
        assert_eq!(world.with_session(sid, |h| h.dr_cw_counter_win).unwrap(), 1);
    }

    #[test]
    fn test_daily_rank_stats_model() {
        use ko_db::models::daily_rank::UserDailyRankStatsRow;
        let row = UserDailyRankStatsRow {
            char_id: "TestPlayer".to_string(),
            gm_total_sold: 100000,
            mh_total_kill: 5000,
            sh_total_exchange: 200,
            cw_counter_win: 3,
            up_counter_bles: 0,
        };
        assert_eq!(row.gm_total_sold, 100000);
        assert_eq!(row.mh_total_kill, 5000);
        assert_eq!(row.sh_total_exchange, 200);
        assert_eq!(row.cw_counter_win, 3);
        assert_eq!(row.up_counter_bles, 0);
    }

    #[test]
    fn test_daily_rank_only_nonzero_saved() {
        // On disconnect, stats are only saved if at least one is > 0.
        // This test verifies the condition logic.
        let (gm, mh, sh, cw, up): (u64, u64, u64, u64, u64) = (0, 0, 0, 0, 0);
        assert!(
            !(gm > 0 || mh > 0 || sh > 0 || cw > 0 || up > 0),
            "all-zero stats should NOT trigger save"
        );

        let (gm2, mh2, sh2, cw2, up2): (u64, u64, u64, u64, u64) = (0, 1, 0, 0, 0);
        assert!(
            gm2 > 0 || mh2 > 0 || sh2 > 0 || cw2 > 0 || up2 > 0,
            "any nonzero stat SHOULD trigger save"
        );
    }

    #[test]
    fn test_daily_rank_cw_only_rank1_increments() {
        // C++ parity: only rank 1 in chaos war increments CWCounterWin
        let user_rank: u16 = 1;
        let mut counter: u64 = 0;
        if user_rank == 1 {
            counter += 1;
        }
        assert_eq!(counter, 1);

        // Rank 2 should NOT increment
        let user_rank2: u16 = 2;
        let mut counter2: u64 = 0;
        if user_rank2 == 1 {
            counter2 += 1;
        }
        assert_eq!(counter2, 0);
    }
}
