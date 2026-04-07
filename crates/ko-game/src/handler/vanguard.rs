//! WIZ_VANGUARD (0xB6) handler -- Wanted Event System.
//! ## Sub-opcodes
//! | Value | Name            | Description                              |
//! |-------|-----------------|------------------------------------------|
//! | 1     | Register        | Player registers for the Wanted event    |
//! | 2     | UserList/Move   | Server sends wanted user list / movement |
//! The Wanted Event selects random players in PK zones and marks them
//! as "wanted". Killing a wanted player earns loyalty and items.
//! Registration is via WIZ_VANGUARD sub-opcode 1.

use std::sync::Arc;

use ko_protocol::{Opcode, Packet, PacketReader};
use rand::seq::SliceRandom;
use tracing::debug;

use crate::session::{ClientSession, SessionState};
use crate::world::{
    WantedEventStatus, WorldState, ZONE_ARDREAM, ZONE_ELMORAD, ZONE_KARUS, ZONE_RONARK_LAND,
    ZONE_RONARK_LAND_BASE,
};
use crate::zone::SessionId;

/// Vanguard sub-opcode constants.
mod sub_opcode {
    /// Player registers for wanted event.
    pub const REGISTER: u8 = 1;
    /// Server sends user list or position update.
    pub const USER_LIST_MOVE: u8 = 2;
}

/// Vanguard user-list/move sub-sub-opcodes.
mod move_sub {
    /// Send the wanted user name list.
    pub const USER_LIST: u8 = 1;
    /// Broadcast wanted user position on the map.
    pub const POSITION: u8 = 2;
}

use crate::world::{NATION_ELMORAD, NATION_KARUS};

/// Handle incoming WIZ_VANGUARD (0xB6) packet.
/// The client sends sub-opcode 1 to register for the wanted event.
pub fn handle(session: &mut ClientSession, packet: Packet) -> anyhow::Result<()> {
    if session.state() != SessionState::InGame {
        return Ok(());
    }

    // Dead players cannot register for wanted events
    if session.world().is_player_dead(session.session_id()) {
        return Ok(());
    }

    let mut reader = PacketReader::new(&packet.data);
    let opcode = reader.read_u8().unwrap_or(0);

    match opcode {
        sub_opcode::REGISTER => handle_wanted_register(session),
        _ => {
            debug!(
                "[{}] WIZ_VANGUARD unhandled sub-opcode={}",
                session.addr(),
                opcode
            );
            Ok(())
        }
    }
}

/// Map a zone ID to a wanted event room index (0-2).
/// Returns `None` if the zone is not a PK zone with a wanted event room.
pub fn wanted_get_room(zone_id: u16) -> Option<usize> {
    match zone_id {
        ZONE_RONARK_LAND => Some(0),
        ZONE_ARDREAM => Some(1),
        ZONE_RONARK_LAND_BASE => Some(2),
        _ => None,
    }
}

/// Map a wanted event room index (0-2) back to a zone ID.
pub fn wanted_get_zone(room: usize) -> Option<u16> {
    match room {
        0 => Some(ZONE_RONARK_LAND),
        1 => Some(ZONE_ARDREAM),
        2 => Some(ZONE_RONARK_LAND_BASE),
        _ => None,
    }
}

/// Build a WIZ_VANGUARD position broadcast packet for a wanted player.
/// Packet format:
/// ```text
/// WIZ_VANGUARD(0xB6) << u8(0x02) << u8(0x02) << u8(0x01) << u8(0x00)
///                     << u16(x) << u16(z) << string(name)
/// ```
/// Sent to the enemy nation in the same zone so they can see the wanted
/// player's position on their minimap.
pub fn build_wanted_position_packet(x: u16, z: u16, name: &str) -> Packet {
    let mut pkt = Packet::new(Opcode::WizVanguard as u8);
    pkt.write_u8(sub_opcode::USER_LIST_MOVE);
    pkt.write_u8(move_sub::POSITION);
    pkt.write_u8(0x01);
    pkt.write_u8(0x00);
    pkt.write_u16(x);
    pkt.write_u16(z);
    pkt.write_string(name);
    pkt
}

/// Build a WIZ_VANGUARD user list packet for broadcasting wanted player names.
/// Packet format:
/// ```text
/// WIZ_VANGUARD(0xB6) << u8(0x02) << u8(0x01) << u8(count)
///                     << [string(name) for each wanted player]
/// ```
pub fn build_wanted_user_list_packet(names: &[String]) -> Packet {
    let mut pkt = Packet::new(Opcode::WizVanguard as u8);
    pkt.write_u8(sub_opcode::USER_LIST_MOVE);
    pkt.write_u8(move_sub::USER_LIST);
    pkt.write_u8(names.len() as u8);
    for name in names {
        pkt.write_string(name);
    }
    pkt
}

/// Broadcast wanted player positions to the enemy nation in a given zone.
/// For each wanted player in the zone, builds a position packet and sends it
/// to the enemy nation. This is called periodically (every 60s) during a
/// running wanted event.
pub fn broadcast_wanted_user_positions(world: &WorldState, zone_id: u16) {
    let wanted_players = world.collect_wanted_players_in_zone(zone_id);

    for (_sid, nation, x, z, name) in &wanted_players {
        let pkt = build_wanted_position_packet(*x, *z, name);

        // Send to the ENEMY nation (Elmorad players see Karus wanted, and vice versa)
        let target_nation = if *nation == NATION_ELMORAD {
            NATION_KARUS
        } else {
            NATION_ELMORAD
        };

        world.broadcast_to_zone_nation(zone_id, target_nation, Arc::new(pkt), None);
    }
}

/// Check all wanted event rooms and broadcast positions for active events.
/// Uses `m_WantedSystemMapShowTime` to throttle to every 60 seconds.
/// This function checks the global map-show timer and, if enough time has
/// elapsed, iterates all 3 wanted rooms and broadcasts positions for rooms
/// in the `Running` state.
pub fn tick_wanted_position_broadcasts(world: &WorldState, now_unix: u64) {
    use std::sync::atomic::Ordering;

    // and elapsed > 60. When last_show == 0 (not yet initialized), skip broadcast entirely.
    let last_show = world.wanted_map_show_time.load(Ordering::Relaxed);
    if last_show == 0 {
        return;
    }
    if now_unix.saturating_sub(last_show) < 60 {
        return;
    }

    // Check if any room is in Running state
    let rooms = world.wanted_rooms().read().clone();

    let mut any_broadcast = false;
    for (room_idx, room) in rooms.iter().enumerate() {
        if room.status != WantedEventStatus::Running {
            continue;
        }
        if let Some(zone_id) = wanted_get_zone(room_idx) {
            broadcast_wanted_user_positions(world, zone_id);
            any_broadcast = true;
        }
    }

    if any_broadcast {
        world
            .wanted_map_show_time
            .store(now_unix, Ordering::Relaxed);
    }
}

