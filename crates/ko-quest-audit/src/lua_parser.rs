//! Parses Lua quest scripts to extract EVENT handlers, SelectMsg calls, and quest actions.

use std::collections::HashMap;
use std::path::Path;

/// A SelectMsg call extracted from Lua.
#[derive(Debug, Clone)]
pub struct SelectMsgCall {
    pub line_num: usize,
    pub flag: i32,
    pub quest_id: i32,
    pub header_text_id: i32,
    pub npc_param: i32,
    /// (button_text_id, next_event) pairs.
    pub buttons: Vec<(i32, i32)>,
}

/// A RunQuestExchange call.
#[derive(Debug, Clone)]
pub struct ExchangeCall {
    pub line_num: usize,
    pub exchange_id: i32,
}

/// A SaveEvent call.
#[derive(Debug, Clone)]
pub struct SaveEventCall {
    pub line_num: usize,
    pub event_id: i32,
}

/// A GiveItem call.
#[derive(Debug, Clone)]
pub struct GiveItemCall {
    pub line_num: usize,
    pub item_id: i32,
    pub count: i32,
}

/// A RobItem call.
#[derive(Debug, Clone)]
pub struct RobItemCall {
    pub line_num: usize,
    pub item_id: i32,
    pub count: i32,
}

/// An NpcMsg call.
#[derive(Debug, Clone)]
pub struct NpcMsgCall {
    pub line_num: usize,
    pub text_id: i32,
}

/// An EVENT handler block in a Lua file.
#[derive(Debug, Clone)]
pub struct EventHandler {
    pub event_id: i32,
    pub line_start: usize,
    pub select_msgs: Vec<SelectMsgCall>,
    pub exchanges: Vec<ExchangeCall>,
    pub save_events: Vec<SaveEventCall>,
    pub give_items: Vec<GiveItemCall>,
    pub rob_items: Vec<RobItemCall>,
    pub npc_msgs: Vec<NpcMsgCall>,
    /// Whether CheckGiveSlot or isRoomForItem appears in this handler.
    pub has_slot_check: bool,
    /// Whether CheckWeight appears in this handler.
    pub has_weight_check: bool,
    /// Whether SearchQuest is called (dispatch handler).
    pub has_search_quest: bool,
    /// Raw lines of the handler for detailed analysis.
    pub line_count: usize,
}

/// All parsed data from a single Lua file.
#[derive(Debug, Clone)]
pub struct LuaFileData {
    pub filename: String,
    pub npc_id: i32,
    pub npc_name: String,
    pub event_handlers: HashMap<i32, EventHandler>,
    /// All event IDs referenced by buttons (target events).
    pub referenced_events: Vec<i32>,
}

/// Parse all Lua files in the quest directory.
pub fn load_all_lua(quest_dir: &Path) -> anyhow::Result<HashMap<String, LuaFileData>> {
    let mut result = HashMap::new();
    let entries = std::fs::read_dir(quest_dir)?;
    let mut count = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "lua") {
            continue;
        }
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        match parse_lua_file(&path, &filename) {
            Ok(data) => {
                result.insert(filename.to_lowercase(), data);
                count += 1;
            }
            Err(e) => {
                tracing::warn!(file = %filename, error = %e, "Failed to parse Lua file");
            }
        }
    }

    tracing::info!(files = count, "Lua files parsed");
    Ok(result)
}

/// Extract NPC ID and name from filename like "18005_Bilbor.lua".
fn parse_filename(filename: &str) -> (i32, String) {
    let stem = filename.strip_suffix(".lua").unwrap_or(filename);
    if let Some(pos) = stem.find('_') {
        let id_str = &stem[..pos];
        let name = &stem[pos + 1..];
        let id = id_str.parse::<i32>().unwrap_or(0);
        (id, name.to_string())
    } else {
        let id = stem.parse::<i32>().unwrap_or(0);
        (id, String::new())
    }
}

