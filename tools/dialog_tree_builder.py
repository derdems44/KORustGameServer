#!/usr/bin/env python3
"""
Dialog Tree Builder v2 — Sniffer capture'dan NPC dialog agaclari cikarir ve Lua uretir.

Mimari:
  1. Tum paketleri decrypt et (dogru AES key ile)
  2. Dialog-relevant event'leri filtrele (SELECT_MSG, NPC_EVENT, QUEST, ITEM_TRADE, vb.)
  3. NPC bazinda grupla (lua filename'e gore)
  4. Her NPC icin dialog conversation'lari izole et
  5. C2S click -> S2C response eslestireleri olustur
  6. Tree + Lua uret, eksikleri raporla

Kullanim:
  python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X
  python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X --npc 29235
  python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X --out-dir Quests/sniffer
"""

import argparse
import struct
import subprocess
import sys
import os
from collections import defaultdict, OrderedDict
from dataclasses import dataclass, field
from typing import Optional, List, Dict, Tuple, Any

from Crypto.Cipher import AES

AES_IV = bytes([
    0x32, 0x4E, 0xAA, 0x58, 0xBC, 0xB3, 0xAE, 0xE3,
    0x6B, 0xC7, 0x4C, 0x56, 0x36, 0x47, 0x34, 0xF2,
])

# Dialog-relevant opcodes
OP_NPC_EVENT   = 0x20
OP_SELECT_MSG  = 0x55
OP_QUEST       = 0x64
OP_ITEM_TRADE  = 0x68
OP_WARP_LIST   = 0x5A
OP_CLASS_CHANGE = 0x68  # same as ITEM_TRADE sub
OP_SHOPPING_MALL = 0x6A

# Known close/action button text IDs
CLOSE_TEXTS = {10, 27, 66}
NO_TEXTS    = {4162, 4528, 23}
YES_TEXTS   = {4161, 4527, 22}
TRIVIAL_TEXTS = CLOSE_TEXTS | NO_TEXTS | YES_TEXTS

# Noise opcodes to skip in raw event stream
NOISE_OPCODES = {
    0x02, 0x0B, 0x06, 0x09, 0x22, 0x42, 0x41, 0xA0,
    0x15, 0x98, 0x01, 0x13, 0x14, 0x19, 0x1E, 0x87,
}

# ---- Data structures ----

@dataclass
class Menu:
    header: int
    buttons: List[int]

    @property
    def sig(self):
        return f"{self.header}|{'_'.join(str(b) for b in self.buttons)}"

    def __hash__(self):
        return hash(self.sig)

    def __eq__(self, other):
        return isinstance(other, Menu) and self.sig == other.sig


@dataclass
class ClickResponse:
    """What happened after clicking a button."""
    menu: Menu           # which menu was shown
    button_idx: int      # which button was clicked
    response_type: str   # MENU, SHOP, QUEST, WARP, CLOSE, ACTION_0xNN
    target_menu: Optional[Menu] = None  # if response_type == MENU


@dataclass
class DialogEvent:
    """A single dialog-relevant event in the packet stream."""
    seq: int
    direction: str  # C2S or S2C
    event_type: str  # NPC_EVENT, SELECT_MSG, QUEST, ITEM_TRADE, WARP, OTHER_0xNN
    lua_name: str = ""
    menu: Optional[Menu] = None  # for S2C SELECT_MSG
    button_idx: int = -1  # for C2S SELECT_MSG
    opcode: int = 0


# ---- Crypto ----

def pkcs7_unpad(data):
    if not data: return None
    pad = data[-1]
    if pad < 1 or pad > 16: return None
    if any(b != pad for b in data[-pad:]): return None
    return data[:-pad]


def aes_decrypt(key_bytes, ciphertext):
    if len(ciphertext) == 0 or len(ciphertext) % 16 != 0:
        return None
    try:
        cipher = AES.new(key_bytes, AES.MODE_CBC, AES_IV)
        raw = cipher.decrypt(ciphertext)
        unpadded = pkcs7_unpad(raw)
        return unpadded if unpadded else raw
    except Exception:
        return None


# ---- DB fetch ----

