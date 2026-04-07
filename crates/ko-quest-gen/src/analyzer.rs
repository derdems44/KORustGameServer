//! Analyzes quest chains and identifies missing EVENT handlers.

use std::collections::{BTreeMap, HashMap, HashSet};

use ko_quest_audit::lua_parser::LuaFileData;
use ko_quest_audit::tbl_loader::TblData;

use crate::mssql_parser::MssqlQuestHelper;

/// A quest chain for one NPC + one quest_id, grouping all status rows.
#[derive(Debug, Clone)]
pub struct QuestChain {
    pub npc_id: i32,
    pub quest_id: i32,
    pub lua_filename: String,
    /// Rows ordered by (event_status, n_index).
    pub rows: Vec<MssqlQuestHelper>,
}

/// A single missing EVENT handler that needs to be generated.
#[derive(Debug, Clone)]
pub struct MissingHandler {
    pub npc_id: i32,
    pub lua_filename: String,
    pub event_id: i32,
    pub handler_type: HandlerType,
    pub quest_chain: QuestChain,
    /// The specific quest_helper row that defines this handler.
    pub source_row: MssqlQuestHelper,
}

/// The type of handler to generate.
#[derive(Debug, Clone)]
pub enum HandlerType {
    /// status=255: Quest intro — auto-accept, SaveEvent to status=0 row.
    IntroAccept { save_event_index: i32 },
    /// status=0: Quest offer dialog — SelectMsg with accept/decline.
    QuestOffer {
        quest_id: i32,
        event_talk: i32,
        quest_menu: i32,
        /// EVENT for accept button.
        accept_event: i32,
    },
    /// Accept button handler for status=0 — SaveEvent to status=1.
    QuestAccept { save_event_index: i32 },
    /// status=1: Check progress (monster or item) — show results dialog.
    ProgressCheck {
        quest_id: i32,
        event_talk: i32,
        exchange_index: i32,
        has_monster_quest: bool,
        /// EVENT when player has completed requirements.
        complete_event: i32,
        /// EVENT when player hasn't completed yet.
        incomplete_event: i32,
    },
    /// Complete handler — RunQuestExchange + SaveEvent.
    QuestComplete {
        quest_id: i32,
        exchange_index: i32,
        save_event_index: i32,
    },
    /// status=2: Completion confirmed — already done, show message.
    CompletionConfirm {
        quest_id: i32,
        exchange_index: i32,
        save_event_index: i32,
    },
    /// status=3: Sub-step — usually same as progress redirect.
    SubStep { quest_id: i32, event_talk: i32 },
    /// status=4: Quest completed — redirect or done message.
    QuestDone { quest_id: i32, event_talk: i32 },
    /// Show map for incomplete objective.
    ShowMap { zone: i32 },
    /// SearchQuest dispatch handler (EVENT 100/190).
    SearchQuestDispatch { npc_id: i32 },
}

/// Analyze MSSQL quest_helper rows + existing Lua files to find missing handlers.
pub fn find_missing_handlers(
    mssql_rows: &[MssqlQuestHelper],
    lua_files: &HashMap<String, LuaFileData>,
    tbl: &TblData,
) -> Vec<MissingHandler> {
    // 1. Group MSSQL rows by (npc_id, quest_id)
    let chains = build_quest_chains(mssql_rows);

    // 2. For each chain, determine expected EVENTs and check which are missing
    let mut missing = Vec::new();

    for chain in &chains {
        let lua_key = chain.lua_filename.to_lowercase();

        // Skip if no Lua file exists at all (that's a MISSING_LUA finding, not our job)
        if !lua_files.contains_key(&lua_key) {
            continue;
        }

        let existing_events: HashSet<i32> = lua_files
            .get(&lua_key)
            .map(|f| f.event_handlers.keys().copied().collect())
            .unwrap_or_default();

        let chain_missing = analyze_chain(chain, &existing_events, tbl);
        missing.extend(chain_missing);
    }

    // 3. Find missing SearchQuest dispatch handlers (EVENT 100/190)
    let dispatch_missing = find_missing_dispatch(mssql_rows, lua_files);
    missing.extend(dispatch_missing);

    tracing::info!(
        chains = chains.len(),
        missing = missing.len(),
        "Quest chain analysis complete"
    );
    missing
}

/// Group MSSQL rows into quest chains by (npc_id, quest_id).
fn build_quest_chains(rows: &[MssqlQuestHelper]) -> Vec<QuestChain> {
    let mut groups: BTreeMap<(i32, i32), Vec<MssqlQuestHelper>> = BTreeMap::new();
    for row in rows {
        // Skip dispatch rows (quest_id=0, trigger=100/190 → SearchQuest)
        if row.s_event_data_index == 0 {
            continue;
        }
        groups
            .entry((row.s_npc_id, row.s_event_data_index))
            .or_default()
            .push(row.clone());
    }

    groups
        .into_iter()
        .map(|((npc_id, quest_id), mut rows)| {
            rows.sort_by_key(|r| (r.b_event_status, r.n_index));
            let lua_filename = rows
                .first()
                .map(|r| r.str_lua_filename.clone())
                .unwrap_or_default();
            QuestChain {
                npc_id,
                quest_id,
                lua_filename,
                rows,
            }
        })
        .collect()
}

