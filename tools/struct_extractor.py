#!/usr/bin/env python3
"""Packet format reverse engineering tool for KO sniffer captures.

Compares multiple instances of the same opcode to find field boundaries
(constant vs variable bytes, likely field types).

Algorithm:
  1. Collect all packets matching (opcode, direction) from decrypted session
  2. Group by plaintext length (different lengths = different sub-opcodes
     or variable-length)
  3. For each length group with 2+ samples:
     - Compare byte-by-byte across all samples
     - Constant bytes (same in all) -> likely flags, opcodes, or padding
     - Variable bytes -> likely fields (IDs, counts, etc.)
  4. Guess field types based on patterns:
     - 1 variable byte alone -> u8
     - 2 consecutive variable bytes -> u16le
     - 4 consecutive variable bytes -> u32le

Usage:
    python tools/struct_extractor.py --session 37 --key 57UWLK49ALRO1C5X --opcode 0x55 --dir S2C
    python tools/struct_extractor.py --session 37 --key 57UWLK49ALRO1C5X --all --min-samples 5
"""

from __future__ import annotations

import argparse
import os
import sys
from collections import defaultdict
from typing import Dict, List, Tuple

# Ensure the tools package is importable when invoked as a script.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from tools.lib.db import decrypt_session
from tools.lib.opcodes import get_name


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------

class FieldGuess:
    """A guessed field at a given offset."""

    def __init__(
        self,
        offset: int,
        size: int,
        field_type: str,
        is_const: bool,
        const_value: int = 0,
        unique_count: int = 0,
        notes: str = "",
    ):
        self.offset = offset
        self.size = size
        self.field_type = field_type
        self.is_const = is_const
        self.const_value = const_value
        self.unique_count = unique_count
        self.notes = notes


# ---------------------------------------------------------------------------
# Core analysis
# ---------------------------------------------------------------------------

def analyze_byte_positions(
    samples: List[bytes],
    length: int,
) -> List[Tuple[bool, int, int]]:
    """Compare byte-by-byte across all samples.

    Returns a list of (is_constant, const_value, unique_count) per offset.
    """
    result = []
    for offset in range(length):
        values = set()
        for s in samples:
            values.add(s[offset])
        if len(values) == 1:
            result.append((True, samples[0][offset], 1))
        else:
            result.append((False, 0, len(values)))
    return result


def guess_fields(
    byte_info: List[Tuple[bool, int, int]],
    num_samples: int,
) -> List[FieldGuess]:
    """Group consecutive bytes into field guesses.

    Strategy:
      - Constant bytes are emitted individually (each is its own field).
      - Consecutive variable bytes are grouped and typed:
        1 byte  -> u8
        2 bytes -> u16le
        3 bytes -> u8 + u16le (or u24?)
        4 bytes -> u32le
        5+ bytes -> split into u32le + remainder recursively
    """
    fields: List[FieldGuess] = []
    length = len(byte_info)
    i = 0

    while i < length:
        is_const, const_val, unique_count = byte_info[i]

        if is_const:
            fields.append(FieldGuess(
                offset=i,
                size=1,
                field_type="const",
                is_const=True,
                const_value=const_val,
                unique_count=1,
                notes=f"always 0x{const_val:02X}",
            ))
            i += 1
        else:
            # Collect consecutive variable bytes
            var_start = i
            while i < length and not byte_info[i][0]:
                i += 1
            var_len = i - var_start

            # Determine max unique count across the span
            max_unique = max(byte_info[var_start + j][2] for j in range(var_len))

            # Split into typed fields
            _emit_variable_fields(fields, byte_info, var_start, var_len, max_unique)

    return fields


def _emit_variable_fields(
    fields: List[FieldGuess],
    byte_info: List[Tuple[bool, int, int]],
    start: int,
    length: int,
    max_unique: int,
) -> None:
    """Emit typed field guesses for a run of variable bytes."""
    pos = start
    remaining = length

    while remaining > 0:
        if remaining >= 4:
            # Prefer u32le for 4-byte chunks
            uniq = max(byte_info[pos + j][2] for j in range(4))
            fields.append(FieldGuess(
                offset=pos,
                size=4,
                field_type="u32le?",
                is_const=False,
                unique_count=uniq,
                notes=f"{uniq} unique values",
            ))
            pos += 4
            remaining -= 4
        elif remaining == 3:
            # u16le + u8 (more common in KO than u24)
            uniq16 = max(byte_info[pos + j][2] for j in range(2))
            fields.append(FieldGuess(
                offset=pos,
                size=2,
                field_type="u16le?",
                is_const=False,
                unique_count=uniq16,
                notes=f"{uniq16} unique values",
            ))
            pos += 2
            remaining -= 2
            uniq8 = byte_info[pos][2]
            fields.append(FieldGuess(
                offset=pos,
                size=1,
                field_type="u8?",
                is_const=False,
                unique_count=uniq8,
                notes=f"{uniq8} unique values",
            ))
            pos += 1
            remaining -= 1
        elif remaining == 2:
            uniq = max(byte_info[pos + j][2] for j in range(2))
            fields.append(FieldGuess(
                offset=pos,
                size=2,
                field_type="u16le?",
                is_const=False,
                unique_count=uniq,
                notes=f"{uniq} unique values",
            ))
            pos += 2
            remaining -= 2
        else:
            # remaining == 1
            uniq = byte_info[pos][2]
            fields.append(FieldGuess(
                offset=pos,
                size=1,
                field_type="u8?",
                is_const=False,
                unique_count=uniq,
                notes=f"{uniq} unique values",
            ))
            pos += 1
            remaining -= 1