def fetch_packets(session_id):
    result = subprocess.run([
        'docker', 'exec', 'ko-postgres', 'psql', '-U', 'koserver', '-d', 'ko_server', '-t', '-A', '-c',
        f"""SELECT seq, direction, encrypted::text, opcode,
               encode(raw_wire, 'hex'), encode(plaintext, 'hex')
            FROM pkt.packets
            WHERE session_id = {session_id}
            ORDER BY seq, direction;"""
    ], capture_output=True, text=True)
    rows = []
    for line in result.stdout.strip().split('\n'):
        if not line: continue
        parts = line.split('|')
        if len(parts) < 6: continue
        rows.append({
            'seq': int(parts[0]),
            'direction': parts[1],
            'encrypted': parts[2] in ('t', 'true'),
            'opcode': int(parts[3]),
            'raw_wire_hex': parts[4],
            'plaintext_hex': parts[5] if parts[5] else None,
        })
    return rows


# ---- Decrypt all ----

def decrypt_all(packets, aes_key_bytes):
    """Decrypt all packets -> list of (seq, direction, opcode, plaintext_bytes)."""
    events = []
    for pkt in packets:
        seq = pkt['seq']
        direction = pkt['direction']
        db_opcode = pkt['opcode']

        if pkt['plaintext_hex']:
            pt = bytes.fromhex(pkt['plaintext_hex'])
            if len(pt) > 0:
                events.append((seq, direction, db_opcode, pt))
            continue

        if pkt['encrypted'] and pkt['raw_wire_hex']:
            wire = bytes.fromhex(pkt['raw_wire_hex'])
            if len(wire) < 7 or wire[:2] != b'\xaa\x55' or wire[-2:] != b'\x55\xaa':
                continue
            if wire[4] != 0x01: continue
            ct = wire[5:-2]
            pt = aes_decrypt(aes_key_bytes, ct)
            if pt and len(pt) > 0:
                opc = pt[1] if direction == 'C2S' and len(pt) >= 2 else pt[0]
                events.append((seq, direction, opc, pt))
            continue

        if not pkt['encrypted'] and pkt['raw_wire_hex']:
            wire = bytes.fromhex(pkt['raw_wire_hex'])
            if len(wire) >= 7 and wire[:2] == b'\xaa\x55' and wire[-2:] == b'\x55\xaa':
                payload = wire[4:-2]
                if payload:
                    events.append((seq, direction, db_opcode, payload))
    return events


# ---- Parse helpers ----

def parse_s2c_select_msg(pt) -> Optional[Tuple[str, Menu]]:
    """Parse S2C WIZ_SELECT_MSG -> (lua_name, Menu) or None."""
    if len(pt) < 12 or pt[0] != 0x55: return None
    if b'.lua' not in pt: return None

    lua_idx = pt.find(b'.lua')
    s = lua_idx - 1
    while s > 0 and pt[s] >= 0x20: s -= 1
    lua_name = pt[s+1:lua_idx+4].decode('ascii', errors='replace')

    tids = []
    off = 10
    while off + 4 <= len(pt):
        t = struct.unpack_from('<I', pt, off)[0]
        if t == 0xFFFFFFFF: break
        tids.append(t)
        off += 4

    if not tids: return None
    return lua_name, Menu(header=tids[0], buttons=tids[1:])


def parse_c2s_select_msg(pt) -> Optional[Tuple[str, int]]:
    """Parse C2S WIZ_SELECT_MSG -> (lua_name, button_index) or None."""
    if len(pt) < 4: return None
    idx = 0
    if pt[0] != 0x55:
        if len(pt) > 1 and pt[1] == 0x55: idx = 1
        else: return None
    btn = pt[idx+1]
    lua_len = pt[idx+2]
    if idx + 3 + lua_len > len(pt): return None
    lua = pt[idx+3:idx+3+lua_len].decode('ascii', errors='replace')
    return lua, btn


# ---- Extract dialog events (filtered, no noise) ----

