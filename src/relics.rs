use crate::utils::{number_between, Character};

#[derive(Debug, Clone)]
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
        let index = number_between(0, pool.len() - 1);
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

    pub fn tiny_chest_is_on(&self) -> bool {
        for relic in &self.list {
            if let Relic::TinyChest(inner) = relic {
                if *inner == 3 {
                    return true;
                }
            }
        }
        false
    }

    pub fn increase_tiny_chest(&mut self) {
        for relic in &mut self.list {
            if let Relic::TinyChest(inner) = relic {
                if *inner == 3 {
                    *inner = 0
                } else {
                    *inner += 1
                }
            }
        }
    }

    pub fn try_use_omamori(&mut self) -> bool {
        for relic in &mut self.list {
            if let Relic::Omamori(charges) = relic {
                if *charges > 0 {
                    *charges -= 1;
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }

    pub fn trigger_neow(&mut self) -> bool {
        for relic in &mut self.list {
            if let Relic::NeowsLament(charges) = relic {
                if *charges > 0 {
                    *charges -= 1;
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Relic {
    Circlet,                // IMPLEMENTED
    BurningBlood,           // IMPLEMENTED
    RingOfSnake,            // IMPLEMENTED
    CrackedCore,            // TODO: PARTIALLY IMPLEMENTED
    PureWater,              // TODO: PARTIALLY IMPLEMENTED
    Akabeko,                // TODO: NOT IMPLEMENTED
    Anchor,                 // IMPLEMENTED
    AncientTeaSet(bool),    // IMPLEMENTED
    ArtOfWar(bool),         // TODO: PARTIALLY IMPLEMENTED
    BagOfMarbles,           // IMPLEMENTED
    BagOfPrep,              // IMPLEMENTED
    BloodVial,              // IMPLEMENTED
    BronzeScales,           // TODO: PARTIALLY IMPLEMENTED
    CentennialPuzzle(bool), // TODO: NOT IMPLEMENTED
    CeramicFish,            // IMPLEMENTED
    Dreamcatcher,           // TODO: NOT IMPLEMENTED
    HappyFlower(u8),        // TODO: NOT IMPLEMENTED
    JuzuBracelet,           // IMPLEMENTED
    Lantern,                // IMPLEMENTED
    MawBank(bool),          // TODO: NOT IMPLEMENTED
    MealTicket,             // IMPLEMENTED
    Nunchaku(u8),           // TODO: NOT IMPLEMENTED
    SmoothStone,            // IMPLEMENTED
    Omamori(u8),            // IMPLEMENTED
    Orichalcum,             // IMPLEMENTED
    PenNib(u8),             // TODO: NOT IMPLEMENTED
    PotionBelt,             // IMPLEMENTED
    PreservedInsect,        // TODO: NOT IMPLEMENTED
    RegalPillow,            // TODO: NOT IMPLEMENTED
    SmilingMask,            // IMPLEMENTED
    Strawberry,             // IMPLEMENTED
    TheBoot,                // IMPLEMENTED
    TinyChest(u8),          // IMPLEMENTED
    ToyOrnithopter,         // IMPLEMENTED
    Vajra,                  // IMPLEMENTED
    WarPaint,               // TODO: NOT IMPLEMENTED
    Whetstone,              // TODO: NOT IMPLEMENTED
    RedSkull,               // TODO: NOT IMPLEMENTED
    SneckoSkull,            // TODO: NOT IMPLEMENTED
    DataDisk,               // IMPLEMENTED
    Damaru,                 // TODO: NOT IMPLEMENTED
    BlueCandle,             // TODO: NOT IMPLEMENTED
    BottledFlame,           // TODO: NOT IMPLEMENTED
    BottledLightning,       // TODO: NOT IMPLEMENTED
    BottledTornado,         // TODO: NOT IMPLEMENTED
    DarkstonePeriapt,       // IMPLEMENTED
    EternalFeather,         // TODO: NOT IMPLEMENTED
    FrozenEgg,              // IMPLEMENTED
    GremlinHorn,            // TODO: NOT IMPLEMENTED
    HornCleat,              // TODO: NOT IMPLEMENTED
    InkBottle(u8),          // TODO: NOT IMPLEMENTED
    Kunai(u8),              // TODO: NOT IMPLEMENTED
    LetterOpener(u8),       // TODO: NOT IMPLEMENTED
    Matryoshka(u8),         // TODO: NOT IMPLEMENTED
    MeatOnTheBone,          // TODO: NOT IMPLEMENTED
    MercuryHourglass,       // TODO: NOT IMPLEMENTED
    MoltenEgg,              // IMPLEMENTED
    MummifiedHand,          // TODO: NOT IMPLEMENTED
    OrnamentalFan(u8),      // TODO: NOT IMPLEMENTED
    Pantograph,             // IMPLEMENTED
    Pear,                   // IMPLEMENTED
    QuestionCard,           // IMPLEMENTED
    Shuriken(u8),           // TODO: NOT IMPLEMENTED
    SingingBowl,            // TODO: NOT IMPLEMENTED
    StrikeDummy,            // TODO: NOT IMPLEMENTED
    Sundial(u8),            // TODO: NOT IMPLEMENTED
    TheCourier,             // TODO: NOT IMPLEMENTED
    ToxicEgg,               // IMPLEMENTED
    WhiteBeastStatue,       // MPLEMENTED
    PaperPhrog,             // IMPLEMENTED
    SelfFormingClay,        // TODO: NOT IMPLEMENTED
    NinjaScroll,            // TODO: PARTIALLY IMPLEMENTED
    PaperKrane,             // IMPLEMENTED
    GoldPlatedCables,       // TODO: NOT IMPLEMENTED
    SymbioticVirus,         // TODO: PARTIALLY IMPLEMENTED
    Duality,                // TODO: NOT IMPLEMENTED
    TeardropLocket,         // TODO: NOT IMPLEMENTED
    BirdFacedUrn,           // TODO: NOT IMPLEMENTED
    Calipers,               // TODO: NOT IMPLEMENTED
    CaptainsWheel,          // TODO: NOT IMPLEMENTED
    DeadBranch,             // TODO: NOT IMPLEMENTED
    DuVuDoll,               // TODO: NOT IMPLEMENTED
    FossilizedHelix,        // TODO: NOT IMPLEMENTED
    GamblersChip,           // TODO: NOT IMPLEMENTED
    Ginger,                 // IMPLEMENTED
    Girya(u8),              // IMPLEMENTED
    IceCream,               // IMPLEMENTED
    IncenseBurner(u8),      // TODO: NOT IMPLEMENTED
    LizardTail(bool),       // TODO: NOT IMPLEMENTED
    Mango,                  // IMPLEMENTED
    OldCoin,                // IMPLEMENTED
    PeacePipe,              // IMPLEMENTED
    Pocketwatch(u8),        // TODO: NOT IMPLEMENTED
    PrayerWheel,            // IMPLEMENTED
    Shovel,                 // IMPLEMENTED
    StoneCalendar,          // TODO: NOT IMPLEMENTED
    ThreadAndNeedle,        // TODO: NOT IMPLEMENTED
    Torii,                  // TODO: NOT IMPLEMENTED
    TungstenRod,            // IMPLEMENTED
    Turnip,                 // IMPLEMENTED
    UnceasingTop,           // TODO: NOT IMPLEMENTED
    WingBoots,              // TODO: NOT IMPLEMENTED
    ChampionsBelt,          // TODO: NOT IMPLEMENTED
    CharonsAshes,           // TODO: NOT IMPLEMENTED
    MagicFlower,            // IMPLEMENTED
    TheSpecimen,            // TODO: NOT IMPLEMENTED
    Tingsha,                // TODO: NOT IMPLEMENTED
    ToughBandages,          // TODO: NOT IMPLEMENTED
    EmotionChip,            // TODO: NOT IMPLEMENTED
    CloakClasp,             // TODO: NOT IMPLEMENTED
    GoldenEye,              // TODO: NOT IMPLEMENTED
    Cauldron,               // TODO: NOT IMPLEMENTED
    ChemicalX,              // TODO: NOT IMPLEMENTED
    ClockworkSouvenir,      // TODO: NOT IMPLEMENTED
    DollysMirror,           // TODO: NOT IMPLEMENTED
    FrozenEye,              // TODO: NOT IMPLEMENTED
    HandDrill,              // TODO: NOT IMPLEMENTED
    LeesWaffle,             // IMPLEMENTED
    MedKit,                 // TODO: NOT IMPLEMENTED
    MembershipCard,         // IMPLEMENTED
    // TODO: Figure out pellets
    OrangePellets(bool, bool, bool), // TODO: NOT IMPLEMENTED
    Orrery,                          // TODO: NOT IMPLEMENTED
    PrismaticShard,                  // TODO: NOT IMPLEMENTED
    SlingOfCourage,                  // IMPLEMENTED
    StrangeSpoon,                    // TODO: NOT IMPLEMENTED
    Abacus,                          // TODO: NOT IMPLEMENTED
    Toolkit,                         // TODO: NOT IMPLEMENTED
    Brimstone,                       // TODO: NOT IMPLEMENTED
    TwistedFunnel,                   // TODO: NOT IMPLEMENTED
    RunicCapacitor,                  // TODO: PARTIALLY IMPLEMENTED
    Melange,                         // TODO: NOT IMPLEMENTED
    Astrolabe,                       // TODO: NOT IMPLEMENTED
    BlackStar,                       // TODO: NOT IMPLEMENTED
    BrokenCrown,                     // IMPLEMENTED
    CallingBell,                     // TODO: NOT IMPLEMENTED
    CoffeeDripper,                   // IMPLEMENTED
    CursedKey,                       // TODO: PARTIALLY IMPLEMENTED
    Ectoplasm,                       // TODO: PARTIALLY IMPLEMENTED
    EmptyCage,                       // TODO: NOT IMPLEMENTED
    FusionHammer,                    // IMPLEMENTED
    PandorasBox,                     // TODO: NOT IMPLEMENTED
    PhilosophersStone,               // TODO: PARTIALLY IMPLEMENTED
    RunicDome,                       // TODO: PARTIALLY IMPLEMENTED
    RunicPyramid,                    // IMPLEMENTED
    SacredBark,                      // TODO: NOT IMPLEMENTED
    SlaversCollar,                   // TODO: PARTIALLY IMPLEMENTED
    SneckoEye,                       // TODO: NOT IMPLEMENTED
    Sozu,                            // TODO: PARTIALLY IMPLEMENTED
    TinyHouse,                       // TODO: NOT IMPLEMENTED
    VelvetChoker,                    // TODO: PARTIALLY IMPLEMENTED
    MarkOfPain,                      // TODO: PARTIALLY IMPLEMENTED
    BlackBlood,                      // IMPLEMENTED
    RunicCube,                       // TODO: NOT IMPLEMENTED
    RingOfTheSerpent,                // TODO: NOT IMPLEMENTED
    WristBlade,                      // TODO: NOT IMPLEMENTED
    HoveringKite(bool),              // TODO: NOT IMPLEMENTED
    NuclearBattery,                  // TODO: PARTIALLY IMPLEMENTED
    Inserter(bool),                  // TODO: NOT IMPLEMENTED
    FrozenCore,                      // TODO: PARTIALLY IMPLEMENTED
    HolyWater,                       // TODO: PARTIALLY IMPLEMENTED
    VioletLotus,                     // TODO: NOT IMPLEMENTED
    BloodyIdol,                      // TODO: NOT IMPLEMENTED
    CultistHeadpiece,                // IMPLEMENTED
    Enchiridion,                     // TODO: NOT IMPLEMENTED
    ClericMask,                      // TODO: NOT IMPLEMENTED
    GoldenIdol,                      // TODO: NOT IMPLEMENTED
    GremlinVisage,                   // TODO: NOT IMPLEMENTED
    MarkOfTheBloom,                  // IMPLEMENTED
    MutagenicStrength,               // TODO: NOT IMPLEMENTED
    NlothsGift,                      // TODO: NOT IMPLEMENTED
    NlothsHungryFace(bool),          // TODO: NOT IMPLEMENTED
    Necronomicon,                    // TODO: NOT IMPLEMENTED
    NeowsLament(u8),                 // IMPLEMENTED
    NilrysCodex(u8),                 // TODO: NOT IMPLEMENTED
    OddMushroom,                     // TODO: NOT IMPLEMENTED
    RedMask,                         // IMPLEMENTED
    SpiritPoop,                      // IMPLEMENTED
    SerpentHead,                     // IMPLEMENTED
    WarpedTongs,                     // TODO: NOT IMPLEMENTED
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
            Self::OrangePellets(false, false, false),
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
