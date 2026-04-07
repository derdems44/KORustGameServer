#!/usr/bin/env python3
"""
Dialog Builder v4 -- Action-aware NPC dialog Lua generator.

Improvements over v3:
  - Parses S2C SelectMsg flag byte (offset 5) and quest_id (offset 6)
  - Generates real action event handlers for SHOP/WARP/MALL responses
  - Uses DB enrichment (npc_template.i_selling_group, quest_helper)
  - Preserves original SelectMsg flags from sniffer capture

Usage:
  python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X
  python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --npc 29235
  python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --out-dir Quests/
  python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --progress
"""

import argparse
import os
import struct
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

# -- Shared library imports --------------------------------------------------
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session, _psql
from tools.lib.opcodes import get_name

# -- Constants ---------------------------------------------------------------

OP_NPC_EVENT     = 0x20
OP_SELECT_MSG    = 0x55
OP_QUEST         = 0x64
OP_ITEM_TRADE    = 0x21  # WIZ_ITEM_TRADE
OP_TRADE_NPC     = 0x50  # WIZ_TRADE_NPC (S2C shop open)
OP_MERCHANT      = 0x68  # WIZ_MERCHANT
OP_WARP_LIST     = 0x4B
OP_SHOPPING_MALL = 0x6A

# Noise opcodes -- every S2C packet with one of these is dropped before pairing
NOISE_OPCODES = frozenset({
    0x02, 0x0B, 0x06, 0x09, 0x22, 0x42, 0x41, 0xA0,
    0x15, 0x98, 0x01, 0x13, 0x14, 0x19, 0x1E, 0x87,
    0x0A, 0x07, 0x1C, 0x1D, 0x16,
})

# Known close / action text IDs for inference
CLOSE_TEXTS  = frozenset({10, 27, 66})
NO_TEXTS     = frozenset({4162, 4528, 23})
YES_TEXTS    = frozenset({4161, 4527, 22, 14})   # 14 = Accept (quest context)
TRADE_TEXTS  = frozenset({28})                    # 28 = Trade (NPC shop)

CLOSE_EVENT = 3001
# Action events start above the menu event range to avoid collisions
ACTION_EVENT_BASE = 5000

# -- Data structures ---------------------------------------------------------

@dataclass
class Menu:
    header: int
    buttons: list       # list of text IDs (int)
    flag: int = 2       # SelectMsg flag from S2C packet (offset 5)
    quest_id: int = 0   # quest_id from S2C packet (offset 6)

    @property
    def sig(self):
        return f"{self.header}|{self.flag}|{'_'.join(str(b) for b in self.buttons)}"

    def __hash__(self):
        return hash(self.sig)

    def __eq__(self, other):
        return isinstance(other, Menu) and self.sig == other.sig


@dataclass
class DialogEvent:
    seq: int
    direction: str          # 'C2S' or 'S2C'
    etype: str              # S2C_MENU, C2S_CLICK, C2S_NPC_EVENT,
                            # S2C_QUEST, S2C_SHOP, S2C_WARP, S2C_MALL
    lua: str = ""
    menu: Optional[Menu] = None
    button_idx: int = -1
    opcode: int = 0


@dataclass
class ClickResponse:
    menu: Menu
    button_idx: int
    response_type: str          # MENU, SHOP, QUEST, WARP, MALL, CLOSE, ACTION
    target_menu: Optional[Menu] = None


# -- Parse helpers -----------------------------------------------------------