def extract_dialog_events(raw_events) -> List[DialogEvent]:
    """Filter raw events to dialog-relevant only."""
    dialog_events = []

    for seq, direction, opcode, pt in raw_events:
        if opcode == OP_SELECT_MSG:
            if direction == 'S2C':
                parsed = parse_s2c_select_msg(pt)
                if parsed:
                    lua, menu = parsed
                    dialog_events.append(DialogEvent(
                        seq=seq, direction='S2C', event_type='SELECT_MSG',
                        lua_name=lua, menu=menu, opcode=opcode
                    ))
            else:
                parsed = parse_c2s_select_msg(pt)
                if parsed:
                    lua, btn = parsed
                    dialog_events.append(DialogEvent(
                        seq=seq, direction='C2S', event_type='SELECT_MSG',
                        lua_name=lua, button_idx=btn, opcode=opcode
                    ))

        elif opcode == OP_NPC_EVENT and direction == 'C2S':
            dialog_events.append(DialogEvent(
                seq=seq, direction='C2S', event_type='NPC_EVENT', opcode=opcode
            ))

        elif direction == 'S2C' and opcode not in NOISE_OPCODES:
            if opcode == OP_QUEST:
                dialog_events.append(DialogEvent(
                    seq=seq, direction='S2C', event_type='QUEST', opcode=opcode
                ))
            elif opcode == OP_ITEM_TRADE:
                dialog_events.append(DialogEvent(
                    seq=seq, direction='S2C', event_type='ITEM_TRADE', opcode=opcode
                ))
            elif opcode == OP_WARP_LIST:
                dialog_events.append(DialogEvent(
                    seq=seq, direction='S2C', event_type='WARP', opcode=opcode
                ))
            elif opcode == OP_SHOPPING_MALL:
                dialog_events.append(DialogEvent(
                    seq=seq, direction='S2C', event_type='SHOPPING_MALL', opcode=opcode
                ))
            # Other S2C opcodes stored as generic action
            elif opcode not in NOISE_OPCODES and opcode != 0x02:
                dialog_events.append(DialogEvent(
                    seq=seq, direction='S2C', event_type=f'ACTION_0x{opcode:02X}',
                    opcode=opcode
                ))

    return dialog_events


# ---- Build per-NPC dialog data ----

def build_npc_dialogs(dialog_events: List[DialogEvent]) -> Dict[str, dict]:
    """Build dialog data per NPC lua_name.

    Returns: { lua_name: {
        'menus': { menu_sig: Menu },
        'clicks': [ ClickResponse, ... ],
        'roots': [ Menu, ... ],
    }}
    """
    npcs = defaultdict(lambda: {
        'menus': {},
        'clicks': [],
        'roots': [],
    })

    # Track active menu per NPC (last S2C SELECT_MSG shown)
    active_menu = {}  # lua_name -> Menu

    for i, evt in enumerate(dialog_events):
        lua = evt.lua_name

        if evt.event_type == 'SELECT_MSG' and evt.direction == 'S2C' and evt.menu:
            npc = npcs[lua]
            menu = evt.menu
            npc['menus'][menu.sig] = menu
            active_menu[lua] = menu

            # First menu after NPC_EVENT = root
            if i > 0 and dialog_events[i-1].event_type == 'NPC_EVENT':
                if menu not in npc['roots']:
                    npc['roots'].append(menu)

        elif evt.event_type == 'SELECT_MSG' and evt.direction == 'C2S' and lua:
            if lua not in active_menu:
                continue

            parent_menu = active_menu[lua]
            btn_idx = evt.button_idx

            # Find next S2C event for this NPC (or any meaningful S2C)
            response = find_response(dialog_events, i, lua)

            click = ClickResponse(
                menu=parent_menu,
                button_idx=btn_idx,
                response_type=response[0],
                target_menu=response[1],
            )
            npcs[lua]['clicks'].append(click)

            # Update active menu if response is a sub-menu
            if response[0] == 'MENU' and response[1]:
                active_menu[lua] = response[1]

    return dict(npcs)


def find_response(events: List[DialogEvent], click_idx: int, lua_name: str) -> Tuple[str, Optional[Menu]]:
    """Find the server's response to a C2S click.

    Looks forward in the event stream for the FIRST meaningful S2C response.
    Skips generic actions that aren't dialog-related.
    """
    for j in range(click_idx + 1, min(click_idx + 50, len(events))):
        evt = events[j]

        # Another C2S click = previous click had no response (CLOSE)
        if evt.direction == 'C2S':
            if evt.event_type == 'NPC_EVENT':
                return ('CLOSE', None)
            if evt.event_type == 'SELECT_MSG' and evt.lua_name == lua_name:
                return ('CLOSE', None)
            continue

        # S2C responses
        if evt.direction == 'S2C':
            if evt.event_type == 'SELECT_MSG' and evt.lua_name == lua_name:
                return ('MENU', evt.menu)
            if evt.event_type == 'SELECT_MSG' and evt.lua_name != lua_name:
                # Different NPC's menu = our dialog closed
                return ('CLOSE', None)
            if evt.event_type == 'QUEST':
                return ('QUEST', None)
            if evt.event_type == 'ITEM_TRADE':
                return ('SHOP', None)
            if evt.event_type == 'WARP':
                return ('WARP', None)
            if evt.event_type == 'SHOPPING_MALL':
                return ('SHOPPING_MALL', None)
            # Skip generic S2C actions (might be background traffic)
            # Only stop on clearly dialog-related responses
            continue

    return ('CLOSE', None)


