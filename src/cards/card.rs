use rand::seq::SliceRandom;

use crate::utils::Character;

impl CardName {
    pub fn random_common(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_commons()),
            Character::Silent => Self::random_from(Self::silent_commons()),
            Character::Defect => Self::random_from(Self::defect_commons()),
            Character::Watcher => Self::random_from(Self::watcher_commons()),
        }
    }

    pub fn random_uncommon(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_uncommons()),
            Character::Silent => Self::random_from(Self::silent_uncommons()),
            Character::Defect => Self::random_from(Self::defect_uncommons()),
            Character::Watcher => Self::random_from(Self::watcher_uncommons()),
        }
    }

    pub fn random_rare(character: Character) -> Self {
        match character {
            Character::Ironclad => Self::random_from(Self::ironclad_rares()),
            Character::Silent => Self::random_from(Self::silent_rares()),
            Character::Defect => Self::random_from(Self::defect_rares()),
            Character::Watcher => Self::random_from(Self::watcher_rares()),
        }
    }

    fn random_from(pool: Vec<Self>) -> Self {
        *pool.choose(&mut rand::thread_rng()).unwrap()
    }

    fn ironclad_commons() -> Vec<Self> {
        vec![
            CardName::ShrugItOff,
            CardName::Cleave,
            CardName::Clothesline,
            CardName::Anger,
            CardName::Armaments,
            CardName::BodySlam,
            CardName::Clash,
            CardName::Flex,
            CardName::Havoc,
            CardName::Headbutt,
            CardName::HeavyBlade,
            CardName::IronWave,
            CardName::PerfectedStrike,
            CardName::PommelStrike,
            CardName::SwordBoomerang,
            CardName::ThunderClap,
            CardName::TrueGrit,
            CardName::TwinStrike,
            CardName::WarCry,
            CardName::WildStrike,
        ]
    }

    fn ironclad_uncommons() -> Vec<Self> {
        vec![
            CardName::Carnage,
            CardName::GhostlyArmor,
            CardName::Bloodletting,
            CardName::BattleTrance,
            CardName::BloodForBlood,
            CardName::BurningPact,
            CardName::Combust,
            CardName::DarkEmbrace,
            CardName::Disarm,
            CardName::Dropkick,
            CardName::DualWield,
            CardName::Entrench,
            CardName::Evolve,
            CardName::FeelNoPain,
            CardName::FireBreathing,
            CardName::FlameBarrier,
            CardName::HemoKinesis,
            CardName::InfernalBlade,
            CardName::Inflame,
            CardName::Intimidate,
            CardName::Metallicize,
            CardName::PowerThrough,
            CardName::Pummel,
            CardName::Rage,
            CardName::Rampage,
            CardName::RecklessCharge,
            CardName::Rupture,
            CardName::SearingBlow,
            CardName::SecondWind,
            CardName::SeeingRed,
            CardName::Sentinel,
            CardName::SeverSoul,
            CardName::Shockwave,
            CardName::SpotWeakness,
            CardName::Uppercut,
            CardName::Whirlwind,
        ]
    }

    fn ironclad_rares() -> Vec<Self> {
        vec![
            CardName::Barricade,
            CardName::Bludgeon,
            CardName::Impervious,
            CardName::Berserk,
            CardName::Brutality,
            CardName::Corruption,
            CardName::DemonForm,
            CardName::DoubleTap,
            CardName::Exhume,
            CardName::Feed,
            CardName::FiendFire,
            CardName::Juggernaut,
            CardName::LimitBreak,
            CardName::Offering,
            CardName::Reaper,
        ]
    }

    fn silent_commons() -> Vec<Self> {
        vec![]
    }

    fn silent_uncommons() -> Vec<Self> {
        vec![]
    }

    fn silent_rares() -> Vec<Self> {
        vec![]
    }

    fn defect_commons() -> Vec<Self> {
        vec![]
    }

    fn defect_uncommons() -> Vec<Self> {
        vec![]
    }

    fn defect_rares() -> Vec<Self> {
        vec![]
    }

    fn watcher_commons() -> Vec<Self> {
        vec![]
    }

    fn watcher_uncommons() -> Vec<Self> {
        vec![]
    }

    fn watcher_rares() -> Vec<Self> {
        vec![]
    }

    pub fn colorless_uncommons() -> Vec<Self> {
        vec![
            Self::BandageUp,
            Self::Blind,
            Self::DarkShackles,
            Self::DeepBreath,
            Self::Discovery,
            Self::DramaticEntrance,
            Self::Enlightenment,
            Self::Finesse,
            Self::FlashOfSteel,
            Self::Forethought,
            Self::GoodInstincts,
            Self::Impatience,
            Self::JackOfAllTrades,
            Self::Madness,
            Self::MindBlast,
            Self::Panacea,
            Self::PanicButton,
            Self::Purity,
            Self::SwiftStrike,
            Self::Trip]
    }

    pub fn colorless_rares() -> Vec<Self> {
        vec![
            Self::Apotheosis,
            Self::Chrysalis,
            Self::HandOfGreed,
            Self::Magnetism,
            Self::MasterOfStrategy,
            Self::Mayhem,
            Self::Metamorphosis,
            Self::Panache,
            Self::SadisticNature,
            Self::SecretTechnique,
            Self::SecretWeapon,
            Self::TheBomb,
            Self::ThinkingAhead,
            Self::Transmutation,
            Self::Violence,
        ]
    }

    pub fn transform_cards(character: Character) -> Vec<Self> {
        let cards = match character {
            Character::Ironclad => vec![
                Self::ironclad_commons(),
                Self::ironclad_uncommons(),
                Self::ironclad_rares(),
            ],
            Character::Silent => vec![
                Self::silent_commons(),
                Self::silent_uncommons(),
                Self::silent_rares(),
            ],
            Character::Defect => vec![
                Self::defect_commons(),
                Self::defect_uncommons(),
                Self::defect_rares(),
            ],
            Character::Watcher => vec![
                Self::watcher_commons(),
                Self::watcher_uncommons(),
                Self::watcher_rares(),
            ],
        };

        cards.concat()
    }

    pub fn transform_curses() -> Vec<Self> {
        vec![
            CardName::Clumsy,
            CardName::Decay,
            CardName::Doubt,
            CardName::Injury,
            CardName::Normality,
            CardName::Pain,
            CardName::Parasite,
            CardName::Regret,
            CardName::Shame,
            CardName::Writhe,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardType {
    Attack,
    Power,
    Skill,
    Status,
    Curse,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CardName {
    Strike,
    Defend,
    Bash,
    Void,
    Slimed,
    Burn,
    Wound,
    Dazed,
    AscendersBane,
    Clumsy,
    CurseOfTheBell,
    Decay,
    Doubt,
    Injury,
    Necronomicurse,
    Normality,
    Pain,
    Parasite,
    Regret,
    Shame,
    Writhe,
    ShrugItOff,
    Cleave,
    Clothesline,
    Carnage,
    Anger,
    GhostlyArmor,
    Barricade,
    Armaments,
    Bloodletting,
    BodySlam,
    Bludgeon,
    Impervious,
    Clash,
    Flex,
    Havoc,
    Headbutt,
    HeavyBlade,
    IronWave,
    PerfectedStrike,
    PommelStrike,
    SwordBoomerang,
    ThunderClap,
    TrueGrit,
    TwinStrike,
    WarCry,
    WildStrike,
    BattleTrance,
    BloodForBlood,
    BurningPact,
    Combust,
    DarkEmbrace,
    Disarm,
    Dropkick,
    DualWield,
    Entrench,
    Evolve,
    FeelNoPain,
    FireBreathing,
    FlameBarrier,
    HemoKinesis,
    InfernalBlade,
    Inflame,
    Intimidate,
    Metallicize,
    PowerThrough,
    Pummel,
    Rage,
    Rampage,
    RecklessCharge,
    Rupture,
    SearingBlow,
    SecondWind,
    SeeingRed,
    Sentinel,
    SeverSoul,
    Shockwave,
    SpotWeakness,
    Uppercut,
    Whirlwind,
    Berserk,
    Brutality,
    Corruption,
    DemonForm,
    DoubleTap,
    Exhume,
    Feed,
    FiendFire,
    Juggernaut,
    LimitBreak,
    Offering,
    Reaper,
    Apparition,
    BandageUp,
    Blind,
    DarkShackles,
    DeepBreath,
    Discovery,
    DramaticEntrance,
    Enlightenment,
    Finesse,
    FlashOfSteel,
    Forethought,
    GoodInstincts,
    Impatience,
    JackOfAllTrades,
    Madness,
    MindBlast,
    Panacea,
    PanicButton,
    Purity,
    SwiftStrike,
    Trip,
    Apotheosis,
    Chrysalis,
    HandOfGreed,
    Magnetism,
    MasterOfStrategy,
    Mayhem,
    Metamorphosis,
    Panache,
    SadisticNature,
    SecretTechnique,
    SecretWeapon,
    TheBomb,
    ThinkingAhead,
    Transmutation,
    Violence,
}
