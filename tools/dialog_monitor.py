#!/usr/bin/env python3
"""
Dialog Monitor v2 -- Real-time NPC dialog coverage with tree view.

Shows NPC dialog menus as a tree. Unknown buttons are highlighted.
Refreshes in-place (no scroll). Sub-menus indented under parent button.

Usage:
  python tools/dialog_monitor.py --session 60 --key Z4Y2UHHZN4L9AY2A
  python tools/dialog_monitor.py --session 60 --key Z4Y2UHHZN4L9AY2A --npc 29235
"""

import argparse
import subprocess
import sys
import time
import os
from collections import defaultdict
from typing import Dict, List, Optional, Set, Tuple

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session, _psql, get_sessions
from tools.dialog_builder import (
    extract_dialog_events, build_npc_data, analyze_npc,
    CLOSE_TEXTS, NO_TEXTS, YES_TEXTS, TRADE_TEXTS, CLOSE_EVENT,
    Menu,
)

# -- ANSI -----------------------------------------------------------------
R   = "\033[0m"     # reset
B   = "\033[1m"     # bold
D   = "\033[2m"     # dim
RED = "\033[91m"
GRN = "\033[92m"
YEL = "\033[93m"
CYN = "\033[96m"
MAG = "\033[95m"
WHT = "\033[97m"
BGRD = "\033[41m"
BGGR = "\033[42m"

_btn_cache: Dict[int, str] = {}   # quest_menu — button labels
_talk_cache: Dict[int, str] = {}  # quest_talk — NPC speech / menu headers
_text_loaded = False

def _load_table_col(table: str, col: str, max_len: int) -> Dict[int, str]:
    cache: Dict[int, str] = {}
    try:
        result = subprocess.run(
            [
                "docker", "exec", "ko-postgres",
                "psql", "-U", "koserver", "-d", "ko_server",
                "-t", "-A", "-c",
                f"SELECT i_num, substr({col}, 1, {max_len}) FROM {table};",
            ],
            capture_output=True, timeout=30,
        )
        raw = result.stdout.decode("utf-8", errors="replace")
        for line in raw.split("\n"):
            if "|" not in line:
                continue
            parts = line.split("|", 1)
            try:
                tid = int(parts[0])
            except ValueError:
                continue
            txt = parts[1].strip()
            if txt:
                cache[tid] = txt
    except Exception:
        pass
    return cache


def load_texts():
    """Load quest_menu (buttons) and quest_talk (headers) from DB."""
    global _text_loaded, _btn_cache, _talk_cache
    if _text_loaded:
        return
    _btn_cache = _load_table_col("quest_menu", "str_menu", 35)
    _talk_cache = _load_table_col("quest_talk", "str_talk", 40)
    _text_loaded = True


def btn_label(tid: int) -> str:
    """Button text from quest_menu."""
    return _btn_cache.get(tid, "")


def header_label(tid: int) -> str:
    """Header/NPC speech text from quest_talk."""
    return _talk_cache.get(tid, "")


_npc_name_cache: Dict[int, str] = {}

def npc_name(npc_id: int) -> str:
    if npc_id in _npc_name_cache:
        return _npc_name_cache[npc_id]
    try:
        n = _psql(f"SELECT str_name FROM npc_template WHERE s_sid = {npc_id};").strip()
        _npc_name_cache[npc_id] = n or f"NPC#{npc_id}"
    except Exception:
        _npc_name_cache[npc_id] = f"NPC#{npc_id}"
    return _npc_name_cache[npc_id]


def get_latest_game_session() -> Optional[dict]:
    for s in get_sessions(limit=5):
        if s.get('server_port') in ('15001', 15001):
            return s
    sessions = get_sessions(limit=1)
    return sessions[0] if sessions else None


# -- Tree builder ---------------------------------------------------------