/// Wanted event item given to the killer of a wanted player.
const WANTED_KILL_ITEM: u32 = 914052000;

/// Loyalty reward for killing a wanted player (solo).
const WANTED_KILL_LOYALTY_SOLO: u32 = 80;

/// Loyalty reward for killing a wanted player (in party).
const WANTED_KILL_LOYALTY_PARTY: u32 = 160;

/// Delay in seconds before warping the killer to their nation town.
const WANTED_KILL_WARP_DELAY_SECS: u64 = 15;

/// Handle the death of a wanted player: clear their wanted status,
/// give rewards to the killer, and schedule a delayed town warp for the killer.
/// Called from the combat/death handler when a wanted player is killed.
/// - Clears `is_wanted` / `wanted_expiry_time` on the victim
/// - Removes the victim from the wanted room list
/// - Gives the killer item + loyalty
/// - Schedules a 15-second delayed warp for the killer to their nation's town
pub fn handle_wanted_kill(world: &Arc<WorldState>, victim_sid: SessionId, killer_sid: SessionId) {
    // Clear wanted status on victim
    world.update_session(victim_sid, |h| {
        h.is_wanted = false;
        h.wanted_expiry_time = 0;
    });

    // Remove from wanted room list
    let victim_zone = world
        .with_session(victim_sid, |h| h.position.zone_id)
        .unwrap_or(0);
    if let Some(room_idx) = wanted_get_room(victim_zone) {
        let victim_nation = world
            .get_character_info(victim_sid)
            .map(|c| c.nation)
            .unwrap_or(0);
        {
            let mut rooms = world.wanted_rooms().write();
            let room = &mut rooms[room_idx];
            if victim_nation == NATION_ELMORAD {
                room.elmo_list.retain(|&s| s != victim_sid);
            } else {
                room.karus_list.retain(|&s| s != victim_sid);
            }
        }
    }

    // Give killer reward item
    world.give_item_with_expiry(killer_sid, WANTED_KILL_ITEM, 1, 0);

    // Give killer loyalty — double if in party
    let in_party = world.is_in_party(killer_sid);
    let loyalty_amount = if in_party {
        WANTED_KILL_LOYALTY_PARTY
    } else {
        WANTED_KILL_LOYALTY_SOLO
    };
    let mut new_loy = 0u32;
    world.update_character_stats(killer_sid, |ch| {
        ch.loyalty = ch.loyalty.saturating_add(loyalty_amount);
        ch.loyalty_monthly = ch.loyalty_monthly.saturating_add(loyalty_amount);
        new_loy = ch.loyalty;
    });
    let mut loy_pkt = Packet::new(Opcode::WizLoyaltyChange as u8);
    loy_pkt.write_u8(1);
    loy_pkt.write_u32(loyalty_amount);
    loy_pkt.write_u32(new_loy);
    world.send_to_session_owned(killer_sid, loy_pkt);

    debug!(
        "Wanted kill: victim={} killer={} — rewards given, scheduling 15s town warp",
        victim_sid, killer_sid
    );

    // Schedule a 15s delayed warp for the killer to their nation town
    schedule_killer_town_warp(Arc::clone(world), killer_sid);
}

/// Schedule a 15-second delayed warp for the killer to their nation's town.
/// After killing a wanted player, the killer is warped back to their nation's
/// town zone to prevent camping. Uses `tokio::spawn` with a 15s sleep.
fn schedule_killer_town_warp(world: Arc<WorldState>, killer_sid: SessionId) {
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(
            WANTED_KILL_WARP_DELAY_SECS,
        ))
        .await;

        // Check if the killer is still in-game
        let killer_info =
            world.with_session(killer_sid, |h| h.character.as_ref().map(|c| c.nation));
        let nation = match killer_info {
            Some(Some(n)) => n,
            _ => return, // Killer disconnected
        };

        // Determine target zone: Karus town (zone 1) or Elmorad town (zone 2)
        let target_zone = if nation == NATION_KARUS {
            ZONE_KARUS
        } else {
            ZONE_ELMORAD
        };

        // Build and send zone change packet (type=3 server teleport, coords=0 for default spawn)
        let pkt = build_wanted_warp_packet(target_zone, nation);
        world.update_position(killer_sid, target_zone, 0.0, 0.0, 0.0);
        world.send_to_session_owned(killer_sid, pkt);

        debug!(
            "Wanted kill warp: killer={} warped to zone={} after 15s delay",
            killer_sid, target_zone
        );
    });
}

/// Build a WIZ_ZONE_CHANGE teleport packet for the wanted kill warp.
/// Coords 0,0 = use default spawn position for the zone.
pub fn build_wanted_warp_packet(zone_id: u16, nation: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizZoneChange as u8);
    pkt.write_u8(3); // ZONE_CHANGE_TELEPORT
    pkt.write_u16(zone_id);
    pkt.write_u16(0); // padding
    pkt.write_u16(0); // x = 0 (use default spawn)
    pkt.write_u16(0); // z = 0 (use default spawn)
    pkt.write_u16(0); // y
    pkt.write_u8(nation);
    pkt.write_u16(0xFFFF); // status
    pkt
}

/// Maximum number of players selected per nation per cycle.
const MAX_SELECTING_USER: usize = 5;

/// Survival reward item ID.
const WANTED_SURVIVAL_ITEM: u32 = 914052000;

/// Survival NP reward.
const WANTED_SURVIVAL_LOYALTY: i32 = 300;

/// Invitation window duration in seconds.
const INVITATION_DURATION_SECS: u64 = 11;

/// Initial delay before the first wanted cycle (seconds).
const INITIAL_NEXT_SELECT_SECS: u64 = 60;

