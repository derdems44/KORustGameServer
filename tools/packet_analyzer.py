#!/usr/bin/env python3
"""Session overview tool for KO sniffer captures.

Shows opcode frequency, group aggregation, unknown detection,
and timeline per session.

Usage:
    python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X
    python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --groups
    python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --unknown
    python tools/packet_analyzer.py --sessions
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from collections import Counter
from typing import Any, Dict, List

# Ensure the tools package is importable when invoked as a script.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session, get_sessions
from tools.lib.opcodes import get_name, get_category, CATEGORIES


# ---------------------------------------------------------------------------
# Analysis
# ---------------------------------------------------------------------------

def analyze_session(session_id: int, aes_key: str) -> Dict[str, Any]:
    """Decrypt and analyze all packets in a session.

    Returns a dict with:
        session_id, total_packets, sources,
        freq_c2s, freq_s2c, first_seen, last_seen, groups
    """
    packets = decrypt_session(session_id, aes_key)

    sources: Counter = Counter()
    freq_c2s: Counter = Counter()
    freq_s2c: Counter = Counter()
    first_seen: Dict[int, int] = {}
    last_seen: Dict[int, int] = {}

    for pkt in packets:
        sources[pkt.source] += 1
        op = pkt.opcode
        if pkt.direction == "C2S":
            freq_c2s[op] += 1
        else:
            freq_s2c[op] += 1
        if op not in first_seen or pkt.seq < first_seen[op]:
            first_seen[op] = pkt.seq
        if op not in last_seen or pkt.seq > last_seen[op]:
            last_seen[op] = pkt.seq

    # Build group aggregation
    groups: Dict[str, Dict[str, Any]] = {}
    all_opcodes = set(freq_c2s.keys()) | set(freq_s2c.keys())
    for cat in CATEGORIES:
        cat_ops = CATEGORIES[cat] & all_opcodes
        if not cat_ops:
            continue
        c2s = sum(freq_c2s.get(op, 0) for op in cat_ops)
        s2c = sum(freq_s2c.get(op, 0) for op in cat_ops)
        groups[cat] = {
            "c2s": c2s,
            "s2c": s2c,
            "total": c2s + s2c,
            "opcodes": sorted(cat_ops),
        }

    # Catch unknown opcodes
    categorized = set()
    for ops in CATEGORIES.values():
        categorized |= ops
    unknown_ops = all_opcodes - categorized
    if unknown_ops:
        c2s = sum(freq_c2s.get(op, 0) for op in unknown_ops)
        s2c = sum(freq_s2c.get(op, 0) for op in unknown_ops)
        groups["unknown"] = {
            "c2s": c2s,
            "s2c": s2c,
            "total": c2s + s2c,
            "opcodes": sorted(unknown_ops),
        }

    return {
        "session_id": session_id,
        "total_packets": len(packets),
        "sources": dict(sources),
        "freq_c2s": dict(freq_c2s),
        "freq_s2c": dict(freq_s2c),
        "first_seen": first_seen,
        "last_seen": last_seen,
        "groups": groups,
    }


# ---------------------------------------------------------------------------
# Formatters
# ---------------------------------------------------------------------------

def _opcode_hex(op: int) -> str:
    return f"0x{op:02X}"


def print_opcode_table(result: Dict[str, Any]) -> None:
    """Print per-opcode frequency table (default view)."""
    sid = result["session_id"]
    total = result["total_packets"]
    sources = result["sources"]
    freq_c2s = result["freq_c2s"]
    freq_s2c = result["freq_s2c"]
    first_seen = result["first_seen"]
    last_seen = result["last_seen"]

    print("=" * 70)
    print(f"SESSION {sid} -- {total} packets")
    print(f"Sources: {sources}")
    print("=" * 70)
    print()

    header = (
        f"{'Opcode':<8} {'Name':<30} {'C2S':>6} {'S2C':>6} "
        f"{'Category':<20} {'Seq Range':<15}"
    )
    print(header)
    print(
        f"{'--------':<8} {'------------------------------':<30} {'------':>6} {'------':>6} "
        f"{'--------------------':<20} {'---------------':<15}"
    )

    all_ops = sorted(
        set(freq_c2s.keys()) | set(freq_s2c.keys()),
        key=lambda op: freq_c2s.get(op, 0) + freq_s2c.get(op, 0),
        reverse=True,
    )

    for op in all_ops:
        c2s = freq_c2s.get(op, 0)
        s2c = freq_s2c.get(op, 0)
        name = get_name(op)
        cat = get_category(op)
        fs = first_seen.get(op, 0)
        ls = last_seen.get(op, 0)
        seq_range = f"{fs}-{ls}"
        print(
            f"{_opcode_hex(op):<8} {name:<30} {c2s:>6} {s2c:>6} "
            f"{cat:<20} {seq_range:<15}"
        )


def print_groups(result: Dict[str, Any]) -> None:
    """Print group-aggregated view."""
    groups = result["groups"]
    print()
    print("--- OPCODE GROUPS ---")

    sorted_groups = sorted(groups.items(), key=lambda kv: kv[1]["total"], reverse=True)
    for cat, info in sorted_groups:
        ops_str = ", ".join(_opcode_hex(op) for op in info["opcodes"])
        print(
            f"  {cat:<24} {info['total']:>5} "
            f"({info['c2s']} C2S, {info['s2c']} S2C)  [{ops_str}]"
        )


def print_unknown(result: Dict[str, Any]) -> None:
    """Print only unknown/uncategorized opcodes."""
    freq_c2s = result["freq_c2s"]
    freq_s2c = result["freq_s2c"]
    first_seen = result["first_seen"]
    last_seen = result["last_seen"]

    categorized = set()
    for ops in CATEGORIES.values():
        categorized |= ops

    all_ops = sorted(set(freq_c2s.keys()) | set(freq_s2c.keys()))
    unknowns = [op for op in all_ops if op not in categorized]

    print()
    print("--- UNKNOWN OPCODES ---")
    if not unknowns:
        print("  (none)")
        return

    for op in unknowns:
        c2s = freq_c2s.get(op, 0)
        s2c = freq_s2c.get(op, 0)
        name = get_name(op)
        fs = first_seen.get(op, 0)
        ls = last_seen.get(op, 0)
        print(
            f"  {_opcode_hex(op)} {name:<30} C2S={c2s:>5}  S2C={s2c:>5}  "
            f"seq={fs}-{ls}"
        )


def print_sessions(sessions: List[dict]) -> None:
    """Print session listing."""
    print()
    print("--- RECENT SESSIONS ---")
    header = (
        f"{'ID':>4}  {'Type':<6}  {'Client IP':<16}  "
        f"{'Server':<22}  {'AES Key':<18}  {'Created'}"
    )
    print(header)
    print("-" * len(header))
    for s in sessions:
        key_display = s["aes_key"] or "(none)"
        server = f"{s['server_ip']}:{s['server_port']}"
        print(
            f"{s['id']:>4}  {s['server_type']:<6}  {s['client_ip']:<16}  "
            f"{server:<22}  {key_display:<18}  {s['created_at']}"
        )


# ---------------------------------------------------------------------------
# Auto-save
# ---------------------------------------------------------------------------

def _serialize_result(result: Dict[str, Any]) -> Dict[str, Any]:
    """Prepare the result dict for JSON serialization.

    Opcode keys in freq dicts are ints which serialize fine,
    but we add hex-keyed copies for readability.
    """
    out = dict(result)
    for key in ("freq_c2s", "freq_s2c", "first_seen", "last_seen"):
        out[key] = {_opcode_hex(int(k)): v for k, v in result[key].items()}
    # Groups: convert opcode lists to hex strings
    out["groups"] = {}
    for cat, info in result["groups"].items():
        out["groups"][cat] = {
            "c2s": info["c2s"],
            "s2c": info["s2c"],
            "total": info["total"],
            "opcodes": [_opcode_hex(op) for op in info["opcodes"]],
        }
    return out


def auto_save(result: Dict[str, Any]) -> str:
    """Save analysis JSON to captures/ directory. Returns the file path."""
    captures_dir = os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "captures",
    )
    os.makedirs(captures_dir, exist_ok=True)
    path = os.path.join(
        captures_dir,
        f"session_{result['session_id']}_analysis.json",
    )
    with open(path, "w", encoding="utf-8") as f:
        json.dump(_serialize_result(result), f, indent=2, ensure_ascii=False)
    return path


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description="KO sniffer session analysis tool",
    )
    parser.add_argument(
        "--session", type=int, default=None,
        help="Session ID to analyze",
    )
    parser.add_argument(
        "--key", type=str, default=None,
        help="AES key (16 ASCII chars)",
    )
    parser.add_argument(
        "--groups", action="store_true",
        help="Show opcode groups instead of individual opcodes",
    )
    parser.add_argument(
        "--unknown", action="store_true",
        help="Show only unknown/uncategorized opcodes",
    )
    parser.add_argument(
        "--json", action="store_true",
        help="Output JSON to stdout",
    )
    parser.add_argument(
        "--sessions", action="store_true",
        help="List recent sessions (ignores --session/--key)",
    )
    args = parser.parse_args()

    # Mode: list sessions
    if args.sessions:
        sessions = get_sessions(limit=30)
        if args.json:
            print(json.dumps(sessions, indent=2, ensure_ascii=False))
        else:
            print_sessions(sessions)
        return

    # Mode: analyze session
    if args.session is None or args.key is None:
        parser.error("--session and --key are required (or use --sessions)")

    result = analyze_session(args.session, args.key)

    # Always auto-save JSON
    save_path = auto_save(result)

    if args.json:
        print(json.dumps(_serialize_result(result), indent=2, ensure_ascii=False))
    elif args.groups:
        print_opcode_table(result)
        print_groups(result)
    elif args.unknown:
        print_opcode_table(result)
        print_unknown(result)
    else:
        print_opcode_table(result)

    if not args.json:
        print(f"\n[saved] {save_path}")


if __name__ == "__main__":
    main()