/// Parse a single Lua file.
fn parse_lua_file(path: &Path, filename: &str) -> anyhow::Result<LuaFileData> {
    let content = std::fs::read_to_string(path)?;
    let (npc_id, npc_name) = parse_filename(filename);

    let lines: Vec<&str> = content.lines().collect();
    let mut event_handlers: HashMap<i32, EventHandler> = HashMap::new();
    let mut referenced_events: Vec<i32> = Vec::new();

    // Pass 1: Find EVENT handler blocks
    let mut current_event: Option<i32> = None;
    let mut current_start: usize = 0;
    let mut current_select_msgs = Vec::new();
    let mut current_exchanges = Vec::new();
    let mut current_save_events = Vec::new();
    let mut current_give_items = Vec::new();
    let mut current_rob_items = Vec::new();
    let mut current_npc_msgs = Vec::new();
    let mut current_has_slot_check = false;
    let mut current_has_weight_check = false;
    let mut current_has_search_quest = false;
    let mut current_line_count: usize = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = line_idx + 1;

        // Detect EVENT handler: "if (EVENT == 190) then" or "if EVENT == 190 then"
        if let Some(event_id) = parse_event_if(trimmed) {
            // Save previous handler if any
            if let Some(prev_id) = current_event.take() {
                let handler = EventHandler {
                    event_id: prev_id,
                    line_start: current_start,
                    select_msgs: std::mem::take(&mut current_select_msgs),
                    exchanges: std::mem::take(&mut current_exchanges),
                    save_events: std::mem::take(&mut current_save_events),
                    give_items: std::mem::take(&mut current_give_items),
                    rob_items: std::mem::take(&mut current_rob_items),
                    npc_msgs: std::mem::take(&mut current_npc_msgs),
                    has_slot_check: std::mem::take(&mut current_has_slot_check),
                    has_weight_check: std::mem::take(&mut current_has_weight_check),
                    has_search_quest: std::mem::take(&mut current_has_search_quest),
                    line_count: std::mem::take(&mut current_line_count),
                };
                event_handlers.insert(prev_id, handler);
            }
            current_event = Some(event_id);
            current_start = line_num;
            continue;
        }

        // Also detect "elseif (EVENT == X) then"
        if let Some(event_id) = parse_elseif_event(trimmed) {
            if let Some(prev_id) = current_event.take() {
                let handler = EventHandler {
                    event_id: prev_id,
                    line_start: current_start,
                    select_msgs: std::mem::take(&mut current_select_msgs),
                    exchanges: std::mem::take(&mut current_exchanges),
                    save_events: std::mem::take(&mut current_save_events),
                    give_items: std::mem::take(&mut current_give_items),
                    rob_items: std::mem::take(&mut current_rob_items),
                    npc_msgs: std::mem::take(&mut current_npc_msgs),
                    has_slot_check: std::mem::take(&mut current_has_slot_check),
                    has_weight_check: std::mem::take(&mut current_has_weight_check),
                    has_search_quest: std::mem::take(&mut current_has_search_quest),
                    line_count: std::mem::take(&mut current_line_count),
                };
                event_handlers.insert(prev_id, handler);
            }
            current_event = Some(event_id);
            current_start = line_num;
            continue;
        }

        // Inside an event handler — extract calls
        if current_event.is_some() {
            current_line_count += 1;

            // Skip Lua single-line comments (-- ...)
            if trimmed.starts_with("--") {
                continue;
            }

            // Detect slot/weight/search checks
            if trimmed.contains("CheckGiveSlot(") || trimmed.contains("isRoomForItem(") {
                current_has_slot_check = true;
            }
            if trimmed.contains("CheckWeight(") {
                current_has_weight_check = true;
            }
            if trimmed.contains("SearchQuest(") {
                current_has_search_quest = true;
            }

            // SelectMsg
            if let Some(sm) = parse_select_msg(trimmed, line_num) {
                for (_, next_event) in &sm.buttons {
                    if *next_event > 0 {
                        referenced_events.push(*next_event);
                    }
                }
                current_select_msgs.push(sm);
            }

            // RunQuestExchange
            if let Some(ex) = parse_exchange(trimmed, line_num) {
                current_exchanges.push(ex);
            }

            // SaveEvent
            if let Some(se) = parse_save_event(trimmed, line_num) {
                current_save_events.push(se);
            }

            // GiveItem
            if let Some(gi) = parse_give_item(trimmed, line_num) {
                current_give_items.push(gi);
            }

            // RobItem
            if let Some(ri) = parse_rob_item(trimmed, line_num) {
                current_rob_items.push(ri);
            }

            // NpcMsg
            if let Some(nm) = parse_npc_msg(trimmed, line_num) {
                current_npc_msgs.push(nm);
            }

            // Also check for EVENT = QuestNum style jump
            if let Some(target) = parse_event_assign(trimmed) {
                referenced_events.push(target);
            }
        }
    }

    // Save last handler
    if let Some(prev_id) = current_event {
        event_handlers.insert(
            prev_id,
            EventHandler {
                event_id: prev_id,
                line_start: current_start,
                select_msgs: current_select_msgs,
                exchanges: current_exchanges,
                save_events: current_save_events,
                give_items: current_give_items,
                rob_items: current_rob_items,
                npc_msgs: current_npc_msgs,
                has_slot_check: current_has_slot_check,
                has_weight_check: current_has_weight_check,
                has_search_quest: current_has_search_quest,
                line_count: current_line_count,
            },
        );
    }

    Ok(LuaFileData {
        filename: filename.to_string(),
        npc_id,
        npc_name,
        event_handlers,
        referenced_events,
    })
}

