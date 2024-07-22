use rand::Rng;

use crate::utils::Character;

pub struct Relics {
    list: Vec<Relic>,
    common_pool: Vec<Relic>,
    uncommon_pool: Vec<Relic>,
    rare_pool: Vec<Relic>,
    shop_pool: Vec<Relic>,
    boss_pool: Vec<Relic>,
}

impl Relics {
    pub fn new(character: Character) -> Self {
        let starter = match character {
            Character::Ironclad => Relic::BurningBlood,
            Character::Silent => Relic::RingOfSnake,
            Character::Defect => Relic::CrackedCore,
            Character::Watcher => Relic::PureWater,
        };

        let mut common_pool = Relic::shared_common();
        common_pool.append(&mut match character {
            Character::Ironclad => Relic::ironclad_common(),
            Character::Silent => Relic::silent_common(),
            Character::Defect => Relic::defect_common(),
            Character::Watcher => Relic::watcher_common(),
        });

        let mut uncommon_pool = Relic::shared_uncommon();
        uncommon_pool.append(&mut match character {
            Character::Ironclad => Relic::ironclad_uncommon(),
            Character::Silent => Relic::silent_uncommon(),
            Character::Defect => Relic::defect_uncommon(),
            Character::Watcher => Relic::watcher_uncommon(),
        });

        let mut rare_pool = Relic::shared_rare();
        rare_pool.append(&mut match character {
            Character::Ironclad => Relic::ironclad_rare(),
            Character::Silent => Relic::silent_rare(),
            Character::Defect => Relic::defect_rare(),
            Character::Watcher => Relic::watcher_rare(),
        });

        let mut shop_pool = Relic::shared_shop();
        shop_pool.append(&mut match character {
            Character::Ironclad => Relic::ironclad_shop(),
            Character::Silent => Relic::silent_shop(),
            Character::Defect => Relic::defect_shop(),
            Character::Watcher => Relic::watcher_shop(),
        });

        let mut boss_pool = Relic::shared_boss();
        boss_pool.append(&mut match character {
            Character::Ironclad => Relic::ironclad_boss(),
            Character::Silent => Relic::silent_boss(),
            Character::Defect => Relic::defect_boss(),
            Character::Watcher => Relic::watcher_boss(),
        });

        Self {
            list: vec![starter],
            common_pool,
            uncommon_pool,
            rare_pool,
            shop_pool,
            boss_pool,
        }
    }

    pub fn random_common(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.common_pool)
    }

    pub fn random_uncommon(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.uncommon_pool)
    }

    pub fn random_rare(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.rare_pool)
    }

    pub fn random_shop(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.shop_pool)
    }

    pub fn random_boss(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.boss_pool)
    }

    fn _random_remove_from_pool(pool: &mut Vec<Relic>) -> Relic {
        if pool.is_empty() {
            return Relic::Circlet;
        }
        let index = rand::thread_rng().gen_range(0..pool.len());
        pool.remove(index)
    }
}

