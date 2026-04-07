#!/usr/bin/env python3
"""Multi-packet flow tracing tool for KO sniffer captures.

Detects and displays named packet sequences (login, game entry,
NPC dialog, combat, trade, zone change, etc.) by matching ordered
flow templates against decrypted session data.

Usage:
    python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X
    python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --flow game_entry
    python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --flow npc_dialog --hex
"""

from __future__ import annotations

import argparse
import os
import sys
from typing import Dict, List, Tuple

# Ensure the tools package is importable when invoked as a script.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session, DecodedPacket
from tools.lib.opcodes import get_name


# ---------------------------------------------------------------------------
# Flow templates -- named patterns of (direction, opcode) tuples
# ---------------------------------------------------------------------------

FLOW_TEMPLATES: Dict[str, List[Tuple[str, int]]] = {
    "game_entry": [
        # v2603 sniffer-verified flow: S2C 0x2B is the first encrypted
        # packet (C2S version check is pre-encryption / plaintext).
        ("S2C", 0x2B),  # VERSION_CHECK + AES key
        ("C2S", 0x01),  # LOGIN
        ("S2C", 0x01),  # LOGIN response
        ("C2S", 0x0C),  # ALLCHAR
        ("S2C", 0x0C),  # ALLCHAR response
        ("C2S", 0x04),  # SEL_CHAR
        ("S2C", 0x04),  # SEL_CHAR response
        ("C2S", 0x0D),  # GAMESTART
        ("S2C", 0x0D),  # GAMESTART response
    ],
    "npc_dialog": [
        ("C2S", 0x20),  # NPC_EVENT
        ("S2C", 0x55),  # SELECT_MSG
    ],
    "npc_trade": [
        ("C2S", 0x20),  # NPC_EVENT
        ("S2C", 0x55),  # SELECT_MSG
        ("C2S", 0x55),  # click
        ("S2C", 0x68),  # MERCHANT (shop opens)
    ],
    "quest_flow": [
        ("C2S", 0x20),  # NPC_EVENT
        ("S2C", 0x55),  # SELECT_MSG
        ("C2S", 0x55),  # click
        ("S2C", 0x64),  # QUEST
    ],
    "zone_change": [
        ("C2S", 0x27),  # ZONE_CHANGE
        ("S2C", 0x27),  # response
        ("C2S", 0x9F),  # LOADING_LOGIN
        ("S2C", 0x9F),  # response
    ],
    "combat_attack": [
        ("C2S", 0x08),  # ATTACK
        ("S2C", 0x08),  # ATTACK result
    ],
    "magic_cast": [
        ("C2S", 0x31),  # MAGIC_PROCESS
        ("S2C", 0x31),  # MAGIC_PROCESS result
    ],
}

MAX_GAP = 200  # maximum packets between template steps
MAX_INSTANCES_DISPLAY = 10  # show at most this many instances per flow


# ---------------------------------------------------------------------------
# Flow detection algorithm
# ---------------------------------------------------------------------------

def find_flows(
    packets: List[DecodedPacket],
    template: List[Tuple[str, int]],
) -> List[List[DecodedPacket]]:
    """Scan the packet stream for occurrences of a flow template.

    Allows gaps (noise packets between template steps -- up to MAX_GAP
    events). When all steps of the template are matched in order, the
    instance is recorded and scanning continues after the last matched
    packet.

    Args:
        packets: Ordered list of decoded packets for the session.
        template: List of (direction, opcode) tuples defining the flow.

    Returns:
        List of matched instances, each being a list of DecodedPackets.
    """
    instances: List[List[DecodedPacket]] = []
    i = 0
    while i < len(packets):
        t_idx = 0
        matched: List[DecodedPacket] = []
        j = i
        while j < len(packets) and t_idx < len(template):
            p = packets[j]
            exp_dir, exp_opc = template[t_idx]
            if p.direction == exp_dir and p.opcode == exp_opc:
                matched.append(p)
                t_idx += 1
            j += 1
            if j - i > MAX_GAP:
                break
        if t_idx == len(template):
            instances.append(matched)
            # Continue scanning after the last matched packet
            # Find the index of the last matched packet in the list
            last_seq = matched[-1].seq
            # Advance past the last matched packet
            next_i = j  # j is already past the last match
            i = next_i
        else:
            i += 1
    return instances