/// List-sending phase duration in seconds.
const LIST_PHASE_SECS: u64 = 10;

/// Running phase duration in seconds (10 minutes).
const RUN_PHASE_SECS: u64 = 600;

/// Handle the WIZ_VANGUARD Register sub-opcode.
/// Called when a player accepts the wanted invitation. Adds them to
/// the room list, sets `is_wanted`, and applies the wanted buff skill.
fn handle_wanted_register(session: &mut ClientSession) -> anyhow::Result<()> {
    let world = session.world().clone();
    let sid = session.session_id();

    // Already wanted — cannot register again
    let already_wanted = world.with_session(sid, |h| h.is_wanted).unwrap_or(true);
    if already_wanted {
        return Ok(());
    }

    // Must be alive
    if world.is_player_dead(sid) {
        return Ok(());
    }

    // Must be in a PK zone
    let zone_id = world.get_position(sid).map(|p| p.zone_id).unwrap_or(0);
    let room_idx = match wanted_get_room(zone_id) {
        Some(r) => r,
        None => return Ok(()),
    };

    // Room must be in Invitation phase
    let is_invitation = {
        let rooms = world.wanted_rooms().read();
        rooms[room_idx].status == WantedEventStatus::Invitation
    };
    if !is_invitation {
        return Ok(());
    }

    let nation = world.get_character_info(sid).map(|c| c.nation).unwrap_or(0);
    if nation == 0 {
        return Ok(());
    }

    // Check if already in list
    {
        let rooms = world.wanted_rooms().read();
        let room = &rooms[room_idx];
        let already = if nation == NATION_ELMORAD {
            room.elmo_list.contains(&sid)
        } else {
            room.karus_list.contains(&sid)
        };
        if already {
            return Ok(());
        }
    }

    // Add to room list
    {
        let mut rooms = world.wanted_rooms().write();
        let room = &mut rooms[room_idx];
        if nation == NATION_ELMORAD {
            room.elmo_list.push(sid);
        } else {
            room.karus_list.push(sid);
        }
    }

    // Set wanted status on player
    let now = now_secs();
    world.update_session(sid, |h| {
        h.is_wanted = true;
        h.wanted_expiry_time = (now + RUN_PHASE_SECS) as u32;
    });

    // C++ applies buff 302166 here — we skip magic buff application
    // since the full magic system would need MagicInstance.Run().
    // The player is functionally wanted via is_wanted flag.

    debug!(
        "Wanted register: sid={} zone={} room={} nation={}",
        sid, zone_id, room_idx, nation
    );

    Ok(())
}

/// Main wanted event lifecycle tick — called every second.
/// State machine:
/// - Disabled → (nextselecttime expired) → Selecting → Invitation
/// - Invitation → (invitationtime expired, has registrants) → ListSending
/// - Invitation → (invitationtime expired, no registrants) → Disabled (reset)
/// - ListSending → (listtime expired) → Running + send user list
/// - Running → (finishtime expired) → Disabled + give survival rewards + reset
pub fn tick_wanted_event_lifecycle(world: &WorldState, now: u64) {
    let auto_wanted = world
        .wanted_auto_enabled
        .load(std::sync::atomic::Ordering::Relaxed);
    if !auto_wanted {
        return;
    }

    for room_idx in 0..3 {
        // Check if the zone is valid (has zone data)
        let zone_id = match wanted_get_zone(room_idx) {
            Some(z) => z,
            None => continue,
        };
        if world.get_zone(zone_id).is_none() {
            continue;
        }

        let status = {
            let rooms = world.wanted_rooms().read();
            rooms[room_idx].status
        };

        match status {
            WantedEventStatus::Disabled => {
                let next_select = {
                    let rooms = world.wanted_rooms().read();
                    rooms[room_idx].next_select_time
                };
                if next_select > 0 && now > next_select {
                    wanted_event_selecting(world, room_idx, zone_id, now);
                }
            }
            WantedEventStatus::Invitation => {
                let invitation_time = {
                    let rooms = world.wanted_rooms().read();
                    rooms[room_idx].invitation_time
                };
                if invitation_time > 0 && now > invitation_time {
                    let (has_elmo, has_karus) = {
                        let rooms = world.wanted_rooms().read();
                        let room = &rooms[room_idx];
                        (!room.elmo_list.is_empty(), !room.karus_list.is_empty())
                    };
                    if !has_elmo && !has_karus {
                        // No one registered → reset
                        wanted_event_reset_data(world, room_idx, now);
                        debug!("Wanted room {}: no registrants, resetting", room_idx);
                    } else {
                        // Move to list-sending phase
                        let mut rooms = world.wanted_rooms().write();
                        rooms[room_idx].status = WantedEventStatus::ListSending;
                        rooms[room_idx].list_time = now + LIST_PHASE_SECS;
                        rooms[room_idx].finish_time = now + RUN_PHASE_SECS;
                        debug!(
                            "Wanted room {}: invitation closed, moving to ListSending",
                            room_idx
                        );
                    }
                }
            }
            WantedEventStatus::ListSending => {
                let list_time = {
                    let rooms = world.wanted_rooms().read();
                    rooms[room_idx].list_time
                };
                if list_time > 0 && now > list_time {
                    {
                        let mut rooms = world.wanted_rooms().write();
                        rooms[room_idx].status = WantedEventStatus::Running;
                    }
                    // Initialize position broadcast timer so tick_wanted_position_broadcasts starts
                    world
                        .wanted_map_show_time
                        .store(now, std::sync::atomic::Ordering::Relaxed);
                    wanted_event_user_list_send(world, room_idx, zone_id);
                    debug!("Wanted room {}: now Running", room_idx);
                }
            }
            WantedEventStatus::Running => {
                let finish_time = {
                    let rooms = world.wanted_rooms().read();
                    rooms[room_idx].finish_time
                };
                if finish_time > 0 && now > finish_time {
                    // C++ sets status=Disabled BEFORE finishing to prevent double-reward
                    {
                        let mut rooms = world.wanted_rooms().write();
                        rooms[room_idx].status = WantedEventStatus::Disabled;
                    }
                    wanted_event_finishing(world, room_idx);
                    wanted_event_reset_data(world, room_idx, now);
                    debug!(
                        "Wanted room {}: finished, rewards given, resetting",
                        room_idx
                    );
                }
            }
        }
    }
}