def build_tree(menus, roots, bmap):
    """Build parent->child relationships from button_map.

    Returns:
      children: { menu_sig -> [(btn_idx, btn_text, child_sig)] }
      root_sigs: set of root menu sigs
    """
    children: Dict[str, List[Tuple[int, int, str]]] = defaultdict(list)
    for (sig, btn_idx), (resp_type, target_sig) in bmap.items():
        if resp_type == 'MENU' and target_sig and target_sig in menus:
            menu = menus[sig]
            btn_text = menu.buttons[btn_idx] if btn_idx < len(menu.buttons) else 0
            children[sig].append((btn_idx, btn_text, target_sig))
    root_sigs = set(m.sig for m in roots) if roots else set()
    return children, root_sigs


def has_unknown_below(sig, menus, bmap, children, visited=None):
    """Check if this menu or any descendant has unknown buttons."""
    if visited is None:
        visited = set()
    if sig in visited:
        return False
    visited.add(sig)

    menu = menus.get(sig)
    if not menu:
        return False
    for btn_idx, btn_text in enumerate(menu.buttons):
        if (sig, btn_idx) not in bmap:
            return True
    for _, _, child_sig in children.get(sig, []):
        if has_unknown_below(child_sig, menus, bmap, children, visited):
            return True
    return False


# -- Render ---------------------------------------------------------------

def render_menu_tree(
    lines: List[str],
    sig: str,
    menus: Dict[str, 'Menu'],
    bmap: dict,
    children: dict,
    root_sigs: Set[str],
    depth: int = 0,
    visited: Set[str] = None,
    parent_info: str = "",
):
    """Recursively render a menu and its sub-menus as indented tree."""
    if visited is None:
        visited = set()
    if sig in visited:
        return
    visited.add(sig)

    menu = menus.get(sig)
    if not menu:
        return

    indent = "  " + "    " * depth
    connector = "|   " * depth

    n_btns = len(menu.buttons)
    is_root = sig in root_sigs
    has_unk = any((sig, i) not in bmap for i in range(n_btns))

    # Check if any descendant has unknowns
    desc_unk = has_unknown_below(sig, menus, bmap, children)

    # Menu header
    flag_s = f"f={menu.flag}" if hasattr(menu, 'flag') and menu.flag != 2 else ""
    tag = f"{B}{MAG}ROOT{R}" if is_root else f"{D}  +->{R}"
    unk_mark = f" {RED}{B}<<<{R}" if has_unk else ""
    parent_s = f" {D}({parent_info}){R}" if parent_info else ""

    # If fully mapped and no descendant unknowns, show compact
    if not has_unk and not desc_unk:
        # Hide fully completed menus (root or child)
        return

    header_txt = header_label(menu.header)
    header_s = f' "{header_txt}"' if header_txt else ""
    lines.append(
        f"{indent}{tag} h={menu.header}{header_s} {flag_s} "
        f"[{n_btns} btn]{unk_mark}{parent_s}"
    )

    # Buttons
    btn_indent = indent + "    "
    child_map = {}
    for bi, bt, cs in children.get(sig, []):
        child_map[bi] = cs

    for btn_idx, btn_text in enumerate(menu.buttons):
        key = (sig, btn_idx)
        label = btn_label(btn_text)
        label_s = f" {D}({label}){R}" if label else ""

        if key in bmap:
            resp_type, target_sig = bmap[key]
            if resp_type == 'MENU' and target_sig:
                target_menu = menus.get(target_sig)
                # Check if target has unknowns
                target_unk = has_unknown_below(
                    target_sig, menus, bmap, children, set(visited)
                )
                if target_unk or (target_sig, 0) not in bmap:
                    arrow_color = YEL
                else:
                    arrow_color = GRN

                lines.append(
                    f"{btn_indent}{arrow_color}[{btn_idx+1}]{R} {btn_text:>6}"
                    f"  {arrow_color}v alt menu{R}{label_s}"
                )
                # Recurse into child menu
                render_menu_tree(
                    lines, target_sig, menus, bmap, children,
                    root_sigs, depth + 1, visited,
                    parent_info=f"buton [{btn_idx+1}] {btn_text}",
                )
            elif resp_type == 'CLOSE':
                lines.append(
                    f"{btn_indent}{D}[{btn_idx+1}]{R} {btn_text:>6}"
                    f"  {D}kapat{R}{label_s}"
                )
            elif resp_type in ('SHOP', 'WARP', 'MALL', 'QUEST'):
                lines.append(
                    f"{btn_indent}{CYN}[{btn_idx+1}]{R} {btn_text:>6}"
                    f"  {CYN}{resp_type}{R}{label_s}"
                )
            else:
                lines.append(
                    f"{btn_indent}{GRN}[{btn_idx+1}]{R} {btn_text:>6}"
                    f"  {GRN}{resp_type}{R}{label_s}"
                )
        else:
            lines.append(
                f"{btn_indent}{RED}{B}[{btn_idx+1}]{R} {btn_text:>6}"
                f"  {BGRD}{B} TIKLA {R}{label_s}"
            )


