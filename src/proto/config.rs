macro_rules! configure {
    ($cfg_name: ident, $($(#[$meta:meta])* $(fn)? $field_name: ident : $field_type: ty = $field_value: expr),*,) => {
        pub struct $cfg_name {
            $(
                $(
                #[$meta]
                 )*
                pub $field_name: $field_type,
             )*
        }

        impl Default for $cfg_name {
            fn default() -> Self {
                $(
                    let $field_name: $field_type = $field_value;
                 )*
                $cfg_name {
                    $(
                        $field_name,
                     )*
                }
            }
        }
    };

    (FIELD $(#[$meta:meta])* $field_name: ident : $field_type: ty = $field_value: expr) => {
            $(
            #[$meta]
             )*
            pub $field_name: $field_type,
    };

    (FIELD $(#[$meta:meta])* $field_name: ident : $field_type: ty = $field_value: block) => {
    };
}

configure! {
    Config,

    /// max amount of maps available on a server
    max_map_per_server: usize = 1500,

    /// max items in player inventory
    max_inventory: usize = 100,

    /// max number of characters per account. Note that changing this setting alone is not enough if the client is not hexed to support more characters as well.
    /// max value tested was 265
    max_card_slots: usize = 4,

    /// max amount of single stacked item
    max_stacked_amount: usize = 30000,

    max_zeny: u32 = std::i32::MAX as u32,

    max_zeny_in_bank: u32 = std::i32::MAX as u32,

    max_fame_point: usize = 1_000_000_000,

    max_cart_items: usize = 100,

    max_skills: usize = 1250,

    default_walk_speed: u32 = 150,

    min_walk_speed: u32 = 20,

    max_walk_speed: u32 = 1000,

    max_storage_slots: usize = 600,

    max_guild_storage_slots: usize = 600,

    max_party_members: usize = 12,

    max_guild_members: usize = 16 + 10 * 6,

    max_guild_positions: usize = 20,

    max_guild_expulsions: usize = 32,

    max_guild_alliances: usize = 16,

    max_guild_skills: usize = 17,

    max_guild_level: usize = 50,

    max_guardians_per_castle: usize = 8,

    max_quest_objectives: usize = 3,

    max_quest_drops: usize = 3,

    max_pc_bonus_script: usize = 50,

    max_item_rdm_opt: usize = 5,

    max_db_name_len: usize = 256,

    max_clan: usize = 500,

    max_clan_alliance: usize = 6,

    // for produce
    min_attribute_amount: usize = 0,
    max_attribute_amount: usize = 4,
    attribute_normal: usize = 0,
    min_star_amount: usize = 0,
    max_star_amount: usize = 3,

    max_status_type: u32 = 5,

    wedding_ring_male_index: u32 = 2634,
    wedding_ring_female_index: u32 = 2635,

    // data length
    character_name_length: usize = 23 + 1,
    passwd_length: usize = 32 + 1,
    npc_name_length: usize = 50,
    event_name_length: usize = npc_name_length + 2 + npc_name_length + 1,
    item_name_length: usize = 50,
    map_name_length: usize = 11 + 1,
    map_name_length_ext: usize = map_name_length + 4,
    pin_code_length: usize = 4,

    max_friends: usize = 40,
    max_memo_points: usize = 3,
    max_skill_cooldown: usize = 20,

    max_fame_list: usize = 10,

    start_account_id: u32 = 2_000_000,
    end_account_id: u32 = 100_000_000,
    start_char_id: u32 = 150_000,

    // mail system
    mail_max_inbox: usize = 30,
    mail_title_length: usize = 40,

    // mercenary system
    mc_skillbase: u32 = 8201,
    max_merc_skill: u32 = 41,

    // elemntal system
    max_elemntal_skill: u32 = 42,
    el_skillbase: u32 = 8401,
    max_ele_skill_tree: usize = 3,
    max_elemental_class: u32 = 12,
    el_class_base: u32 = 2114,
    el_calss_max: u32 = el_class_base + max_elemental_class - 1,

    // achievement system
    max_achievement_objectives: usize = 10,
    max_achievement_dependents: usize = 20,
    achievement_name_lenght: usize = 50,

    // ??
    md_mask: u32 = 0x000ffff,
    atr_mask: u32 = 0x0ff0000,
    cl_mask: u32 = 0xf000000,
}