def analyze_opcode(
    packets: List[bytes],
    opcode: int,
    direction: str,
) -> Dict[int, Tuple[int, List[FieldGuess]]]:
    """Analyze all packets for a given opcode and direction.

    Groups packets by plaintext length, then runs field analysis
    on each group with 2+ samples.

    Returns:
        Dict mapping length -> (sample_count, field_guesses)
    """
    # Group by length
    by_length: Dict[int, List[bytes]] = defaultdict(list)
    for pt in packets:
        by_length[len(pt)].append(pt)

    results: Dict[int, Tuple[int, List[FieldGuess]]] = {}

    for length in sorted(by_length.keys()):
        samples = by_length[length]
        count = len(samples)

        if count < 2:
            # Single sample -- mark all bytes as unknown, still show layout
            byte_info = []
            for offset in range(length):
                byte_info.append((True, samples[0][offset], 1))
            fields = []
            for offset in range(length):
                val = samples[0][offset]
                fields.append(FieldGuess(
                    offset=offset,
                    size=1,
                    field_type="??",
                    is_const=True,
                    const_value=val,
                    unique_count=1,
                    notes=f"0x{val:02X} (single sample)",
                ))
            results[length] = (count, fields)
        else:
            byte_info = analyze_byte_positions(samples, length)
            fields = guess_fields(byte_info, count)
            results[length] = (count, fields)

    return results


# ---------------------------------------------------------------------------
# Formatters
# ---------------------------------------------------------------------------

def format_opcode_analysis(
    opcode: int,
    direction: str,
    total_samples: int,
    analysis: Dict[int, Tuple[int, List[FieldGuess]]],
) -> str:
    """Format the analysis result as a human-readable string."""
    lines = []
    name = get_name(opcode)

    lines.append("=" * 70)
    lines.append(
        f"{direction} 0x{opcode:02X} {name} -- {total_samples} samples"
    )
    lines.append("=" * 70)

    if not analysis:
        lines.append("  (no packets found)")
        return "\n".join(lines)

    for length in sorted(analysis.keys()):
        count, fields = analysis[length]
        lines.append("")
        lines.append(
            f"  Length {length} bytes ({count} sample{'s' if count != 1 else ''}):"
        )
        lines.append(
            f"  {'Offset':<8} {'Hex':<8} {'Type':<12} {'Notes'}"
        )
        lines.append(
            f"  {'--------':<8} {'--------':<8} {'------------':<12} "
            f"{'------------------------------'}"
        )

        for field in fields:
            offset_str = str(field.offset)
            if field.is_const:
                hex_str = f"0x{field.const_value:02X}"
            else:
                hex_str = "--"

            type_str = field.field_type
            notes_str = field.notes

            lines.append(
                f"  {offset_str:<8} {hex_str:<8} {type_str:<12} {notes_str}"
            )

    lines.append("")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description="KO packet struct reverse engineering tool",
    )
    parser.add_argument(
        "--session", type=int, required=True,
        help="Sniffer session ID",
    )
    parser.add_argument(
        "--key", type=str, required=True,
        help="AES key (16 ASCII chars)",
    )
    parser.add_argument(
        "--opcode", type=str, default=None,
        help="Analyze specific opcode (e.g. 0x55)",
    )
    parser.add_argument(
        "--dir", type=str, default="S2C",
        choices=["S2C", "C2S"],
        help="Packet direction (default: S2C)",
    )
    parser.add_argument(
        "--all", action="store_true",
        help="Analyze all opcodes with enough samples",
    )
    parser.add_argument(
        "--min-samples", type=int, default=5,
        help="Minimum samples required for --all mode (default: 5)",
    )
    args = parser.parse_args()

    if not args.all and args.opcode is None:
        parser.error("Either --opcode or --all is required")

    # Decrypt session
    print(f"Decrypting session {args.session}...")
    all_packets = decrypt_session(args.session, args.key)
    print(f"  {len(all_packets)} packets decrypted")

    # Filter by direction
    dir_packets = [p for p in all_packets if p.direction == args.dir]
    print(f"  {len(dir_packets)} {args.dir} packets")

    if args.all:
        _run_all_mode(dir_packets, args.dir, args.min_samples)
    else:
        opcode_val = int(args.opcode, 16) if args.opcode.startswith("0x") else int(args.opcode)
        _run_single_opcode(dir_packets, opcode_val, args.dir)


def _run_single_opcode(
    dir_packets: list,
    opcode: int,
    direction: str,
) -> None:
    """Analyze a single opcode."""
    matching = [p.plaintext for p in dir_packets if p.opcode == opcode]
    if not matching:
        print(f"\n  No {direction} packets found for opcode 0x{opcode:02X}")
        return

    analysis = analyze_opcode(matching, opcode, direction)
    output = format_opcode_analysis(opcode, direction, len(matching), analysis)
    print(output)


def _run_all_mode(
    dir_packets: list,
    direction: str,
    min_samples: int,
) -> None:
    """Analyze all opcodes with enough samples."""
    # Group by opcode
    by_opcode: Dict[int, List[bytes]] = defaultdict(list)
    for p in dir_packets:
        by_opcode[p.opcode].append(p.plaintext)

    # Filter by min_samples and sort by sample count descending
    qualifying = [
        (op, pkts) for op, pkts in by_opcode.items()
        if len(pkts) >= min_samples
    ]
    qualifying.sort(key=lambda x: len(x[1]), reverse=True)

    if not qualifying:
        print(f"\n  No opcodes with >= {min_samples} samples in {direction}")
        return

    print(f"\n  {len(qualifying)} opcodes with >= {min_samples} samples")
    print()

    for opcode, packets in qualifying:
        analysis = analyze_opcode(packets, opcode, direction)
        output = format_opcode_analysis(opcode, direction, len(packets), analysis)
        print(output)


if __name__ == "__main__":
    main()