/// Analyze a single quest chain to find missing handlers.
fn analyze_chain(
    chain: &QuestChain,
    existing_events: &HashSet<i32>,
    tbl: &TblData,
) -> Vec<MissingHandler> {
    let mut missing = Vec::new();

    // Index chain rows by status for lookups
    let mut by_status: BTreeMap<i32, Vec<&MssqlQuestHelper>> = BTreeMap::new();
    for row in &chain.rows {
        by_status.entry(row.b_event_status).or_default().push(row);
    }

    // Find the n_index for a given status (first row of that status)
    let index_for_status = |status: i32| -> Option<i32> {
        by_status
            .get(&status)
            .and_then(|rows| rows.first())
            .map(|r| r.n_index)
    };

    // Check if quest has monster exchange
    let has_monster = tbl.monster_exchanges.contains_key(&chain.quest_id);

    // Collect all trigger EVENTs that exist in the chain (to avoid generating conflicts)
    let chain_triggers: HashSet<i32> = chain.rows.iter().map(|r| r.n_event_trigger_index).collect();

    for row in &chain.rows {
        let trigger = row.n_event_trigger_index;
        if trigger == 0 || trigger == 100 || trigger == 190 {
            // Even for dispatch triggers, check if n_event_complete_index needs a handler
            if row.b_event_status == 1
                && row.n_event_complete_index != 0
                && !existing_events.contains(&row.n_event_complete_index)
                && !chain_triggers.contains(&row.n_event_complete_index)
            {
                let save_index = index_for_status(2)
                    .unwrap_or_else(|| index_for_status(4).unwrap_or(row.n_index));
                missing.push(MissingHandler {
                    npc_id: chain.npc_id,
                    lua_filename: chain.lua_filename.clone(),
                    event_id: row.n_event_complete_index,
                    handler_type: HandlerType::QuestComplete {
                        quest_id: chain.quest_id,
                        exchange_index: row.n_exchange_index,
                        save_event_index: save_index,
                    },
                    quest_chain: chain.clone(),
                    source_row: row.clone(),
                });
            }
            continue; // dispatch events, handled by SearchQuest
        }

        // If trigger already exists in Lua, still check sub-handlers
        if existing_events.contains(&trigger) {
            // Trigger handler exists but n_event_complete_index handler might be missing
            if row.b_event_status == 1
                && row.n_event_complete_index != 0
                && !existing_events.contains(&row.n_event_complete_index)
                && !chain_triggers.contains(&row.n_event_complete_index)
            {
                let save_index = index_for_status(2)
                    .unwrap_or_else(|| index_for_status(4).unwrap_or(row.n_index));
                missing.push(MissingHandler {
                    npc_id: chain.npc_id,
                    lua_filename: chain.lua_filename.clone(),
                    event_id: row.n_event_complete_index,
                    handler_type: HandlerType::QuestComplete {
                        quest_id: chain.quest_id,
                        exchange_index: row.n_exchange_index,
                        save_event_index: save_index,
                    },
                    quest_chain: chain.clone(),
                    source_row: row.clone(),
                });
            }
            continue;
        }

        let handler_type = match row.b_event_status {
            255 => {
                // Not started → auto-accept, transition to next status.
                // Try status=0 first, then 1, then 2, then 4.
                let save_index = index_for_status(0)
                    .or_else(|| index_for_status(1))
                    .or_else(|| index_for_status(2))
                    .or_else(|| index_for_status(4));
                if let Some(save_event_index) = save_index {
                    HandlerType::IntroAccept { save_event_index }
                } else {
                    continue;
                }
            }
            0 => {
                // Quest available → show dialog with accept/decline
                let accept_event = trigger + 1;
                let accept_save_index = index_for_status(1).unwrap_or_else(|| {
                    // No status=1 row → try status=4 (simple accept-only quest)
                    index_for_status(4).unwrap_or(row.n_index)
                });

                // Generate the QuestOffer handler
                let offer = HandlerType::QuestOffer {
                    quest_id: chain.quest_id,
                    event_talk: row.n_event_talk_index,
                    quest_menu: row.s_quest_menu,
                    accept_event,
                };

                // Also generate the accept handler if it doesn't exist
                if !existing_events.contains(&accept_event)
                    && !chain_triggers.contains(&accept_event)
                {
                    missing.push(MissingHandler {
                        npc_id: chain.npc_id,
                        lua_filename: chain.lua_filename.clone(),
                        event_id: accept_event,
                        handler_type: HandlerType::QuestAccept {
                            save_event_index: accept_save_index,
                        },
                        quest_chain: chain.clone(),
                        source_row: row.clone(),
                    });
                }

                offer
            }
            1 => {
                // In progress → check requirements
                // n_event_complete_index = the "quest complete" callback EVENT
                let complete_event = if row.n_event_complete_index != 0 {
                    row.n_event_complete_index
                } else {
                    trigger + 1
                };
                // incomplete = close dialog or show map
                let incomplete_event = if row.n_event_complete_index != 0 {
                    // DB specifies complete event separately, so trigger+1 is available
                    // for incomplete (ShowMap). Use -1 if no zone to show.
                    if row.b_zone > 0 {
                        trigger + 1
                    } else {
                        -1
                    }
                } else {
                    // No DB complete index → trigger+1 is complete, trigger+2 is incomplete
                    trigger + 2
                };

                // Also generate the complete handler (RunQuestExchange + SaveEvent)
                if !existing_events.contains(&complete_event)
                    && !chain_triggers.contains(&complete_event)
                {
                    let save_index = index_for_status(2)
                        .unwrap_or_else(|| index_for_status(4).unwrap_or(row.n_index));
                    missing.push(MissingHandler {
                        npc_id: chain.npc_id,
                        lua_filename: chain.lua_filename.clone(),
                        event_id: complete_event,
                        handler_type: HandlerType::QuestComplete {
                            quest_id: chain.quest_id,
                            exchange_index: row.n_exchange_index,
                            save_event_index: save_index,
                        },
                        quest_chain: chain.clone(),
                        source_row: row.clone(),
                    });
                }

                // Also generate ShowMap for incomplete_event if needed
                if incomplete_event > 0
                    && !existing_events.contains(&incomplete_event)
                    && !chain_triggers.contains(&incomplete_event)
                {
                    missing.push(MissingHandler {
                        npc_id: chain.npc_id,
                        lua_filename: chain.lua_filename.clone(),
                        event_id: incomplete_event,
                        handler_type: HandlerType::ShowMap { zone: row.b_zone },
                        quest_chain: chain.clone(),
                        source_row: row.clone(),
                    });
                }

                HandlerType::ProgressCheck {
                    quest_id: chain.quest_id,
                    event_talk: row.n_event_talk_index,
                    exchange_index: row.n_exchange_index,
                    has_monster_quest: has_monster,
                    complete_event,
                    incomplete_event,
                }
            }
            2 => {
                // Ready to complete → show "already done" or run exchange
                HandlerType::CompletionConfirm {
                    quest_id: chain.quest_id,
                    exchange_index: row.n_exchange_index,
                    save_event_index: index_for_status(4).unwrap_or(row.n_index),
                }
            }
            3 => {
                // Sub-step → show "in progress" message
                HandlerType::SubStep {
                    quest_id: chain.quest_id,
                    event_talk: row.n_event_talk_index,
                }
            }
            4 => {
                // Completed → show done message or redirect
                HandlerType::QuestDone {
                    quest_id: chain.quest_id,
                    event_talk: row.n_event_talk_index,
                }
            }
            _ => continue,
        };

        missing.push(MissingHandler {
            npc_id: chain.npc_id,
            lua_filename: chain.lua_filename.clone(),
            event_id: trigger,
            handler_type,
            quest_chain: chain.clone(),
            source_row: row.clone(),
        });
    }

    missing
}

