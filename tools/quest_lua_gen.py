#!/usr/bin/env python3
"""
Quest Lua Generator -- builds Lua NPC scripts from .tbl data and/or sniffer captures.

Modes:
  1. TBL-only:      --npc 13013 --tbl-only
  2. Sniffer-only:   --npc 31772 --sniffer-only
  3. Merged (default): TBL structure + sniffer text IDs where available

Usage:
  # TBL-only for single NPC
  python quest_lua_gen.py --npc 13013 --dsn "..." --tbl-only

  # Sniffer-only
  python quest_lua_gen.py --npc 31772 --dsn "..." --sniffer-only

  # Merged mode (default)
  python quest_lua_gen.py --npc 13013 --dsn "..."

  # Batch: all NPCs with quest_helper entries
  python quest_lua_gen.py --all --dsn "..." --out-dir Quests/

  # Output to file
  python quest_lua_gen.py --npc 13013 --dsn "..." --out Quests/13013_Patric.lua
"""

import argparse
import os
import struct
import sys
from collections import OrderedDict, defaultdict

try:
    import psycopg2
except ImportError:
    print("pip install psycopg2-binary", file=sys.stderr)
    sys.exit(1)

DEFAULT_DSN = "host=localhost port=5433 dbname=ko_server user=koserver password=$DB_PASSWORD"
DEFAULT_TBL_PATH = r"C:\PvpKo\Data\Quest_Helper.tbl"

# Standard button text IDs used in SelectMsg
BTN_OK = 10
BTN_DECLINE = 13
BTN_LEAVE = 14
BTN_ACCEPT = 22
BTN_REJECT = 23
BTN_CONTINUE = 24
BTN_CHALLENGE = 25
BTN_CANCEL = 27
BTN_MAP = 18
BTN_REWARD = 41

# Class ID groupings for per-class handlers
CLASS_WARRIOR = [1, 5, 6, 13, 14, 15]
CLASS_ROGUE = [2, 7, 8]
CLASS_MAGE = [3, 9, 10]
CLASS_PRIEST = [4, 11, 12]
CLASS_GROUPS = [
    ("Warrior", CLASS_WARRIOR),
    ("Rogue", CLASS_ROGUE),
    ("Mage", CLASS_MAGE),
    ("Priest", CLASS_PRIEST),
]

STATUS_NEW = 0
STATUS_IN_PROGRESS = 1
STATUS_COMPLETABLE = 2
STATUS_COMPLETED = 3
STATUS_ANY = 4
STATUS_PREQUEST = 255

STATUS_LABELS = {
    0: "not started",
    1: "in progress",
    2: "completable",
    3: "completed",
    4: "any state",
    255: "pre-quest",
}


# -----------------------------------------------------------------------
# Sniffer packet parsing (Mode 2: sniffer-only)
# -----------------------------------------------------------------------

def parse_s2c_select_msg(data: bytes) -> dict | None:
    """Parse S2C WIZ_SELECT_MSG (opcode 0x55) packet."""
    if len(data) < 62 or data[0] != 0x55:
        return None
    pos = 1
    event_sid = struct.unpack_from("<I", data, pos)[0]; pos += 4
    flag = data[pos]; pos += 1
    quest_id = struct.unpack_from("<i", data, pos)[0]; pos += 4
    header_text = struct.unpack_from("<i", data, pos)[0]; pos += 4
    buttons = []
    for _ in range(6):
        if pos + 8 > len(data):
            break
        text_id = struct.unpack_from("<i", data, pos)[0]; pos += 4
        event_id = struct.unpack_from("<i", data, pos)[0]; pos += 4
        if text_id != -1:
            buttons.append((text_id, event_id))
    lua_len = data[pos] if pos < len(data) else 0; pos += 1
    lua_name = data[pos:pos + lua_len].decode("ascii", errors="replace")
    return {
        "event_sid": event_sid,
        "flag": flag,
        "quest_id": quest_id,
        "header": header_text,
        "buttons": buttons,
        "lua": lua_name,
    }


def parse_c2s_select_msg(data: bytes) -> dict | None:
    """Parse C2S WIZ_SELECT_MSG (opcode 0x55) packet."""
    if len(data) < 4 or data[1] != 0x55:
        return None
    menu_id = data[2]
    lua_len = data[3]
    lua_name = data[4:4 + lua_len].decode("ascii", errors="replace")
    return {"menu_id": menu_id, "lua": lua_name}


def build_sniffer_dialog_tree(conn, lua_filename, session_id=None, initial_event=500):
    """Build EVENT -> SelectMsg mapping from sniffer packets."""
    cur = conn.cursor()
    lua_hex = lua_filename.encode("ascii").hex()
    sfilt = f"AND p.session_id = {session_id}" if session_id else ""

    cur.execute(
        f"SELECT p.id, p.plaintext FROM pkt.packets p "
        f"WHERE p.opcode=85 AND p.direction='S2C' "
        f"AND p.plaintext IS NOT NULL "
        f"AND encode(p.plaintext,'hex') LIKE %s {sfilt} ORDER BY p.id",
        ["%" + lua_hex + "%"],
    )
    s2c_rows = [(r[0], "S2C", parse_s2c_select_msg(bytes(r[1]))) for r in cur.fetchall()]
    s2c_rows = [(i, t, p) for i, t, p in s2c_rows if p]

    cur.execute(
        f"SELECT p.id, p.plaintext FROM pkt.packets p "
        f"WHERE p.opcode=85 AND p.direction='C2S' "
        f"AND p.plaintext IS NOT NULL "
        f"AND encode(p.plaintext,'hex') LIKE %s {sfilt} ORDER BY p.id",
        ["%" + lua_hex + "%"],
    )
    c2s_rows = [(r[0], "C2S", parse_c2s_select_msg(bytes(r[1]))) for r in cur.fetchall()]
    c2s_rows = [(i, t, p) for i, t, p in c2s_rows if p and p["lua"] == lua_filename]

    cur.execute(
        f"SELECT p.id FROM pkt.packets p "
        f"WHERE p.opcode=32 AND p.direction='C2S' "
        f"AND p.plaintext IS NOT NULL {sfilt} ORDER BY p.id",
    )
    npc_rows = [(r[0], "NPC", None) for r in cur.fetchall()]

    all_pkts = sorted(s2c_rows + c2s_rows + npc_rows, key=lambda x: x[0])

    tree = OrderedDict()
    last_s2c = None
    cur_event = initial_event

    for _, ptype, parsed in all_pkts:
        if ptype == "NPC":
            cur_event = initial_event
        elif ptype == "S2C":
            if cur_event not in tree:
                tree[cur_event] = parsed
            last_s2c = parsed
        elif ptype == "C2S":
            if last_s2c and parsed["menu_id"] < len(last_s2c["buttons"]):
                cur_event = last_s2c["buttons"][parsed["menu_id"]][1]
            else:
                cur_event = -1

    return tree


