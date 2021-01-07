use crate::models::keyset::KeySet;
use crate::models::layer::{Layer, Layout};
use crate::models::Model;
use crate::prelude::*;
use crate::types::{Key, KeyEv, Mod, PhysEv, KC};
use enumset::enum_set;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref US_LAYER: Layer = Layer::new(&[
        Key::new(KC::Num0, enum_set!()),
        Key::new(KC::Num1, enum_set!()),
        Key::new(KC::Num2, enum_set!()),
        Key::new(KC::Num3, enum_set!()),
        Key::new(KC::Num4, enum_set!()),
        Key::new(KC::Num5, enum_set!()),
        Key::new(KC::Num6, enum_set!()),
        Key::new(KC::Num7, enum_set!()),
        Key::new(KC::Num8, enum_set!()),
        Key::new(KC::Num9, enum_set!()),
        Key::new(KC::A, enum_set!()),
        Key::new(KC::B, enum_set!()),
        Key::new(KC::C, enum_set!()),
        Key::new(KC::D, enum_set!()),
        Key::new(KC::E, enum_set!()),
        Key::new(KC::F, enum_set!()),
        Key::new(KC::G, enum_set!()),
        Key::new(KC::H, enum_set!()),
        Key::new(KC::I, enum_set!()),
        Key::new(KC::J, enum_set!()),
        Key::new(KC::K, enum_set!()),
        Key::new(KC::L, enum_set!()),
        Key::new(KC::M, enum_set!()),
        Key::new(KC::N, enum_set!()),
        Key::new(KC::O, enum_set!()),
        Key::new(KC::P, enum_set!()),
        Key::new(KC::Q, enum_set!()),
        Key::new(KC::R, enum_set!()),
        Key::new(KC::S, enum_set!()),
        Key::new(KC::T, enum_set!()),
        Key::new(KC::U, enum_set!()),
        Key::new(KC::V, enum_set!()),
        Key::new(KC::W, enum_set!()),
        Key::new(KC::X, enum_set!()),
        Key::new(KC::Y, enum_set!()),
        Key::new(KC::Z, enum_set!()),
        Key::new(KC::F1, enum_set!()),
        Key::new(KC::F2, enum_set!()),
        Key::new(KC::F3, enum_set!()),
        Key::new(KC::F4, enum_set!()),
        Key::new(KC::F5, enum_set!()),
        Key::new(KC::F6, enum_set!()),
        Key::new(KC::F7, enum_set!()),
        Key::new(KC::F8, enum_set!()),
        Key::new(KC::F9, enum_set!()),
        Key::new(KC::F10, enum_set!()),
        Key::new(KC::F11, enum_set!()),
        Key::new(KC::F12, enum_set!()),
        Key::new(KC::Enter, enum_set!()),
        Key::new(KC::Esc, enum_set!()),
        Key::new(KC::Backspace, enum_set!()),
        Key::new(KC::Tab, enum_set!()),
        Key::new(KC::Space, enum_set!()),
        Key::new(KC::Insert, enum_set!()),
        Key::new(KC::Delete, enum_set!()),
        Key::new(KC::Home, enum_set!()),
        Key::new(KC::End, enum_set!()),
        Key::new(KC::PageUp, enum_set!()),
        Key::new(KC::PageDn, enum_set!()),
        Key::new(KC::Up, enum_set!()),
        Key::new(KC::Down, enum_set!()),
        Key::new(KC::Left, enum_set!()),
        Key::new(KC::Right, enum_set!()),
        Key::new(KC::NumLock, enum_set!()),
        Key::new(KC::ScrollLock, enum_set!()),
        Key::new(KC::MediaVolDown, enum_set!()),
        Key::new(KC::Pause, enum_set!()),
        Key::new(KC::App, enum_set!()),
        Key::new(KC::Minus, enum_set!()),
        Key::new(KC::Equals, enum_set!()),
        Key::new(KC::LeftBracket, enum_set!()),
        Key::new(KC::RightBracket, enum_set!()),
        Key::new(KC::Backslash, enum_set!()),
        Key::new(KC::Semicolon, enum_set!()),
        Key::new(KC::Quote, enum_set!()),
        Key::new(KC::Grave, enum_set!()),
        Key::new(KC::Comma, enum_set!()),
        Key::new(KC::Dot, enum_set!()),
        Key::new(KC::Slash, enum_set!()),
        Key::new(KC::None, enum_set!(Mod::Alt)),
        Key::new(KC::None, enum_set!(Mod::Ctrl)),
        Key::new(KC::None, enum_set!(Mod::Shift)),
        Key::new(KC::None, enum_set!(Mod::Super)),
        Key::new(KC::None, enum_set!(Mod::Alt)),
        Key::new(KC::None, enum_set!(Mod::Ctrl)),
        Key::new(KC::None, enum_set!(Mod::Shift)),
    ]);
    pub static ref US_LAYOUT: Layout = Layout::new().with_layer(US_LAYER.clone());
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct USModel {
    layout: &'static Layout,
    ks: KeySet,
}

impl USModel {
    pub fn new() -> Self {
        Self { layout: &US_LAYOUT, ks: KeySet::new() }
    }

    fn get_key(&self, phys: u32) -> Key {
        self.layout.layers[0].keys[phys as usize]
    }
}

impl Model for USModel {
    fn valid(&mut self, pev: PhysEv) -> bool {
        let kev = KeyEv::new(self.get_key(pev.phys), pev.count);
        self.ks.valid(kev)
    }

    fn event(&mut self, pev: PhysEv) -> Vec<KeyEv> {
        self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.count))
    }
}