// ─── Line parsers ───────────────────────────────────────────────────────────

/// Parse "if (EVENT == 190) then" or "if EVENT == 190 then".
fn parse_event_if(line: &str) -> Option<i32> {
    let line = line.trim();
    if !line.starts_with("if") {
        return None;
    }
    // Remove "if" prefix, optional parens
    let rest = &line[2..];
    parse_event_equals(rest)
}

/// Parse "elseif (EVENT == 190) then" or "elseif EVENT == 190 then".
fn parse_elseif_event(line: &str) -> Option<i32> {
    let line = line.trim();
    if !line.starts_with("elseif") {
        return None;
    }
    let rest = &line[6..];
    parse_event_equals(rest)
}

/// Find "EVENT == <number>" in a string and extract the number.
fn parse_event_equals(s: &str) -> Option<i32> {
    let pos = s.find("EVENT")?;
    let after_event = &s[pos + 5..];
    // Skip whitespace and optional parens
    let after_event = after_event.trim_start();
    let after_event = after_event.strip_prefix("==")?;
    let after_event = after_event.trim_start();
    // Extract number
    let end = after_event
        .find(|c: char| !c.is_ascii_digit() && c != '-')
        .unwrap_or(after_event.len());
    if end == 0 {
        return None;
    }
    after_event[..end].parse::<i32>().ok()
}

/// Parse "EVENT = QuestNum" or "EVENT = 500" style.
fn parse_event_assign(line: &str) -> Option<i32> {
    // Look for "EVENT = <number>" (single = not ==)
    let pos = line.find("EVENT")?;
    let after = line[pos + 5..].trim_start();
    // Must be single =, not ==
    if !after.starts_with('=') || after.starts_with("==") {
        return None;
    }
    let after = after[1..].trim_start();
    // Try to parse a direct number
    let end = after
        .find(|c: char| !c.is_ascii_digit() && c != '-')
        .unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    after[..end].parse::<i32>().ok()
}

/// Parse SelectMsg call. Format: SelectMsg(UID, flag, quest_id, text_id, NPC, btn1, evt1, ...)
fn parse_select_msg(line: &str, line_num: usize) -> Option<SelectMsgCall> {
    let start = line.find("SelectMsg(")?;
    let rest = &line[start + 10..];
    let end = rest.find(')')?;
    let args_str = &rest[..end];

    let args = parse_comma_args(args_str);
    if args.len() < 5 {
        return None;
    }

    let flag = parse_int_arg(&args[1])?;
    let quest_id = parse_int_arg(&args[2]).unwrap_or(-1);
    let header_text_id = parse_int_arg(&args[3]).unwrap_or(0);
    let npc_param = parse_int_arg(&args[4]).unwrap_or(0);

    let mut buttons = Vec::new();
    let mut i = 5;
    while i + 1 < args.len() {
        let btn_text = parse_int_arg(&args[i]).unwrap_or(0);
        let next_event = parse_int_arg(&args[i + 1]).unwrap_or(-1);
        buttons.push((btn_text, next_event));
        i += 2;
    }

    Some(SelectMsgCall {
        line_num,
        flag,
        quest_id,
        header_text_id,
        npc_param,
        buttons,
    })
}

