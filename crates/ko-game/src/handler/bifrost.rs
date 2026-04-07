//! WIZ_BIFROST (0x7B) handler -- Bifrost / Beef Roast Event lifecycle.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/BeefEventNew.cpp`
//!                `KOOriginalGameServer/GameServer/EventMainTimer.cpp`
//!
//! ## Sub-opcodes (TempleOpCodes from packets.h)
//!
//! | Value | Name            | Description                              |
//! |-------|-----------------|------------------------------------------|
//! | 2     | BIFROST_EVENT   | Bifrost event remaining time             |
//! | 3     | TEMPLE_SCREEN   | Temple event screen info                 |
//! | 5     | MONSTER_SQUARD  | Monster squad timer (Monster Stone)      |
//!
//! ## Lifecycle
//!
//! ```text
//! Inactive ──[GM +bifroststart]──> Active (monument attackable)
//! Active ──[monument destroyed]──> Farming (winner enters, loser waits)
//! Active ──[timer expires, no kill]──> Reset (draw)
//! Farming ──[farming_end_time reached]──> Reset (kick all, finish notice)
//! ```

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use ko_protocol::{Opcode, Packet, PacketReader};
use tracing::debug;

use crate::session::{ClientSession, SessionState};
use crate::world::{WorldState, ZONE_BIFROST, ZONE_RONARK_LAND};

/// Bifrost sub-opcode constants.
const BIFROST_EVENT: u8 = 2;

/// Notice type constants for `broadcast_beef_notice`.
///
/// C++ Reference: `BeefEventNew.cpp:122-212` — switch(NoticeType)
pub const NOTICE_START: u8 = 1;
pub const NOTICE_DRAW: u8 = 2;
pub const NOTICE_VICTORY: u8 = 3;
pub const NOTICE_FINISH: u8 = 4;
pub const NOTICE_LOSER_SIGN: u8 = 5;

/// Default monument phase duration in minutes.
///
/// C++ Reference: `pPlayInfo->MonumentTime * MINUTE` — typical default 120 minutes.
const DEFAULT_MONUMENT_MINUTES: u32 = 120;

/// Handle incoming WIZ_BIFROST (0x7B) packet.
///
/// C++ Reference: `EventSigningSystem.cpp:5-41` -- `SendEventRemainingTime()`
pub async fn handle(session: &mut ClientSession, packet: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&packet.data);
    let sub = reader.read_u8().unwrap_or(0);

    debug!("[{}] WIZ_BIFROST sub-opcode={}", session.addr(), sub);

    match sub {
        BIFROST_EVENT => {
            let remaining = session.world().get_bifrost_remaining_secs();
            let mut resp = Packet::new(Opcode::WizBifrost as u8);
            resp.write_u8(BIFROST_EVENT);
            resp.write_u32(remaining);
            session.send_packet(&resp).await?;
        }
        _ => {
            debug!(
                "[{}] WIZ_BIFROST unhandled sub-opcode={}",
                session.addr(),
                sub
            );
        }
    }

    Ok(())
}

// ── Event Lifecycle ──────────────────────────────────────────────────────────

/// Result of a single Bifrost timer tick.
#[derive(Debug, PartialEq, Eq)]
pub enum BifrostTickResult {
    /// Nothing happened.
    Idle,
    /// Monument timer expired with no kill (draw). Event reset needed.
    Draw,
    /// Loser nation sign-in time reached.
    LoserSignOpened,
    /// Farming phase expired. Event reset + kick needed.
    FarmingExpired,
}