/// Select random players in a PK zone and send invitation packets.
fn wanted_event_selecting(world: &WorldState, room_idx: usize, zone_id: u16, now: u64) {
    // Clear any leftover lists
    {
        let mut rooms = world.wanted_rooms().write();
        rooms[room_idx].elmo_list.clear();
        rooms[room_idx].karus_list.clear();
    }

    // Collect all alive, in-game players in this zone — separate by nation
    let (mut elmo_candidates, mut karus_candidates) = world.collect_zone_alive_by_nation(zone_id);

    if elmo_candidates.is_empty() && karus_candidates.is_empty() {
        // No candidates — reset and try again later
        wanted_event_reset_data(world, room_idx, now);
        return;
    }

    let mut rng = rand::thread_rng();
    let mut any_selected = false;

    // Select up to MAX_SELECTING_USER from each nation and send invitation
    let invitation_pkt = build_wanted_invitation_packet();

    for candidates in [&mut elmo_candidates, &mut karus_candidates] {
        candidates.shuffle(&mut rng);
        for &sid in candidates.iter().take(MAX_SELECTING_USER) {
            world.send_to_session(sid, &invitation_pkt);
            any_selected = true;
        }
    }

    if !any_selected {
        wanted_event_reset_data(world, room_idx, now);
        return;
    }

    // Transition to Invitation phase
    {
        let mut rooms = world.wanted_rooms().write();
        rooms[room_idx].status = WantedEventStatus::Invitation;
        rooms[room_idx].invitation_time = now + INVITATION_DURATION_SECS;
    }

    debug!(
        "Wanted room {}: selected candidates (elmo={}, karus={}), inviting for {}s",
        room_idx,
        elmo_candidates.len().min(MAX_SELECTING_USER),
        karus_candidates.len().min(MAX_SELECTING_USER),
        INVITATION_DURATION_SECS
    );
}

/// Build the WIZ_VANGUARD invitation packet sent to selected players.
/// ```text
/// Packet result(WIZ_VANGUARD, uint8(0x01));
/// result << uint8(0x01) << uint8(0x01);
/// ```
fn build_wanted_invitation_packet() -> Packet {
    let mut pkt = Packet::new(Opcode::WizVanguard as u8);
    pkt.write_u8(sub_opcode::REGISTER);
    pkt.write_u8(0x01);
    pkt.write_u8(0x01);
    pkt
}

/// Send the wanted user name list to the enemy nation in the zone.
fn wanted_event_user_list_send(world: &WorldState, room_idx: usize, zone_id: u16) {
    // Clone SID lists under a short lock, then resolve names without holding wanted_rooms.
    // Avoids lock-ordering risk between wanted_rooms RwLock and sessions DashMap.
    let (elmo_sids, karus_sids) = {
        let rooms = world.wanted_rooms().read();
        let room = &rooms[room_idx];
        (room.elmo_list.clone(), room.karus_list.clone())
    };

    let elmo_names: Vec<String> = elmo_sids
        .iter()
        .filter_map(|&sid| world.get_session_name(sid))
        .collect();
    let karus_names: Vec<String> = karus_sids
        .iter()
        .filter_map(|&sid| world.get_session_name(sid))
        .collect();

    // Send Elmorad wanted list to Karus players, and vice versa
    if !elmo_names.is_empty() {
        let pkt = build_wanted_user_list_packet(&elmo_names);
        world.broadcast_to_zone_nation(zone_id, NATION_KARUS, Arc::new(pkt), None);
    }
    if !karus_names.is_empty() {
        let pkt = build_wanted_user_list_packet(&karus_names);
        world.broadcast_to_zone_nation(zone_id, NATION_ELMORAD, Arc::new(pkt), None);
    }
}

/// Give survival rewards to all remaining wanted players at event end.
fn wanted_event_finishing(world: &WorldState, room_idx: usize) {
    let all_sids: Vec<SessionId> = {
        let rooms = world.wanted_rooms().read();
        let room = &rooms[room_idx];
        room.elmo_list
            .iter()
            .chain(room.karus_list.iter())
            .copied()
            .collect()
    };

    for sid in all_sids {
        // Only reward alive, in-game players who are still wanted
        let is_valid = world
            .with_session(sid, |h| {
                h.is_wanted
                    && h.character
                        .as_ref()
                        .is_some_and(|ch| ch.hp > 0 && ch.res_hp_type != crate::world::USER_DEAD)
            })
            .unwrap_or(false);

        if !is_valid {
            continue;
        }

        // Give survival reward item
        world.give_item_with_expiry(sid, WANTED_SURVIVAL_ITEM, 1, 0);

        // Give 300 loyalty — C++ WandetEvent.cpp:80 uses defaults (false,false,true)
        crate::systems::loyalty::send_loyalty_change(
            world,
            sid,
            WANTED_SURVIVAL_LOYALTY,
            false,
            false,
            true,
        );

        // Clear wanted status
        world.update_session(sid, |h| {
            h.is_wanted = false;
            h.wanted_expiry_time = 0;
        });

        debug!("Wanted survival reward: sid={}", sid);
    }
}

/// Reset the wanted event room data for the next cycle.
fn wanted_event_reset_data(world: &WorldState, room_idx: usize, now: u64) {
    let mut rooms = world.wanted_rooms().write();
    let room = &mut rooms[room_idx];
    room.elmo_list.clear();
    room.karus_list.clear();
    room.status = WantedEventStatus::Disabled;
    room.next_select_time = now + INITIAL_NEXT_SELECT_SECS;
    room.invitation_time = 0;
    room.list_time = 0;
    room.finish_time = 0;
}

/// Return the current UNIX timestamp in seconds.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Initialize the wanted event rooms at server startup.
/// Sets `next_select_time` = now + 60s for each room, and enables the system
/// if auto_wanted is configured.
pub fn initialize_wanted_rooms(world: &WorldState) {
    let now = now_secs();
    let mut rooms = world.wanted_rooms().write();
    for room in rooms.iter_mut() {
        room.next_select_time = now + INITIAL_NEXT_SELECT_SECS;
        room.status = WantedEventStatus::Disabled;
    }
    // Enable the auto-wanted flag (can be controlled by server settings)
    world
        .wanted_auto_enabled
        .store(true, std::sync::atomic::Ordering::Relaxed);
    debug!(
        "Wanted event system initialized: 3 rooms, next_select in {}s",
        INITIAL_NEXT_SELECT_SECS
    );
}

