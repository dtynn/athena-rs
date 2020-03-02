pub enum ItemTypes {
    Healing = 0,
    Unknown,      //1
    Usable,       //2
    Etc,          //3
    Armor,        //4
    Weapon,       //5
    Card,         //6
    Petegg,       //7
    Petarmor,     //8
    Unknown2,     //9
    Ammo,         //10
    Delayconsume, //11
    Shadowgear,   //12
    Cash = 18,
    MAX,
}

pub enum MonsterMode {
    None = 0x0000000,
    Canmove = 0x0000001,
    Looter = 0x0000002,
    Aggressive = 0x0000004,
    Assist = 0x0000008,
    CastsensorIdle = 0x0000010,
    NorandomWalk = 0x0000020,
    NocastSkill = 0x0000040,
    Canattack = 0x0000080,
    //FREE					= 0x0000100,
    CastsensorChase = 0x0000200,
    Changechase = 0x0000400,
    Angry = 0x0000800,
    ChangetargetMelee = 0x0001000,
    ChangetargetChase = 0x0002000,
    Targetweak = 0x0004000,
    Randomtarget = 0x0008000,
    Ignoremelee = 0x0010000,
    Ignoremagic = 0x0020000,
    Ignoreranged = 0x0040000,
    Mvp = 0x0080000,
    Ignoremisc = 0x0100000,
    KnockbackImmune = 0x0200000,
    TeleportBlock = 0x0400000,
    //FREE					= 0x0800000,
    FixedItemdrop = 0x1000000,
    Detector = 0x2000000,
    StatusImmune = 0x4000000,
    SkillImmune = 0x8000000,
}

pub enum QuestState {
    Inactive,
    Active,
    Complete,
}
