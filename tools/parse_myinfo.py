#!/usr/bin/env python3
"""Parse original server MyInfo binary and verify every field."""
import struct, sys

with open('captures/myinfo_original.bin', 'rb') as f:
    data = f.read()

pos = 0
def r(fmt, sz, name):
    global pos
    v = struct.unpack_from(fmt, data, pos)[0]
    if v != 0:
        print(f'  [{pos:4d}] {name:25s} {sz}B = {v}')
    pos += sz
    return v

def r_sbyte(name):
    global pos
    slen = data[pos]; pos += 1
    s = data[pos:pos+slen]; pos += slen
    print(f'  [{pos-1-slen:4d}] {name:25s} {1+slen}B = "{s.decode("ascii",errors="replace")}"')
    return s

def r_item(prefix):
    global pos
    iid = struct.unpack_from('<I', data, pos)[0]
    dur = struct.unpack_from('<h', data, pos+4)[0]
    cnt = struct.unpack_from('<H', data, pos+6)[0]
    if iid > 0:
        print(f'  [{pos:4d}] {prefix:25s} id={iid} dur={dur} cnt={cnt}')
    pos += 19
    return iid

print(f'Total: {len(data)} bytes\n')

# Section 1: Identity
r('<B', 1, 'opcode')
r('<I', 4, 'socketID')
r_sbyte('name')
r('<H', 2, 'posX')
r('<H', 2, 'posZ')
r('<H', 2, 'posY')
r('<B', 1, 'nation')
r('<B', 1, 'race')
r('<h', 2, 'class')
r('<B', 1, 'hairColor')
r('<I', 4, 'hairPacked')
r('<B', 1, 'face')
r('<B', 1, 'title2')
r('<B', 1, 'title1')
r('<B', 1, 'rank')
r('<B', 1, 'level')
r('<h', 2, 'statPoints')
r('<q', 8, 'maxExp')
r('<q', 8, 'curExp')
r('<I', 4, 'loyalty')
r('<I', 4, 'loyaltyMonthly')

# Clan section
clan_id = r('<h', 2, 'clanID')
r('<B', 1, 'clanFame')

print(f'\n*** CLAN SECTION at {pos}, clanID={clan_id}')
print(f'*** Next 20 bytes: {data[pos:pos+20].hex()}')

if clan_id > 0:
    print('  HAS CLAN - variable length')
    # Would need to parse clan data here
else:
    # No clan - check what comes next
    # Our current assumption: 14 bytes padding (u64=0, u16=0xFFFF, u32=0)
    pad = data[pos:pos+14]
    print(f'  NO CLAN padding(14): {pad.hex()}')
    pos += 14

print(f'\n*** After clan: pos={pos}')
print(f'*** Next 20 bytes: {data[pos:pos+20].hex()}')

# 8 unknown
unk8 = data[pos:pos+8]
print(f'  [{pos:4d}] unk8                      8B = {unk8.hex()}')
pos += 8

# HP/MP
r('<h', 2, 'maxHP')
r('<h', 2, 'curHP')
r('<h', 2, 'maxMP')
r('<h', 2, 'curMP')
r('<I', 4, 'maxWeight')
r('<I', 4, 'itemWeight')

for s in ['STR','STA','DEX','INT','CHA']:
    r('<B', 1, f'base{s}')
    r('<B', 1, f'bonus{s}')

r('<H', 2, 'totalHit')
r('<H', 2, 'totalAC')
for res in ['fire','cold','light','magic','disease','poison']:
    r('<B', 1, f'{res}R')

r('<I', 4, 'gold')
r('<B', 1, 'authority')
r('<b', 1, 'knightsRank')
r('<b', 1, 'personalRank')

for i in range(9):
    r('<B', 1, f'skill{i}')

print(f'\n*** INVENTORY at {pos} ***')

# Phase 1: 14 equip
for i in range(14): r_item(f'equip[{i}]')
print(f'  After equip: {pos}')

# Phase 2: 28 bag
for i in range(28): r_item(f'bag[{i}]')
print(f'  After bag: {pos}')

# Phase 3: 9 cospre
for i in range(9): r_item(f'cospre[{i}]')
print(f'  After cospre(9): {pos}')

# Phase 4: 3 special
for i in range(3): r_item(f'special[{i}]')
print(f'  After special(3): {pos}')

# Phase 5: 36 mbag
for i in range(36): r_item(f'mbag[{i}]')
print(f'  After mbag(36): {pos}')

remaining = len(data) - pos
print(f'\n*** AFTER 90 ITEMS: pos={pos}, remaining={remaining} ***')
print(f'*** Next 20 bytes: {data[pos:pos+20].hex()}')

# Try parsing as: 4 extra items + trailer (43 bytes) = 76 + 43 = 119
if remaining == 119:
    print('*** 119 = 76(4 items) + 43(trailer) - MATCH! ***')
    for i in range(4): r_item(f'extra[{i}]')

# Try parsing as: just trailer (43 bytes)
elif remaining == 43:
    print('*** 43 = trailer only - MATCH! ***')

else:
    print(f'*** UNEXPECTED remaining={remaining} ***')
    # Try 4 extras anyway
    if remaining > 76:
        print('  Trying 4 extra items...')
        for i in range(4): r_item(f'extra[{i}]')

# Trailer
print(f'\n*** TRAILER at {pos}, remaining={len(data)-pos} ***')
r('<B', 1, 'accountStatus')
prem_cnt = r('<B', 1, 'premiumCount')
for i in range(prem_cnt):
    r('<B', 1, f'premType[{i}]')
    r('<h', 2, f'premTime[{i}]')
r('<B', 1, 'activePremType')
r('<B', 1, 'collRaceEnabled')
r('<I', 4, 'returnSymbolOK')
for i in range(5): r('<B', 1, f'skillSave[{i}]')
r('<B', 1, 'petType')
r('<h', 2, 'genieTime')
r('<B', 1, 'rebLevel')
for s in ['STR','STA','DEX','INT','CHA']: r('<B', 1, f'reb{s}')
r('<q', 8, 'sealedExp')
r('<h', 2, 'coverTitle')
r('<h', 2, 'skillTitle')
r('<I', 4, 'mannerPoint')
r('<B', 1, 'premInUse')
r('<B', 1, 'hidingHelmet')
r('<B', 1, 'unkUI1')
r('<B', 1, 'unkUI2')
r('<B', 1, 'hidingCospre')

print(f'\n*** FINAL: {pos}/{len(data)} bytes ***')
if pos != len(data):
    print(f'*** {len(data)-pos} BYTES REMAINING: {data[pos:].hex()} ***')