def render(session_id, key, analyses, total_pkts, filter_npc):
    """Build full display as string list, then print at once."""
    lines: List[str] = []
    now = time.strftime("%H:%M:%S")

    lines.append(f"{B}{CYN}+{'=' * 56}+{R}")
    lines.append(
        f"{B}{CYN}|{R} {B}Dialog Monitor v2{R} "
        f"| Ses:{session_id} | {total_pkts} pkt | {now}"
    )
    lines.append(
        f"{B}{CYN}|{R} {D}Key:{key[:8]}... | 1s refresh | Ctrl+C cikis{R}"
    )
    lines.append(f"{B}{CYN}+{'=' * 56}+{R}")
    lines.append("")

    if not analyses:
        lines.append(f"  {D}NPC'ye tikla...{R}")
    else:
        # Sort: most unknowns first, hide completed
        sorted_npcs = sorted(
            analyses.items(),
            key=lambda x: len(x[1].get('unknown', [])),
            reverse=True,
        )

        # Show completed NPCs as one-liner summary
        done_count = sum(1 for _, a in sorted_npcs if not a['unknown'])
        if done_count:
            done_names = [n.split('_')[0] for n, a in sorted_npcs if not a['unknown']]
            lines.append(f"  {GRN}{done_count} NPC TAMAM:{R} {D}{', '.join(done_names)}{R}")
            lines.append("")

        for lua_name, a in sorted_npcs:
            if not a['unknown']:
                continue  # skip completed NPCs
            npc_id = 0
            try:
                npc_id = int(lua_name.split('_')[0])
            except (ValueError, IndexError):
                pass
            if filter_npc and npc_id != filter_npc:
                continue

            name = npc_name(npc_id) if npc_id else lua_name
            menus = a['menus']
            roots = a.get('roots', [])
            bmap = a['button_map']
            total = a['total']
            covered = a['mapped'] + a['inferred']
            n_unk = len(a['unknown'])
            pct = covered * 100 / max(total, 1)

            if n_unk == 0:
                status = f"{BGGR}{B} TAMAM {R}"
            elif pct >= 80:
                status = f"{YEL}{B}%{pct:.0f}{R}"
            else:
                status = f"{BGRD}{B} %{pct:.0f} {R}"

            lines.append(f"{B}{'=' * 56}{R}")
            lines.append(
                f" {B}{WHT}{name} ({npc_id}){R}"
                f"  {status}  {D}[{covered}/{total}]{R}"
                f"  {RED}{n_unk} eksik{R}" if n_unk else
                f" {B}{WHT}{name} ({npc_id}){R}"
                f"  {status}  {D}[{covered}/{total}]{R}"
            )
            lines.append("")

            # Build tree
            children, root_sigs = build_tree(menus, roots, bmap)

            # Render from each root (numbered if multiple)
            visited: Set[str] = set()
            for ri, root_menu in enumerate(roots):
                if len(roots) > 1:
                    n_rb = len(root_menu.buttons)
                    lines.append(
                        f"  {B}{MAG}--- ROOT-{ri+1} ---{R}"
                        f" {D}h={root_menu.header} [{n_rb} btn]{R}"
                    )
                render_menu_tree(
                    lines, root_menu.sig, menus, bmap, children,
                    root_sigs, depth=0, visited=visited,
                )

            # Any orphan menus not reached from roots
            for sig in menus:
                if sig not in visited:
                    render_menu_tree(
                        lines, sig, menus, bmap, children,
                        root_sigs, depth=0, visited=visited,
                        parent_info="orphan",
                    )

            lines.append("")

        # Summary
        gt = sum(a['total'] for a in analyses.values())
        gc = sum(a['mapped'] + a['inferred'] for a in analyses.values())
        gu = sum(len(a['unknown']) for a in analyses.values())
        gp = gc * 100 / max(gt, 1)
        lines.append(f"{B}{'=' * 56}{R}")
        lines.append(
            f"  {B}TOPLAM:{R} {len(analyses)} NPC | "
            f"{GRN}{gc} ok{R} | "
            f"{RED}{gu} eksik{R} | "
            f"{B}%{gp:.0f}{R}"
        )

    # Clear screen and print
    if os.name == 'nt':
        subprocess.run(['cmd', '/c', 'cls'], shell=False)
    else:
        sys.stdout.write("\033[H\033[J")
    sys.stdout.write("\n".join(lines) + "\n")
    sys.stdout.flush()


