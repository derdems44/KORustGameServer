# KO Packet Analysis & NPC Dialog Tools

Reverse-engineering and development tools for Knight Online server protocol analysis.
All tools share the `lib/` module for crypto, opcode lookup, and database access.

## Prerequisites

```bash
pip install psycopg2-binary pycryptodome
```

All tools (except `quest_lua_gen.py`) access the sniffer database via `docker exec ko-postgres psql`.
Make sure the `ko-postgres` container is running.

`quest_lua_gen.py` connects directly via psycopg2 (requires `--dsn` argument).

## Database Schema

### Sniffer Schema (`pkt.*`)

These tables are populated by the external packet sniffer (`ko_sniffer.py`).

```
pkt.sessions
  id             SERIAL PRIMARY KEY
  client_ip      TEXT
  server_ip      TEXT
  server_port    INT
  server_type    TEXT        -- 'login' or 'game'
  aes_key_ascii  TEXT        -- 16-char ASCII AES key from 0x2B handshake
  created_at     TIMESTAMPTZ

pkt.packets
  id             SERIAL PRIMARY KEY
  session_id     INT REFERENCES pkt.sessions(id)
  seq            INT           -- packet sequence number
  direction      TEXT          -- 'C2S' or 'S2C'
  encrypted      BOOLEAN
  opcode         INT
  opcode_name    TEXT
  raw_wire       BYTEA         -- full wire bytes [AA55]...[55AA]
  plaintext      BYTEA         -- decrypted payload (if available)
  payload_len    INT

pkt.crypto_keys
  id             SERIAL PRIMARY KEY
  session_id     INT REFERENCES pkt.sessions(id)
  key_ascii      TEXT          -- AES key string
  captured_at    TIMESTAMPTZ
```

### Game Schema (selected tables used by tools)

```
npc_template
  s_sid            INT PRIMARY KEY   -- NPC proto ID
  str_name         TEXT
  by_group         INT               -- 0=not clickable, 3=clickable
  i_selling_group  INT               -- links to item_sell_table

quest_helper
  n_index                INT PRIMARY KEY
  b_level                INT          -- min level required
  b_class                INT          -- 1=War, 2=Rog, 3=Mag, 4=Pri, 5=All
  b_nation               INT          -- 1=Karus, 2=ElMorad, 3=Both
  b_quest_type           INT
  s_npc_id               INT          -- NPC proto ID
  s_event_data_index     INT          -- quest ID in user_quest
  b_event_status         INT          -- 0=not started, 1=ongoing, 2=done, 255=always
  n_event_trigger_index  INT          -- EVENT value for Lua
  n_event_complete_index INT
  n_exchange_index       INT          -- item_exchange recipe
  n_event_talk_index     INT          -- SelectMsg header text ID
  str_lua_filename       TEXT
  s_quest_menu           INT          -- quest_menu text ID

quest_menu
  n_index   INT PRIMARY KEY
  str_talk  TEXT              -- button label text

quest_talk
  n_index   INT PRIMARY KEY
  str_talk  TEXT              -- NPC speech / dialog text
```

---

## Tool Reference

### 1. packet_analyzer.py -- Session Overview

Shows opcode frequency, category groups, unknown detection per session.

```bash
# List all sniffer sessions
python tools/packet_analyzer.py --sessions

# Analyze a session (opcode frequency table)
python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X

# Group by 17 protocol categories
python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --groups

# Show only unhandled/unknown opcodes
python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --unknown

# Export as JSON
python tools/packet_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --json
```

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID |
| `--key KEY` | AES key (16-char ASCII from `pkt.sessions.aes_key_ascii`) |
| `--sessions` | List available sessions |
| `--groups` | Aggregate by category |
| `--unknown` | Show only unknown opcodes |
| `--json` | JSON output |

---

### 2. flow_analyzer.py -- Multi-Packet Flow Tracing

Detects named packet sequences (login, game entry, NPC dialog, combat, trade, zone change)
by matching ordered flow templates against decrypted session data.

```bash
# Show all detected flows
python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X

# Trace a specific flow pattern
python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --flow game_entry

# Include hex dump of matched packets
python tools/flow_analyzer.py --session 37 --key 57UWLK49ALRO1C5X --flow npc_dialog --hex
```

Built-in flow templates:
- `game_entry` -- VERSION_CHECK -> LOGIN -> ALLCHAR -> SEL_CHAR -> GAMESTART -> MYINFO
- `npc_dialog` -- NPC_EVENT -> SELECT_MSG -> QUEST cycles
- `login` -- LS_LOGIN_REQ -> LS_LOGIN_ACK -> LS_SERVERLIST
- `combat` -- ATTACK sequences with HP_CHANGE
- `trade` -- EXCHANGE request/accept/confirm
- `zone_change` -- ZONE_CHANGE -> GAMESTART -> MYINFO

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID |
| `--key KEY` | AES key |
| `--flow NAME` | Specific flow to trace (omit for all) |
| `--hex` | Show hex dump of matched packets |

