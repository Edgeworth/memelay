use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{Kc, KcSet, KeyEv, PhysEv};
use derive_more::Display;
use enumset::enum_set;
use lazy_static::lazy_static;
use smallvec::SmallVec;

lazy_static! {
    pub static ref US_LAYOUT: Layout = Layout::new(&[
        enum_set!(Kc::Num0),
        enum_set!(Kc::Num1),
        enum_set!(Kc::Num2),
        enum_set!(Kc::Num3),
        enum_set!(Kc::Num4),
        enum_set!(Kc::Num5),
        enum_set!(Kc::Num6),
        enum_set!(Kc::Num7),
        enum_set!(Kc::Num8),
        enum_set!(Kc::Num9),
        enum_set!(Kc::A),
        enum_set!(Kc::B),
        enum_set!(Kc::C),
        enum_set!(Kc::D),
        enum_set!(Kc::E),
        enum_set!(Kc::F),
        enum_set!(Kc::G),
        enum_set!(Kc::H),
        enum_set!(Kc::I),
        enum_set!(Kc::J),
        enum_set!(Kc::K),
        enum_set!(Kc::L),
        enum_set!(Kc::M),
        enum_set!(Kc::N),
        enum_set!(Kc::O),
        enum_set!(Kc::P),
        enum_set!(Kc::Q),
        enum_set!(Kc::R),
        enum_set!(Kc::S),
        enum_set!(Kc::T),
        enum_set!(Kc::U),
        enum_set!(Kc::V),
        enum_set!(Kc::W),
        enum_set!(Kc::X),
        enum_set!(Kc::Y),
        enum_set!(Kc::Z),
        enum_set!(Kc::F1),
        enum_set!(Kc::F2),
        enum_set!(Kc::F3),
        enum_set!(Kc::F4),
        enum_set!(Kc::F5),
        enum_set!(Kc::F6),
        enum_set!(Kc::F7),
        enum_set!(Kc::F8),
        enum_set!(Kc::F9),
        enum_set!(Kc::F10),
        enum_set!(Kc::F11),
        enum_set!(Kc::F12),
        enum_set!(Kc::Enter),
        enum_set!(Kc::Esc),
        enum_set!(Kc::Backspace),
        enum_set!(Kc::Tab),
        enum_set!(Kc::Space),
        enum_set!(Kc::Insert),
        enum_set!(Kc::Delete),
        enum_set!(Kc::Home),
        enum_set!(Kc::End),
        enum_set!(Kc::PageUp),
        enum_set!(Kc::PageDn),
        enum_set!(Kc::Up),
        enum_set!(Kc::Down),
        enum_set!(Kc::Left),
        enum_set!(Kc::Right),
        enum_set!(Kc::NumLock),
        enum_set!(Kc::ScrollLock),
        enum_set!(Kc::MediaVolDown),
        enum_set!(Kc::Pause),
        enum_set!(Kc::App),
        enum_set!(Kc::Minus),
        enum_set!(Kc::Equals),
        enum_set!(Kc::LeftBracket),
        enum_set!(Kc::RightBracket),
        enum_set!(Kc::Backslash),
        enum_set!(Kc::Semicolon),
        enum_set!(Kc::Quote),
        enum_set!(Kc::Grave),
        enum_set!(Kc::Comma),
        enum_set!(Kc::Dot),
        enum_set!(Kc::Slash),
        enum_set!(Kc::Alt),
        enum_set!(Kc::Ctrl),
        enum_set!(Kc::Shift),
        enum_set!(Kc::Super),
        enum_set!(Kc::Alt),
        enum_set!(Kc::Ctrl),
        enum_set!(Kc::Shift),
    ]);
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "phys: {}, key state: {}", phys, ks)]
pub struct UsModel {
    layout: &'static Layout,
    phys: CountMap<usize>,
    ks: KeyAutomata,
}

impl Default for UsModel {
    fn default() -> Self {
        Self::new()
    }
}

impl UsModel {
    pub fn new() -> Self {
        Self { layout: &US_LAYOUT, phys: CountMap::new(), ks: KeyAutomata::new() }
    }

    pub fn get_key(&self, phys: usize) -> KcSet {
        self.layout.keys[phys as usize]
    }
}

impl Model for UsModel {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<SmallVec<[KeyEv; 4]>> {
        if !(0..=1).contains(&self.phys.adjust_count(pev.phys, pev.press)) {
            return None;
        }
        self.ks.event(KeyEv::new(self.get_key(pev.phys), pev.press), cnst)
    }
}