/// Handle the zone change or disconnect of a wanted player (no killer).
/// in `WandetEvent.cpp:265-285` and `ZoneChangeWarpHandler.cpp:289`
/// Called when a wanted player zone changes or disconnects. Clears
/// their wanted status and removes them from the event room without
/// awarding any killer rewards.
pub fn handle_wanted_logout(world: &WorldState, victim_sid: SessionId) {
    // Clear wanted status on victim
    world.update_session(victim_sid, |h| {
        h.is_wanted = false;
        h.wanted_expiry_time = 0;
    });

    // Remove from wanted room list
    let victim_zone = world
        .with_session(victim_sid, |h| h.position.zone_id)
        .unwrap_or(0);
    if let Some(room_idx) = wanted_get_room(victim_zone) {
        let victim_nation = world
            .get_character_info(victim_sid)
            .map(|c| c.nation)
            .unwrap_or(0);
        {
            let mut rooms = world.wanted_rooms().write();
            let room = &mut rooms[room_idx];
            if victim_nation == NATION_ELMORAD {
                room.elmo_list.retain(|&s| s != victim_sid);
            } else {
                room.karus_list.retain(|&s| s != victim_sid);
            }
        }
    }

    debug!(
        "Wanted logout: victim={} — wanted status cleared, no killer rewards",
        victim_sid
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{CharacterInfo, Position};
    use ko_protocol::Opcode;
    use tokio::sync::mpsc;

    #[test]
    fn test_vanguard_opcode_value() {
        assert_eq!(Opcode::WizVanguard as u8, 0xB6);
    }

    #[test]
    fn test_vanguard_register_sub_opcode() {
        assert_eq!(sub_opcode::REGISTER, 1);
    }

    #[test]
    fn test_wanted_get_room_mapping() {
        assert_eq!(wanted_get_room(ZONE_RONARK_LAND), Some(0));
        assert_eq!(wanted_get_room(ZONE_ARDREAM), Some(1));
        assert_eq!(wanted_get_room(ZONE_RONARK_LAND_BASE), Some(2));
        assert_eq!(wanted_get_room(21), None); // Moradon is not PK
        assert_eq!(wanted_get_room(0), None);
    }

    #[test]
    fn test_wanted_get_zone_mapping() {
        assert_eq!(wanted_get_zone(0), Some(ZONE_RONARK_LAND));
        assert_eq!(wanted_get_zone(1), Some(ZONE_ARDREAM));
        assert_eq!(wanted_get_zone(2), Some(ZONE_RONARK_LAND_BASE));
        assert_eq!(wanted_get_zone(3), None);
    }

    #[test]
    fn test_wanted_room_zone_roundtrip() {
        // room -> zone -> room should be identity
        for room in 0..3 {
            let zone = wanted_get_zone(room).unwrap();
            assert_eq!(wanted_get_room(zone), Some(room));
        }
    }

    #[test]
    fn test_build_wanted_position_packet_format() {
        let pkt = build_wanted_position_packet(1234, 5678, "TestPlayer");
        assert_eq!(pkt.opcode, Opcode::WizVanguard as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(sub_opcode::USER_LIST_MOVE)); // 0x02
        assert_eq!(r.read_u8(), Some(move_sub::POSITION)); // 0x02
        assert_eq!(r.read_u8(), Some(0x01));
        assert_eq!(r.read_u8(), Some(0x00));
        assert_eq!(r.read_u16(), Some(1234)); // x
        assert_eq!(r.read_u16(), Some(5678)); // z
        let name = r.read_string().unwrap();
        assert_eq!(name, "TestPlayer");
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_wanted_position_packet_zero_coords() {
        let pkt = build_wanted_position_packet(0, 0, "A");
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(0x02));
        assert_eq!(r.read_u8(), Some(0x02));
        assert_eq!(r.read_u8(), Some(0x01));
        assert_eq!(r.read_u8(), Some(0x00));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_u16(), Some(0));
        let name = r.read_string().unwrap();
        assert_eq!(name, "A");
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_wanted_user_list_packet_empty() {
        let pkt = build_wanted_user_list_packet(&[]);
        assert_eq!(pkt.opcode, Opcode::WizVanguard as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(sub_opcode::USER_LIST_MOVE));
        assert_eq!(r.read_u8(), Some(move_sub::USER_LIST));
        assert_eq!(r.read_u8(), Some(0)); // count = 0
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_wanted_user_list_packet_with_names() {
        let names = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
        let pkt = build_wanted_user_list_packet(&names);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(sub_opcode::USER_LIST_MOVE));
        assert_eq!(r.read_u8(), Some(move_sub::USER_LIST));
        assert_eq!(r.read_u8(), Some(3)); // count

        assert_eq!(r.read_string().unwrap(), "Alpha");
        assert_eq!(r.read_string().unwrap(), "Beta");
        assert_eq!(r.read_string().unwrap(), "Gamma");
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_wanted_event_status_default() {
        let status = WantedEventStatus::default();
        assert_eq!(status, WantedEventStatus::Disabled);
    }

    #[test]
    fn test_nation_constants() {
        assert_eq!(NATION_KARUS, 1);
        assert_eq!(NATION_ELMORAD, 2);
    }

    #[test]
    fn test_wanted_position_packet_matches_cpp_format() {
        // C++ format: WIZ_VANGUARD << u8(0x02) << u8(0x02) << u8(0x01) << u8(0x00)
        //             << u16(GetX()) << u16(GetZ()) << GetName()
        let pkt = build_wanted_position_packet(616, 341, "warrior99");
        let mut r = PacketReader::new(&pkt.data);

        // Sub-opcode structure matches C++ WandetEvent.cpp:245-246
        assert_eq!(r.read_u8(), Some(0x02)); // USER_LIST_MOVE
        assert_eq!(r.read_u8(), Some(0x02)); // POSITION sub
        assert_eq!(r.read_u8(), Some(0x01)); // flag
        assert_eq!(r.read_u8(), Some(0x00)); // padding
        assert_eq!(r.read_u16(), Some(616)); // X coordinate
        assert_eq!(r.read_u16(), Some(341)); // Z coordinate

        let name = r.read_string().unwrap();
        assert_eq!(name, "warrior99");
        assert_eq!(r.remaining(), 0);
    }

    // ── Wanted kill reward constants ──────────────────────────────────

    #[test]
    fn test_wanted_kill_item_constant() {
        // C++ WandetEvent.cpp:288 — GiveItem("Wanted Event", 914052000, 1)
        assert_eq!(WANTED_KILL_ITEM, 914052000);
    }

    #[test]
    fn test_wanted_kill_loyalty_constants() {
        // C++ WandetEvent.cpp:289 — `isInParty() ? 160 : 80`
        assert_eq!(WANTED_KILL_LOYALTY_SOLO, 80);
        assert_eq!(WANTED_KILL_LOYALTY_PARTY, 160);
    }

    #[test]
    fn test_wanted_kill_warp_delay() {
        assert_eq!(WANTED_KILL_WARP_DELAY_SECS, 15);
    }

    // ── build_wanted_warp_packet ──────────────────────────────────────

    #[test]
    fn test_build_wanted_warp_packet_karus() {
        let pkt = build_wanted_warp_packet(ZONE_KARUS, NATION_KARUS);
        assert_eq!(pkt.opcode, Opcode::WizZoneChange as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(3)); // type=3 server teleport
        assert_eq!(r.read_u16(), Some(ZONE_KARUS)); // target zone
        assert_eq!(r.read_u16(), Some(0)); // padding
        assert_eq!(r.read_u16(), Some(0)); // x default
        assert_eq!(r.read_u16(), Some(0)); // z default
        assert_eq!(r.read_u16(), Some(0)); // y
        assert_eq!(r.read_u8(), Some(NATION_KARUS)); // nation
        assert_eq!(r.read_u16(), Some(0xFFFF)); // status
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_build_wanted_warp_packet_elmorad() {
        let pkt = build_wanted_warp_packet(ZONE_ELMORAD, NATION_ELMORAD);
        assert_eq!(pkt.opcode, Opcode::WizZoneChange as u8);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(3));
        assert_eq!(r.read_u16(), Some(ZONE_ELMORAD));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_u16(), Some(0));
        assert_eq!(r.read_u8(), Some(NATION_ELMORAD));
        assert_eq!(r.read_u16(), Some(0xFFFF));
        assert_eq!(r.remaining(), 0);
    }

    // ── handle_wanted_kill (WorldState integration) ──────────────────

    type WantedKillSetup = (
        Arc<WorldState>,
        SessionId,
        SessionId,
        mpsc::UnboundedReceiver<Arc<Packet>>,
        mpsc::UnboundedReceiver<Arc<Packet>>,
    );

    /// Helper to create a WorldState with two registered players (victim + killer).
    fn setup_wanted_kill_world() -> WantedKillSetup {
        let world = Arc::new(WorldState::new());
        let victim_sid: SessionId = 1;
        let killer_sid: SessionId = 2;
        let (victim_tx, victim_rx) = mpsc::unbounded_channel();
        let (killer_tx, killer_rx) = mpsc::unbounded_channel();
        world.register_session(victim_sid, victim_tx);
        world.register_session(killer_sid, killer_tx);

        // Victim is Elmorad, in Ronark Land (a wanted zone)
        let victim_info = CharacterInfo {
            session_id: victim_sid,
            name: "VictimPlayer".into(),
            nation: NATION_ELMORAD,
            race: 1,
            class: 101,
            level: 60,
            face: 1,
            hair_rgb: 0,
            rank: 0,
            title: 0,
            max_hp: 1000,
            hp: 0, // dead
            max_mp: 500,
            mp: 500,
            max_sp: 0,
            sp: 0,
            equipped_items: [0; 14],
            bind_zone: 21,
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
            loyalty: 100,
            loyalty_monthly: 50,
            authority: 1,
            knights_id: 0,
            fame: 0,
            party_id: None,
            exp: 0,
            max_exp: 100_000_000,
            exp_seal_status: false,
            sealed_exp: 0,
            item_weight: 0,
            max_weight: 5000,
            res_hp_type: 0x01,
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
        };
        let victim_pos = Position {
            zone_id: ZONE_RONARK_LAND,
            x: 100.0,
            y: 0.0,
            z: 100.0,
            region_x: 0,
            region_z: 0,
        };
        world.register_ingame(victim_sid, victim_info, victim_pos);

        // Set victim as wanted
        world.update_session(victim_sid, |h| {
            h.is_wanted = true;
            h.wanted_expiry_time = 999999999;
        });

        // Add victim to wanted room list (Ronark Land = room 0, Elmorad)
        {
            let mut rooms = world.wanted_rooms().write();
            rooms[0].status = WantedEventStatus::Running;
            rooms[0].elmo_list.push(victim_sid);
        }

        // Killer is Karus, in same zone
        let killer_info = CharacterInfo {
            session_id: killer_sid,
            name: "KillerPlayer".into(),
            nation: NATION_KARUS,
            race: 1,
            class: 101,
            level: 60,
            face: 1,
            hair_rgb: 0,
            rank: 0,
            title: 0,
            max_hp: 1000,
            hp: 500,
            max_mp: 500,
            mp: 500,
            max_sp: 0,
            sp: 0,
            equipped_items: [0; 14],
            bind_zone: 21,
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
            loyalty: 200,
            loyalty_monthly: 100,
            authority: 1,
            knights_id: 0,
            fame: 0,
            party_id: None,
            exp: 0,
            max_exp: 100_000_000,
            exp_seal_status: false,
            sealed_exp: 0,
            item_weight: 0,
            max_weight: 5000,
            res_hp_type: 0x01,
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
        };
        let killer_pos = Position {
            zone_id: ZONE_RONARK_LAND,
            x: 110.0,
            y: 0.0,
            z: 110.0,
            region_x: 0,
            region_z: 0,
        };
        world.register_ingame(killer_sid, killer_info, killer_pos);

        (world, victim_sid, killer_sid, victim_rx, killer_rx)
    }

    #[tokio::test]
    async fn test_handle_wanted_kill_clears_victim_status() {
        let (world, victim_sid, killer_sid, _vrx, _krx) = setup_wanted_kill_world();

        // Verify victim is wanted before kill
        let is_wanted = world.with_session(victim_sid, |h| h.is_wanted).unwrap();
        assert!(is_wanted, "victim should be wanted before kill");

        handle_wanted_kill(&world, victim_sid, killer_sid);

        // Verify victim's wanted status is cleared
        let (wanted, expiry) = world
            .with_session(victim_sid, |h| (h.is_wanted, h.wanted_expiry_time))
            .unwrap();
        assert!(!wanted, "victim wanted flag should be cleared");
        assert_eq!(expiry, 0, "victim wanted_expiry_time should be 0");
    }

    #[tokio::test]
    async fn test_handle_wanted_kill_removes_from_room_list() {
        let (world, victim_sid, killer_sid, _vrx, _krx) = setup_wanted_kill_world();

        // Verify victim is in elmo_list before kill
        let in_list = world
            .wanted_rooms()
            .read()
            .first()
            .map(|r| r.elmo_list.contains(&victim_sid))
            .unwrap_or(false);
        assert!(in_list, "victim should be in elmo_list before kill");

        handle_wanted_kill(&world, victim_sid, killer_sid);

        // Verify victim is removed from elmo_list
        let still_in = world
            .wanted_rooms()
            .read()
            .first()
            .map(|r| r.elmo_list.contains(&victim_sid))
            .unwrap_or(false);
        assert!(!still_in, "victim should be removed from elmo_list");
    }

    #[tokio::test]
    async fn test_handle_wanted_kill_gives_killer_loyalty() {
        let (world, victim_sid, killer_sid, _vrx, mut krx) = setup_wanted_kill_world();

        handle_wanted_kill(&world, victim_sid, killer_sid);

        // Verify killer's loyalty increased by WANTED_KILL_LOYALTY (80)
        let ch = world.get_character_info(killer_sid).unwrap();
        assert_eq!(ch.loyalty, 280, "loyalty should be 200 + 80");
        assert_eq!(
            ch.loyalty_monthly, 180,
            "loyalty_monthly should be 100 + 80"
        );

        // Verify loyalty change packet was sent to killer
        // Drain packets until we find WizLoyaltyChange
        let mut found_loyalty = false;
        while let Ok(pkt) = krx.try_recv() {
            if pkt.opcode == Opcode::WizLoyaltyChange as u8 {
                let mut r = PacketReader::new(&pkt.data);
                assert_eq!(r.read_u8(), Some(1)); // sub-opcode 1 = give
                assert_eq!(r.read_u32(), Some(80)); // amount
                assert_eq!(r.read_u32(), Some(280)); // new total
                found_loyalty = true;
                break;
            }
        }
        assert!(
            found_loyalty,
            "should have received WizLoyaltyChange packet"
        );
    }

    #[test]
    fn test_handle_wanted_logout_clears_status() {
        let (world, victim_sid, _killer_sid, _vrx, _krx) = setup_wanted_kill_world();

        handle_wanted_logout(&world, victim_sid);

        // Verify victim's wanted status is cleared
        let (wanted, expiry) = world
            .with_session(victim_sid, |h| (h.is_wanted, h.wanted_expiry_time))
            .unwrap();
        assert!(!wanted, "victim wanted flag should be cleared on logout");
        assert_eq!(expiry, 0, "victim wanted_expiry_time should be 0 on logout");
    }

    #[test]
    fn test_handle_wanted_logout_removes_from_room() {
        let (world, victim_sid, _killer_sid, _vrx, _krx) = setup_wanted_kill_world();

        handle_wanted_logout(&world, victim_sid);

        let still_in = world
            .wanted_rooms()
            .read()
            .first()
            .map(|r| r.elmo_list.contains(&victim_sid))
            .unwrap_or(false);
        assert!(!still_in, "victim should be removed on logout");
    }

    #[test]
    fn test_handle_wanted_logout_no_killer_rewards() {
        let (world, victim_sid, killer_sid, _vrx, mut krx) = setup_wanted_kill_world();

        // Killer should not receive any packets on victim logout
        handle_wanted_logout(&world, victim_sid);

        // Verify killer loyalty unchanged
        let ch = world.get_character_info(killer_sid).unwrap();
        assert_eq!(
            ch.loyalty, 200,
            "killer loyalty should not change on logout"
        );

        // Verify no packets to killer
        assert!(
            krx.try_recv().is_err(),
            "killer should not receive any packets on victim logout"
        );
    }

    #[tokio::test]
    async fn test_handle_wanted_kill_karus_victim_removes_from_karus_list() {
        let world = Arc::new(WorldState::new());
        let victim_sid: SessionId = 10;
        let killer_sid: SessionId = 20;
        let (vtx, _vrx) = mpsc::unbounded_channel();
        let (ktx, _krx) = mpsc::unbounded_channel();
        world.register_session(victim_sid, vtx);
        world.register_session(killer_sid, ktx);

        let victim_info = CharacterInfo {
            session_id: victim_sid,
            name: "KarusVictim".into(),
            nation: NATION_KARUS,
            race: 1,
            class: 101,
            level: 60,
            face: 1,
            hair_rgb: 0,
            rank: 0,
            title: 0,
            max_hp: 1000,
            hp: 0,
            max_mp: 500,
            mp: 500,
            max_sp: 0,
            sp: 0,
            equipped_items: [0; 14],
            bind_zone: 21,
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
            max_exp: 100_000_000,
            exp_seal_status: false,
            sealed_exp: 0,
            item_weight: 0,
            max_weight: 5000,
            res_hp_type: 0x01,
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
        };
        let victim_pos = Position {
            zone_id: ZONE_ARDREAM,
            x: 50.0,
            y: 0.0,
            z: 50.0,
            region_x: 0,
            region_z: 0,
        };
        world.register_ingame(victim_sid, victim_info, victim_pos);
        world.update_session(victim_sid, |h| {
            h.is_wanted = true;
            h.wanted_expiry_time = 999999;
        });

        // Add to karus_list in room 1 (Ardream)
        {
            let mut rooms = world.wanted_rooms().write();
            rooms[1].status = WantedEventStatus::Running;
            rooms[1].karus_list.push(victim_sid);
        }

        let killer_info = CharacterInfo {
            session_id: killer_sid,
            name: "ElmoKiller".into(),
            nation: NATION_ELMORAD,
            race: 1,
            class: 101,
            level: 60,
            face: 1,
            hair_rgb: 0,
            rank: 0,
            title: 0,
            max_hp: 1000,
            hp: 500,
            max_mp: 500,
            mp: 500,
            max_sp: 0,
            sp: 0,
            equipped_items: [0; 14],
            bind_zone: 21,
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
            max_exp: 100_000_000,
            exp_seal_status: false,
            sealed_exp: 0,
            item_weight: 0,
            max_weight: 5000,
            res_hp_type: 0x01,
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
        };
        let killer_pos = Position {
            zone_id: ZONE_ARDREAM,
            x: 55.0,
            y: 0.0,
            z: 55.0,
            region_x: 0,
            region_z: 0,
        };
        world.register_ingame(killer_sid, killer_info, killer_pos);

        handle_wanted_kill(&world, victim_sid, killer_sid);

        // Verify removed from karus_list
        let in_karus = world
            .wanted_rooms()
            .read()
            .get(1)
            .map(|r| r.karus_list.contains(&victim_sid))
            .unwrap_or(false);
        assert!(!in_karus, "karus victim should be removed from karus_list");
    }

    // ── Lifecycle tick tests ──────────────────────────────────────────

    #[test]
    fn test_lifecycle_skips_unloaded_zones() {
        let world = WorldState::new();
        initialize_wanted_rooms(&world);

        let initial_next = world.wanted_rooms().read()[0].next_select_time;

        // Advance past initial next_select_time — but zone data is not loaded
        // so the tick skips all rooms (get_zone returns None).
        let now = initial_next + 10;
        tick_wanted_event_lifecycle(&world, now);

        let rooms = world.wanted_rooms().read();
        // Room should remain unchanged since zone data is absent
        assert_eq!(rooms[0].status, WantedEventStatus::Disabled);
        assert_eq!(rooms[0].next_select_time, initial_next);
    }

    #[test]
    fn test_lifecycle_not_enabled_skips() {
        let world = WorldState::new();
        // Don't call initialize_wanted_rooms — auto_wanted stays false
        let now = now_secs() + 1000;
        tick_wanted_event_lifecycle(&world, now);
        // Should be a no-op
        let rooms = world.wanted_rooms().read();
        assert_eq!(rooms[0].status, WantedEventStatus::Disabled);
    }

    #[test]
    fn test_initialize_wanted_rooms_sets_state() {
        let world = WorldState::new();
        initialize_wanted_rooms(&world);

        let rooms = world.wanted_rooms().read();
        for room in rooms.iter() {
            assert_eq!(room.status, WantedEventStatus::Disabled);
            assert!(room.next_select_time > 0);
        }
        assert!(world
            .wanted_auto_enabled
            .load(std::sync::atomic::Ordering::Relaxed));
    }

    #[test]
    fn test_reset_data_sets_next_select() {
        let world = WorldState::new();
        initialize_wanted_rooms(&world);

        let now = 5000u64;
        wanted_event_reset_data(&world, 0, now);

        let rooms = world.wanted_rooms().read();
        assert_eq!(rooms[0].status, WantedEventStatus::Disabled);
        assert!(rooms[0].next_select_time >= now + INITIAL_NEXT_SELECT_SECS);
    }

    #[test]
    fn test_build_wanted_invitation_packet() {
        let pkt = build_wanted_invitation_packet();
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8(), Some(sub_opcode::REGISTER));
        assert_eq!(r.read_u8(), Some(1)); // sub-sub
        assert_eq!(r.read_u8(), Some(1)); // success flag
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn test_lifecycle_constants() {
        assert_eq!(MAX_SELECTING_USER, 5);
        assert_eq!(INVITATION_DURATION_SECS, 11);
        assert_eq!(LIST_PHASE_SECS, 10);
        assert_eq!(RUN_PHASE_SECS, 600);
        assert_eq!(INITIAL_NEXT_SELECT_SECS, 60);
    }

    /// Wanted kill reward: item 914052000 + loyalty (solo=80, party=160).
    #[test]
    fn test_wanted_kill_rewards() {
        assert_eq!(WANTED_KILL_ITEM, 914_052_000);
        assert_eq!(WANTED_KILL_LOYALTY_SOLO, 80);
        assert_eq!(WANTED_KILL_LOYALTY_PARTY, 160);
        // Party loyalty = 2× solo
        assert_eq!(WANTED_KILL_LOYALTY_PARTY, WANTED_KILL_LOYALTY_SOLO * 2);
        // Item ID in 914M range (event reward)
        assert!(WANTED_KILL_ITEM >= 900_000_000);
    }

    /// Wanted event has exactly 4 phases: Disabled → Invitation → ListSending → Running.
    #[test]
    fn test_wanted_event_status_phases() {
        let default_status = WantedEventStatus::default();
        assert!(matches!(default_status, WantedEventStatus::Disabled));
        // All 4 variants exist
        let _d = WantedEventStatus::Disabled;
        let _i = WantedEventStatus::Invitation;
        let _l = WantedEventStatus::ListSending;
        let _r = WantedEventStatus::Running;
    }

    /// Move sub-opcodes: USER_LIST (1) and POSITION (2) are sequential.
    #[test]
    fn test_move_sub_opcodes_sequential() {
        assert_eq!(move_sub::USER_LIST, 1);
        assert_eq!(move_sub::POSITION, 2);
        assert_eq!(move_sub::POSITION - move_sub::USER_LIST, 1);
    }

    /// RUN_PHASE_SECS (600) = 10 minutes.
    #[test]
    fn test_run_phase_is_10_minutes() {
        assert_eq!(RUN_PHASE_SECS, 600);
        assert_eq!(RUN_PHASE_SECS / 60, 10);
        // Warp delay (15s) is much shorter than run phase
        assert_eq!(WANTED_KILL_WARP_DELAY_SECS, 15);
        assert!(WANTED_KILL_WARP_DELAY_SECS < RUN_PHASE_SECS);
    }

    /// Wanted zones: Ronark Land, Ardream, Ronark Land Base are PK zones.
    #[test]
    fn test_wanted_pk_zones_distinct() {
        let zones = [ZONE_RONARK_LAND, ZONE_ARDREAM, ZONE_RONARK_LAND_BASE];
        assert_eq!(zones.len(), 3);
        // All distinct
        assert_ne!(zones[0], zones[1]);
        assert_ne!(zones[1], zones[2]);
        assert_ne!(zones[0], zones[2]);
    }
}