---

### 3. struct_extractor.py -- Packet Format Reverse Engineering

Compares multiple instances of the same opcode to find field boundaries
(constant vs variable bytes, likely field types).

```bash
# Analyze a specific opcode
python tools/struct_extractor.py --session 37 --key 57UWLK49ALRO1C5X --opcode 0x55 --dir S2C

# Scan all opcodes with 5+ samples
python tools/struct_extractor.py --session 37 --key 57UWLK49ALRO1C5X --all --min-samples 5
```

Algorithm:
1. Collect all packets matching (opcode, direction)
2. Group by plaintext length
3. Compare byte-by-byte: constant bytes = flags/opcodes, variable = fields
4. Guess field types: 1 var byte = u8, 2 consecutive = u16le, 4 = u32le

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID |
| `--key KEY` | AES key |
| `--opcode 0xNN` | Target opcode (hex) |
| `--dir C2S/S2C` | Packet direction |
| `--all` | Analyze all opcodes |
| `--min-samples N` | Minimum samples to analyze (default: 3) |

---

### 4. session_diff.py -- Original vs Custom Server Comparison

Compares two sniffer sessions by opcode frequency. Reports missing, extra,
and count-different opcodes.

```bash
python tools/session_diff.py --original 37 --ours 38 --key1 KEY1 --key2 KEY2
```

Output columns:
- **Missing** -- original server sends but we don't
- **Extra** -- we send but original doesn't
- **Different** -- both have but >30% count difference

| Flag | Description |
|------|-------------|
| `--original ID` | Original server session ID |
| `--ours ID` | Our server session ID |
| `--key1 KEY` | AES key for original session |
| `--key2 KEY` | AES key for our session |

---

### 5. dialog_builder.py -- NPC Dialog Lua Generator (v4)

Action-aware NPC dialog Lua generator from sniffer captures.
Parses SelectMsg flag bytes to detect SHOP, EXCHANGE, DISASSEMBLE, WARP, MALL actions.

```bash
# Preview all NPC dialogs in a session
python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X

# Generate Lua for a specific NPC
python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --npc 29235

# Batch generate to directory
python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --out-dir Quests/

# Show progress (coverage %)
python tools/dialog_builder.py --session 37 --key 57UWLK49ALRO1C5X --progress
```

Action flags detected:

| Flag | Action | Lua binding generated |
|------|--------|----------------------|
| 9 | Pet Shop | `OpenTradeNpc(UID)` |
| 14 | Pet Random | close event |
| 18 | Item Exchange | `RunExchange(UID, ...)` |
| 21 | NPC Shop | `OpenTradeNpc(UID)` |
| 27 | Disassemble | close event |
| 70 | Special UI | close event |

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID |
| `--key KEY` | AES key |
| `--npc PROTO_ID` | Single NPC to generate |
| `--out-dir DIR` | Output directory for Lua files |
| `--progress` | Show coverage statistics |

DB tables read: `npc_template`, `quest_helper`, `quest_menu`, `quest_talk`

---

### 6. dialog_monitor.py -- Real-Time Dialog Coverage Monitor

Live-updating tree view of NPC dialog coverage. Shows which buttons
have been clicked and which are still missing. Refreshes every 1 second.

```bash
python tools/dialog_monitor.py --session 60 --key Z4Y2UHHZN4L9AY2A

# Monitor a specific NPC only
python tools/dialog_monitor.py --session 60 --key Z4Y2UHHZN4L9AY2A --npc 29235
```

Display:
- Green: button clicked / dialog captured
- Red: button not yet clicked
- Completed NPCs are hidden (only missing shown)
- Button labels from `quest_menu`, dialog text from `quest_talk`

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID (live/active) |
| `--key KEY` | AES key |
| `--npc PROTO_ID` | Monitor single NPC |

---

### 7. dialog_tree_builder.py -- Dialog Tree Visualization

Builds complete NPC dialog trees from sniffer captures. Shows parent-child
button relationships and generates Lua scripts.

```bash
# Show dialog trees for all NPCs
python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X

# Single NPC tree
python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X --npc 29235

# Generate Lua to directory
python tools/dialog_tree_builder.py --session 37 --key 57UWLK49ALRO1C5X --out-dir Quests/sniffer
```

| Flag | Description |
|------|-------------|
| `--session ID` | Sniffer session ID |
| `--key KEY` | AES key |
| `--npc PROTO_ID` | Single NPC |
| `--out-dir DIR` | Output Lua directory |

