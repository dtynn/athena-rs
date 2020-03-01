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
