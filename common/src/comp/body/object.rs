use rand::{seq::SliceRandom, thread_rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Body {
    Bomb,
    Scarecrow,
    ChestVines,
    Chest,
    ChestDark,
    ChestDemon,
    ChestGold,
    ChestLight,
    ChestOpen,
    ChestSkull,
    Pumpkin1,
    Pumpkin2,
    Pumpkin3,
    Pumpkin4,
    Pumpkin5,
    Campfire,
    LanternGround,
    LanternGroundOpen,
    LanternStanding2,
    LanternStanding,
    PotionBlue,
    PotionGreen,
    PotionRed,
    Crate,
}

impl Body {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        *(&ALL_OBJECTS).choose(&mut rng).unwrap()
    }
}

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
const ALL_OBJECTS: [Body; 24] = [
    Body::Bomb,
    Body::Scarecrow,
    Body::ChestVines,
    Body::Chest,
    Body::ChestDark,
    Body::ChestDemon,
    Body::ChestGold,
    Body::ChestLight,
    Body::ChestOpen,
    Body::ChestSkull,
    Body::Pumpkin1,
    Body::Pumpkin2,
    Body::Pumpkin3,
    Body::Pumpkin4,
    Body::Pumpkin5,
    Body::Campfire,
    Body::LanternGround,
    Body::LanternGroundOpen,
    Body::LanternStanding,
    Body::LanternStanding2,
    Body::PotionRed,
    Body::PotionBlue,
    Body::PotionGreen,
    Body::Crate,
];
=======
const ALL_OBJECTS: [Body; 5] = [Body::Bomb, Body::Scarecrow, Body::Chest, Body::Pumpkin, Body::Campfire];
>>>>>>> Add campfire
=======
const ALL_OBJECTS: [Body; 5] = [
=======
const ALL_OBJECTS: [Body; 12] = [
>>>>>>> added lanterns and potions
=======
const ALL_OBJECTS: [Body; 16] = [
>>>>>>> MOAR Pumpkins!
    Body::Bomb,
    Body::Scarecrow,
    Body::Chest,
    Body::Pumpkin1,
    Body::Pumpkin2,
    Body::Pumpkin3,
    Body::Pumpkin4,
    Body::Pumpkin5,
    Body::Campfire,
    Body::LanternGround,
    Body::LanternGroundOpen,
    Body::LanternStanding,
    Body::LanternStanding2,
    Body::PotionRed,
    Body::PotionBlue,
    Body::PotionGreen,
];
>>>>>>> formatting
