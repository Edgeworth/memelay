use derive_more::Display;

use crate::types::Kc;

#[must_use]
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display("{:?}", keys)]
pub struct Layout {
    pub keys: Vec<Kc>,
}

impl Layout {
    pub fn new(keys: Vec<Kc>) -> Self {
        Self { keys }
    }

    pub fn size(&self) -> usize {
        self.keys.len()
    }
}

pub const QWERTY_KEYS: [Kc; 30] = [
    Kc::Q,
    Kc::W,
    Kc::E,
    Kc::R,
    Kc::T,
    Kc::Y,
    Kc::U,
    Kc::I,
    Kc::O,
    Kc::P,
    Kc::A,
    Kc::S,
    Kc::D,
    Kc::F,
    Kc::G,
    Kc::H,
    Kc::J,
    Kc::K,
    Kc::L,
    Kc::Semicolon,
    Kc::Z,
    Kc::X,
    Kc::C,
    Kc::V,
    Kc::B,
    Kc::N,
    Kc::M,
    Kc::Comma,
    Kc::Dot,
    Kc::Slash,
];

pub const COLEMAK_DHM_KEYS: [Kc; 30] = [
    Kc::Q,
    Kc::W,
    Kc::F,
    Kc::P,
    Kc::B,
    Kc::J,
    Kc::L,
    Kc::U,
    Kc::Y,
    Kc::Semicolon,
    Kc::A,
    Kc::R,
    Kc::S,
    Kc::T,
    Kc::G,
    Kc::M,
    Kc::N,
    Kc::E,
    Kc::I,
    Kc::O,
    Kc::Z,
    Kc::X,
    Kc::C,
    Kc::D,
    Kc::V,
    Kc::K,
    Kc::H,
    Kc::Comma,
    Kc::Dot,
    Kc::Slash,
];