def parse_s2c_select_msg(pt: bytes) -> Optional[Tuple[str, Menu]]:
    """Parse S2C WIZ_SELECT_MSG -> (lua_name, Menu) or None.

    Wire format:
      [opcode:u8] [event_sid:u32le] [flag:u8] [quest_id:i32le]
      [header_text:i32le] [btn_text:i32le * 12] [lua_filename:sbyte_string]
    """
    if len(pt) < 12 or pt[0] != 0x55:
        return None
    if b'.lua' not in pt:
        return None

    # Extract flag (offset 5) and quest_id (offset 6)
    flag = pt[5]
    quest_id = struct.unpack_from('<i', pt, 6)[0]

    # Find lua filename by locating '.lua' and walking backwards
    lua_idx = pt.find(b'.lua')
    s = lua_idx - 1
    while s > 0 and pt[s] >= 0x20:
        s -= 1
    lua_name = pt[s + 1 : lua_idx + 4].decode('ascii', errors='replace')

    # Text IDs start at offset 10, each u32le, terminated by 0xFFFFFFFF
    tids = []
    off = 10
    while off + 4 <= len(pt):
        t = struct.unpack_from('<I', pt, off)[0]
        if t == 0xFFFFFFFF:
            break
        tids.append(t)
        off += 4

    if not tids:
        return None
    return lua_name, Menu(
        header=tids[0], buttons=tids[1:], flag=flag, quest_id=quest_id,
    )


def parse_c2s_select_msg(pt: bytes) -> Optional[Tuple[str, int]]:
    """Parse C2S WIZ_SELECT_MSG -> (lua_name, button_index) or None."""
    if len(pt) < 4:
        return None
    idx = 0
    if pt[0] != 0x55:
        if len(pt) > 1 and pt[1] == 0x55:
            idx = 1       # skip xor_seq byte
        else:
            return None
    btn = pt[idx + 1]
    lua_len = pt[idx + 2]
    if idx + 3 + lua_len > len(pt):
        return None
    lua = pt[idx + 3 : idx + 3 + lua_len].decode('ascii', errors='replace')
    return lua, btn


# -- Phase 2: extract dialog events -----------------------------------------

def extract_dialog_events(decoded_packets) -> List[DialogEvent]:
    """Filter decoded packets to ONLY dialog-relevant events."""
    events: List[DialogEvent] = []

    for pkt in decoded_packets:
        seq = pkt.seq
        direction = pkt.direction
        opcode = pkt.opcode
        pt = pkt.plaintext

        # -- C2S NPC_EVENT (dialog start) --
        if opcode == OP_NPC_EVENT and direction == 'C2S':
            events.append(DialogEvent(
                seq=seq, direction='C2S', etype='C2S_NPC_EVENT', opcode=opcode,
            ))
            continue

        # -- SELECT_MSG --
        if opcode == OP_SELECT_MSG:
            if direction == 'S2C':
                parsed = parse_s2c_select_msg(pt)
                if parsed:
                    lua, menu = parsed
                    events.append(DialogEvent(
                        seq=seq, direction='S2C', etype='S2C_MENU',
                        lua=lua, menu=menu, opcode=opcode,
                    ))
                elif len(pt) >= 6:
                    # Action-only SelectMsg: flag tells client what to do
                    action_flag = pt[5]
                    # Map known action flags to event types
                    FLAG_ACTION_MAP = {
                        18: 'S2C_EXCHANGE',   # item exchange/craft UI
                        21: 'S2C_SHOP',       # NPC shop
                        22: 'S2C_MALL',       # premium shop
                        27: 'S2C_DISASSEMBLE', # disassemble UI
                        70: 'S2C_SPECIAL_UI',  # special UI (indestructible remove etc.)
                    }
                    etype = FLAG_ACTION_MAP.get(action_flag)
                    if etype:
                        events.append(DialogEvent(
                            seq=seq, direction='S2C', etype=etype,
                            opcode=opcode,
                        ))
            else:
                parsed = parse_c2s_select_msg(pt)
                if parsed:
                    lua, btn = parsed
                    events.append(DialogEvent(
                        seq=seq, direction='C2S', etype='C2S_CLICK',
                        lua=lua, button_idx=btn, opcode=opcode,
                    ))
            continue

        # -- S2C dialog responses (only non-noise) --
        if direction == 'S2C' and opcode not in NOISE_OPCODES:
            if opcode == OP_QUEST:
                events.append(DialogEvent(
                    seq=seq, direction='S2C', etype='S2C_QUEST', opcode=opcode,
                ))
            elif opcode in (OP_ITEM_TRADE, OP_MERCHANT, OP_TRADE_NPC):
                events.append(DialogEvent(
                    seq=seq, direction='S2C', etype='S2C_SHOP', opcode=opcode,
                ))
            elif opcode == OP_WARP_LIST:
                events.append(DialogEvent(
                    seq=seq, direction='S2C', etype='S2C_WARP', opcode=opcode,
                ))
            elif opcode == OP_SHOPPING_MALL:
                events.append(DialogEvent(
                    seq=seq, direction='S2C', etype='S2C_MALL', opcode=opcode,
                ))
            # Everything else is silently dropped (noise).

    return events


