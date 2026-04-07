//! Timed notice background tick system — periodic server announcements.
//!
//! C++ Reference: `ServerStartStopHandler.cpp` — `Timer_TimedNotice`
//!   - Checks every 60 seconds whether any timed notice is due for broadcast.
//!
//! ## Overview
//!
//! Loads all `timed_notice` rows from the database at startup. Each notice has
//! a configurable interval (`time_minutes`) and target zone (`zone_id`). The
//! tick runs every 60 seconds and broadcasts any due notices as `WIZ_CHAT`
//! packets with the configured chat type (typically `PUBLIC_CHAT = 7`).
//!
//! If `zone_id == 0`, the notice is broadcast to all connected players.
//! Otherwise, it is broadcast only to players in the specified zone.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ko_protocol::{Opcode, Packet};
use tracing::{debug, info, warn};

use crate::world::WorldState;

/// Tick interval for checking timed notices (60 seconds).
///
/// C++ Reference: `Timer_TimedNotice` fires every 60,000ms.
const TIMED_NOTICE_TICK_SECS: u64 = 60;

/// Minimum allowed interval in minutes (clamped).
const MIN_INTERVAL_MINUTES: i32 = 1;

/// Internal state for a scheduled notice.
struct ScheduledNotice {
    /// Chat type (e.g., 7 = PUBLIC_CHAT).
    notice_type: u8,
    /// Message text to broadcast.
    notice: String,
    /// Target zone ID (0 = all zones).
    zone_id: u16,
    /// Broadcast interval.
    interval: Duration,
    /// When the next broadcast should occur.
    next_broadcast_at: Instant,
}

/// Start the timed notice background task.
///
/// Loads all notices from the database, then ticks every 60 seconds to
/// broadcast any due notices. Returns a `JoinHandle` so the caller can
/// abort on shutdown.
///
/// C++ Reference: `CGameServerDlg::Timer_TimedNotice()`
pub fn start_timed_notice_task(
    world: Arc<WorldState>,
    pool: ko_db::DbPool,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        // Load notices from DB at startup.
        let notices = load_notices(&pool).await;
        if notices.is_empty() {
            info!("timed_notice: no notices configured, tick disabled");
            return;
        }
        info!("timed_notice: loaded {} notice(s)", notices.len());

        let mut scheduled = build_schedule(&notices);

        let mut interval = tokio::time::interval(Duration::from_secs(TIMED_NOTICE_TICK_SECS));
        loop {
            interval.tick().await;
            process_timed_notice_tick(&world, &mut scheduled);
        }
    })
}

/// Load timed notices from the database.
async fn load_notices(pool: &ko_db::DbPool) -> Vec<ko_db::models::timed_notice::TimedNoticeRow> {
    let repo = ko_db::repositories::timed_notice::TimedNoticeRepository::new(pool);
    match repo.load_all().await {
        Ok(rows) => rows,
        Err(e) => {
            warn!("timed_notice: failed to load notices from DB: {}", e);
            Vec::new()
        }
    }
}

/// Build the initial schedule from loaded notice rows.
fn build_schedule(
    notices: &[ko_db::models::timed_notice::TimedNoticeRow],
) -> HashMap<i32, ScheduledNotice> {
    let now = Instant::now();
    let mut map = HashMap::new();

    for row in notices {
        // Skip empty notices.
        if row.notice.is_empty() {
            debug!(
                "timed_notice: skipping index {} (empty notice text)",
                row.n_index
            );
            continue;
        }

        // Clamp interval to minimum of 1 minute.
        let minutes = row.time_minutes.max(MIN_INTERVAL_MINUTES);
        let interval = Duration::from_secs(minutes as u64 * 60);

        map.insert(
            row.n_index,
            ScheduledNotice {
                notice_type: row.notice_type as u8,
                notice: row.notice.clone(),
                zone_id: row.zone_id as u16,
                interval,
                next_broadcast_at: now + interval,
            },
        );
    }

    map
}

/// Process one tick — broadcast any due notices.
fn process_timed_notice_tick(world: &WorldState, scheduled: &mut HashMap<i32, ScheduledNotice>) {
    let now = Instant::now();

    for (idx, sched) in scheduled.iter_mut() {
        if now < sched.next_broadcast_at {
            continue;
        }

        // Build the chat packet for this notice.
        let pkt = build_notice_packet(sched.notice_type, &sched.notice);

        // Broadcast to all or specific zone.
        if sched.zone_id == 0 {
            world.broadcast_to_all(Arc::new(pkt), None);
            debug!("timed_notice: broadcast index {} to all zones", idx);
        } else {
            world.broadcast_to_zone(sched.zone_id, Arc::new(pkt), None);
            debug!(
                "timed_notice: broadcast index {} to zone {}",
                idx, sched.zone_id
            );
        }

        // Schedule next broadcast.
        sched.next_broadcast_at = now + sched.interval;
    }
}

