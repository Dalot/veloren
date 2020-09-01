use crate::{make_case_elim, make_proj_elim};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

make_proj_elim!(
    body,
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct Body {
        pub torso: Torso,
        pub tail: Tail,
    }
);

impl Body {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self {
            torso: *(&ALL_TORSOS).choose(&mut rng).unwrap(),
            tail: *(&ALL_TAILS).choose(&mut rng).unwrap(),
        }
    }
}

make_case_elim!(
    torso,
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum Torso {
        Default = 0,
    }
);

const ALL_TORSOS: [Torso; 1] = [Torso::Default];

make_case_elim!(
    tail,
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum Tail {
        Default = 0,
    }
);

const ALL_TAILS: [Tail; 1] = [Tail::Default];
