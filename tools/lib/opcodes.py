"""Shared opcode registry for Knight Online v2603 packet tools.

Provides opcode-to-name mappings for game and login servers,
17-category classification, and reverse lookups.
"""

# ---------------------------------------------------------------------------
# Game server opcodes  (~130 entries, v2603)
# ---------------------------------------------------------------------------
GAME_OPCODES: dict[int, str] = {
    0x01: "WIZ_LOGIN", 0x02: "WIZ_NEW_CHAR", 0x03: "WIZ_DEL_CHAR",
    0x04: "WIZ_SEL_CHAR", 0x05: "WIZ_SEL_NATION", 0x06: "WIZ_MOVE",
    0x07: "WIZ_USER_INOUT", 0x08: "WIZ_ATTACK", 0x09: "WIZ_ROTATE",
    0x0A: "WIZ_NPC_INOUT", 0x0B: "WIZ_NPC_MOVE", 0x0C: "WIZ_ALLCHAR_INFO_REQ",
    0x0D: "WIZ_GAMESTART", 0x0E: "WIZ_MYINFO", 0x0F: "WIZ_LOGOUT",
    0x10: "WIZ_CHAT", 0x11: "WIZ_DEAD", 0x12: "WIZ_REGENE",
    0x13: "WIZ_TIME", 0x14: "WIZ_WEATHER", 0x15: "WIZ_REGIONCHANGE",
    0x16: "WIZ_REQ_USERIN", 0x17: "WIZ_HP_CHANGE", 0x18: "WIZ_MP_CHANGE",
    0x19: "WIZ_NATION_CHAT", 0x1A: "WIZ_EXP_CHANGE", 0x1B: "WIZ_LEVEL_CHANGE",
    0x1C: "WIZ_NPC_REGION", 0x1D: "WIZ_REQ_NPCIN", 0x1E: "WIZ_WARP",
    0x1F: "WIZ_ITEM_MOVE",
    0x20: "WIZ_NPC_EVENT", 0x21: "WIZ_ITEM_TRADE", 0x22: "WIZ_TARGET_HP",
    0x23: "WIZ_ITEM_DROP", 0x24: "WIZ_BUNDLE_OPEN", 0x25: "WIZ_TRADE_NPC",
    0x26: "WIZ_ITEM_GET", 0x27: "WIZ_ZONE_CHANGE", 0x28: "WIZ_POINT_CHANGE",
    0x29: "WIZ_STATE_CHANGE", 0x2A: "WIZ_LOYALTY_CHANGE",
    0x2B: "WIZ_VERSION_CHECK", 0x2C: "WIZ_CRYPTION", 0x2D: "WIZ_USERLOOK_CHANGE",
    0x2E: "WIZ_NOTICE", 0x2F: "WIZ_PARTY",
    0x30: "WIZ_EXCHANGE", 0x31: "WIZ_MAGIC_PROCESS", 0x32: "WIZ_SKILLPT_CHANGE",
    0x33: "WIZ_OBJECT_EVENT", 0x34: "WIZ_CLASS_CHANGE", 0x35: "WIZ_CHAT_TARGET",
    0x36: "WIZ_CONCURRENT_USER", 0x37: "WIZ_DATASAVE", 0x38: "WIZ_DURATION",
    0x3A: "WIZ_REPAIR_NPC", 0x3B: "WIZ_ITEM_REPAIR",
    0x3C: "WIZ_KNIGHTS_PROCESS", 0x3D: "WIZ_ITEM_COUNT_CHANGE",
    0x3E: "WIZ_KNIGHTS_LIST", 0x3F: "WIZ_ITEM_REMOVE",
    0x40: "WIZ_OPERATOR", 0x41: "WIZ_SPEEDHACK_CHECK", 0x42: "WIZ_COMPRESS_PACKET",
    0x43: "WIZ_SERVER_CHECK", 0x44: "WIZ_CONTINUOUS_PACKET",
    0x45: "WIZ_WAREHOUSE", 0x46: "WIZ_SERVER_CHANGE",
    0x47: "WIZ_REPORT_BUG", 0x48: "WIZ_HOME", 0x49: "WIZ_FRIEND_PROCESS",
    0x4A: "WIZ_GOLD_CHANGE", 0x4B: "WIZ_WARP_LIST", 0x4C: "WIZ_VIRTUAL_SERVER",
    0x4F: "WIZ_PARTY_BBS",
    0x52: "WIZ_CLIENT_EVENT", 0x53: "WIZ_MAP_EVENT", 0x54: "WIZ_WEIGHT_CHANGE",
    0x55: "WIZ_SELECT_MSG", 0x56: "WIZ_NPC_SAY", 0x57: "WIZ_BATTLE_EVENT",
    0x58: "WIZ_AUTHORITY_CHANGE", 0x59: "WIZ_EDIT_BOX", 0x5A: "WIZ_SANTA",
    0x5B: "WIZ_ITEM_UPGRADE", 0x5C: "WIZ_CLAN_PREMIUM",
    0x5E: "WIZ_ZONEABILITY", 0x5F: "WIZ_EVENT", 0x60: "WIZ_STEALTH",
    0x61: "WIZ_ROOM_PACKET", 0x62: "WIZ_ROOM", 0x63: "WIZ_CLAN_BATTLE",
    0x64: "WIZ_QUEST", 0x65: "WIZ_PP_CARD", 0x66: "WIZ_KISS",
    0x67: "WIZ_RECOMMEND_USER",
    0x68: "WIZ_MERCHANT", 0x69: "WIZ_MERCHANT_INOUT",
    0x6A: "WIZ_SHOPPING_MALL", 0x6B: "WIZ_SERVER_INDEX", 0x6C: "WIZ_EFFECT",
    0x6D: "WIZ_SIEGE", 0x6E: "WIZ_NAME_CHANGE", 0x6F: "WIZ_WEBPAGE",
    0x70: "WIZ_CAPE", 0x71: "WIZ_PREMIUM", 0x72: "WIZ_HACKTOOL",
    0x73: "WIZ_RENTAL", 0x74: "WIZ_ITEM_EXPIRATION", 0x75: "WIZ_CHALLENGE",
    0x76: "WIZ_PET", 0x77: "WIZ_CHINA", 0x78: "WIZ_KING",
    0x79: "WIZ_SKILLDATA", 0x7A: "WIZ_PROGRAM_CHECK", 0x7B: "WIZ_BIFROST",
    0x7C: "WIZ_REPORT", 0x7D: "WIZ_LOGOSSHOUT",
    0x80: "WIZ_RANK", 0x81: "WIZ_STORY", 0x82: "WIZ_NATION_TRANSFER",
    0x83: "WIZ_TERRAIN_EFFECTS", 0x84: "WIZ_MOVING_TOWER", 0x85: "WIZ_CAPTURE",
    0x86: "WIZ_MINING", 0x87: "WIZ_HELMET", 0x88: "WIZ_PVP",
    0x89: "WIZ_CHANGE_HAIR", 0x8B: "WIZ_VIPWAREHOUSE", 0x8D: "WIZ_GENDER_CHANGE",
    0x90: "WIZ_LOYALTY_SHOP", 0x91: "WIZ_CLANPOINTS_BATTLE",
    0x92: "WIZ_MAX_HP_CHANGE", 0x95: "WIZ_SEAL",
    0x97: "WIZ_GENIE", 0x98: "WIZ_USER_INFO", 0x99: "WIZ_USER_ACHIEVE",
    0x9A: "WIZ_EXP_SEAL", 0x9B: "WIZ_KURIAN_SP_CHANGE",
    0x9C: "WIZ_CONTINUOUS_DATA", 0x9F: "WIZ_LOADING_LOGIN",
    0xA0: "WIZ_UNK_A0", 0xA5: "WIZ_ACHIEVEMENT2", 0xA9: "WIZ_COLLECTION1",
    0xAC: "WIZ_PREMIUM2", 0xAD: "WIZ_UNK_AD",
    0xB4: "WIZ_COLLECTION2", 0xB5: "WIZ_SOUND", 0xB6: "WIZ_VANGUARD",
    0xB7: "WIZ_ATTENDANCE", 0xB8: "WIZ_UPGRADE_NOTICE",
    0xB9: "WIZ_PRESET", 0xBA: "WIZ_AUTO_DROP", 0xBD: "WIZ_MERCHANT_LIST",
    0xC0: "WIZ_CAPTCHA", 0xC1: "WIZ_UNK_C1", 0xC2: "WIZ_DAILY_RANK",
    0xC3: "WIZ_COSTUME", 0xC5: "WIZ_SOUL", 0xC7: "WIZ_DAILY_QUEST",
    0xC8: "WIZ_KILL_ASSIST", 0xCB: "WIZ_AWAKENING", 0xCC: "WIZ_ENCHANT",
    0xCE: "WIZ_CHALLENGE2", 0xCF: "WIZ_ABILITY",
    0xD0: "WIZ_GUILD_BANK", 0xD1: "WIZ_CLAN_WAREHOUSE",
    0xD3: "WIZ_REBIRTH", 0xD5: "WIZ_TERRITORY", 0xD6: "WIZ_WORLD_BOSS",
    0xD7: "WIZ_SEASON", 0xDB: "WIZ_ADD_MSG",
    0xE0: "WIZ_CINDERELLA", 0xE8: "WIZ_PARTY_HP",
    0xE9: "WIZ_EXT_HOOK", 0xEA: "WIZ_UNK_EA", 0xEF: "WIZ_KNIGHT_ROYALE",
    0xFA: "WIZ_GAMEGUARD_RESP", 0xFB: "WIZ_GAMEGUARD_CHAL",
    0xFC: "WIZ_GAMEGUARD_HB", 0xFD: "WIZ_GAMEGUARD_AUTH",
    0xFE: "WIZ_GAMEGUARD_KEY", 0xFF: "WIZ_UNK_FF",
}