# -----------------------------------------------------------------------
# DB data loaders
# -----------------------------------------------------------------------

def load_quest_helpers(conn, npc_id):
    """Load quest_helper entries for an NPC from DB."""
    cur = conn.cursor()
    cur.execute(
        """
        SELECT n_index, b_message_type, b_level, b_class, b_nation, b_quest_type,
               s_event_data_index, b_event_status, n_event_trigger_index,
               n_event_complete_index, n_exchange_index, n_event_talk_index,
               str_lua_filename, s_quest_menu, s_npc_main, s_quest_solo
        FROM quest_helper WHERE s_npc_id = %s ORDER BY n_index
        """,
        [npc_id],
    )
    cols = [
        "n_index", "msg_type", "level", "cls", "nation", "quest_type",
        "event_data", "event_status", "trigger", "complete", "exchange",
        "talk", "lua", "menu", "npc_main", "quest_solo",
    ]
    return [dict(zip(cols, row)) for row in cur.fetchall()]


def load_quest_monsters(conn):
    """Load quest_monster table: quest_id -> {group_idx: {npc_ids: [...], count: N}}."""
    cur = conn.cursor()
    try:
        cur.execute("SELECT * FROM quest_monster")
    except Exception:
        return {}
    cols = [d[0] for d in cur.description]
    result = {}
    for row in cur.fetchall():
        rd = dict(zip(cols, row))
        quest_num = rd["s_quest_num"]
        groups = []
        for i in range(1, 5):
            npc_ids = [rd[f"s_num{i}{c}"] for c in "abcd" if rd.get(f"s_num{i}{c}", 0) > 0]
            count = rd.get(f"s_count{i}", 0)
            if npc_ids and count > 0:
                groups.append({"npc_ids": npc_ids, "count": count})
        if groups:
            result[quest_num] = groups
    return result


def load_all_npcs_with_helpers(conn):
    """Return list of (npc_id, lua_filename) for all NPCs with quest_helper entries."""
    cur = conn.cursor()
    cur.execute(
        """
        SELECT DISTINCT s_npc_id, str_lua_filename
        FROM quest_helper
        WHERE str_lua_filename != '' AND str_lua_filename IS NOT NULL
        ORDER BY s_npc_id
        """
    )
    # Group by npc_id, pick first non-empty lua filename
    seen = {}
    for npc_id, lua in cur.fetchall():
        if npc_id not in seen:
            seen[npc_id] = lua
    return list(seen.items())


# -----------------------------------------------------------------------
# Quest data analysis
# -----------------------------------------------------------------------

def build_quest_structure(helpers):
    """
    Analyze quest_helper entries and build a structured representation.

    Returns:
        initial_entries: entries with event_data=0 (NPC click router)
        quests: {quest_id: QuestInfo} where QuestInfo has per-status data
    """
    initial = []
    quests = defaultdict(lambda: defaultdict(list))

    for h in helpers:
        qid = h["event_data"]
        status = h["event_status"]
        if qid == 0:
            initial.append(h)
        else:
            quests[qid][status].append(h)

    return initial, dict(quests)


def has_per_class_entries(entries):
    """Check if a set of entries has separate per-class rows (class != 5)."""
    classes = set(e["cls"] for e in entries)
    return len(classes) > 1 or (len(classes) == 1 and 5 not in classes)


def get_class_exchange_map(entries):
    """
    For per-class entries, build {class_id: (n_index, exchange_index)} map.
    Returns None if all entries share the same exchange (no per-class split needed).
    """
    by_cls = {}
    for e in entries:
        by_cls[e["cls"]] = (e["n_index"], e["exchange"])
    exchanges = set(v[1] for v in by_cls.values())
    if len(exchanges) <= 1 and 5 in by_cls:
        return None  # All same exchange, no class split needed
    return by_cls


def pick_representative(entries):
    """Pick a representative entry (prefer class=5/any, else first)."""
    for e in entries:
        if e["cls"] == 5:
            return e
    return entries[0]


# -----------------------------------------------------------------------
# Lua code generation (TBL-based)
# -----------------------------------------------------------------------