/// Perform one Bifrost timer tick (called every 1 second from event_system).
///
/// C++ Reference: `SingleOtherEventLocalTimer()` in `EventMainTimer.cpp:320-331`
///                `EventMainTimer()` in `EventMainTimer.cpp:244-258`
pub fn bifrost_tick(world: &WorldState) -> BifrostTickResult {
    // Decrement remaining seconds
    let prev = world.get_bifrost_remaining_secs();
    if prev > 0 {
        world.set_bifrost_remaining_secs(prev - 1);
    }

    let beef = world.get_beef_event();

    if !beef.is_active {
        return BifrostTickResult::Idle;
    }

    // Monument phase: timer reached 0 without monument being destroyed → draw
    if !beef.is_monument_dead && prev > 0 && prev - 1 == 0 {
        return BifrostTickResult::Draw;
    }

    // Farming phase checks
    if beef.is_monument_dead {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Check loser nation sign-in
        if !beef.is_loser_sign && beef.loser_sign_time > 0 && now >= beef.loser_sign_time {
            world.update_beef_event(|b| {
                b.is_loser_sign = true;
            });
            return BifrostTickResult::LoserSignOpened;
        }

        // Check farming phase expiry
        if beef.farming_end_time > 0 && now >= beef.farming_end_time {
            return BifrostTickResult::FarmingExpired;
        }
    }

    BifrostTickResult::Idle
}

/// Start the Bifrost event (monument phase).
///
/// C++ Reference: `BeefEventManuelOpening()` in `EventMainTimer.cpp:106-138`
pub fn bifrost_start(world: &WorldState, monument_minutes: Option<u32>) {
    let beef = world.get_beef_event();
    if beef.is_active {
        tracing::warn!("Bifrost event already active, ignoring start");
        return;
    }

    let minutes = monument_minutes.unwrap_or(DEFAULT_MONUMENT_MINUTES);
    let secs = minutes * 60;

    world.update_beef_event(|b| {
        b.is_active = true;
        b.is_attackable = true;
        b.is_monument_dead = false;
        b.winner_nation = 0;
        b.is_farming_play = false;
        b.farming_end_time = 0;
        b.loser_sign_time = 0;
        b.is_loser_sign = false;
    });

    world.set_bifrost_remaining_secs(secs);

    // Broadcast time + start notice to Bifrost/Ronark Land zones
    broadcast_beef_time_update(world);
    broadcast_beef_notice(world, NOTICE_START);

    tracing::info!(
        "Bifrost event started: monument_time={}min ({}s)",
        minutes,
        secs
    );
}

/// Reset the Bifrost event to inactive state.
///
/// C++ Reference: `ResetBeefEvent()` in `BeefEventNew.cpp:28-98`
pub fn bifrost_reset(world: &WorldState) {
    world.update_beef_event(|b| {
        b.is_active = false;
        b.is_attackable = false;
        b.is_monument_dead = false;
        b.is_farming_play = false;
        b.winner_nation = 0;
        b.farming_end_time = 0;
        b.loser_sign_time = 0;
        b.is_loser_sign = false;
    });

    world.set_bifrost_remaining_secs(0);

    tracing::info!("Bifrost event reset to inactive");
}

/// Check if a player can enter the Bifrost zone.
///
/// C++ Reference: `CUser::BeefEventLogin()` in `BeefEventNew.cpp:243-254`
///                `CharacterSelectionHandler.cpp:720` — selchar zone gate
///
/// Returns `true` if the player should be redirected to home zone.
pub fn should_redirect_from_bifrost(world: &WorldState, player_nation: u8) -> bool {
    let beef = world.get_beef_event();

    // Not active → redirect
    if !beef.is_active {
        return true;
    }

    // Active but monument not dead → redirect (still in combat phase)
    if !beef.is_monument_dead {
        return true;
    }

    // Monument dead but not farming → redirect
    if !beef.is_farming_play {
        return true;
    }

    // Farming active: winner nation can enter, loser needs sign-in permission
    if beef.winner_nation != 0 && beef.winner_nation != player_nation && !beef.is_loser_sign {
        return true;
    }

    false
}

// ── Broadcasting ──────────────────────────────────────────────────────────────

/// Broadcast remaining time to all players in Bifrost and Ronark Land zones.
///
/// C++ Reference: `BeefEventUpdateTime()` in `BeefEventNew.cpp:215-240`
pub fn broadcast_beef_time_update(world: &WorldState) {
    let remaining = world.get_bifrost_remaining_secs();
    let mut pkt = Packet::new(Opcode::WizBifrost as u8);
    pkt.write_u8(BIFROST_EVENT);
    pkt.write_u32(remaining);

    let arc_pkt = Arc::new(pkt);
    world.broadcast_to_zone(ZONE_BIFROST, Arc::clone(&arc_pkt), None);
    world.broadcast_to_zone(ZONE_RONARK_LAND, Arc::clone(&arc_pkt), None);
}