# ---- Analyze and report ----

def analyze_npc(lua_name: str, data: dict) -> dict:
    """Analyze dialog completeness for one NPC."""
    menus = data['menus']
    clicks = data['clicks']
    roots = data['roots']

    # Build button map: for each (menu_sig, btn_idx), what's the response?
    button_map = {}  # (menu_sig, btn_idx) -> (response_type, target_menu_sig)
    for click in clicks:
        key = (click.menu.sig, click.button_idx)
        resp = click.response_type
        target = click.target_menu.sig if click.target_menu else None

        # Keep MENU responses over CLOSE (if we clicked same button multiple times)
        if key in button_map:
            old_resp = button_map[key][0]
            if old_resp == 'MENU' and resp != 'MENU':
                continue  # keep the MENU mapping
        button_map[key] = (resp, target)

    # Count coverage
    total_buttons = 0
    mapped_buttons = 0
    inferred_buttons = 0
    unknown_buttons = []

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


# ---- Lua generation ----

def generate_lua(lua_name: str, analysis: dict) -> str:
    """Generate Lua script from analyzed dialog data."""
    menus = analysis['menus']
    roots = analysis['roots']
    bmap = analysis['button_map']

    npc_id = 0
    try:
        npc_id = int(lua_name.split('_')[0])
    except ValueError:
        pass

    # Fetch NPC name from DB
    npc_name = lua_name
    if npc_id:
        result = subprocess.run([
            'docker', 'exec', 'ko-postgres', 'psql', '-U', 'koserver', '-d', 'ko_server', '-t', '-A', '-c',
            f"SELECT str_name FROM npc_template WHERE s_sid = {npc_id};"
        ], capture_output=True, text=True)
        name = result.stdout.strip()
        if name:
            npc_name = name

    # Assign event IDs
    CLOSE_EVENT = 3001
    event_ids = {}  # menu_sig -> event_id
    counter = 100

    # Roots first
    for root in roots:
        event_ids[root.sig] = counter
        counter += 1

    # Then all other menus
    for sig in menus:
        if sig not in event_ids:
            event_ids[sig] = counter
            counter += 1

    lines = []
    lines.append(f"local Ret = 0;")
    lines.append(f"local NPC = {npc_id};")
    lines.append(f"")
    lines.append(f"-- {npc_name}")
    lines.append(f"-- Auto-generated from sniffer capture (dialog_tree_builder v2)")
    lines.append(f"-- {len(menus)} menus, {analysis['mapped']} mapped, "
                 f"{analysis['inferred']} inferred, {len(analysis['unknown'])} unknown")
    lines.append(f"")

    # Sort by event ID
    sorted_menus = sorted(menus.items(), key=lambda x: event_ids.get(x[0], 9999))

    for sig, menu in sorted_menus:
        eid = event_ids[sig]
        is_root = menu in roots

        lines.append(f"-- {'ROOT ' if is_root else ''}header={menu.header}")
        lines.append(f"if (EVENT == {eid}) then")

        args = []
        for btn_idx, btn_text in enumerate(menu.buttons):
            key = (sig, btn_idx)
            if key in bmap:
                resp_type, target_sig = bmap[key]
                if resp_type == 'MENU' and target_sig and target_sig in event_ids:
                    args.append(f"{btn_text}, {event_ids[target_sig]}")
                elif resp_type in ('CLOSE', 'ACTION'):
                    args.append(f"{btn_text}, {CLOSE_EVENT}")
                elif resp_type == 'SHOP':
                    args.append(f"{btn_text}, {CLOSE_EVENT} -- SHOP")
                elif resp_type == 'QUEST':
                    args.append(f"{btn_text}, {CLOSE_EVENT} -- QUEST")
                elif resp_type == 'WARP':
                    args.append(f"{btn_text}, {CLOSE_EVENT} -- WARP")
                else:
                    args.append(f"{btn_text}, {CLOSE_EVENT} -- {resp_type}")
            else:
                args.append(f"{btn_text}, {CLOSE_EVENT} --[[ UNKNOWN ]]")

        arg_str = ", ".join(args)
        lines.append(f"\tSelectMsg(UID, 2, 0, {menu.header}, NPC, {arg_str});")
        lines.append(f"end")
        lines.append(f"")

    lines.append(f"-- Close dialog")
    lines.append(f"if (EVENT == {CLOSE_EVENT}) then")
    lines.append(f"\tRet = 1;")
    lines.append(f"end")
    lines.append(f"")

    return "\n".join(lines)


# ---- Main ----