# -- Phase 3: build per-NPC dialog data -------------------------------------

def build_npc_data(events: List[DialogEvent]) -> Dict[str, dict]:
    """Group by NPC lua_name, track active_menu, pair clicks with responses."""
    npcs: Dict[str, dict] = defaultdict(lambda: {
        'menus': {},
        'clicks': [],
        'roots': [],
    })

    active_menu: Dict[str, Menu] = {}   # lua_name -> last shown Menu

    for i, evt in enumerate(events):
        # -- S2C menu shown --
        if evt.etype == 'S2C_MENU' and evt.menu:
            lua = evt.lua
            npc = npcs[lua]
            menu = evt.menu
            npc['menus'][menu.sig] = menu
            active_menu[lua] = menu

            # First menu right after NPC_EVENT = root menu
            if i > 0 and events[i - 1].etype == 'C2S_NPC_EVENT':
                if menu not in npc['roots']:
                    npc['roots'].append(menu)

        # -- C2S click on a button --
        elif evt.etype == 'C2S_CLICK' and evt.lua:
            lua = evt.lua
            if lua not in active_menu:
                continue

            parent_menu = active_menu[lua]
            btn_idx = evt.button_idx

            # Look forward (max 30 events) for the response
            resp_type, resp_menu = _find_response(events, i, lua)

            npcs[lua]['clicks'].append(ClickResponse(
                menu=parent_menu,
                button_idx=btn_idx,
                response_type=resp_type,
                target_menu=resp_menu,
            ))

            # Update active menu if response is a sub-menu
            if resp_type == 'MENU' and resp_menu:
                active_menu[lua] = resp_menu

    return dict(npcs)


def _find_response(
    events: List[DialogEvent], click_idx: int, lua_name: str,
) -> Tuple[str, Optional[Menu]]:
    """Scan forward from click_idx to find the server response."""
    limit = min(click_idx + 30, len(events))

    for j in range(click_idx + 1, limit):
        r = events[j]

        # Another C2S before any S2C = CLOSE (dialog ended / new click)
        if r.direction == 'C2S':
            break

        # -- S2C responses --
        if r.etype == 'S2C_MENU' and r.lua == lua_name:
            return ('MENU', r.menu)
        if r.etype == 'S2C_MENU' and r.lua != lua_name:
            break   # different NPC, our dialog closed
        if r.etype == 'S2C_QUEST':
            return ('QUEST', None)
        if r.etype == 'S2C_SHOP':
            return ('SHOP', None)
        if r.etype == 'S2C_EXCHANGE':
            return ('EXCHANGE', None)
        if r.etype == 'S2C_DISASSEMBLE':
            return ('DISASSEMBLE', None)
        if r.etype == 'S2C_SPECIAL_UI':
            return ('SPECIAL_UI', None)
        if r.etype == 'S2C_WARP':
            return ('WARP', None)
        if r.etype == 'S2C_MALL':
            return ('MALL', None)

    return ('CLOSE', None)


# -- Phase 4: analyze -------------------------------------------------------

