//! constants
//!

/// max amount of maps available on a server
pub const MAX_MAP_PER_SERVER: usize = 1500;

/// max items in player inventory
pub const MAX_INVENROY: usize = 100;

/// max number of characters per account. Note that changing this setting alone is not enough if the client is not hexed to support more characters as well.
/// max value tested was 265
pub const MAX_CARD_SLOTS: usize = 4;

/// max amount of single stacked item
pub const MAX_STACKED_AMOUNT: usize = 30000;

pub const MAX_ZENY: u32 = std::i32::MAX as u32;

pub const MAX_ZENY_IN_BANK: u32 = std::i32::MAX as u32;

pub const MAX_FAME_POINT: usize = 1_000_000_000;

pub const MAX_CART_ITEMS: usize = 100;

pub const MAX_SKILLS: usize = 1250;

pub const DEFAULT_WALK_SPEED: u32 = 150;

pub const MIN_WALK_SPEED: u32 = 20;

pub const MAX_WALK_SPEED: u32 = 1000;

pub const MAX_STORAGE_SLOTS: usize = 600;

pub const MAX_GUILD_STORAGE_SLOTS: usize = 600;

pub const MAX_PARTY_MEMBERS: usize = 12;

pub const MAX_GUILD_MEMBERS: usize = 16 + 10 * 6;

pub const MAX_GUILD_POSITIONS: usize = 20;

pub const MAX_GUILD_EXPULSIONS: usize = 32;

pub const MAX_GUILD_ALLIANCES: usize = 16;

pub const MAX_GUILD_SKILLS: usize = 17;

pub const MAX_GUILD_LEVEL: usize = 50;

pub const MAX_GUARDIANS_PER_CASTLE: usize = 8;

pub const MAX_QUEST_OBJECTIVES: usize = 3;

pub const MAX_QUEST_DROPS: usize = 3;

pub const MAX_PC_BONUS_SCRIPT: usize = 50;

pub const MAX_ITEM_RDM_OPT: usize = 5;

pub const MAX_DB_NAME_LEN: usize = 256;

pub const MAX_CLAN: usize = 500;

pub const MAX_CLAN_ALLIANCE: usize = 6;

// for produce
pub const MIN_ATTRIBUTE_AMOUNT: usize = 0;
pub const MAX_ATTRIBUTE_AMOUNT: usize = 4;
pub const ATTRIBUTE_NORMAL: usize = 0;
pub const MIN_STAR_AMOUNT: usize = 0;
pub const MAX_STAR_AMOUNT: usize = 3;

pub const MAX_STATUS_TYPE: u32 = 5;

pub const WEDDING_RING_MALE_INDEX: u32 = 2634;
pub const WEDDING_RING_FEMALE_INDEX: u32 = 2635;

// data length
pub const CHARACTER_NAME_LENGTH: usize = 23 + 1;
pub const PASSWD_LENGTH: usize = 32 + 1;
pub const NPC_NAME_LENGTH: usize = 50;
pub const EVENT_NAME_LENGTH: usize = NPC_NAME_LENGTH + 2 + NPC_NAME_LENGTH + 1;
pub const ITEM_NAME_LENGTH: usize = 50;
pub const MAP_NAME_LENGTH: usize = 11 + 1;
pub const MAP_NAME_LENGTH_EXT: usize = MAP_NAME_LENGTH + 4;
pub const PIN_CODE_LENGTH: usize = 4;

pub const MAX_FRIENDS: usize = 40;
pub const MAX_MEMO_POINTS: usize = 3;
pub const MAX_SKILL_COOLDOWN: usize = 20;

pub const MAX_FAME_LIST: usize = 10;

pub const START_ACCOUNT_ID: u32 = 2_000_000;
pub const END_ACCOUNT_ID: u32 = 100_000_000;
pub const START_CHAR_ID: u32 = 150_000;

// mail system
pub const MAIL_MAX_INBOX: usize = 30;
pub const MAIL_TITLE_LENGTH: usize = 40;

// mercenary system
pub const MC_SKILLBASE: u32 = 8201;
pub const MAX_MERC_SKILL: u32 = 41;

// elemntal system
pub const MAX_ELEMNTAL_SKILL: u32 = 42;
pub const EL_SKILLBASE: u32 = 8401;
pub const MAX_ELE_SKILL_TREE: usize = 3;
pub const MAX_ELEMENTAL_CLASS: u32 = 12;
pub const EL_CLASS_BASE: u32 = 2114;
pub const EL_CALSS_MAX: u32 = EL_CLASS_BASE + MAX_ELEMENTAL_CLASS - 1;

// achievement system
pub const MAX_ACHIEVEMENT_OBJECTIVES: usize = 10;
pub const MAX_ACHIEVEMENT_DEPENDENTS: usize = 20;
pub const ACHIEVEMENT_NAME_LENGHT: usize = 50;

// ??
pub const MD_MASK: u32 = 0x000ffff;
pub const ATR_MASK: u32 = 0x0ff0000;
pub const CL_MASK: u32 = 0xf000000;
