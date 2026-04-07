//! Builds a complete quest flow map for each NPC by cross-referencing TBL and Lua data.

use std::collections::HashMap;

use crate::lua_parser::LuaFileData;
use crate::tbl_loader::{TblData, TblQuestHelper};

/// A complete dialog step in a quest flow.
#[derive(Debug, Clone)]
pub struct DialogStep {
    /// The EVENT ID that triggers this step.
    pub event_id: i32,
    /// Dialog text (from quest_talk TBL).
    pub dialog_text: Option<String>,
    /// Dialog text ID.
    pub dialog_text_id: i32,
    /// Buttons shown to player.
    pub buttons: Vec<DialogButton>,
    /// Item exchange triggered (if any).
    pub exchange_id: Option<i32>,
    /// Exchange details from TBL.
    pub exchange_detail: Option<ExchangeDetail>,
    /// SaveEvent calls (quest progression).
    pub save_events: Vec<i32>,
    /// Items given to player.
    pub give_items: Vec<(i32, i32)>,
    /// Items taken from player.
    pub rob_items: Vec<(i32, i32)>,
    /// Whether this handler exists in Lua.
    pub has_lua_handler: bool,
}

/// A button in a dialog.
#[derive(Debug, Clone)]
pub struct DialogButton {
    /// Button text ID (from quest_menu TBL).
    pub text_id: i32,
    /// Button text (resolved from TBL).
    pub text: Option<String>,
    /// Next EVENT to trigger on click.
    pub next_event: i32,
}

/// Item exchange details.
#[derive(Debug, Clone)]
pub struct ExchangeDetail {
    pub exchange_id: i32,
    pub random_flag: i32,
    /// (item_id, count) required.
    pub required_items: Vec<(i32, i32)>,
    /// (item_id, count, time) given.
    pub reward_items: Vec<(i32, i32, i32)>,
}

/// A complete quest chain for one NPC.
#[derive(Debug)]
pub struct NpcQuestMap {
    pub npc_id: i32,
    pub npc_name: String,
    /// Lua filename from quest_helper.
    pub lua_filename: String,
    /// quest_helper entries for this NPC.
    pub helper_entries: Vec<TblQuestHelper>,
    /// All quest IDs (event_data_index) for this NPC.
    pub quest_ids: Vec<i32>,
    /// event_id → DialogStep (the complete flow).
    pub steps: HashMap<i32, DialogStep>,
    /// Monster kill requirements (quest_id → groups).
    pub monster_requirements: HashMap<i32, Vec<(Vec<i32>, i32)>>,
    /// Whether this NPC exists in client quest_npc_desc.
    pub in_client_npc_desc: bool,
    /// Client portrait text_id (from quest_npc_desc col_8).
    pub client_portrait_text_id: Option<i32>,
}

/// Build quest flow maps for all NPCs.
pub fn build_all_quest_maps(
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
) -> Vec<NpcQuestMap> {
    let npc_ids = tbl.all_quest_npc_ids();
    let mut maps = Vec::with_capacity(npc_ids.len());

    for npc_id in &npc_ids {
        if let Some(map) = build_npc_quest_map(*npc_id, tbl, lua_files) {
            maps.push(map);
        }
    }

    tracing::info!(npc_count = maps.len(), "Quest maps built");
    maps
}