/// Parse RunQuestExchange(UID, exchange_id, ...).
fn parse_exchange(line: &str, line_num: usize) -> Option<ExchangeCall> {
    let funcnames = [
        "RunQuestExchange(",
        "RunCountExchange(",
        "RunRandomExchange(",
        "CheckExchange(",
    ];
    for funcname in &funcnames {
        if let Some(start) = line.find(funcname) {
            let rest = &line[start + funcname.len()..];
            let end = rest.find(')')?;
            let args = parse_comma_args(&rest[..end]);
            if args.len() >= 2 {
                if let Some(id) = parse_int_arg(&args[1]) {
                    return Some(ExchangeCall {
                        line_num,
                        exchange_id: id,
                    });
                }
            }
        }
    }
    None
}

/// Parse SaveEvent(UID, event_id).
fn parse_save_event(line: &str, line_num: usize) -> Option<SaveEventCall> {
    let start = line.find("SaveEvent(")?;
    let rest = &line[start + 10..];
    let end = rest.find(')')?;
    let args = parse_comma_args(&rest[..end]);
    if args.len() >= 2 {
        if let Some(id) = parse_int_arg(&args[1]) {
            return Some(SaveEventCall {
                line_num,
                event_id: id,
            });
        }
    }
    None
}

/// Parse GiveItem(UID, item_id, count) or GiveItem(UID, item_id).
fn parse_give_item(line: &str, line_num: usize) -> Option<GiveItemCall> {
    let start = line.find("GiveItem(")?;
    let rest = &line[start + 9..];
    let end = rest.find(')')?;
    let args = parse_comma_args(&rest[..end]);
    if args.len() >= 2 {
        let item_id = parse_int_arg(&args[1])?;
        let count = if args.len() >= 3 {
            parse_int_arg(&args[2]).unwrap_or(1)
        } else {
            1
        };
        return Some(GiveItemCall {
            line_num,
            item_id,
            count,
        });
    }
    None
}

/// Parse RobItem(UID, item_id, count).
fn parse_rob_item(line: &str, line_num: usize) -> Option<RobItemCall> {
    let start = line.find("RobItem(")?;
    let rest = &line[start + 8..];
    let end = rest.find(')')?;
    let args = parse_comma_args(&rest[..end]);
    if args.len() >= 3 {
        let item_id = parse_int_arg(&args[1])?;
        let count = parse_int_arg(&args[2])?;
        return Some(RobItemCall {
            line_num,
            item_id,
            count,
        });
    }
    None
}

/// Parse NpcMsg(UID, text_id, NPC).
fn parse_npc_msg(line: &str, line_num: usize) -> Option<NpcMsgCall> {
    let start = line.find("NpcMsg(")?;
    let rest = &line[start + 7..];
    let end = rest.find(')')?;
    let args = parse_comma_args(&rest[..end]);
    if args.len() >= 2 {
        if let Some(text_id) = parse_int_arg(&args[1]) {
            return Some(NpcMsgCall { line_num, text_id });
        }
    }
    None
}

/// Split comma-separated args, trimming whitespace.
fn parse_comma_args(s: &str) -> Vec<String> {
    s.split(',').map(|a| a.trim().to_string()).collect()
}

/// Try to parse a string as i32 (handling variable names like "NPC" or "UID" gracefully).
fn parse_int_arg(s: &str) -> Option<i32> {
    let s = s.trim();
    // Handle negative numbers
    if s.starts_with('-') {
        return s.parse::<i32>().ok();
    }
    // Handle plain numbers
    s.parse::<i32>().ok()
}