# ---------------------------------------------------------------------------
# Output formatting
# ---------------------------------------------------------------------------

def print_flow_results(
    flow_name: str,
    template: List[Tuple[str, int]],
    instances: List[List[DecodedPacket]],
    show_hex: bool = False,
) -> None:
    """Print results for a single flow template."""
    count = len(instances)
    print("=" * 50)
    print(f"Flow: {flow_name} -- {count} instance{'s' if count != 1 else ''} found")

    if count == 0:
        # Show the template for reference
        print("  Template:")
        for direction, opcode in template:
            name = get_name(opcode)
            print(f"    {direction} 0x{opcode:02X} {name}")
        print()
        return

    display_count = min(count, MAX_INSTANCES_DISPLAY)
    for idx in range(display_count):
        inst = instances[idx]
        first_seq = inst[0].seq
        last_seq = inst[-1].seq
        print()
        print(f"  Instance {idx + 1} (seq {first_seq}-{last_seq}):")
        for pkt in inst:
            name = get_name(pkt.opcode)
            size = len(pkt.plaintext)
            print(
                f"    {pkt.direction} seq={pkt.seq:>5} "
                f"0x{pkt.opcode:02X} {name:<30} ({size} bytes)"
            )
            if show_hex:
                hex_str = pkt.plaintext.hex()
                # Truncate long hex dumps to 80 chars
                if len(hex_str) > 80:
                    hex_str = hex_str[:80] + "..."
                print(f"         {hex_str}")

    if count > MAX_INSTANCES_DISPLAY:
        print(
            f"\n  ... and {count - MAX_INSTANCES_DISPLAY} more "
            f"instance{'s' if count - MAX_INSTANCES_DISPLAY != 1 else ''} "
            f"(showing first {MAX_INSTANCES_DISPLAY})"
        )
    print()


def print_summary(all_results: Dict[str, List[List[DecodedPacket]]]) -> None:
    """Print a compact summary table of all flows."""
    print("=" * 50)
    print("FLOW SUMMARY")
    print("=" * 50)
    print(f"  {'Flow Name':<20} {'Instances':>10}")
    print(f"  {'--------------------':<20} {'----------':>10}")
    for name, instances in sorted(all_results.items()):
        print(f"  {name:<20} {len(instances):>10}")
    print()


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description="KO multi-packet flow tracing tool",
    )
    parser.add_argument(
        "--session", type=int, required=True,
        help="Session ID to analyze",
    )
    parser.add_argument(
        "--key", type=str, required=True,
        help="AES key (16 ASCII chars)",
    )
    parser.add_argument(
        "--flow", type=str, default="all",
        help=(
            "Specific flow name to detect, or 'all' (default: all). "
            f"Available: {', '.join(sorted(FLOW_TEMPLATES.keys()))}"
        ),
    )
    parser.add_argument(
        "--hex", action="store_true",
        help="Show hex dumps for each packet in flow",
    )
    args = parser.parse_args()

    # Validate --flow
    if args.flow != "all" and args.flow not in FLOW_TEMPLATES:
        parser.error(
            f"Unknown flow '{args.flow}'. "
            f"Available: {', '.join(sorted(FLOW_TEMPLATES.keys()))}"
        )

    # Determine which templates to run
    if args.flow == "all":
        templates = FLOW_TEMPLATES
    else:
        templates = {args.flow: FLOW_TEMPLATES[args.flow]}

    # Decrypt session
    print(f"Decrypting session {args.session}...")
    packets = decrypt_session(args.session, args.key)
    print(f"Decoded {len(packets)} packets.")
    print()

    # Run flow detection
    all_results: Dict[str, List[List[DecodedPacket]]] = {}
    for flow_name, template in sorted(templates.items()):
        instances = find_flows(packets, template)
        all_results[flow_name] = instances
        print_flow_results(flow_name, template, instances, show_hex=args.hex)

    # Print summary when showing all flows
    if args.flow == "all":
        print_summary(all_results)


if __name__ == "__main__":
    main()