/// Build a `WIZ_CHAT` notice packet for server-side announcements and per-player feedback.
///
/// Used for both timed server broadcasts and per-player notices (rejection
/// messages, KC balance updates). v2525 vanilla client dispatch range is
/// 0x06-0xD7, so WIZ_CHAT (0x12) is the only reliable text display opcode.
/// WIZ_ADD_MSG (0xDB) is outside this range and silently dropped.
///
/// Common chat types:
/// - `7` = PUBLIC_CHAT — system/GM announcements, shows in general tab
/// - `8` = WAR_SYSTEM_CHAT — war system messages
///
/// Uses the same wire format as `ChatPacket::Construct` (chat.rs):
/// ```text
/// [u8 chat_type] [u8 nation=1] [u32 sender_id=0xFFFFFFFF]
/// [u8 name_len=0] (empty sender name)
/// [u16 msg_len] [bytes message]
/// [i8 personal_rank=0] [u8 authority=0] [u8 system_msg=0]
/// ```
///
/// C++ ChatPacket::Construct defaults: bNation=1, senderID=-1, systemmsg=0.
pub fn build_notice_packet(chat_type: u8, message: &str) -> Packet {
    let mut pkt = Packet::new(Opcode::WizChat as u8);
    // C++ Reference: ChatPacket::Construct defaults
    pkt.write_u8(chat_type);
    pkt.write_u8(1); // nation = 1 (C++ default bNation=1)
    pkt.write_u32(0xFFFFFFFF); // sender_id = -1 as u32 (C++ default senderID=-1)
    pkt.write_sbyte_string(""); // empty sender name
    pkt.write_string(message); // message text (DByte: u16 len prefix)
    pkt.write_i8(0); // personal_rank = 0
    pkt.write_u8(0); // authority = 0
    pkt.write_u8(0); // system_msg = 0 (C++ default systemmsg=0)
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_db::models::timed_notice::TimedNoticeRow;
    use ko_protocol::PacketReader;

    /// Build a minimal CharacterInfo for test sessions.
    fn test_character(sid: crate::zone::SessionId) -> crate::world::CharacterInfo {
        crate::world::CharacterInfo {
            session_id: sid,
            name: format!("TestPlayer{}", sid),
            nation: 1,
            race: 1,
            class: 101,
            level: 60,
            face: 1,
            hair_rgb: 0,
            rank: 0,
            title: 0,
            max_hp: 1000,
            hp: 1000,
            max_mp: 500,
            mp: 500,
            max_sp: 0,
            sp: 0,
            equipped_items: [0u32; 14],
            bind_zone: 0,
            bind_x: 0.0,
            bind_z: 0.0,
            str: 60,
            sta: 60,
            dex: 60,
            intel: 60,
            cha: 60,
            free_points: 0,
            skill_points: [0u8; 10],
            gold: 0,
            loyalty: 0,
            loyalty_monthly: 0,
            authority: 1,
            knights_id: 0,
            fame: 0,
            party_id: None,
            exp: 0,
            max_exp: 100_000,
            exp_seal_status: false,
            sealed_exp: 0,
            item_weight: 0,
            max_weight: 1000,
            res_hp_type: 1,
            rival_id: -1,
            rival_expiry_time: 0,
            anger_gauge: 0,
            manner_point: 0,
            rebirth_level: 0,
            reb_str: 0,
            reb_sta: 0,
            reb_dex: 0,
            reb_intel: 0,
            reb_cha: 0,
            cover_title: 0,
        }
    }

    // ── Notice Packet Format Tests ──────────────────────────────────────

    #[test]
    fn test_build_notice_packet_public_chat() {
        let pkt = build_notice_packet(7, "Server maintenance in 10 minutes!");
        assert_eq!(pkt.opcode, Opcode::WizChat as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(7)); // chat_type = PUBLIC_CHAT
        assert_eq!(r.read_u8(), Some(1)); // nation = 1 (C++ default)
        assert_eq!(r.read_u32(), Some(0xFFFFFFFF)); // sender_id = -1 (C++ default)
                                                    // SByte string (empty name)
        let sender_name = r.read_sbyte_string();
        assert_eq!(sender_name.as_deref(), Some(""));
        // DByte string (message)
        let msg = r.read_string();
        assert_eq!(msg.as_deref(), Some("Server maintenance in 10 minutes!"));
        assert_eq!(r.read_i8(), Some(0)); // personal_rank
        assert_eq!(r.read_u8(), Some(0)); // authority
        assert_eq!(r.read_u8(), Some(0)); // system_msg = 0 (C++ default)
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_notice_packet_empty_message() {
        let pkt = build_notice_packet(7, "");
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(7)); // chat_type
        assert_eq!(r.read_u8(), Some(1)); // nation = 1
        assert_eq!(r.read_u32(), Some(0xFFFFFFFF)); // sender_id = -1
        assert_eq!(r.read_u8(), Some(0)); // name_len
        assert_eq!(r.read_u16(), Some(0)); // message_len = 0
        assert_eq!(r.read_i8(), Some(0)); // personal_rank
        assert_eq!(r.read_u8(), Some(0)); // authority
        assert_eq!(r.read_u8(), Some(0)); // system_msg = 0
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_notice_packet_different_chat_type() {
        // Type 8 = WAR_SYSTEM_CHAT
        let pkt = build_notice_packet(8, "War begins!");
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(8)); // chat_type = WAR_SYSTEM_CHAT
        assert_eq!(r.read_u8(), Some(1)); // nation = 1
        assert_eq!(r.read_u32(), Some(0xFFFFFFFF)); // sender_id = -1
    }

    // ── Schedule Building Tests ─────────────────────────────────────────

    #[test]
    fn test_build_schedule_empty_input() {
        let notices: Vec<TimedNoticeRow> = Vec::new();
        let schedule = build_schedule(&notices);
        assert!(schedule.is_empty());
    }

    #[test]
    fn test_build_schedule_single_notice() {
        let notices = vec![TimedNoticeRow {
            n_index: 1,
            notice_type: 7,
            notice: "Hello world".to_string(),
            zone_id: 0,
            time_minutes: 5,
        }];
        let schedule = build_schedule(&notices);
        assert_eq!(schedule.len(), 1);
        let sched = schedule.get(&1).unwrap();
        assert_eq!(sched.notice_type, 7);
        assert_eq!(sched.notice, "Hello world");
        assert_eq!(sched.zone_id, 0);
        assert_eq!(sched.interval, Duration::from_secs(5 * 60));
    }

    #[test]
    fn test_build_schedule_clamps_minimum_interval() {
        let notices = vec![TimedNoticeRow {
            n_index: 1,
            notice_type: 7,
            notice: "Test".to_string(),
            zone_id: 0,
            time_minutes: 0, // should be clamped to 1
        }];
        let schedule = build_schedule(&notices);
        let sched = schedule.get(&1).unwrap();
        assert_eq!(sched.interval, Duration::from_secs(60)); // 1 minute minimum
    }

    #[test]
    fn test_build_schedule_negative_interval_clamped() {
        let notices = vec![TimedNoticeRow {
            n_index: 1,
            notice_type: 7,
            notice: "Test".to_string(),
            zone_id: 0,
            time_minutes: -5, // should be clamped to 1
        }];
        let schedule = build_schedule(&notices);
        let sched = schedule.get(&1).unwrap();
        assert_eq!(sched.interval, Duration::from_secs(60)); // 1 minute minimum
    }

    #[test]
    fn test_build_schedule_skips_empty_notice() {
        let notices = vec![TimedNoticeRow {
            n_index: 1,
            notice_type: 7,
            notice: "".to_string(), // empty notice text
            zone_id: 0,
            time_minutes: 5,
        }];
        let schedule = build_schedule(&notices);
        assert!(schedule.is_empty(), "empty notice text should be skipped");
    }

    #[test]
    fn test_build_schedule_zone_specific() {
        let notices = vec![TimedNoticeRow {
            n_index: 2,
            notice_type: 7,
            notice: "Zone event!".to_string(),
            zone_id: 21, // specific zone
            time_minutes: 10,
        }];
        let schedule = build_schedule(&notices);
        let sched = schedule.get(&2).unwrap();
        assert_eq!(sched.zone_id, 21);
        assert_eq!(sched.interval, Duration::from_secs(10 * 60));
    }

    #[test]
    fn test_build_schedule_multiple_notices() {
        let notices = vec![
            TimedNoticeRow {
                n_index: 1,
                notice_type: 7,
                notice: "Notice 1".to_string(),
                zone_id: 0,
                time_minutes: 5,
            },
            TimedNoticeRow {
                n_index: 2,
                notice_type: 8,
                notice: "Notice 2".to_string(),
                zone_id: 10,
                time_minutes: 15,
            },
            TimedNoticeRow {
                n_index: 3,
                notice_type: 7,
                notice: "".to_string(), // skipped
                zone_id: 0,
                time_minutes: 1,
            },
        ];
        let schedule = build_schedule(&notices);
        assert_eq!(schedule.len(), 2); // third notice skipped (empty)
        assert!(schedule.contains_key(&1));
        assert!(schedule.contains_key(&2));
        assert!(!schedule.contains_key(&3));
    }

    // ── Tick Scheduling Tests ───────────────────────────────────────────

    #[test]
    fn test_tick_broadcasts_due_notice_to_all() {
        let world = WorldState::new();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        world.register_session(1, tx);
        // Set a character so the session is "in-game"
        world.update_session(1, |h| {
            h.character = Some(test_character(1));
        });

        let mut scheduled = HashMap::new();
        scheduled.insert(
            1,
            ScheduledNotice {
                notice_type: 7,
                notice: "Test broadcast".to_string(),
                zone_id: 0, // all zones
                interval: Duration::from_secs(300),
                next_broadcast_at: Instant::now()
                    .checked_sub(Duration::from_secs(1))
                    .unwrap_or(Instant::now()), // already due
            },
        );

        process_timed_notice_tick(&world, &mut scheduled);

        // Should have received a WIZ_CHAT packet
        let pkt = rx.try_recv().expect("should receive broadcast packet");
        assert_eq!(pkt.opcode, Opcode::WizChat as u8);

        // next_broadcast_at should have been rescheduled
        let sched = scheduled.get(&1).unwrap();
        assert!(
            sched.next_broadcast_at
                > Instant::now()
                    .checked_sub(Duration::from_secs(1))
                    .unwrap_or(Instant::now())
        );
    }

    #[test]
    fn test_tick_does_not_broadcast_before_due() {
        let world = WorldState::new();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        world.register_session(1, tx);
        world.update_session(1, |h| {
            h.character = Some(test_character(1));
        });

        let mut scheduled = HashMap::new();
        scheduled.insert(
            1,
            ScheduledNotice {
                notice_type: 7,
                notice: "Not yet".to_string(),
                zone_id: 0,
                interval: Duration::from_secs(300),
                next_broadcast_at: Instant::now() + Duration::from_secs(300), // not due yet
            },
        );

        process_timed_notice_tick(&world, &mut scheduled);

        // No packet should be sent
        assert!(
            rx.try_recv().is_err(),
            "should not broadcast before due time"
        );
    }

    #[test]
    fn test_tick_zone_specific_broadcast() {
        let world = WorldState::new();

        // Session 1: in zone 21
        let (tx1, mut rx1) = tokio::sync::mpsc::unbounded_channel();
        world.register_session(1, tx1);
        world.update_session(1, |h| {
            h.character = Some(test_character(1));
            h.position.zone_id = 21;
        });

        // Session 2: in zone 1 (different zone)
        let (tx2, mut rx2) = tokio::sync::mpsc::unbounded_channel();
        world.register_session(2, tx2);
        world.update_session(2, |h| {
            h.character = Some(test_character(2));
            h.position.zone_id = 1;
        });

        let mut scheduled = HashMap::new();
        scheduled.insert(
            1,
            ScheduledNotice {
                notice_type: 7,
                notice: "Zone 21 event!".to_string(),
                zone_id: 21, // only zone 21
                interval: Duration::from_secs(600),
                next_broadcast_at: Instant::now()
                    .checked_sub(Duration::from_secs(1))
                    .unwrap_or(Instant::now()),
            },
        );

        process_timed_notice_tick(&world, &mut scheduled);

        // Session 1 (zone 21) should receive the notice
        let pkt = rx1
            .try_recv()
            .expect("zone 21 session should receive notice");
        assert_eq!(pkt.opcode, Opcode::WizChat as u8);

        // Session 2 (zone 1) should NOT receive the notice
        assert!(
            rx2.try_recv().is_err(),
            "zone 1 session should not receive zone-21 notice"
        );
    }

    #[test]
    fn test_tick_empty_schedule_no_op() {
        let world = WorldState::new();
        let mut scheduled = HashMap::new();
        // Should not panic with empty schedule
        process_timed_notice_tick(&world, &mut scheduled);
    }

    // ── Constants Tests ─────────────────────────────────────────────────

    #[test]
    fn test_timed_notice_tick_interval() {
        assert_eq!(TIMED_NOTICE_TICK_SECS, 60);
    }

    #[test]
    fn test_min_interval_minutes() {
        assert_eq!(MIN_INTERVAL_MINUTES, 1);
    }
}