#[derive(Copy, Clone)]
pub enum Relic {
    Circlet,
    BurningBlood,
    RingOfSnake,
    CrackedCore,
    PureWater,
    Akabeko,
    Anchor,
    AncientTeaSet(bool),
    ArtOfWar(bool),
    BagOfMarbles,
    BagOfPrep,
    BloodVial,
    BronzeScales,
    CentennialPuzzle(bool),
    CeramicFish,
    Dreamcatcher,
    HappyFlower(u8),
    JuzuBracelet,
    Lantern,
    MawBank(bool),
    MealTicket,
    Nunchaku(u8),
    SmoothStone,
    Omamori(u8),
    Orichalcum,
    PenNib(u8),
    PotionBelt,
    PreservedInsect,
    RegalPillow,
    SmilingMask,
    Strawberry,
    TheBoot,
    TinyChest(u8),
    ToyOrnithopter,
    Vajra,
    WarPaint,
    Whetstone,
    RedSkull,
    SneckoSkull,
    DataDisk,
    Damaru,
    BlueCandle,
    BottledFlame, //TODO: Figure these out
    BottledLightning,
    BottledTornado,
    DarkstonePeriapt,
    EternalFeather,
    FrozenEgg,
    GremlinHorn,
    HornCleat,
    InkBottle(u8),
    Kunai(u8),
    LetterOpener(u8),
    Matryoshka(u8),
    MeatOnTheBone,
    MercuryHourglass,
    MoltenEgg,
    MummifiedHand,
    OrnamentalFan(u8),
    Pantograph,
    Pear,
    QuestionCard,
    Shuriken(u8),
    SingingBowl,
    StrikeDummy,
    Sundial(u8),
    TheCourier,
    ToxicEgg,
    WhiteBeastStatue,
    PaperPhrog,
    SelfFormingClay,
    NinjaScroll,
    PaperKrane,
    GoldPlatedCables,
    SymbioticVirus,
    Duality,
    TeardropLocket,
    BirdFacedUrn,
    Calipers,
    CaptainsWheel,
    DeadBranch,
    DuVuDoll,
    FossilizedHelix,
    GamblersChip,
    Ginger,
    Girya(u8),
    IceCream,
    IncenseBurner(u8),
    LizardTail(bool),
    Mango,
    OldCoin,
    PeacePipe,
    Pocketwatch(u8),
    PrayerWheel,
    Shovel,
    StoneCalendar,
    ThreadAndNeedle,
    Torii,
    TungstenRod,
    Turnip,
    UnceasingTop,
    WingBoots,
    ChampionsBelt,
    CharonsAshes,
    MagicFlower,
    TheSpecimen,
    Tingsha,
    ToughBandages,
    EmotionChip,
    CloakClasp,
    GoldenEye,
    Cauldron,
    ChemicalX,
    ClockworkSouvenir,
    DollysMirror,
    FrozenEye,
    HandDrill,
    LeesWaffle,
    MedKit,
    MembershipCard,
    // TODO: Figure out pellets
    OrangePellets,
    Orrery,
    PrismaticShard,
    SlingOfCourage,
    StrangeSpoon,
    Abacus,
    Toolkit,
    Brimstone,
    TwistedFunnel,
    RunicCapacitor,
    Melange,
    Astrolabe,
    BlackStar,
    BrokenCrown,
    CallingBell,
    CoffeeDripper,
    CursedKey,
    Ectoplasm,
    EmptyCage,
    FusionHammer,
    PandorasBox,
    PhilosophersStone,
    RunicDome,
    RunicPyramid,
    SacredBark,
    SlaversCollar,
    SneckoEye,
    Sozu,
    TinyHouse,
    VelvetChoker,
    MarkOfPain,
    BlackBlood,
    RunicCube,
    RingOfTheSerpent,
    WristBlade,
    HoveringKite(bool),
    NuclearBattery,
    Inserter(bool),
    FrozenCore,
    HolyWater,
    VioletLotus,
    BloodyIdol,
    CultistHeadpiece,
    Enchiridion,
    ClericMask,
    GoldenIdol,
    GremlinVisage,
    MarkOfTheBloom,
    MutagenicStrength,
    NlothsGift,
    NlothsHungryFace,
    Necronomicon,
    NeowsLament,
    NilrysCodex(u8),
    OddMushroom,
    RedMask,
    SpiritPoop,
    SerpentHead,
    WarpedTongs,
}