# ---------------------------------------------------------------------------
# Login server opcodes  (~17 entries)
# ---------------------------------------------------------------------------
LOGIN_OPCODES: dict[int, str] = {
    0x01: "LS_PING", 0x02: "LS_DOWNLOAD_INFO", 0x03: "LS_NOTICE",
    0x42: "LS_COMPRESS_PACKET", 0x51: "LS_HANDSHAKE",
    0xA1: "LS_SERVER_REDIRECT", 0xA6: "LS_SERVER_SELECT",
    0xC0: "LS_GAME_RELAY",
    0xF2: "LS_CRYPTION", 0xF3: "LS_LOGIN_REQ", 0xF4: "LS_NATION_SELECT",
    0xF5: "LS_CHAR_LIST", 0xF6: "LS_VERSION_CHECK", 0xF7: "LS_UNK_F7",
    0xF8: "LS_KICK", 0xFA: "LS_CRYPTO_KEY", 0xFD: "LS_NEWS",
}

# ---------------------------------------------------------------------------
# 17 functional categories  (opcode -> category)
# ---------------------------------------------------------------------------
CATEGORIES: dict[str, set[int]] = {
    "login_session":    {0xF2, 0xF3, 0xF5, 0xF6, 0xFD, 0x2B, 0x2C, 0x9F, 0xC0},
    "character_entry":  {0x01, 0x02, 0x03, 0x04, 0x05, 0x0C, 0x0D, 0x0E, 0x0F, 0x89, 0x8D},
    "movement_pos":     {0x06, 0x07, 0x09, 0x15, 0x16, 0x1C, 0x1D, 0x98, 0x60},
    "npc_monster":      {0x0A, 0x0B, 0x20, 0x56},
    "combat_magic":     {0x08, 0x11, 0x12, 0x17, 0x18, 0x22, 0x28, 0x29, 0x31, 0x1A, 0x1B, 0x92, 0x9B},
    "item_inventory":   {0x1F, 0x21, 0x23, 0x24, 0x26, 0x3D, 0x3F, 0x54, 0x38},
    "npc_dialog_quest": {0x55, 0x64, 0x33, 0xC7},
    "trade_merchant":   {0x25, 0x30, 0x68, 0x69, 0xBD},
    "social_chat":      {0x10, 0x19, 0x2E, 0x35, 0x49, 0x66, 0x81, 0xDB},
    "clan_knights":     {0x3C, 0x3E, 0x63, 0x70, 0x91, 0xD0, 0xD1},
    "upgrade_craft":    {0x34, 0x3A, 0x3B, 0x5B, 0x86, 0xB8, 0xCB, 0xCC, 0xD3},
    "events":           {0x52, 0x53, 0x57, 0x5F, 0x61, 0x62, 0x6D, 0x7B, 0x88, 0xE0, 0xEF},
    "king_ranking":     {0x78, 0x80, 0x85, 0xC2, 0xD5, 0xD6, 0xD7},
    "pet_costume":      {0x76, 0x87, 0x97, 0xC3, 0xC5},
    "premium_cash":     {0x5A, 0x5C, 0x6A, 0x71, 0x73, 0x90, 0xAC, 0xB6, 0xB7},
    "system_anticheat": {0x36, 0x37, 0x40, 0x41, 0x42, 0x43, 0x44, 0x47, 0x72, 0x7A, 0x7C, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE},
    "warp_zone":        {0x1E, 0x27, 0x46, 0x48, 0x4B, 0x5E},
}

# Reverse lookup: opcode int -> category name
OPCODE_TO_CATEGORY: dict[int, str] = {}
for _cat, _opcodes in CATEGORIES.items():
    for _op in _opcodes:
        OPCODE_TO_CATEGORY[_op] = _cat


def get_name(opcode: int, is_login: bool = False) -> str:
    """Return human-readable name for an opcode.

    Args:
        opcode: The opcode byte value (0x00-0xFF).
        is_login: If True, look up in LOGIN_OPCODES first.

    Returns:
        The symbolic name (e.g. "WIZ_SELECT_MSG") or "UNK_0xNN" if unknown.
    """
    if is_login:
        name = LOGIN_OPCODES.get(opcode)
        if name is not None:
            return name
    name = GAME_OPCODES.get(opcode)
    if name is not None:
        return name
    return f"UNK_0x{opcode:02X}"


def get_category(opcode: int) -> str:
    """Return the functional category for a game opcode.

    Args:
        opcode: The opcode byte value.

    Returns:
        Category name (e.g. "combat_magic") or "unknown".
    """
    return OPCODE_TO_CATEGORY.get(opcode, "unknown")