# -- Main -----------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description='Dialog Monitor v2')
    parser.add_argument('--session', type=int, default=None)
    parser.add_argument('--key', type=str, default=None)
    parser.add_argument('--npc', type=int, default=None)
    parser.add_argument('--interval', type=int, default=1)
    args = parser.parse_args()

    sid = args.session
    key = args.key

    if sid is None:
        print(f"{CYN}Session tespiti...{R}")
        sess = get_latest_game_session()
        if sess:
            sid = sess['id']
            key = key or sess.get('aes_key', '')
            print(f"  Session {sid} ({sess.get('server_port','?')})")
        else:
            print(f"{RED}Session bulunamadi.{R}")
            sys.exit(1)

    if not key:
        try:
            k = _psql(
                f"SELECT key_ascii FROM pkt.crypto_keys "
                f"WHERE session_id = {sid} AND context='game' "
                f"ORDER BY id LIMIT 1;"
            )
            if k.strip():
                key = k.strip()
        except Exception:
            pass

    if not key or len(key) != 16:
        print(f"{RED}AES key gerekli. --key ile ver.{R}")
        sys.exit(1)

    print(f"{D}Text tablolari yukleniyor...{R}")
    load_texts()
    print(f"  {GRN}{len(_btn_cache)} buton + {len(_talk_cache)} dialog yuklendi{R}")

    last_count = 0
    cached = {}

    while True:
        try:
            cnt = int(_psql(
                f"SELECT COUNT(*) FROM pkt.packets WHERE session_id={sid};"
            ).strip() or "0")

            if cnt != last_count:
                last_count = cnt
                decoded = decrypt_session(sid, key)
                events = extract_dialog_events(decoded)
                nd = build_npc_data(events)
                if args.npc:
                    ns = str(args.npc)
                    nd = {k: v for k, v in nd.items()
                          if k.split('_')[0] == ns or ns in k}
                cached = {n: analyze_npc(n, nd[n]) for n in sorted(nd)}

            render(sid, key, cached, cnt, args.npc)
            time.sleep(args.interval)

        except KeyboardInterrupt:
            print(f"\n{YEL}Durduruldu.{R}")
            break
        except Exception as e:
            print(f"{RED}Hata: {e}{R}")
            import traceback; traceback.print_exc()
            time.sleep(args.interval)


if __name__ == '__main__':
    main()