def analyze_npc(lua_name: str, data: dict) -> dict:
    """Count mapped / inferred / unknown buttons for one NPC."""
    menus = data['menus']
    clicks = data['clicks']
    roots = data['roots']

    # Build button map: (menu_sig, btn_idx) -> (response_type, target_sig)
    button_map: Dict[Tuple[str, int], Tuple[str, Optional[str]]] = {}

    # Priority: MENU > SHOP/WARP/MALL/QUEST > ACTION > CLOSE
    RESP_PRIORITY = {'MENU': 4, 'SHOP': 3, 'EXCHANGE': 3, 'DISASSEMBLE': 3,
                     'SPECIAL_UI': 3, 'WARP': 3, 'MALL': 3,
                     'QUEST': 3, 'ACTION': 2, 'CLOSE': 1}

    for click in clicks:
        key = (click.menu.sig, click.button_idx)
        resp = click.response_type
        target = click.target_menu.sig if click.target_menu else None

        # Get the button text for this click
        btn_text = click.menu.buttons[click.button_idx] \
            if click.button_idx < len(click.menu.buttons) else -1

        # If button text is a known CLOSE/NO text, override action responses
        # (these are sniffer false positives — OK/Close buttons don't open shops)
        if btn_text in CLOSE_TEXTS | NO_TEXTS and resp in ('SHOP', 'EXCHANGE', 'DISASSEMBLE', 'SPECIAL_UI', 'WARP', 'MALL'):
            resp = 'CLOSE'

        if key in button_map:
            old_pri = RESP_PRIORITY.get(button_map[key][0], 0)
            new_pri = RESP_PRIORITY.get(resp, 0)
            if new_pri <= old_pri:
                continue
        button_map[key] = (resp, target)

    total_buttons = 0
    mapped_buttons = 0
    inferred_buttons = 0
    unknown_buttons: List[Tuple[str, int, int, Menu]] = []

    for sig, menu in menus.items():
        for btn_idx, btn_text in enumerate(menu.buttons):
            total_buttons += 1
            key = (sig, btn_idx)

            if key in button_map:
                mapped_buttons += 1
            elif btn_text in CLOSE_TEXTS | NO_TEXTS:
                inferred_buttons += 1
                button_map[key] = ('CLOSE', None)
            elif btn_text in YES_TEXTS:
                inferred_buttons += 1
                button_map[key] = ('ACTION', None)
            elif btn_text in TRADE_TEXTS:
                inferred_buttons += 1
                button_map[key] = ('SHOP', None)
            else:
                unknown_buttons.append((sig, btn_idx, btn_text, menu))

    return {
        'menus': menus,
        'roots': roots,
        'button_map': button_map,
        'total': total_buttons,
        'mapped': mapped_buttons,
        'inferred': inferred_buttons,
        'unknown': unknown_buttons,
    }


# -- DB enrichment ----------------------------------------------------------

def _npc_name_from_db(npc_id: int) -> Optional[str]:
    """Try to fetch NPC name from DB."""
    try:
        name = _psql(f"SELECT str_name FROM npc_template WHERE s_sid = {npc_id};")
        return name.strip() if name.strip() else None
    except Exception:
        return None


def _npc_selling_group(npc_id: int) -> int:
    """Look up the NPC's selling group from npc_template."""
    try:
        val = _psql(
            f"SELECT i_selling_group FROM npc_template WHERE s_sid = {npc_id};"
        )
        return int(val.strip()) if val.strip() else 0
    except Exception:
        return 0


# -- Phase 5: Lua generation ------------------------------------------------