/// Broadcast a Bifrost event notice to players in Bifrost and Ronark Land zones.
///
/// C++ Reference: `BeefEventSendNotice()` in `BeefEventNew.cpp:101-213`
///
/// Uses WAR_SYSTEM_CHAT (chat type 8) with server resource strings.
pub fn broadcast_beef_notice(world: &WorldState, notice_type: u8) {
    let beef = world.get_beef_event();
    let message = match notice_type {
        NOTICE_START => "[Announcement] Beef Roast event has started!".to_string(),
        NOTICE_DRAW => "[Announcement] Beef Roast event ended in a draw!".to_string(),
        NOTICE_VICTORY => {
            let nation_name = if beef.winner_nation == 2 {
                "El Morad"
            } else {
                "Karus"
            };
            format!("[Announcement] {nation_name} has destroyed the monument!")
        }
        NOTICE_FINISH => "[Announcement] Beef Roast farming phase has ended!".to_string(),
        NOTICE_LOSER_SIGN => {
            let loser_name = if beef.winner_nation == 2 {
                "Karus"
            } else {
                "El Morad"
            };
            format!("[Announcement] {loser_name} can now enter Bifrost!")
        }
        _ => {
            tracing::warn!("Beef event notice: unknown type {}", notice_type);
            return;
        }
    };

    let pkt = build_war_system_chat(&message);
    let arc_pkt = Arc::new(pkt);
    world.broadcast_to_zone(ZONE_BIFROST, Arc::clone(&arc_pkt), None);
    world.broadcast_to_zone(ZONE_RONARK_LAND, Arc::clone(&arc_pkt), None);
}