/// Build quest flow map for a single NPC.
pub fn build_npc_quest_map(
    npc_id: i32,
    tbl: &TblData,
    lua_files: &HashMap<String, LuaFileData>,
) -> Option<NpcQuestMap> {
    let helper_indices = tbl.helpers_by_npc.get(&npc_id)?;
    let helper_entries: Vec<TblQuestHelper> = helper_indices
        .iter()
        .map(|&i| tbl.quest_helpers[i].clone())
        .collect();

    if helper_entries.is_empty() {
        return None;
    }

    // Get Lua filename from first helper entry
    let lua_filename = helper_entries[0].lua_filename.clone();
    let lua_key = lua_filename.to_lowercase();
    let lua_data = lua_files.get(&lua_key);

    // Get NPC name from Lua data or filename
    let npc_name = lua_data.map(|d| d.npc_name.clone()).unwrap_or_default();

    // Collect all unique quest IDs
    let mut quest_ids: Vec<i32> = helper_entries.iter().map(|h| h.event_data_index).collect();
    quest_ids.sort();
    quest_ids.dedup();

    // Build dialog steps from both TBL and Lua
    let mut steps = HashMap::new();

    // From quest_helper entries: extract event_trigger, event_complete, exchange_index
    for helper in &helper_entries {
        // Entry event (trigger)
        if helper.event_trigger > 0 {
            let step = build_step(helper.event_trigger, helper, tbl, lua_data);
            steps.insert(helper.event_trigger, step);
        }
        // Complete event
        if helper.event_complete > 0 && helper.event_complete != helper.event_trigger {
            let step = build_step(helper.event_complete, helper, tbl, lua_data);
            steps.insert(helper.event_complete, step);
        }
        // Talk event
        if helper.event_talk > 0
            && helper.event_talk != helper.event_trigger
            && helper.event_talk != helper.event_complete
        {
            let step = build_step(helper.event_talk, helper, tbl, lua_data);
            steps.insert(helper.event_talk, step);
        }
    }

    // Also add all Lua EVENT handlers not yet in steps
    if let Some(lua) = lua_data {
        for (&event_id, handler) in &lua.event_handlers {
            steps.entry(event_id).or_insert_with(|| {
                let mut step = DialogStep {
                    event_id,
                    dialog_text: None,
                    dialog_text_id: 0,
                    buttons: Vec::new(),
                    exchange_id: None,
                    exchange_detail: None,
                    save_events: Vec::new(),
                    give_items: Vec::new(),
                    rob_items: Vec::new(),
                    has_lua_handler: true,
                };
                populate_from_lua(&mut step, handler, tbl);
                step
            });
        }
    }

    // Monster requirements
    let mut monster_requirements = HashMap::new();
    for qid in &quest_ids {
        if let Some(mex) = tbl.monster_exchanges.get(qid) {
            let groups: Vec<(Vec<i32>, i32)> = mex
                .groups
                .iter()
                .map(|g| (g.monster_ids.clone(), g.count))
                .collect();
            monster_requirements.insert(*qid, groups);
        }
    }

    // Client NPC desc
    let npc_desc = tbl.npc_descs.get(&npc_id);
    let in_client = npc_desc.is_some();
    let portrait_id = npc_desc.map(|d| d.col_8);

    Some(NpcQuestMap {
        npc_id,
        npc_name,
        lua_filename,
        helper_entries,
        quest_ids,
        steps,
        monster_requirements,
        in_client_npc_desc: in_client,
        client_portrait_text_id: portrait_id,
    })
}

fn build_step(
    event_id: i32,
    helper: &TblQuestHelper,
    tbl: &TblData,
    lua_data: Option<&LuaFileData>,
) -> DialogStep {
    let mut step = DialogStep {
        event_id,
        dialog_text: None,
        dialog_text_id: 0,
        buttons: Vec::new(),
        exchange_id: None,
        exchange_detail: None,
        save_events: Vec::new(),
        give_items: Vec::new(),
        rob_items: Vec::new(),
        has_lua_handler: false,
    };

    // Get dialog text from quest_talk
    if helper.event_talk > 0 {
        if let Some(talk) = tbl.quest_talks.get(&helper.event_talk) {
            step.dialog_text = Some(talk.text.clone());
            step.dialog_text_id = talk.text_id;
        }
    }

    // Get exchange details
    if helper.exchange_index > 0 {
        step.exchange_id = Some(helper.exchange_index);
        if let Some(ex) = tbl.item_exchanges.get(&helper.exchange_index) {
            step.exchange_detail = Some(ExchangeDetail {
                exchange_id: ex.index,
                random_flag: ex.random_flag,
                required_items: ex.origin_items.clone(),
                reward_items: ex.exchange_items.clone(),
            });
        }
    }

    // Populate from Lua handler if available
    if let Some(lua) = lua_data {
        if let Some(handler) = lua.event_handlers.get(&event_id) {
            step.has_lua_handler = true;
            populate_from_lua(&mut step, handler, tbl);
        }
    }

    step
}

fn populate_from_lua(
    step: &mut DialogStep,
    handler: &crate::lua_parser::EventHandler,
    tbl: &TblData,
) {
    step.has_lua_handler = true;

    // Extract buttons from SelectMsg calls
    for sm in &handler.select_msgs {
        if step.dialog_text_id == 0 && sm.header_text_id > 0 {
            step.dialog_text_id = sm.header_text_id;
            if let Some(talk) = tbl.quest_talks.get(&sm.header_text_id) {
                step.dialog_text = Some(talk.text.clone());
            }
        }
        for (btn_text_id, next_event) in &sm.buttons {
            let text = tbl.quest_menus.get(btn_text_id).map(|m| m.text.clone());
            step.buttons.push(DialogButton {
                text_id: *btn_text_id,
                text,
                next_event: *next_event,
            });
        }
    }

    // Exchanges
    for ex in &handler.exchanges {
        if step.exchange_id.is_none() {
            step.exchange_id = Some(ex.exchange_id);
            if let Some(exd) = tbl.item_exchanges.get(&ex.exchange_id) {
                step.exchange_detail = Some(ExchangeDetail {
                    exchange_id: exd.index,
                    random_flag: exd.random_flag,
                    required_items: exd.origin_items.clone(),
                    reward_items: exd.exchange_items.clone(),
                });
            }
        }
    }

    // Save events
    for se in &handler.save_events {
        step.save_events.push(se.event_id);
    }

    // Give items
    for gi in &handler.give_items {
        step.give_items.push((gi.item_id, gi.count));
    }

    // Rob items
    for ri in &handler.rob_items {
        step.rob_items.push((ri.item_id, ri.count));
    }
}
