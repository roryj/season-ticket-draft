use std::{fmt::Debug, fmt::Display};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Drafter {
    Rory,
    Jordy,
    Connor,
    Dan,
}

impl Debug for Drafter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Drafter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rory => write!(f, "Rory"),
            Self::Jordy => write!(f, "Jordy"),
            Self::Connor => write!(f, "Connor"),
            Self::Dan => write!(f, "Dan"),
        }
    }
}