class LuaGenerator:
    """Generates Lua quest scripts from quest_helper DB data."""

    def __init__(self, npc_id, lua_filename, helpers, quest_monsters, sniffer_tree=None):
        self.npc_id = npc_id
        self.lua_filename = lua_filename
        self.helpers = helpers
        self.quest_monsters = quest_monsters
        self.sniffer_tree = sniffer_tree or {}
        self.lines = []
        self.generated_events = set()
        self.close_events = set()
        self.initial_entries, self.quests = build_quest_structure(helpers)

    def generate(self) -> str:
        """Main generation entry point."""
        self._emit_header()
        self._emit_initial_router()
        self._emit_quest_handlers()
        self._emit_sniffer_unmatched()
        self._emit_close_handlers()
        return "\n".join(self.lines)

    def _emit(self, line=""):
        self.lines.append(line)

    def _emit_header(self):
        self._emit("local Ret = 0;")
        self._emit(f"local NPC = {self.npc_id};")
        self._emit("")
        source = "quest_helper DB"
        if self.sniffer_tree:
            source += " + sniffer capture"
        self._emit(f"-- Auto-generated by quest_lua_gen.py")
        self._emit(f"-- Source: {source}")
        self._emit(f"-- Lua: {self.lua_filename}")
        self._emit("")

    def _emit_initial_router(self):
        """Emit the initial NPC click handler with SearchQuest routing."""
        if not self.initial_entries:
            return

        trigger = self.initial_entries[0]["trigger"]
        talk_id = self.initial_entries[0].get("talk", 0)

        # Check if there's sniffer data for this event
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Initial NPC click (from sniffer)")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        # Gather all quests that route through SearchQuest (status 2 shares trigger)
        has_quests = bool(self.quests)

        self._emit(f"-- Initial NPC click")
        self._emit(f"if (EVENT == {trigger}) then")
        if has_quests:
            self._emit(f"\tQuestNum = SearchQuest(UID, NPC);")
            self._emit(f"\tif (QuestNum == 0) then")
            # No quests available text
            no_quest_text = talk_id if talk_id > 0 else 0
            self._emit(f"\t\tSelectMsg(UID, 2, -1, {no_quest_text}, NPC, {BTN_OK}, -1);")
            self._emit(f"\telseif (QuestNum > 1 and QuestNum < 100) then")
            busy_text = talk_id if talk_id > 0 else 0
            self._emit(f"\t\tNpcMsg(UID, {busy_text}, NPC);")
            self._emit(f"\telse")
            self._emit(f"\t\tEVENT = QuestNum;")
            self._emit(f"\tend")
        else:
            # Simple dialog NPC - just show a message
            if talk_id > 0:
                self._emit(f"\tSelectMsg(UID, 2, -1, {talk_id}, NPC, {BTN_OK}, -1);")
            else:
                self._emit(f"\tSelectMsg(UID, 2, -1, 0, NPC, {BTN_OK}, -1);")
        self._emit("end")
        self._emit("")
        self.generated_events.add(trigger)

    def _quest_has_unique_triggers(self, qid, statuses):
        """Check if a quest has any triggers not yet generated."""
        for status in statuses:
            for entry in statuses[status]:
                if entry["trigger"] not in self.generated_events:
                    return True
                if entry["complete"] > 0 and entry["complete"] not in self.generated_events:
                    return True
        return False

    def _emit_quest_handlers(self):
        """Emit handlers for each quest, organized by quest ID."""
        for qid in sorted(self.quests.keys()):
            statuses = self.quests[qid]
            # Skip quests where all triggers are already generated (shared trigger NPCs)
            if not self._quest_has_unique_triggers(qid, statuses):
                continue
            self._emit_quest(qid, statuses)

    def _emit_quest(self, qid, statuses):
        """Emit all handlers for a single quest."""
        # Get representative entry for quest metadata
        all_entries = []
        for ents in statuses.values():
            all_entries.extend(ents)
        rep = pick_representative(all_entries)
        level = rep.get("level", 0)
        talk_id = rep.get("talk", 0)
        exchange = rep.get("exchange", 0)
        quest_type = rep.get("quest_type", 1)

        self._emit(f"-- ======= Quest {qid} (level {level}) =======")

        # Process each status in order
        for status in [STATUS_PREQUEST, STATUS_NEW, STATUS_IN_PROGRESS,
                       STATUS_COMPLETABLE, STATUS_COMPLETED, STATUS_ANY]:
            if status not in statuses:
                continue
            entries = statuses[status]
            self._emit_status_handler(qid, status, entries)

        # Emit complete events that haven't been generated yet
        for status in sorted(statuses.keys()):
            for entry in statuses[status]:
                comp = entry["complete"]
                if comp > 0 and comp not in self.generated_events:
                    self._emit_complete_event(qid, status, entry)

    def _emit_status_handler(self, qid, status, entries):
        """Emit handler(s) for a specific quest status."""
        rep = pick_representative(entries)
        trigger = rep["trigger"]
        talk_id = rep.get("talk", 0)
        exchange = rep.get("exchange", 0)
        n_index = rep["n_index"]
        menu = rep.get("menu", 0)

        # Skip if this trigger event was already generated
        if trigger in self.generated_events:
            return

        label = STATUS_LABELS.get(status, f"status={status}")

        if status == STATUS_PREQUEST:
            self._emit_prequest(qid, trigger, entries, label)
        elif status == STATUS_NEW:
            self._emit_new_quest(qid, trigger, entries, label)
        elif status == STATUS_IN_PROGRESS:
            self._emit_in_progress(qid, trigger, entries, label)
        elif status == STATUS_COMPLETABLE:
            self._emit_completable(qid, trigger, entries, label)
        elif status == STATUS_COMPLETED:
            self._emit_completed(qid, trigger, entries, label)
        elif status == STATUS_ANY:
            self._emit_any_status(qid, trigger, entries, label)

    def _emit_prequest(self, qid, trigger, entries, label):
        """Status 255: Pre-quest introduction. Shows dialog then advances to status 0.

        Pattern from hand-written scripts:
        - Trigger 170: SelectMsg with talk_id, then SaveEvent to advance
        - Or just SaveEvent directly if no dialog text
        - Nation-specific entries (nation=1 vs nation=2) show different text
        """
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)

        # Check if sniffer has this event
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        # Find the corresponding status=0 entry for this quest to get its n_index
        status0_entries = self.quests.get(qid, {}).get(STATUS_NEW, [])
        target_n_index = None
        if status0_entries:
            target_n_index = pick_representative(status0_entries)["n_index"]
        # If no status=0, try to use first entry's own n_index
        if not target_n_index:
            target_n_index = rep["n_index"]

        # Check if there are nation-specific 255 entries with different triggers
        # (e.g. trigger 170 for nation 1, trigger 172 for nation 2)
        nation_entries = {}
        for e in entries:
            n = e.get("nation", 3)
            if n in (1, 2):
                nation_entries[n] = e

        # Get the status=0 trigger to use for ShowMap/continue
        status0_trigger = None
        if status0_entries:
            status0_trigger = pick_representative(status0_entries)["trigger"]

        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")

        if talk_id > 0 and status0_trigger:
            # Show quest introduction with Continue/Decline
            self._emit(f"\tSelectMsg(UID, 2, {qid}, {talk_id}, NPC, {BTN_CONTINUE}, {trigger + 1});")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            # Continue handler -> ShowMap + SaveEvent
            accept_evt = trigger + 1
            if accept_evt not in self.generated_events:
                self._emit(f"if (EVENT == {accept_evt}) then")
                self._emit(f"\tShowMap(UID, 4);")
                self._emit(f"\tSaveEvent(UID, {target_n_index});")
                self._emit("end")
                self._emit("")
                self.generated_events.add(accept_evt)
        elif target_n_index:
            # No talk text: just advance directly
            self._emit(f"\tSaveEvent(UID, {target_n_index});")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
        else:
            # No status=0 entry found: show introduction
            if talk_id > 0:
                self._emit(f"\tSelectMsg(UID, 2, {qid}, {talk_id}, NPC, {BTN_OK}, -1);")
            else:
                self._emit(f"\tSelectMsg(UID, 2, {qid}, 0, NPC, {BTN_OK}, -1);")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)

    def _get_status1_n_index(self, qid, status0_entries):
        """Get the status=1 n_index for SaveEvent in accept handlers.

        In KO, accepting a quest advances it from status 0 to status 1,
        so SaveEvent must use the status=1 entry's n_index.
        Falls back to status=0 n_index if no status=1 entry exists.
        """
        status1_entries = self.quests.get(qid, {}).get(STATUS_IN_PROGRESS, [])
        if status1_entries:
            return status1_entries
        return status0_entries

    def _emit_new_quest(self, qid, trigger, entries, label):
        """Status 0: Quest accept handler. Shows quest description and accept/decline."""
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)
        exchange = rep.get("exchange", 0)
        n_index = rep["n_index"]
        menu = rep.get("menu", 0)

        # Check sniffer
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        # Determine if this is a quest with monster sub-quests
        has_monstersub = qid in self.quest_monsters

        # Get the correct entries for SaveEvent (status=1 to advance quest)
        save_entries = self._get_status1_n_index(qid, entries)
        save_rep = pick_representative(save_entries)
        save_n_index = save_rep["n_index"]

        # Accept dialog
        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")

        if has_monstersub:
            # Monster quest: check if sub-quest slot is free
            self._emit(f"\tMonsterSub = ExistMonsterQuestSub(UID);")
            self._emit(f"\tif (MonsterSub == 0) then")
            header = talk_id if talk_id > 0 else 0
            accept_evt = trigger + 1
            self._emit(f"\t\tSelectMsg(UID, 4, {qid}, {header}, NPC, {BTN_ACCEPT}, {accept_evt}, {BTN_REJECT}, -1);")
            self._emit(f"\telse")
            self._emit(f"\t\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_OK}, -1);")
            self._emit(f"\tend")
        else:
            header = talk_id if talk_id > 0 else 0
            accept_evt = trigger + 1
            self._emit(f"\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_ACCEPT}, {accept_evt}, {BTN_REJECT}, -1);")

        self._emit("end")
        self._emit("")
        self.generated_events.add(trigger)

        # Accept confirmation handler -> SaveEvent with status=1 n_index
        accept_evt = trigger + 1
        if accept_evt not in self.generated_events:
            self._emit(f"if (EVENT == {accept_evt}) then")
            self._emit_save_event(save_entries, save_n_index)
            self._emit("end")
            self._emit("")
            self.generated_events.add(accept_evt)

    def _get_reward_entries(self, qid, status1_entries):
        """Get the correct entries for reward/completion SaveEvent.

        Completing a quest advances it from status 1 to status 2,
        so SaveEvent must use the status=2 entry's n_index.
        Falls back to status=1 n_index if no status=2 entry exists.
        """
        status2_entries = self.quests.get(qid, {}).get(STATUS_COMPLETABLE, [])
        if status2_entries:
            return status2_entries
        return status1_entries

    def _emit_in_progress(self, qid, trigger, entries, label):
        """Status 1: Quest in-progress handler. Checks monster/item requirements."""
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)
        complete_evt = rep.get("complete", 0)
        exchange = rep.get("exchange", 0)
        n_index = rep["n_index"]

        # Check sniffer
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        header = talk_id if talk_id > 0 else 0
        has_monsters = qid in self.quest_monsters

        # Get reward entries (status=2) for SaveEvent on completion
        reward_entries = self._get_reward_entries(qid, entries)
        reward_rep = pick_representative(reward_entries)
        reward_n_index = reward_rep["n_index"]

        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")

        if has_monsters:
            monster_groups = self.quest_monsters[qid]
            count_needed = monster_groups[0]["count"]
            show_map_evt = trigger + 1
            reward_evt = trigger + 2

            self._emit(f"\tMonsterCount = CountMonsterQuestSub(UID, {qid}, 1);")
            self._emit(f"\tif (MonsterCount < {count_needed}) then")
            self._emit(f"\t\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_MAP}, {show_map_evt});")
            self._emit(f"\telse")
            self._emit(f"\t\tSelectMsg(UID, 4, {qid}, {header}, NPC, {BTN_REWARD}, {reward_evt}, {BTN_CANCEL}, -1);")
            self._emit(f"\tend")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)

            # ShowMap event
            if show_map_evt not in self.generated_events:
                self._emit(f"if (EVENT == {show_map_evt}) then")
                self._emit(f"\tShowMap(UID, 1);")
                self._emit("end")
                self._emit("")
                self.generated_events.add(show_map_evt)

            # Reward event (complete check + exchange)
            if reward_evt not in self.generated_events:
                self._emit(f"if (EVENT == {reward_evt}) then")
                self._emit(f"\tQuestStatusCheck = GetQuestStatus(UID, {qid});")
                self._emit(f"\tif (QuestStatusCheck == 2) then")
                self._emit(f"\t\tSelectMsg(UID, 2, -1, {header}, NPC, {BTN_OK}, -1);")
                self._emit(f"\telse")
                self._emit(f"\tMonsterCount = CountMonsterQuestSub(UID, {qid}, 1);")
                self._emit(f"\tif (MonsterCount < {count_needed}) then")
                self._emit(f"\t\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_MAP}, {show_map_evt});")
                self._emit(f"\telse")

                # Per-class exchange or single exchange
                reward_class_map = get_class_exchange_map(reward_entries)
                if reward_class_map and len(reward_class_map) > 1:
                    self._emit_class_exchange(reward_class_map, "\t")
                elif exchange > 0:
                    self._emit(f"\tRunQuestExchange(UID, {exchange});")
                    self._emit(f"\tSaveEvent(UID, {reward_n_index});")
                else:
                    self._emit(f"\tSaveEvent(UID, {reward_n_index});")

                self._emit(f"\tend")
                self._emit(f"\tend")
                self._emit("end")
                self._emit("")
                self.generated_events.add(reward_evt)
        else:
            # Non-monster quest progress: generic check
            self._emit(f"\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_OK}, -1);")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)

    def _emit_completable(self, qid, trigger, entries, label):
        """Status 2: Quest completable. Usually routed via SearchQuest (shared trigger)."""
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)
        exchange = rep.get("exchange", 0)
        n_index = rep["n_index"]

        # Check sniffer
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        # Status 2 often shares trigger with initial router (SearchQuest routes here)
        # so trigger might already be generated. If so, we skip it
        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")
        self._emit(f"\tQuestStatusCheck = GetQuestStatus(UID, {qid});")
        self._emit(f"\tif (QuestStatusCheck == 2) then")
        self._emit(f"\t\tSelectMsg(UID, 2, -1, {talk_id if talk_id > 0 else 0}, NPC, {BTN_OK}, -1);")
        self._emit(f"\telse")

        class_map = get_class_exchange_map(entries)
        if class_map and len(class_map) > 1:
            self._emit_class_exchange(class_map, "\t")
        elif exchange > 0:
            self._emit(f"\t\tRunQuestExchange(UID, {exchange});")
            self._emit_save_event(entries, n_index, indent="\t\t")
        else:
            self._emit_save_event(entries, n_index, indent="\t\t")

        self._emit(f"\tend")
        self._emit("end")
        self._emit("")
        self.generated_events.add(trigger)

    def _emit_completed(self, qid, trigger, entries, label):
        """Status 3: Quest completed, show post-completion message."""
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)

        # Check sniffer
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")
        header = talk_id if talk_id > 0 else 0
        self._emit(f"\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_OK}, -1);")
        self._emit("end")
        self._emit("")
        self.generated_events.add(trigger)

    def _emit_any_status(self, qid, trigger, entries, label):
        """Status 4: Any quest state, usually same trigger as status 0."""
        if trigger in self.generated_events:
            return

        rep = pick_representative(entries)
        talk_id = rep.get("talk", 0)

        # Check sniffer
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self._emit(f"-- Quest {qid}, {label}")
            self._emit(f"if (EVENT == {trigger}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(trigger)
            return

        # Status 4 usually shares trigger with another status; if we already generated
        # the trigger, there's nothing extra to do
        self._emit(f"-- Quest {qid}, {label}")
        self._emit(f"if (EVENT == {trigger}) then")
        header = talk_id if talk_id > 0 else 0
        self._emit(f"\tSelectMsg(UID, 2, {qid}, {header}, NPC, {BTN_OK}, -1);")
        self._emit("end")
        self._emit("")
        self.generated_events.add(trigger)

    def _emit_complete_event(self, qid, status, entry):
        """Emit a quest complete event handler for entries with n_event_complete_index.

        The complete event fires AFTER the quest has been advanced.
        If the source is status=1 (in progress), the complete event should
        use status=3 (completed) n_indexes as the target SaveEvent.
        """
        comp = entry["complete"]
        if comp in self.generated_events or comp <= 0:
            return

        exchange = entry.get("exchange", 0)
        talk_id = entry.get("talk", 0)

        # Check sniffer
        if comp in self.sniffer_tree:
            d = self.sniffer_tree[comp]
            self._emit(f"-- Quest {qid} complete event")
            self._emit(f"if (EVENT == {comp}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(comp)
            return

        # Determine the target entries for SaveEvent:
        # Complete event from status=1 should advance to status=3 (completed)
        # Complete event from status=0 should advance to status=1 (in progress)
        if status == STATUS_IN_PROGRESS:
            target_status = STATUS_COMPLETED
        elif status == STATUS_NEW:
            target_status = STATUS_IN_PROGRESS
        else:
            target_status = status + 1

        target_entries = self.quests.get(qid, {}).get(target_status, [])
        if not target_entries:
            # Fall back to the source status entries
            target_entries = self.quests.get(qid, {}).get(status, [])

        target_rep = pick_representative(target_entries)
        target_n_index = target_rep["n_index"]

        self._emit(f"-- Quest {qid} complete event")
        self._emit(f"if (EVENT == {comp}) then")

        # Complete events only SaveEvent (no RunQuestExchange).
        # The exchange is handled in the reward handler, not the complete callback.
        class_map = get_class_exchange_map(target_entries)

        if class_map and len(class_map) > 1:
            self._emit_class_save_event(class_map, "\t")
        else:
            self._emit(f"\tSaveEvent(UID, {target_n_index});")

        self._emit("end")
        self._emit("")
        self.generated_events.add(comp)

    def _emit_save_event(self, entries, n_index, indent="\t"):
        """Emit SaveEvent, handling per-class n_index if needed."""
        class_map = get_class_exchange_map(entries) if len(entries) > 1 else None
        if class_map and len(class_map) > 1:
            self._emit_class_save_event(class_map, indent)
        else:
            self._emit(f"{indent}SaveEvent(UID, {n_index});")

    def _emit_class_exchange(self, class_map, indent):
        """Emit per-class RunQuestExchange + SaveEvent block."""
        self._emit(f"{indent}Class = CheckClass(UID);")
        first = True
        for label, class_ids in CLASS_GROUPS:
            ids_in_map = [cid for cid in class_ids if cid in class_map]
            if not ids_in_map:
                # Try to find a matching entry using the first class of this group
                base_cls = class_ids[0]
                if base_cls in class_map:
                    ids_in_map = [base_cls]

            if not ids_in_map:
                continue

            n_idx, exch = class_map[ids_in_map[0]]
            conditions = " or ".join(f"Class == {c}" for c in class_ids)
            kw = "if" if first else "elseif"
            self._emit(f"{indent}{kw} ({conditions}) then -- {label}")
            if exch > 0:
                self._emit(f"{indent}\tRunQuestExchange(UID, {exch});")
            self._emit(f"{indent}\tSaveEvent(UID, {n_idx});")
            first = False

        if not first:
            self._emit(f"{indent}end")

    def _emit_class_save_event(self, class_map, indent):
        """Emit per-class SaveEvent block (no exchange)."""
        self._emit(f"{indent}Class = CheckClass(UID);")
        first = True
        for label, class_ids in CLASS_GROUPS:
            ids_in_map = [cid for cid in class_ids if cid in class_map]
            if not ids_in_map:
                base_cls = class_ids[0]
                if base_cls in class_map:
                    ids_in_map = [base_cls]

            if not ids_in_map:
                continue

            n_idx, _ = class_map[ids_in_map[0]]
            conditions = " or ".join(f"Class == {c}" for c in class_ids)
            kw = "if" if first else "elseif"
            self._emit(f"{indent}{kw} ({conditions}) then -- {label}")
            self._emit(f"{indent}\tSaveEvent(UID, {n_idx});")
            first = False

        if not first:
            self._emit(f"{indent}end")

    def _emit_sniffer_unmatched(self):
        """Emit sniffer dialog events that don't match any quest_helper entry."""
        if not self.sniffer_tree:
            return
        unmatched = [e for e in self.sniffer_tree
                     if e not in self.generated_events and e != -1]
        if not unmatched:
            return

        self._emit("-- ======= Additional dialogs from sniffer (no quest match) =======")
        for evt in unmatched:
            d = self.sniffer_tree[evt]
            self._emit(f"if (EVENT == {evt}) then")
            self._emit(f"\t{self._format_select_msg(d)}")
            self._emit("end")
            self._emit("")
            self.generated_events.add(evt)

    def _emit_close_handlers(self):
        """Emit close/exit event handlers."""
        self.close_events.add(3001)
        for evt in sorted(self.close_events):
            if evt not in self.generated_events and evt > 0:
                self._emit(f"if (EVENT == {evt}) then")
                self._emit(f"\tRet = 1;")
                self._emit("end")
                self._emit("")

    def _format_select_msg(self, dialog):
        """Format a sniffer SelectMsg dialog as Lua code."""
        btn_args = self._format_btn_args(dialog["buttons"])
        return f"SelectMsg(UID, {dialog['flag']}, {dialog['quest_id']}, {dialog['header']}, NPC, {btn_args});"

    def _format_btn_args(self, buttons):
        """Format button (text_id, event_id) pairs for SelectMsg call."""
        args = []
        for text_id, evt_id in buttons:
            args.append(str(text_id))
            if evt_id == -1:
                args.append("3001")
                self.close_events.add(3001)
            else:
                args.append(str(evt_id))
        return ", ".join(args)


# -----------------------------------------------------------------------
# Event NPC handler (quest_type=4, all triggers same)
# -----------------------------------------------------------------------

class EventNpcGenerator:
    """
    Generates Lua for event NPCs where all quests share the same trigger
    (like NPC 31772 Aset where everything is trigger=100).
    These NPCs are handled entirely by Lua and SearchQuest routing.
    """

    def __init__(self, npc_id, lua_filename, helpers, sniffer_tree=None):
        self.npc_id = npc_id
        self.lua_filename = lua_filename
        self.helpers = helpers
        self.sniffer_tree = sniffer_tree or {}
        self.lines = []

    def generate(self) -> str:
        initial, quests = build_quest_structure(self.helpers)
        trigger = initial[0]["trigger"] if initial else 100

        self.lines.append("local Ret = 0;")
        self.lines.append(f"local NPC = {self.npc_id};")
        self.lines.append("")
        self.lines.append("-- Auto-generated by quest_lua_gen.py")
        self.lines.append(f"-- Event NPC: all quests route through trigger {trigger}")
        self.lines.append(f"-- Lua: {self.lua_filename}")
        self.lines.append(f"-- Quest IDs: {sorted(quests.keys())}")
        self.lines.append("")

        # All events share the same trigger, so just emit the SearchQuest router
        if trigger in self.sniffer_tree:
            d = self.sniffer_tree[trigger]
            self.lines.append(f"if (EVENT == {trigger}) then")
            btn_args = []
            for text_id, evt_id in d["buttons"]:
                btn_args.append(str(text_id))
                btn_args.append(str(evt_id) if evt_id != -1 else "3001")
            self.lines.append(f"\tSelectMsg(UID, {d['flag']}, {d['quest_id']}, {d['header']}, NPC, {', '.join(btn_args)});")
            self.lines.append("end")
            self.lines.append("")
        else:
            self.lines.append(f"if (EVENT == {trigger}) then")
            self.lines.append(f"\tQuestNum = SearchQuest(UID, NPC);")
            self.lines.append(f"\tif (QuestNum == 0) then")
            self.lines.append(f"\t\tSelectMsg(UID, 2, -1, 0, NPC, {BTN_OK}, -1);")
            self.lines.append(f"\telseif (QuestNum > 1 and QuestNum < 100) then")
            self.lines.append(f"\t\tNpcMsg(UID, 0, NPC);")
            self.lines.append(f"\telse")
            self.lines.append(f"\t\tEVENT = QuestNum;")
            self.lines.append(f"\tend")
            self.lines.append("end")
            self.lines.append("")

        # Check for any non-standard triggers
        other_triggers = set()
        for h in self.helpers:
            t = h["trigger"]
            if t != trigger and t > 0:
                other_triggers.add(t)

        for t in sorted(other_triggers):
            if t in self.sniffer_tree:
                d = self.sniffer_tree[t]
                btn_args = []
                for text_id, evt_id in d["buttons"]:
                    btn_args.append(str(text_id))
                    btn_args.append(str(evt_id) if evt_id != -1 else "3001")
                self.lines.append(f"if (EVENT == {t}) then")
                self.lines.append(f"\tSelectMsg(UID, {d['flag']}, {d['quest_id']}, {d['header']}, NPC, {', '.join(btn_args)});")
                self.lines.append("end")
                self.lines.append("")
            else:
                # Find which quest this trigger belongs to
                for h in self.helpers:
                    if h["trigger"] == t:
                        self.lines.append(f"-- Quest {h['event_data']} trigger {t} (status {h['event_status']})")
                        self.lines.append(f"if (EVENT == {t}) then")
                        talk = h.get("talk", 0)
                        self.lines.append(f"\tSelectMsg(UID, 2, {h['event_data']}, {talk if talk > 0 else 0}, NPC, {BTN_OK}, -1);")
                        self.lines.append("end")
                        self.lines.append("")
                        break

        # Emit sniffer unmatched
        generated = {trigger} | other_triggers
        if self.sniffer_tree:
            for evt, d in self.sniffer_tree.items():
                if evt not in generated and evt != -1:
                    btn_args = []
                    for text_id, evt_id in d["buttons"]:
                        btn_args.append(str(text_id))
                        btn_args.append(str(evt_id) if evt_id != -1 else "3001")
                    self.lines.append(f"if (EVENT == {evt}) then")
                    self.lines.append(f"\tSelectMsg(UID, {d['flag']}, {d['quest_id']}, {d['header']}, NPC, {', '.join(btn_args)});")
                    self.lines.append("end")
                    self.lines.append("")

        # Close handler
        self.lines.append("if (EVENT == 3001) then")
        self.lines.append("\tRet = 1;")
        self.lines.append("end")
        self.lines.append("")

        return "\n".join(self.lines)


# -----------------------------------------------------------------------
# Sniffer-only generator (Mode 2)
# -----------------------------------------------------------------------

def generate_sniffer_only_lua(tree, npc_id, lua_filename):
    """Mode 2: generate from sniffer data only."""
    lines = [
        "local Ret = 0;",
        f"local NPC = {npc_id};",
        "",
        "-- Auto-generated from sniffer capture (sniffer-only mode)",
        f"-- Lua: {lua_filename}",
        "",
    ]
    close_events = set()
    for event_id, dialog in tree.items():
        if event_id == -1:
            continue
        btn_args = []
        for text_id, evt_id in dialog["buttons"]:
            btn_args.append(str(text_id))
            if evt_id == -1:
                btn_args.append("3001")
                close_events.add(3001)
            else:
                btn_args.append(str(evt_id))
        lines.append(f"if (EVENT == {event_id}) then")
        lines.append(
            f"\tSelectMsg(UID, {dialog['flag']}, {dialog['quest_id']}, "
            f"{dialog['header']}, NPC, {', '.join(btn_args)});"
        )
        lines.append("end")
        lines.append("")
    for evt in sorted(close_events):
        if evt not in tree:
            lines.append(f"if (EVENT == {evt}) then")
            lines.append("\tRet = 1;")
            lines.append("end")
            lines.append("")
    return "\n".join(lines)


# -----------------------------------------------------------------------
# Classify NPC type
# -----------------------------------------------------------------------

def classify_npc(helpers):
    """
    Classify an NPC based on its quest_helper entries.

    Returns:
        'event_npc' - All/most quests share the same trigger, no real quest logic
                      (event NPCs like 31772 where everything is trigger=100)
        'quest_npc' - Standard quest NPC with varied triggers and quest state machines
        'empty' - No meaningful entries
    """
    if not helpers:
        return "empty"

    triggers = set(h["trigger"] for h in helpers)

    # If all entries share exactly one trigger, it's an event NPC
    if len(triggers) == 1:
        return "event_npc"

    # If most entries share a trigger and all quest entries have zero exchange/talk
    # (i.e. no real quest logic, just SearchQuest routing), it's an event NPC
    from collections import Counter
    trigger_counts = Counter(h["trigger"] for h in helpers)
    most_common_trigger, most_common_count = trigger_counts.most_common(1)[0]

    # Count entries that have non-trivial quest data (exchange, talk, or unique trigger)
    nontrivial = sum(1 for h in helpers
                     if h["exchange"] > 0 or h["talk"] > 0
                     or h["complete"] > 0)

    if most_common_count / len(helpers) > 0.8 and nontrivial < len(helpers) * 0.1:
        return "event_npc"

    return "quest_npc"


# -----------------------------------------------------------------------
# Main
# -----------------------------------------------------------------------

def generate_for_npc(conn, npc_id, lua_filename=None, session_id=None,
                     tbl_only=False, sniffer_only=False, verbose=True):
    """Generate Lua for a single NPC. Returns (lua_code, lua_filename)."""
    # Determine Lua filename
    if not lua_filename:
        cur = conn.cursor()
        cur.execute(
            "SELECT DISTINCT str_lua_filename FROM quest_helper "
            "WHERE s_npc_id = %s AND str_lua_filename != '' LIMIT 1",
            [npc_id],
        )
        row = cur.fetchone()
        lua_filename = row[0] if row else f"{npc_id}_NPC.lua"

    if verbose:
        print(f"  Lua filename: {lua_filename}", file=sys.stderr)

    # Load quest_helper entries
    helpers = []
    if not sniffer_only:
        helpers = load_quest_helpers(conn, npc_id)
        if verbose:
            print(f"  quest_helper: {len(helpers)} entries", file=sys.stderr)

    # Determine initial trigger for sniffer tree
    initial_trigger = 500
    if helpers:
        init = [h for h in helpers if h["event_data"] == 0 and h["event_status"] == 0]
        if init:
            initial_trigger = init[0]["trigger"]

    # Load sniffer dialog tree
    sniffer_tree = None
    if not tbl_only:
        if verbose:
            print(f"  Fetching sniffer data for {lua_filename}...", file=sys.stderr)
        sniffer_tree = build_sniffer_dialog_tree(conn, lua_filename, session_id, initial_trigger)
        if sniffer_tree:
            if verbose:
                print(f"  {len(sniffer_tree)} dialog events from sniffer", file=sys.stderr)
        else:
            sniffer_tree = None
            if verbose:
                print(f"  No sniffer dialog data found", file=sys.stderr)

    # Generate Lua
    if sniffer_only:
        if not sniffer_tree:
            if verbose:
                print("ERROR: No sniffer data found", file=sys.stderr)
            return None, lua_filename
        return generate_sniffer_only_lua(sniffer_tree, npc_id, lua_filename), lua_filename

    if not helpers:
        if verbose:
            print(f"  WARNING: No quest_helper entries for NPC {npc_id}", file=sys.stderr)
        if sniffer_tree:
            return generate_sniffer_only_lua(sniffer_tree, npc_id, lua_filename), lua_filename
        return None, lua_filename

    # Load quest_monster data
    quest_monsters = load_quest_monsters(conn)

    # Classify NPC type
    npc_type = classify_npc(helpers)
    if verbose:
        print(f"  NPC type: {npc_type}", file=sys.stderr)

    if npc_type == "event_npc":
        gen = EventNpcGenerator(npc_id, lua_filename, helpers, sniffer_tree)
    else:
        gen = LuaGenerator(npc_id, lua_filename, helpers, quest_monsters, sniffer_tree)

    return gen.generate(), lua_filename


def sync_quest_helper_from_tbl(conn, tbl_path: str, npc_filter: int | None = None):
    """
    Import Quest_Helper.tbl into quest_helper DB table via UPSERT.

    .tbl has 21 columns, DB has 19. Column mapping:
      tbl[0]  → n_index              tbl[11] → s_event_data_index
      tbl[1]  → b_message_type       tbl[12] → b_event_status
      tbl[2]  → b_level              tbl[13] → n_event_trigger_index
      tbl[3]  → n_exp                tbl[14] → n_event_complete_index
      tbl[4]  → SKIP                 tbl[15] → n_exchange_index
      tbl[5]  → SKIP                 tbl[16] → n_event_talk_index
      tbl[6]  → b_class              tbl[17] → str_lua_filename
      tbl[7]  → b_nation             tbl[18] → s_quest_menu
      tbl[8]  → b_quest_type         tbl[19] → s_npc_main
      tbl[9]  → b_zone               tbl[20] → s_quest_solo
      tbl[10] → s_npc_id
    """
    import subprocess, tempfile

    # Step 1: Export .tbl to SQL using ko-tbl-import
    print(f"Decrypting {tbl_path}...", file=sys.stderr)
    sql_tmp = os.path.join(tempfile.gettempdir(), "quest_helper_tbl_export.sql")
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    result = subprocess.run(
        ["cargo", "run", "-p", "ko-tbl-import", "--", "--file", tbl_path, "--sql-out", sql_tmp],
        cwd=project_root, capture_output=True, text=True, timeout=120,
    )
    if result.returncode != 0:
        print(f"ko-tbl-import failed: {result.stderr}", file=sys.stderr)
        sys.exit(1)

    # Step 2: Parse the SQL export
    rows = []
    with open(sql_tmp, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line.startswith("("):
                continue
            line = line.rstrip(",").rstrip(";")
            # Parse row values handling quoted strings
            inner = line.strip("()")
            parts = []
            in_quote = False
            current = ""
            for ch in inner:
                if ch == "'" and not in_quote:
                    in_quote = True; current += ch
                elif ch == "'" and in_quote:
                    in_quote = False; current += ch
                elif ch == "," and not in_quote:
                    parts.append(current.strip()); current = ""
                else:
                    current += ch
            parts.append(current.strip())

            if len(parts) < 21:
                continue

            # Map tbl columns to DB columns (skip col[4] and col[5])
            n_index = int(parts[0])
            s_npc_id = int(parts[10])

            if npc_filter and s_npc_id != npc_filter:
                continue

            row = (
                n_index,                    # n_index
                int(parts[1]),              # b_message_type
                int(parts[2]),              # b_level
                int(parts[3]),              # n_exp
                int(parts[6]),              # b_class
                int(parts[7]),              # b_nation
                int(parts[8]),              # b_quest_type
                int(parts[9]),              # b_zone
                s_npc_id,                   # s_npc_id
                int(parts[11]),             # s_event_data_index
                int(parts[12]),             # b_event_status
                int(parts[13]),             # n_event_trigger_index
                int(parts[14]),             # n_event_complete_index
                int(parts[15]),             # n_exchange_index
                int(parts[16]),             # n_event_talk_index
                parts[17].strip("'"),       # str_lua_filename
                int(parts[18]),             # s_quest_menu
                int(parts[19]),             # s_npc_main
                int(parts[20]),             # s_quest_solo
            )
            rows.append(row)

    print(f"Parsed {len(rows)} rows from .tbl" +
          (f" (NPC {npc_filter} only)" if npc_filter else ""), file=sys.stderr)

    if not rows:
        print("No rows to import", file=sys.stderr)
        return

    # Step 3: UPSERT into DB
    cur = conn.cursor()
    cols = ("n_index, b_message_type, b_level, n_exp, b_class, b_nation, "
            "b_quest_type, b_zone, s_npc_id, s_event_data_index, b_event_status, "
            "n_event_trigger_index, n_event_complete_index, n_exchange_index, "
            "n_event_talk_index, str_lua_filename, s_quest_menu, s_npc_main, s_quest_solo")
    update_set = ", ".join(
        f"{c}=EXCLUDED.{c}" for c in cols.split(", ") if c != "n_index"
    )
    placeholders = ", ".join(["%s"] * 19)

    batch_size = 100
    inserted = 0
    for i in range(0, len(rows), batch_size):
        batch = rows[i:i + batch_size]
        values_list = ", ".join(
            cur.mogrify(f"({placeholders})", row).decode() for row in batch
        )
        sql = (f"INSERT INTO quest_helper ({cols}) VALUES {values_list} "
               f"ON CONFLICT (n_index) DO UPDATE SET {update_set}")
        cur.execute(sql)
        inserted += len(batch)

    conn.commit()

    # Verify
    if npc_filter:
        cur.execute("SELECT count(*) FROM quest_helper WHERE s_npc_id = %s", [npc_filter])
    else:
        cur.execute("SELECT count(*) FROM quest_helper")
    total = cur.fetchone()[0]

    print(f"UPSERT complete: {inserted} rows processed, {total} total in DB" +
          (f" for NPC {npc_filter}" if npc_filter else ""), file=sys.stderr)


def main():
    parser = argparse.ArgumentParser(
        description="Generate quest Lua from .tbl data and/or sniffer"
    )
    parser.add_argument("--npc", type=int, help="NPC proto ID (e.g. 31772)")
    parser.add_argument("--lua", help="Lua filename override")
    parser.add_argument("--dsn", default=DEFAULT_DSN, help="PostgreSQL DSN")
    parser.add_argument("--session", type=int, default=None, help="Sniffer session ID")
    parser.add_argument("--sniffer-only", action="store_true", help="Sniffer data only")
    parser.add_argument("--tbl-only", action="store_true", help="TBL data only (no sniffer)")
    parser.add_argument("--out", default=None, help="Output file path")
    parser.add_argument("--out-dir", default=None, help="Output directory (with --all)")
    parser.add_argument("--all", action="store_true", help="Generate for all NPCs")
    parser.add_argument("--sync-db", action="store_true",
                        help="Import Quest_Helper.tbl into quest_helper DB table (UPSERT)")
    parser.add_argument("--tbl-file", default=DEFAULT_TBL_PATH,
                        help="Path to Quest_Helper.tbl (for --sync-db)")
    parser.add_argument("--sync-npc", type=int, default=None,
                        help="Only sync entries for this NPC (with --sync-db)")
    args = parser.parse_args()

    if not args.npc and not args.lua and not args.all and not args.sync_db:
        parser.error("--npc, --lua, --all, or --sync-db required")

    conn = psycopg2.connect(args.dsn)

    if args.sync_db:
        sync_quest_helper_from_tbl(conn, args.tbl_file, args.sync_npc)
        conn.close()
        return

    if args.all:
        # Batch mode: generate for all NPCs
        npcs = load_all_npcs_with_helpers(conn)
        out_dir = args.out_dir or "Quests"
        os.makedirs(out_dir, exist_ok=True)
        print(f"Generating Lua for {len(npcs)} NPCs into {out_dir}/", file=sys.stderr)
        generated = 0
        skipped = 0
        for npc_id, lua_filename in npcs:
            print(f"\nNPC {npc_id} ({lua_filename}):", file=sys.stderr)
            lua_code, fname = generate_for_npc(
                conn, npc_id, lua_filename,
                session_id=args.session,
                tbl_only=args.tbl_only,
                sniffer_only=args.sniffer_only,
            )
            if lua_code:
                out_path = os.path.join(out_dir, fname)
                # Don't overwrite existing hand-written files
                if os.path.exists(out_path):
                    out_path = os.path.join(out_dir, f"gen_{fname}")
                with open(out_path, "w", encoding="utf-8") as f:
                    f.write(lua_code)
                print(f"  -> {out_path}", file=sys.stderr)
                generated += 1
            else:
                print(f"  -> SKIPPED (no data)", file=sys.stderr)
                skipped += 1
        print(f"\nDone: {generated} generated, {skipped} skipped", file=sys.stderr)
        conn.close()
        return

    # Single NPC mode
    if args.lua and not args.npc:
        # Extract NPC ID from lua filename
        parts = args.lua.split("_")
        try:
            args.npc = int(parts[0])
        except (ValueError, IndexError):
            parser.error("Cannot extract NPC ID from --lua filename, use --npc")

    print(f"NPC {args.npc}:", file=sys.stderr)
    lua_code, lua_filename = generate_for_npc(
        conn, args.npc, args.lua,
        session_id=args.session,
        tbl_only=args.tbl_only,
        sniffer_only=args.sniffer_only,
    )

    if not lua_code:
        print("ERROR: No data to generate from", file=sys.stderr)
        conn.close()
        sys.exit(1)

    if args.out:
        os.makedirs(os.path.dirname(args.out) or ".", exist_ok=True)
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(lua_code)
        print(f"Written to {args.out}", file=sys.stderr)
    else:
        print(lua_code)

    conn.close()


if __name__ == "__main__":
    main()