/// Find missing SearchQuest dispatch handlers (EVENT 100/190).
fn find_missing_dispatch(
    mssql_rows: &[MssqlQuestHelper],
    lua_files: &HashMap<String, LuaFileData>,
) -> Vec<MissingHandler> {
    let mut missing = Vec::new();

    // Group dispatch rows by (lua_filename, trigger) — one SearchQuest per file per dispatch event
    let mut dispatch_groups: BTreeMap<(String, i32), MssqlQuestHelper> = BTreeMap::new();
    for row in mssql_rows {
        let trigger = row.n_event_trigger_index;
        if trigger != 100 && trigger != 190 {
            continue;
        }
        let key = (row.str_lua_filename.clone(), trigger);
        dispatch_groups.entry(key).or_insert_with(|| row.clone());
    }

    for ((lua_filename, trigger), row) in &dispatch_groups {
        let lua_key = lua_filename.to_lowercase();

        // Skip if no Lua file exists
        if !lua_files.contains_key(&lua_key) {
            continue;
        }

        let existing_events: HashSet<i32> = lua_files
            .get(&lua_key)
            .map(|f| f.event_handlers.keys().copied().collect())
            .unwrap_or_default();

        // Skip if the dispatch EVENT already exists in Lua
        if existing_events.contains(trigger) {
            continue;
        }

        let chain = QuestChain {
            npc_id: row.s_npc_id,
            quest_id: 0,
            lua_filename: lua_filename.clone(),
            rows: vec![row.clone()],
        };

        missing.push(MissingHandler {
            npc_id: row.s_npc_id,
            lua_filename: lua_filename.clone(),
            event_id: *trigger,
            handler_type: HandlerType::SearchQuestDispatch {
                npc_id: row.s_npc_id,
            },
            quest_chain: chain,
            source_row: row.clone(),
        });
    }

    tracing::info!(
        dispatch_missing = missing.len(),
        "Dispatch handler analysis complete"
    );
    missing
}
