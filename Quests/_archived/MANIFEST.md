# Archived Quest Lua Files

Moved here because they are orphans (not referenced by any quest_helper entry)
and confirmed safe to archive.

## How to Restore
```bash
# Restore a single file:
mv Quests/_archived/<filename> Quests/

# Restore all:
mv Quests/_archived/*.lua Quests/
```

## Archived Files (Sprint 638)

| File | Reason | Original |
|------|--------|----------|
| `TheThyke.lua` | Empty (0 bytes) | - |
| `20510_Victory.lua` | Only contains "DELETED" text | - |
| `3300_quest - Orj.lua` | Exact duplicate of `3300_quest.lua` (diff=0) | `3300_quest.lua` |
| `13016_Keite1.lua` | Old version of `13016_keite.lua` (135 vs 216 lines) | `13016_keite.lua` |
| `19073_Shojin - V2.lua` | Old "V2" version of `19073_Shojin.lua` | `19073_Shojin.lua` |

## NOT Archived (58 files with active content)

These orphan Lua files have manual handlers and may be used by event/system NPCs:
- 8 SPELLI files (class-specific quest branches)
- 16 Voyeur files (observation/zone-change NPCs)
- 10 Prison files (TBL references with different filename format)
- 5 Karus mirror files (15424_dela, 15438_Beldan, etc.)
- 4 TheThyke kill quest files
- 12 Event/system NPCs (SkillOpener, NTSJOB, CastleWar, Pontus, etc.)
- 3 Other (3300_quest, 3500_quest, Chef)
