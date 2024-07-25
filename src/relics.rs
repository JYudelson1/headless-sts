use rand::Rng;

use crate::utils::{number_between, Character};

pub struct Relics {
    pub list: Vec<Relic>,
    common_pool: Vec<Relic>,
    uncommon_pool: Vec<Relic>,
    rare_pool: Vec<Relic>,
    shop_pool: Vec<Relic>,
    boss_pool: Vec<Relic>,
}

impl Relics {
    // TODO: Some relics do not spawn at certain floors (e.g. shovel)
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

    pub fn contains(&self, relic: Relic) -> bool {
        for r in &self.list {
            if *r == relic {
                return true;
            }
        }
        return false;
    }

    pub fn reset_start_of_combat(&mut self) {
        for relic in &mut self.list {
            match relic {
                Relic::CentennialPuzzle(_) => *relic = Relic::CentennialPuzzle(true),
                _ => (),
            }
        }
    }

    pub fn reset_start_of_turn(&mut self) {
        for relic in &mut self.list {
            match relic {
                Relic::ArtOfWar(_) => *relic = Relic::ArtOfWar(true),
                Relic::LetterOpener(_) => *relic = Relic::LetterOpener(0),
                Relic::OrnamentalFan(_) => *relic = Relic::OrnamentalFan(0),
                Relic::Kunai(_) => *relic = Relic::Kunai(0),
                Relic::Shuriken(_) => *relic = Relic::Shuriken(0),
                Relic::Pocketwatch(_) => *relic = Relic::Pocketwatch(0),
                Relic::HoveringKite(_) => *relic = Relic::HoveringKite(true),
                _ => (),
            }
        }
    }

    pub fn add(&mut self, relic: Relic) {
        self.list.push(relic);
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn random_common(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.common_pool)
    }