---

### 8. quest_lua_gen.py -- Quest Lua Generator from TBL Data

Generates NPC quest Lua scripts from `quest_helper` database table and/or
sniffer captures. Supports TBL-only, sniffer-only, and merged modes.

```bash
# Sync Quest_Helper.tbl to database (all 11,399 rows)
python tools/quest_lua_gen.py --sync-db --dsn "host=localhost port=5432 ..."

# Sync single NPC
python tools/quest_lua_gen.py --sync-npc 31772 --dsn "..."

# Generate Lua for single NPC (merged mode)
python tools/quest_lua_gen.py --npc 31772 --dsn "..." --out Quests/31772_Aset.lua

# Generate all NPCs
python tools/quest_lua_gen.py --all --dsn "..." --out-dir Quests/

# TBL-only mode (no sniffer data)
python tools/quest_lua_gen.py --npc 13013 --dsn "..." --tbl-only

# Sniffer-only mode
python tools/quest_lua_gen.py --npc 31772 --dsn "..." --sniffer-only
```

| Flag | Description |
|------|-------------|
| `--npc PROTO_ID` | Single NPC to generate |
| `--all` | Generate all NPCs |
| `--dsn DSN` | PostgreSQL connection string |
| `--out FILE` | Output file path |
| `--out-dir DIR` | Output directory |
| `--tbl-only` | Use only quest_helper DB data |
| `--sniffer-only` | Use only sniffer capture data |
| `--sync-db` | Import Quest_Helper.tbl to DB |
| `--sync-npc ID` | Import single NPC from TBL |
| `--tbl-path PATH` | Quest_Helper.tbl file path |

DB tables read/write: `quest_helper` (read/write), `quest_menu`, `quest_talk`, `pkt.packets`

---

### 9. parse_myinfo.py -- MyInfo Packet Parser

Parses a binary MyInfo (0x0E) packet dump and prints every field with offset.
Useful for verifying the server's myinfo builder against original captures.

```bash
python tools/parse_myinfo.py
```

Reads `captures/myinfo_original.bin` by default. Edit the script to change path.

---

## Shared Library (`lib/`)

### lib/ko_crypto.py

AES-128-CBC decryption for Knight Online wire protocol.

```python
from tools.lib.ko_crypto import decrypt_wire_packet, aes_decrypt

# Decrypt a full wire packet [AA55]...[55AA]
result = decrypt_wire_packet(raw_bytes, aes_key_bytes, "C2S")
if result:
    opcode, plaintext = result

# Raw AES decrypt
plaintext = aes_decrypt(key_bytes, ciphertext)
```

Constants:
- `AES_IV` = `32 4E AA 58 BC B3 AE E3 6B C7 4C 56 36 47 34 F2`
- `AES_FLAG` = `0x01` (encrypted packet marker)

Wire format:
```
[AA 55] [payload_len:u16le] [flag] [payload...] [55 AA]
  flag=0x01 -> AES encrypted
  C2S: plaintext[0]=xor_seq, plaintext[1]=opcode
  S2C: plaintext[0]=opcode
```

### lib/opcodes.py

Opcode-to-name registry for ~130 game opcodes and ~20 login opcodes.
Provides 17-category classification.

```python
from tools.lib.opcodes import get_name, get_category, CATEGORIES

get_name(0x0E)        # -> "WIZ_MYINFO"
get_category(0x0E)    # -> "Core"
```

Categories: Core, Character, Movement, Combat, Magic, Items, Trade, Social, Party, Knights, Quest, Zone, NPC, Events, System, Bot, Unknown

### lib/db.py

Database access layer via `docker exec ko-postgres psql`.

```python
from tools.lib.db import get_sessions, decrypt_session

# List sessions
sessions = get_sessions(limit=20)

# Decrypt all packets in a session
packets = decrypt_session(session_id=37, aes_key="57UWLK49ALRO1C5X")
for pkt in packets:
    print(f"{pkt.direction} {pkt.opcode_name} ({len(pkt.plaintext)} bytes)")
```

Container requirement: `ko-postgres` Docker container running with `koserver` user.

---

## Typical Workflow

```
1. Start sniffer       -> populates pkt.sessions, pkt.packets
2. packet_analyzer.py  -> overview of what opcodes were captured
3. flow_analyzer.py    -> trace specific protocol flows
4. struct_extractor.py -> reverse-engineer unknown packet formats
5. dialog_builder.py   -> generate NPC dialog Lua scripts
6. dialog_monitor.py   -> live-track coverage while clicking NPCs
7. session_diff.py     -> compare original vs custom server
8. quest_lua_gen.py    -> generate quest Lua from TBL + sniffer
```