def main():
    parser = argparse.ArgumentParser(description='Build NPC dialog trees from sniffer captures')
    parser.add_argument('--session', type=int, required=True)
    parser.add_argument('--key', type=str, required=True, help='AES key (16 ASCII chars)')
    parser.add_argument('--npc', type=int, help='Filter to specific NPC template ID')
    parser.add_argument('--out-dir', type=str, help='Output directory for Lua files')
    parser.add_argument('--report-only', action='store_true')
    args = parser.parse_args()

    key_bytes = args.key.encode('ascii')
    if len(key_bytes) != 16:
        print(f"Error: AES key must be 16 bytes, got {len(key_bytes)}", file=sys.stderr)
        sys.exit(1)

    # Step 1: Fetch & decrypt
    print(f"[1/4] Fetching session {args.session}...")
    packets = fetch_packets(args.session)
    print(f"  {len(packets)} packets")

    print(f"[2/4] Decrypting...")
    raw_events = decrypt_all(packets, key_bytes)
    print(f"  {len(raw_events)} events decrypted")

    # Step 2: Filter to dialog events
    print(f"[3/4] Extracting dialog events...")
    dialog_events = extract_dialog_events(raw_events)

    n_s2c = sum(1 for e in dialog_events if e.direction == 'S2C' and e.event_type == 'SELECT_MSG')
    n_c2s = sum(1 for e in dialog_events if e.direction == 'C2S' and e.event_type == 'SELECT_MSG')
    n_npc = sum(1 for e in dialog_events if e.event_type == 'NPC_EVENT')
    print(f"  SELECT_MSG: {n_s2c} S2C, {n_c2s} C2S | NPC_EVENT: {n_npc}")

    # Step 3: Build per-NPC dialog data
    npc_data = build_npc_dialogs(dialog_events)

    if args.npc:
        npc_data = {k: v for k, v in npc_data.items()
                    if str(args.npc) in k.split('_')[0]}

    print(f"  {len(npc_data)} NPCs found")

    # Step 4: Analyze & report
    print(f"\n[4/4] Analysis:\n")

    grand_total = 0
    grand_mapped = 0
    grand_inferred = 0
    grand_unknown = 0

    for lua_name in sorted(npc_data):
        data = npc_data[lua_name]
        analysis = analyze_npc(lua_name, data)

        pct = (analysis['mapped'] + analysis['inferred']) * 100 / max(analysis['total'], 1)
        status = 'DONE' if len(analysis['unknown']) == 0 else f'{pct:.0f}%'

        print(f"{'='*60}")
        print(f"[{status}] {lua_name}")
        print(f"  Menus: {len(analysis['menus'])} | Roots: {len(analysis['roots'])}")
        print(f"  Buttons: {analysis['total']} total, "
              f"{analysis['mapped']} mapped, "
              f"{analysis['inferred']} inferred, "
              f"{len(analysis['unknown'])} unknown")

        if analysis['unknown']:
            print(f"\n  UNMAPPED buttons (need to click on original server):")
            for sig, btn_idx, btn_text, menu in analysis['unknown']:
                print(f"    Menu h={menu.header} btn[{btn_idx}] text={btn_text}")

        grand_total += analysis['total']
        grand_mapped += analysis['mapped']
        grand_inferred += analysis['inferred']
        grand_unknown += len(analysis['unknown'])

        # Generate Lua
        if not args.report_only and args.out_dir:
            os.makedirs(args.out_dir, exist_ok=True)
            # Clean filename
            npc_part = lua_name.replace('.lua', '')
            out_file = os.path.join(args.out_dir, f"{npc_part}.lua")
            lua_code = generate_lua(lua_name, analysis)
            with open(out_file, 'w', encoding='utf-8') as f:
                f.write(lua_code)
            print(f"  Lua -> {out_file}")

    print(f"\n{'='*60}")
    pct = (grand_mapped + grand_inferred) * 100 / max(grand_total, 1)
    print(f"TOTAL: {len(npc_data)} NPCs, {grand_total} buttons")
    print(f"  Mapped:   {grand_mapped} (from sniffer clicks)")
    print(f"  Inferred: {grand_inferred} (Yes/No/OK/Close)")
    print(f"  Unknown:  {grand_unknown} (need clicking)")
    print(f"  Coverage: {pct:.1f}%")
    if grand_unknown > 0:
        print(f"\nRe-run after clicking unknowns. Sniffer captures new data automatically.")
    else:
        print(f"\nAll buttons resolved!")
    print(f"{'='*60}")


if __name__ == '__main__':
    main()
