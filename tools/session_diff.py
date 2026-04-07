#!/usr/bin/env python3
"""Compare packet streams between original server and our server.

Decrypts two sniffer sessions and reports:
  - Missing opcodes (original sends, we don't)
  - Extra opcodes (we send, original doesn't)
  - Significant count differences (both have, but >30% off)

Usage:
    python tools/session_diff.py --original 37 --ours 38 --key1 KEY1 --key2 KEY2
    python tools/session_diff.py --original 37 --ours 37 --key1 KEY --key2 KEY
"""

from __future__ import annotations

import argparse
import os
import sys
from collections import Counter
from typing import List, Tuple

# Ensure the tools package is importable when invoked as a script.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session, DecodedPacket
from tools.lib.opcodes import get_name


# ---------------------------------------------------------------------------
# Comparison logic
# ---------------------------------------------------------------------------

def compare_sessions(
    pkts_orig: List[DecodedPacket],
    pkts_ours: List[DecodedPacket],
) -> Tuple[list, list, list]:
    """Compare two decoded packet lists by (direction, opcode) frequency.

    Returns:
        (missing, extra, different) where each is a list of tuples:
        - missing:   (direction, opcode, opcode_name, orig_count)
        - extra:     (direction, opcode, opcode_name, ours_count)
        - different: (direction, opcode, opcode_name, orig_count, ours_count)
    """
    orig_opcodes: Counter = Counter(
        (p.direction, p.opcode) for p in pkts_orig
    )
    ours_opcodes: Counter = Counter(
        (p.direction, p.opcode) for p in pkts_ours
    )

    all_keys = set(orig_opcodes) | set(ours_opcodes)

    missing = []
    extra = []
    different = []

    for key in sorted(all_keys, key=lambda k: (k[0], k[1])):
        direction, opcode = key
        o_cnt = orig_opcodes.get(key, 0)
        u_cnt = ours_opcodes.get(key, 0)
        name = get_name(opcode)

        if o_cnt > 0 and u_cnt == 0:
            missing.append((direction, opcode, name, o_cnt))
        elif o_cnt == 0 and u_cnt > 0:
            extra.append((direction, opcode, name, u_cnt))
        else:
            # Both have it -- check for >30% difference
            max_cnt = max(o_cnt, u_cnt)
            if abs(o_cnt - u_cnt) > max_cnt * 0.3:
                different.append((direction, opcode, name, o_cnt, u_cnt))

    return missing, extra, different


# ---------------------------------------------------------------------------
# Output formatting (ASCII only -- Windows cp1254 safe)
# ---------------------------------------------------------------------------

def print_report(
    missing: list,
    extra: list,
    different: list,
    orig_total: int,
    ours_total: int,
    orig_session: int,
    ours_session: int,
) -> None:
    """Print the comparison report to stdout."""
    sep = "=" * 60
    print(sep)
    print("SESSION DIFF REPORT")
    print(sep)
    print(f"  Original session: {orig_session}  ({orig_total} packets)")
    print(f"  Our session:      {ours_session}  ({ours_total} packets)")
    print(sep)

    # -- Missing --
    print()
    print("MISSING (original sends, we don't):")
    if missing:
        for direction, opcode, name, count in missing:
            print(f"  {direction} 0x{opcode:02X} {name:<40s} x{count}")
    else:
        print("  None!")

    # -- Extra --
    print()
    print("EXTRA (we send, original doesn't):")
    if extra:
        for direction, opcode, name, count in extra:
            print(f"  {direction} 0x{opcode:02X} {name:<40s} x{count}")
    else:
        print("  None!")

    # -- Significant count differences --
    print()
    print("SIGNIFICANT COUNT DIFFERENCES (>30%):")
    if different:
        for direction, opcode, name, o_cnt, u_cnt in different:
            print(
                f"  {direction} 0x{opcode:02X} {name:<40s} "
                f"original={o_cnt}  ours={u_cnt}"
            )
    else:
        print("  None!")

    print()
    print(sep)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description="Compare packet streams between original and our server.",
    )
    parser.add_argument(
        "--original", type=int, required=True,
        help="Original server session ID",
    )
    parser.add_argument(
        "--ours", type=int, required=True,
        help="Our server session ID",
    )
    parser.add_argument(
        "--key1", type=str, required=True,
        help="AES key for original session (ASCII)",
    )
    parser.add_argument(
        "--key2", type=str, required=True,
        help="AES key for our server session (ASCII)",
    )
    return parser.parse_args()


def main() -> None:
    """Entry point."""
    args = parse_args()

    print(f"Decrypting original session {args.original} ...")
    pkts_orig = decrypt_session(args.original, args.key1)
    print(f"  -> {len(pkts_orig)} packets decoded")

    print(f"Decrypting our session {args.ours} ...")
    pkts_ours = decrypt_session(args.ours, args.key2)
    print(f"  -> {len(pkts_ours)} packets decoded")

    missing, extra, different = compare_sessions(pkts_orig, pkts_ours)

    print_report(
        missing, extra, different,
        orig_total=len(pkts_orig),
        ours_total=len(pkts_ours),
        orig_session=args.original,
        ours_session=args.ours,
    )


if __name__ == "__main__":
    main()