def generate_lua(lua_name: str, analysis: dict) -> str:
    """Produce a Lua script from analyzed dialog data with action handlers."""
    menus = analysis['menus']
    roots = analysis['roots']
    bmap = analysis['button_map']

    # Extract NPC ID from lua_name (e.g. "31772_Aset.lua" -> 31772)
    npc_id = 0
    try:
        npc_id = int(lua_name.split('_')[0])
    except (ValueError, IndexError):
        pass

    npc_name = lua_name
    if npc_id:
        db_name = _npc_name_from_db(npc_id)
        if db_name:
            npc_name = db_name

    # Look up NPC capabilities from DB
    selling_group = _npc_selling_group(npc_id) if npc_id else 0

    # Look up the REAL trigger event ID from quest_helper DB
    root_trigger = 100  # default
    if npc_id:
        trigger_str = _psql(
            f"SELECT n_event_trigger_index FROM quest_helper "
            f"WHERE s_npc_id = {npc_id} AND s_event_data_index = 0 "
            f"AND b_event_status = 0 LIMIT 1;"
        )
        if trigger_str.strip():
            try:
                root_trigger = int(trigger_str.strip())
            except ValueError:
                pass

    # Assign event IDs: roots get the REAL trigger from quest_helper
    event_ids: Dict[str, int] = {}
    counter = root_trigger

    for root in roots:
        event_ids[root.sig] = counter
        counter += 1

    for sig in menus:
        if sig not in event_ids:
            # Skip CLOSE_EVENT range
            if counter == CLOSE_EVENT:
                counter = CLOSE_EVENT + 10
            event_ids[sig] = counter
            counter += 1

    # Collect action events: list of (action_event_id, action_type, btn_text)
    action_events: List[Tuple[int, str, int]] = []
    action_counter = ACTION_EVENT_BASE

    # Pre-scan: assign event IDs for action buttons
    action_map: Dict[str, int] = {}  # "type:btn_text" -> event_id (dedup)

    def get_action_eid(action_type: str, btn_text: int) -> int:
        nonlocal action_counter
        key = f"{action_type}:{btn_text}"
        if key in action_map:
            return action_map[key]
        eid = action_counter
        action_counter += 1
        action_map[key] = eid
        action_events.append((eid, action_type, btn_text))
        return eid

    # -- Build Lua lines --
    lines: List[str] = []
    lines.append("local Ret = 0;")
    lines.append(f"local NPC = {npc_id};")
    lines.append("")
    lines.append(f"-- {npc_name}")
    lines.append(f"-- Auto-generated from sniffer capture (dialog_builder v4)")

    caps = []
    if selling_group:
        caps.append(f"shop={selling_group}")
    action_types_found = set()
    for (_, (resp_type, _)) in bmap.items():
        if resp_type in ('SHOP', 'WARP', 'MALL', 'QUEST'):
            action_types_found.add(resp_type)
    if action_types_found:
        caps.append(f"actions={'+'.join(sorted(action_types_found))}")

    stats = (f"{len(menus)} menus, {analysis['mapped']} mapped, "
             f"{analysis['inferred']} inferred, {len(analysis['unknown'])} unknown")
    if caps:
        stats += f" [{', '.join(caps)}]"
    lines.append(f"-- {stats}")
    lines.append("")

    # Sort menus by event ID
    sorted_menus = sorted(menus.items(), key=lambda x: event_ids.get(x[0], 9999))

    for sig, menu in sorted_menus:
        eid = event_ids[sig]
        is_root = menu in roots

        lines.append(f"-- {'ROOT: ' if is_root else ''}header={menu.header}"
                     f" flag={menu.flag}")
        lines.append(f"if (EVENT == {eid}) then")

        args: List[str] = []
        for btn_idx, btn_text in enumerate(menu.buttons):
            key = (sig, btn_idx)
            if key in bmap:
                resp_type, target_sig = bmap[key]
                if resp_type == 'MENU' and target_sig and target_sig in event_ids:
                    args.append(f"{btn_text}, {event_ids[target_sig]}")
                elif resp_type == 'CLOSE':
                    args.append(f"{btn_text}, {CLOSE_EVENT}")
                elif resp_type == 'ACTION':
                    args.append(f"{btn_text}, {CLOSE_EVENT}")
                elif resp_type == 'SHOP':
                    action_eid = get_action_eid('SHOP', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'EXCHANGE':
                    action_eid = get_action_eid('EXCHANGE', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'DISASSEMBLE':
                    action_eid = get_action_eid('DISASSEMBLE', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'SPECIAL_UI':
                    action_eid = get_action_eid('SPECIAL_UI', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'QUEST':
                    action_eid = get_action_eid('QUEST', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'WARP':
                    action_eid = get_action_eid('WARP', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                elif resp_type == 'MALL':
                    action_eid = get_action_eid('MALL', btn_text)
                    args.append(f"{btn_text}, {action_eid}")
                else:
                    args.append(f"{btn_text}, {CLOSE_EVENT}")
            else:
                args.append(f"{btn_text}, {CLOSE_EVENT} --[[ TODO: unknown ]]")

        arg_str = ", ".join(args)
        # Use the original flag from sniffer capture
        lines.append(
            f"\tSelectMsg(UID, {menu.flag}, {menu.quest_id}, "
            f"{menu.header}, NPC, {arg_str});"
        )
        lines.append("end")
        lines.append("")

    # -- Action event handlers --
    if action_events:
        lines.append("-- ═══ Action handlers (sniffer-verified) ═══")
        lines.append("")

    for action_eid, action_type, btn_text in action_events:
        if action_type == 'SHOP':
            lines.append(f"-- SHOP action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            if selling_group > 0:
                lines.append(f"\tOpenTradeNpc(UID);")
            else:
                lines.append(
                    f"\tSelectMsg(UID, 21, -1, -1, NPC, -1, -1); "
                    f"-- selling_group=0, fallback to flag 21"
                )
            lines.append("end")
            lines.append("")

        elif action_type == 'EXCHANGE':
            lines.append(f"-- EXCHANGE action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(f"\tSelectMsg(UID, 18, -1, -1, NPC);")
            lines.append("end")
            lines.append("")

        elif action_type == 'DISASSEMBLE':
            lines.append(f"-- DISASSEMBLE action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(f"\tSelectMsg(UID, 27, -1, -1, NPC);")
            lines.append("end")
            lines.append("")

        elif action_type == 'SPECIAL_UI':
            lines.append(f"-- SPECIAL UI action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(f"\tSelectMsg(UID, 70, -1, -1, NPC);")
            lines.append("end")
            lines.append("")

        elif action_type == 'WARP':
            lines.append(f"-- WARP action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(f"\tSendWarpList(UID);")
            lines.append("end")
            lines.append("")

        elif action_type == 'MALL':
            lines.append(f"-- MALL/Premium Shop action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(f"\tOpenShoppingMall(UID, 1);")
            lines.append("end")
            lines.append("")

        elif action_type == 'QUEST':
            lines.append(f"-- QUEST action (btn_text={btn_text})")
            lines.append(f"if (EVENT == {action_eid}) then")
            lines.append(
                f"\t-- TODO: wire quest logic (SaveEvent, RunExchange, etc.)"
            )
            lines.append(f"\tRet = 1;")
            lines.append("end")
            lines.append("")

    # -- Close handler --
    lines.append("-- Close dialog")
    lines.append(f"if (EVENT == {CLOSE_EVENT}) then")
    lines.append("\tRet = 1;")
    lines.append("end")
    lines.append("")

    return "\n".join(lines)


# -- CLI / main --------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description='Dialog Builder v4 -- action-aware NPC dialog Lua generator')
    parser.add_argument('--session', type=int, required=True,
                        help='Sniffer session ID')
    parser.add_argument('--key', type=str, required=True,
                        help='AES key (16 ASCII chars)')
    parser.add_argument('--npc', type=int, default=None,
                        help='Filter to specific NPC template ID')
    parser.add_argument('--out-dir', type=str, default=None,
                        help='Output directory for Lua files')
    parser.add_argument('--progress', action='store_true',
                        help='Show only coverage summary (no per-button detail)')
    args = parser.parse_args()

    key = args.key
    if len(key) != 16:
        print(f"Error: AES key must be 16 ASCII chars, got {len(key)}",
              file=sys.stderr)
        sys.exit(1)

    # ---- Phase 1: decrypt session ----
    print(f"[1/5] Decrypting session {args.session} ...")
    decoded = decrypt_session(args.session, key)
    print(f"  {len(decoded)} packets decoded")

    # ---- Phase 2: extract dialog events ----
    print("[2/5] Extracting dialog events ...")
    dialog_events = extract_dialog_events(decoded)

    n_s2c_menu  = sum(1 for e in dialog_events if e.etype == 'S2C_MENU')
    n_c2s_click = sum(1 for e in dialog_events if e.etype == 'C2S_CLICK')
    n_npc_event = sum(1 for e in dialog_events if e.etype == 'C2S_NPC_EVENT')
    n_quest     = sum(1 for e in dialog_events if e.etype == 'S2C_QUEST')
    n_shop      = sum(1 for e in dialog_events if e.etype == 'S2C_SHOP')
    n_warp      = sum(1 for e in dialog_events if e.etype == 'S2C_WARP')
    n_mall      = sum(1 for e in dialog_events if e.etype == 'S2C_MALL')
    print(f"  S2C_MENU: {n_s2c_menu} | C2S_CLICK: {n_c2s_click} | "
          f"NPC_EVENT: {n_npc_event}")
    print(f"  QUEST: {n_quest} | SHOP: {n_shop} | WARP: {n_warp} | "
          f"MALL: {n_mall}")

    # ---- Phase 3: build NPC data ----
    print("[3/5] Building NPC dialog data ...")
    npc_data = build_npc_data(dialog_events)

    if args.npc:
        npc_str = str(args.npc)
        npc_data = {k: v for k, v in npc_data.items()
                    if k.split('_')[0] == npc_str or npc_str in k}

    print(f"  {len(npc_data)} NPCs found")

    # ---- Phase 4: analyze ----
    print("[4/5] Analyzing coverage ...")
    grand_total   = 0
    grand_mapped  = 0
    grand_inferred = 0
    grand_unknown = 0

    analyses: Dict[str, dict] = {}
    for lua_name in sorted(npc_data):
        analysis = analyze_npc(lua_name, npc_data[lua_name])
        analyses[lua_name] = analysis

        grand_total    += analysis['total']
        grand_mapped   += analysis['mapped']
        grand_inferred += analysis['inferred']
        grand_unknown  += len(analysis['unknown'])

    # Print summary
    print("")
    for lua_name in sorted(analyses):
        a = analyses[lua_name]
        pct = (a['mapped'] + a['inferred']) * 100 / max(a['total'], 1)
        status = 'DONE' if not a['unknown'] else f"{pct:.0f}%"

        # Count action types
        action_counts = defaultdict(int)
        for (_, (resp_type, _)) in a['button_map'].items():
            if resp_type in ('SHOP', 'WARP', 'MALL', 'QUEST'):
                action_counts[resp_type] += 1
        action_str = ""
        if action_counts:
            action_str = " " + " ".join(
                f"[{t}:{c}]" for t, c in sorted(action_counts.items())
            )

        print(f"  [{status:>4s}] {lua_name}: "
              f"{len(a['menus'])} menus, "
              f"{a['total']} btns "
              f"({a['mapped']} mapped + {a['inferred']} inferred "
              f"+ {len(a['unknown'])} unknown)"
              f"{action_str}")

        if not args.progress and a['unknown']:
            for sig, btn_idx, btn_text, menu in a['unknown']:
                print(f"         unmapped: h={menu.header} btn[{btn_idx}] "
                      f"text={btn_text}")

    total_pct = (grand_mapped + grand_inferred) * 100 / max(grand_total, 1)
    print("")
    print(f"  TOTAL: {len(analyses)} NPCs, {grand_total} buttons")
    print(f"    mapped={grand_mapped}  inferred={grand_inferred}  "
          f"unknown={grand_unknown}  coverage={total_pct:.1f}%")

    # ---- Phase 5: generate Lua ----
    if args.out_dir:
        print(f"\n[5/5] Generating Lua -> {args.out_dir}")
        os.makedirs(args.out_dir, exist_ok=True)
        for lua_name, analysis in analyses.items():
            npc_part = lua_name.replace('.lua', '')
            out_file = os.path.join(args.out_dir, f"{npc_part}.lua")
            lua_code = generate_lua(lua_name, analysis)
            with open(out_file, 'w', encoding='utf-8') as f:
                f.write(lua_code)
            print(f"  -> {out_file}")
    else:
        print("\n[5/5] Lua generation skipped (use --out-dir to enable)")

    print("\nDone.")


if __name__ == '__main__':
    main()
