use crate::models::key_automata::KeyAutomata;
use crate::models::layer::{Layer, Layout};
use crate::models::Model;
use crate::prelude::*;
use crate::types::{KCSet, KeyEv, PhysEv, KC};
use enumset::enum_set;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref US_LAYER: Layer = Layer::new(&[
        enum_set!(KC::Num0),
        enum_set!(KC::Num1),
        enum_set!(KC::Num2),
        enum_set!(KC::Num3),
        enum_set!(KC::Num4),
        enum_set!(KC::Num5),
        enum_set!(KC::Num6),
        enum_set!(KC::Num7),
        enum_set!(KC::Num8),
        enum_set!(KC::Num9),
        enum_set!(KC::A),
        enum_set!(KC::B),
        enum_set!(KC::C),
        enum_set!(KC::D),
        enum_set!(KC::E),
        enum_set!(KC::F),
        enum_set!(KC::G),
        enum_set!(KC::H),
        enum_set!(KC::I),
        enum_set!(KC::J),
        enum_set!(KC::K),
        enum_set!(KC::L),
        enum_set!(KC::M),
        enum_set!(KC::N),
        enum_set!(KC::O),
        enum_set!(KC::P),
        enum_set!(KC::Q),
        enum_set!(KC::R),
        enum_set!(KC::S),
        enum_set!(KC::T),
        enum_set!(KC::U),
        enum_set!(KC::V),
        enum_set!(KC::W),
        enum_set!(KC::X),
        enum_set!(KC::Y),
        enum_set!(KC::Z),
        enum_set!(KC::F1),
        enum_set!(KC::F2),
        enum_set!(KC::F3),
        enum_set!(KC::F4),
        enum_set!(KC::F5),
        enum_set!(KC::F6),
        enum_set!(KC::F7),
        enum_set!(KC::F8),
        enum_set!(KC::F9),
        enum_set!(KC::F10),
        enum_set!(KC::F11),
        enum_set!(KC::F12),
        enum_set!(KC::Enter),
        enum_set!(KC::Esc),
        enum_set!(KC::Backspace),
        enum_set!(KC::Tab),
        enum_set!(KC::Space),
        enum_set!(KC::Insert),
        enum_set!(KC::Delete),
        enum_set!(KC::Home),
        enum_set!(KC::End),
        enum_set!(KC::PageUp),
        enum_set!(KC::PageDn),
        enum_set!(KC::Up),
        enum_set!(KC::Down),
        enum_set!(KC::Left),
        enum_set!(KC::Right),
        enum_set!(KC::NumLock),
        enum_set!(KC::ScrollLock),
        enum_set!(KC::MediaVolDown),
        enum_set!(KC::Pause),
        enum_set!(KC::App),
        enum_set!(KC::Minus),
        enum_set!(KC::Equals),
        enum_set!(KC::LeftBracket),
        enum_set!(KC::RightBracket),
        enum_set!(KC::Backslash),
        enum_set!(KC::Semicolon),
        enum_set!(KC::Quote),
        enum_set!(KC::Grave),
        enum_set!(KC::Comma),
        enum_set!(KC::Dot),
        enum_set!(KC::Slash),
        enum_set!(KC::Alt),
        enum_set!(KC::Ctrl),
        enum_set!(KC::Shift),
        enum_set!(KC::Super),
        enum_set!(KC::Alt),
        enum_set!(KC::Ctrl),
        enum_set!(KC::Shift),
    ]);
    pub static ref US_LAYOUT: Layout = Layout::new().with_layer(US_LAYER.clone());
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct USModel {
    layout: &'static Layout,
    ks: KeyAutomata,
}

impl USModel {
    pub fn new() -> Self {
        Self { layout: &US_LAYOUT, ks: KeyAutomata::new() }
    }

    fn get_key(&self, phys: u32) -> KCSet {
        self.layout.layers[0].keys[phys as usize]
    }
}

impl Model for USModel {
    fn valid(&mut self, pev: PhysEv) -> bool {
        let kev = KeyEv::new(self.get_key(pev.phys), pev.press);
        self.ks.valid(kev)
    }

    fn event(&mut self, pev: PhysEv) -> Vec<KCSet> {
        self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.press))
    }
}