impl Relic {
    pub fn shared_common() -> Vec<Self> {
        vec![
            Self::Akabeko,
            Self::Anchor,
            Self::ArtOfWar(true),
            Self::AncientTeaSet(false),
            Self::BagOfMarbles,
            Self::BagOfPrep,
            Self::BloodVial,
            Self::BronzeScales,
            Self::CentennialPuzzle(true),
            Self::CeramicFish,
            Self::Dreamcatcher,
            Self::HappyFlower(0),
            Self::JuzuBracelet,
            Self::Lantern,
            Self::MawBank(true),
            Self::Nunchaku(0),
            Self::SmoothStone,
            Self::Omamori(2),
            Self::PenNib(0),
            Self::PreservedInsect,
            Self::PotionBelt,
            Self::RegalPillow,
            Self::SmilingMask,
            Self::Strawberry,
            Self::TheBoot,
            Self::TinyChest(0),
            Self::ToyOrnithopter,
            Self::Vajra,
            Self::WarPaint,
            Self::Whetstone,
        ]
    }
    pub fn shared_uncommon() -> Vec<Self> {
        vec![
            Self::BlueCandle,
            Self::BottledFlame,
            Self::BottledLightning,
            Self::BottledTornado,
            Self::DarkstonePeriapt,
            Self::EternalFeather,
            Self::FrozenEgg,
            Self::GremlinHorn,
            Self::HornCleat,
            Self::InkBottle(0),
            Self::Kunai(0),
            Self::LetterOpener(0),
            Self::Matryoshka(2),
            Self::MeatOnTheBone,
            Self::MercuryHourglass,
            Self::MoltenEgg,
            Self::SmoothStone,
            Self::MummifiedHand,
            Self::OrnamentalFan(0),
            Self::Pantograph,
            Self::Pear,
            Self::QuestionCard,
            Self::Shuriken(0),
            Self::SingingBowl,
            Self::StrikeDummy,
            Self::Sundial(0),
            Self::TheCourier,
            Self::ToxicEgg,
            Self::WhiteBeastStatue,
        ]
    }
    pub fn shared_rare() -> Vec<Self> {
        vec![
            Self::BirdFacedUrn,
            Self::Calipers,
            Self::CaptainsWheel,
            Self::DeadBranch,
            Self::DuVuDoll,
            Self::FossilizedHelix,
            Self::GamblersChip,
            Self::Ginger,
            Self::Girya(0),
            Self::IceCream,
            Self::IncenseBurner(0),
            Self::LizardTail(true),
            Self::Mango,
            Self::OldCoin,
            Self::PeacePipe,
            Self::Pocketwatch(0),
            Self::PrayerWheel,
            Self::Shovel,
            Self::StoneCalendar,
            Self::ThreadAndNeedle,
            Self::Torii,
            Self::TungstenRod,
            Self::Turnip,
            Self::UnceasingTop,
            Self::WingBoots,
        ]
    }
    pub fn shared_boss() -> Vec<Self> {
        vec![
            Self::Astrolabe,
            Self::BlackStar,
            Self::BrokenCrown,
            Self::CallingBell,
            Self::CoffeeDripper,
            Self::CursedKey,
            Self::Ectoplasm,
            Self::EmptyCage,
            Self::FusionHammer,
            Self::PandorasBox,
            Self::PhilosophersStone,
            Self::RunicDome,
            Self::RunicPyramid,
            Self::SacredBark,
            Self::SlaversCollar,
            Self::SneckoEye,
            Self::Sozu,
            Self::TinyHouse,
            Self::VelvetChoker,
        ]
    }
    pub fn shared_shop() -> Vec<Self> {
        vec![
            Self::Cauldron,
            Self::ChemicalX,
            Self::ClockworkSouvenir,
            Self::DollysMirror,
            Self::FrozenEye,
            Self::HandDrill,
            Self::LeesWaffle,
            Self::MedKit,
            Self::MembershipCard,
            Self::OrangePellets,
            Self::Orrery,
            Self::PrismaticShard,
            Self::SlingOfCourage,
            Self::StrangeSpoon,
            Self::Abacus,
            Self::Toolkit,
        ]
    }
    pub fn ironclad_common() -> Vec<Self> {
        vec![Self::RedSkull]
    }
    pub fn silent_common() -> Vec<Self> {
        vec![Self::SneckoSkull]
    }
    pub fn defect_common() -> Vec<Self> {
        vec![Self::DataDisk]
    }
    pub fn watcher_common() -> Vec<Self> {
        vec![Self::Damaru]
    }
    pub fn ironclad_uncommon() -> Vec<Self> {
        vec![Self::PaperPhrog, Self::SelfFormingClay]
    }
    pub fn silent_uncommon() -> Vec<Self> {
        vec![Self::NinjaScroll, Self::PaperKrane]
    }
    pub fn defect_uncommon() -> Vec<Self> {
        vec![Self::GoldPlatedCables, Self::SymbioticVirus]
    }
    pub fn watcher_uncommon() -> Vec<Self> {
        vec![Self::Duality, Self::TeardropLocket]
    }
    pub fn ironclad_rare() -> Vec<Self> {
        vec![Self::ChampionsBelt, Self::CharonsAshes, Self::MagicFlower]
    }
    pub fn silent_rare() -> Vec<Self> {
        vec![Self::TheSpecimen, Self::Tingsha, Self::ToughBandages]
    }
    pub fn defect_rare() -> Vec<Self> {
        vec![Self::EmotionChip]
    }
    pub fn watcher_rare() -> Vec<Self> {
        vec![Self::CloakClasp, Self::GoldenEye]
    }
    pub fn ironclad_shop() -> Vec<Self> {
        vec![Self::Brimstone]
    }
    pub fn silent_shop() -> Vec<Self> {
        vec![Self::TwistedFunnel]
    }
    pub fn defect_shop() -> Vec<Self> {
        vec![Self::RunicCapacitor]
    }
    pub fn watcher_shop() -> Vec<Self> {
        vec![Self::Melange]
    }
    pub fn ironclad_boss() -> Vec<Self> {
        vec![Self::BlackBlood, Self::MarkOfPain, Self::RunicCube]
    }
    pub fn silent_boss() -> Vec<Self> {
        vec![
            Self::RingOfTheSerpent,
            Self::WristBlade,
            Self::HoveringKite(true),
        ]
    }
    pub fn defect_boss() -> Vec<Self> {
        vec![
            Self::FrozenCore,
            Self::Inserter(false),
            Self::NuclearBattery,
        ]
    }
    pub fn watcher_boss() -> Vec<Self> {
        vec![Self::HolyWater, Self::VioletLotus]
    }
}