    pub fn random_uncommon(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.uncommon_pool)
    }

    pub fn random_rare(&mut self) -> Relic {
        //TODO: If you have 2 out these 3 relics: Shovel ,PeacePipe, and Girya, the third one will not spawn
        Self::_random_remove_from_pool(&mut self.rare_pool)
    }

    pub fn random_shop(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.shop_pool)
    }

    pub fn random_boss(&mut self) -> Relic {
        Self::_random_remove_from_pool(&mut self.boss_pool)
    }

    pub fn random_elite(&mut self) -> Relic {
        let x = rand::random::<f32>();

        if x < 0.5 {
            self.random_common()
        } else if x < 0.83 {
            self.random_uncommon()
        } else {
            self.random_rare()
        }
    }

    fn _random_remove_from_pool(pool: &mut Vec<Relic>) -> Relic {
        if pool.is_empty() {
            return Relic::Circlet;
        }
        let index = number_between(0, pool.len());
        pool.remove(index)
    }

    pub fn has_valid_girya(&self) -> bool {
        for relic in &self.list {
            if let Relic::Girya(amt) = relic {
                if *amt < 3 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }

    pub fn increase_girya(&mut self) {
        for relic in &mut self.list {
            if let Relic::Girya(amt) = relic {
                *amt += 1;
                return;
            }
        }
    }

    pub fn turn_on_tea_set(&mut self) {
        for relic in &mut self.list {
            if let Relic::AncientTeaSet(inner) = relic {
                *inner = true;
                return;
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Relic {
    Circlet,                // IMPLEMENTED
    BurningBlood,           // IMPLEMENTED
    RingOfSnake,            // PARTIALLY IMPLEMENTED
    CrackedCore,            // PARTIALLY IMPLEMENTED
    PureWater,              // PARTIALLY IMPLEMENTED
    Akabeko,                // NOT IMPLEMENTED
    Anchor,                 // IMPLEMENTED
    AncientTeaSet(bool),    // IMPLEMENTED
    ArtOfWar(bool),         // PARTIALLY IMPLEMENTED
    BagOfMarbles,           // IMPLEMENTED
    BagOfPrep,              // IMPLEMENTED
    BloodVial,              // IMPLEMENTED
    BronzeScales,           // PARTIALLY IMPLEMENTED
    CentennialPuzzle(bool), // NOT IMPLEMENTED
    CeramicFish,            // NOT IMPLEMENTED
    Dreamcatcher,           // NOT IMPLEMENTED
    HappyFlower(u8),        // NOT IMPLEMENTED
    JuzuBracelet,           // NOT IMPLEMENTED
    Lantern,                // IMPLEMENTED
    MawBank(bool),          // NOT IMPLEMENTED
    MealTicket,             // NOT IMPLEMENTED
    Nunchaku(u8),           // NOT IMPLEMENTED
    SmoothStone,            // IMPLEMENTED
    Omamori(u8),            // NOT IMPLEMENTED
    Orichalcum,             // NOT IMPLEMENTED
    PenNib(u8),             // NOT IMPLEMENTED
    PotionBelt,             // IMPLEMENTED
    PreservedInsect,        // NOT IMPLEMENTED
    RegalPillow,            // NOT IMPLEMENTED
    SmilingMask,            // NOT IMPLEMENTED
    Strawberry,             // IMPLEMENTED
    TheBoot,                // NOT IMPLEMENTED
    TinyChest(u8),          // NOT IMPLEMENTED
    ToyOrnithopter,         // NOT IMPLEMENTED
    Vajra,                  // NOT IMPLEMENTED
    WarPaint,               // NOT IMPLEMENTED
    Whetstone,              // NOT IMPLEMENTED
    RedSkull,               // NOT IMPLEMENTED
    SneckoSkull,            // NOT IMPLEMENTED
    DataDisk,               // IMPLEMENTED
    Damaru,                 // NOT IMPLEMENTED
    BlueCandle,             // NOT IMPLEMENTED
    //TODO: Figure these out
    BottledFlame,      // NOT IMPLEMENTED
    BottledLightning,  // NOT IMPLEMENTED
    BottledTornado,    // NOT IMPLEMENTED
    DarkstonePeriapt,  // NOT IMPLEMENTED
    EternalFeather,    // NOT IMPLEMENTED
    FrozenEgg,         // NOT IMPLEMENTED
    GremlinHorn,       // NOT IMPLEMENTED
    HornCleat,         // NOT IMPLEMENTED
    InkBottle(u8),     // NOT IMPLEMENTED
    Kunai(u8),         // NOT IMPLEMENTED
    LetterOpener(u8),  // NOT IMPLEMENTED
    Matryoshka(u8),    // NOT IMPLEMENTED
    MeatOnTheBone,     // NOT IMPLEMENTED
    MercuryHourglass,  // NOT IMPLEMENTED
    MoltenEgg,         // NOT IMPLEMENTED
    MummifiedHand,     // NOT IMPLEMENTED
    OrnamentalFan(u8), // NOT IMPLEMENTED
    Pantograph,        // NOT IMPLEMENTED
    Pear,              // IMPLEMENTED
    QuestionCard,      // IMPLEMENTED
    Shuriken(u8),      // NOT IMPLEMENTED
    SingingBowl,       // NOT IMPLEMENTED
    StrikeDummy,       // NOT IMPLEMENTED
    Sundial(u8),       // NOT IMPLEMENTED
    TheCourier,        // NOT IMPLEMENTED
    ToxicEgg,          // NOT IMPLEMENTED
    WhiteBeastStatue,  // NOT IMPLEMENTED
    PaperPhrog,        // IMPLEMENTED
    SelfFormingClay,   // NOT IMPLEMENTED
    NinjaScroll,       // PARTIALLY IMPLEMENTED
    PaperKrane,        // IMPLEMENTED
    GoldPlatedCables,  // NOT IMPLEMENTED
    SymbioticVirus,    // PARTIALLY IMPLEMENTED
    Duality,           // NOT IMPLEMENTED
    TeardropLocket,    // NOT IMPLEMENTED
    BirdFacedUrn,      // NOT IMPLEMENTED
    Calipers,          // NOT IMPLEMENTED
    CaptainsWheel,     // NOT IMPLEMENTED
    DeadBranch,        // NOT IMPLEMENTED
    DuVuDoll,          // NOT IMPLEMENTED
    FossilizedHelix,   // NOT IMPLEMENTED
    GamblersChip,      // NOT IMPLEMENTED
    Ginger,            // IMPLEMENTED
    Girya(u8),         // IMPLEMENTED
    IceCream,          // NOT IMPLEMENTED
    IncenseBurner(u8), // NOT IMPLEMENTED
    LizardTail(bool),  // NOT IMPLEMENTED
    Mango,             // IMPLEMENTED
    OldCoin,           // IMPLEMENTED
    PeacePipe,         // PARTIALLY IMPLEMENTED
    Pocketwatch(u8),   // NOT IMPLEMENTED
    PrayerWheel,       // IMPLEMENTED
    Shovel,            // NOT IMPLEMENTED
    StoneCalendar,     // NOT IMPLEMENTED
    ThreadAndNeedle,   // NOT IMPLEMENTED
    Torii,             // NOT IMPLEMENTED
    TungstenRod,       // NOT IMPLEMENTED
    Turnip,            // IMPLEMENTED
    UnceasingTop,      // NOT IMPLEMENTED
    WingBoots,         // NOT IMPLEMENTED
    ChampionsBelt,     // NOT IMPLEMENTED
    CharonsAshes,      // NOT IMPLEMENTED
    MagicFlower,       // IMPLEMENTED
    TheSpecimen,       // NOT IMPLEMENTED
    Tingsha,           // NOT IMPLEMENTED
    ToughBandages,     // NOT IMPLEMENTED
    EmotionChip,       // NOT IMPLEMENTED
    CloakClasp,        // NOT IMPLEMENTED
    GoldenEye,         // NOT IMPLEMENTED
    Cauldron,          // NOT IMPLEMENTED
    ChemicalX,         // NOT IMPLEMENTED
    ClockworkSouvenir, // NOT IMPLEMENTED
    DollysMirror,      // NOT IMPLEMENTED
    FrozenEye,         // NOT IMPLEMENTED
    HandDrill,         // NOT IMPLEMENTED
    LeesWaffle,        // IMPLEMENTED
    MedKit,            // NOT IMPLEMENTED
    MembershipCard,    // NOT IMPLEMENTED
    // TODO: Figure out pellets
    OrangePellets,          // NOT IMPLEMENTED
    Orrery,                 // NOT IMPLEMENTED
    PrismaticShard,         // NOT IMPLEMENTED
    SlingOfCourage,         // NOT IMPLEMENTED
    StrangeSpoon,           // NOT IMPLEMENTED
    Abacus,                 // NOT IMPLEMENTED
    Toolkit,                // NOT IMPLEMENTED
    Brimstone,              // NOT IMPLEMENTED
    TwistedFunnel,          // NOT IMPLEMENTED
    RunicCapacitor,         // PARTIALLY IMPLEMENTED
    Melange,                // NOT IMPLEMENTED
    Astrolabe,              // NOT IMPLEMENTED
    BlackStar,              // NOT IMPLEMENTED
    BrokenCrown,            // IMPLEMENTED
    CallingBell,            // NOT IMPLEMENTED
    CoffeeDripper,          // IMPLEMENTED
    CursedKey,              // PARTIALLY IMPLEMENTED
    Ectoplasm,              // PARTIALLY IMPLEMENTED
    EmptyCage,              // NOT IMPLEMENTED
    FusionHammer,           // IMPLEMENTED
    PandorasBox,            // NOT IMPLEMENTED
    PhilosophersStone,      // PARTIALLY IMPLEMENTED
    RunicDome,              // PARTIALLY IMPLEMENTED
    RunicPyramid,           // NOT IMPLEMENTED
    SacredBark,             // NOT IMPLEMENTED
    SlaversCollar,          // PARTIALLY IMPLEMENTED
    SneckoEye,              // NOT IMPLEMENTED
    Sozu,                   // PARTIALLY IMPLEMENTED
    TinyHouse,              // NOT IMPLEMENTED
    VelvetChoker,           // PARTIALLY IMPLEMENTED
    MarkOfPain,             // PARTIALLY IMPLEMENTED
    BlackBlood,             // IMPLEMENTED
    RunicCube,              // NOT IMPLEMENTED
    RingOfTheSerpent,       // NOT IMPLEMENTED
    WristBlade,             // NOT IMPLEMENTED
    HoveringKite(bool),     // NOT IMPLEMENTED
    NuclearBattery,         // PARTIALLY IMPLEMENTED
    Inserter(bool),         // NOT IMPLEMENTED
    FrozenCore,             // PARTIALLY IMPLEMENTED
    HolyWater,              // PARTIALLY IMPLEMENTED
    VioletLotus,            // NOT IMPLEMENTED
    BloodyIdol,             // NOT IMPLEMENTED
    CultistHeadpiece,       // IMPLEMENTED
    Enchiridion,            // NOT IMPLEMENTED
    ClericMask,             // NOT IMPLEMENTED
    GoldenIdol,             // NOT IMPLEMENTED
    GremlinVisage,          // NOT IMPLEMENTED
    MarkOfTheBloom,         // IMPLEMENTED
    MutagenicStrength,      // NOT IMPLEMENTED
    NlothsGift,             // NOT IMPLEMENTED
    NlothsHungryFace(bool), // NOT IMPLEMENTED
    Necronomicon,           // NOT IMPLEMENTED
    NeowsLament(u8),        // NOT IMPLEMENTED
    NilrysCodex(u8),        // NOT IMPLEMENTED
    OddMushroom,            // NOT IMPLEMENTED
    RedMask,                // IMPLEMENTED
    SpiritPoop,             // IMPLEMENTED
    SerpentHead,            // NOT IMPLEMENTED
    WarpedTongs,            // NOT IMPLEMENTED
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
            Self::MealTicket,
            Self::MawBank(true),
            Self::Nunchaku(0),
            Self::SmoothStone,
            Self::Orichalcum,
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