/// Build a WAR_SYSTEM_CHAT packet (chat type 8).
///
/// C++ Reference: `ChatPacket::Construct(&x, WAR_SYSTEM_CHAT, &notice)`
fn build_war_system_chat(message: &str) -> Packet {
    // Same format as timed_notice::build_notice_packet but with chat_type=8
    crate::systems::timed_notice::build_notice_packet(8, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bifrost_event_packet_inactive() {
        let mut resp = Packet::new(Opcode::WizBifrost as u8);
        resp.write_u8(BIFROST_EVENT);
        resp.write_u32(0);

        assert_eq!(resp.opcode, 0x7B);
        let mut reader = PacketReader::new(&resp.data);
        assert_eq!(reader.read_u8(), Some(2));
        assert_eq!(reader.read_u32(), Some(0));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_bifrost_event_packet_active() {
        let remaining: u32 = 300;
        let mut resp = Packet::new(Opcode::WizBifrost as u8);
        resp.write_u8(BIFROST_EVENT);
        resp.write_u32(remaining);

        assert_eq!(resp.opcode, 0x7B);
        let mut reader = PacketReader::new(&resp.data);
        assert_eq!(reader.read_u8(), Some(2));
        assert_eq!(reader.read_u32(), Some(300));
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_bifrost_tick_idle_when_inactive() {
        let world = WorldState::new();
        let result = bifrost_tick(&world);
        assert_eq!(result, BifrostTickResult::Idle);
    }

    #[test]
    fn test_bifrost_tick_draw_when_timer_expires() {
        let world = WorldState::new();

        // Set up active event with 1 second remaining
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_attackable = true;
        });
        world.set_bifrost_remaining_secs(1);

        let result = bifrost_tick(&world);
        assert_eq!(result, BifrostTickResult::Draw);
        assert_eq!(world.get_bifrost_remaining_secs(), 0);
    }

    #[test]
    fn test_bifrost_tick_decrement() {
        let world = WorldState::new();

        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_attackable = true;
        });
        world.set_bifrost_remaining_secs(100);

        let result = bifrost_tick(&world);
        assert_eq!(result, BifrostTickResult::Idle);
        assert_eq!(world.get_bifrost_remaining_secs(), 99);
    }

    #[test]
    fn test_bifrost_tick_farming_expired() {
        let world = WorldState::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_monument_dead = true;
            b.is_farming_play = true;
            b.winner_nation = 1;
            b.farming_end_time = now - 1; // already expired
            b.loser_sign_time = now - 2;
            b.is_loser_sign = true;
        });
        world.set_bifrost_remaining_secs(10);

        let result = bifrost_tick(&world);
        assert_eq!(result, BifrostTickResult::FarmingExpired);
    }

    #[test]
    fn test_bifrost_tick_loser_sign_opened() {
        let world = WorldState::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_monument_dead = true;
            b.is_farming_play = true;
            b.winner_nation = 2;
            b.farming_end_time = now + 3600; // still far in future
            b.loser_sign_time = now - 1; // already passed
            b.is_loser_sign = false;
        });
        world.set_bifrost_remaining_secs(100);

        let result = bifrost_tick(&world);
        assert_eq!(result, BifrostTickResult::LoserSignOpened);

        // Verify is_loser_sign was set
        let beef = world.get_beef_event();
        assert!(beef.is_loser_sign);
    }

    #[test]
    fn test_should_redirect_inactive() {
        let world = WorldState::new();
        assert!(should_redirect_from_bifrost(&world, 1));
    }

    #[test]
    fn test_should_redirect_active_no_monument_kill() {
        let world = WorldState::new();
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_attackable = true;
        });
        // Monument not dead → redirect (can't enter during combat)
        assert!(should_redirect_from_bifrost(&world, 1));
    }

    #[test]
    fn test_should_not_redirect_winner_nation() {
        let world = WorldState::new();
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_monument_dead = true;
            b.is_farming_play = true;
            b.winner_nation = 1; // Karus won
        });
        // Winner nation (Karus=1) can enter
        assert!(!should_redirect_from_bifrost(&world, 1));
    }

    #[test]
    fn test_should_redirect_loser_no_sign() {
        let world = WorldState::new();
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_monument_dead = true;
            b.is_farming_play = true;
            b.winner_nation = 1; // Karus won
            b.is_loser_sign = false;
        });
        // Loser nation (Elmorad=2) cannot enter yet
        assert!(should_redirect_from_bifrost(&world, 2));
    }

    #[test]
    fn test_should_not_redirect_loser_with_sign() {
        let world = WorldState::new();
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_monument_dead = true;
            b.is_farming_play = true;
            b.winner_nation = 1;
            b.is_loser_sign = true; // Loser sign-in opened
        });
        // Loser nation (Elmorad=2) CAN enter now
        assert!(!should_redirect_from_bifrost(&world, 2));
    }

    #[test]
    fn test_bifrost_reset_clears_all() {
        let world = WorldState::new();
        world.update_beef_event(|b| {
            b.is_active = true;
            b.is_attackable = true;
            b.is_monument_dead = true;
            b.winner_nation = 2;
            b.is_farming_play = true;
            b.farming_end_time = 9999;
            b.loser_sign_time = 8888;
            b.is_loser_sign = true;
        });
        world.set_bifrost_remaining_secs(5000);

        bifrost_reset(&world);

        let beef = world.get_beef_event();
        assert!(!beef.is_active);
        assert!(!beef.is_attackable);
        assert!(!beef.is_monument_dead);
        assert_eq!(beef.winner_nation, 0);
        assert!(!beef.is_farming_play);
        assert_eq!(beef.farming_end_time, 0);
        assert_eq!(beef.loser_sign_time, 0);
        assert!(!beef.is_loser_sign);
        assert_eq!(world.get_bifrost_remaining_secs(), 0);
    }

    #[test]
    fn test_notice_message_content() {
        // Just verify the packet builds without panicking
        let pkt = build_war_system_chat("Test notice");
        assert_eq!(pkt.opcode, Opcode::WizChat as u8);
    }
}
