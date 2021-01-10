use crate::constants::Constants;
use enumset::{enum_set, EnumSet, EnumSetType};
use rand::seq::IteratorRandom;
use rand_distr::{Distribution, WeightedAliasIndex};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, EnumString, Display)]
pub enum Finger {
    LP,
    LR,
    LM,
    LI,
    LT,
    RP,
    RR,
    RM,
    RI,
    RT,
}

pub trait KCSetExt {
    fn regular(&self) -> KCSet;
    fn mods(&self) -> KCSet;
}

pub type KCSet = EnumSet<KC>;

impl KCSetExt for KCSet {
    fn regular(&self) -> KCSet {
        self.iter().filter(|x| !x.is_mod()).collect()
    }

    fn mods(&self) -> KCSet {
        self.iter().filter(|x| x.is_mod()).collect()
    }
}

pub fn rand_kcset<R: rand::Rng + ?Sized>(r: &mut R, cnst: &Constants) -> KCSet {
    let mod_idx = WeightedAliasIndex::new(cnst.num_mod_assigned_weights.clone()).unwrap();
    let reg_idx = WeightedAliasIndex::new(cnst.num_reg_assigned_weights.clone()).unwrap();
    let num_mod = mod_idx.sample(r);
    let num_reg = reg_idx.sample(r);
    let mods = KC::iter().filter(|k| k.is_mod()).collect::<Vec<_>>();
    let regs = KC::iter().filter(|k| !k.is_mod()).collect::<Vec<_>>();
    let mods = mods.iter().choose_multiple(r, num_mod).iter().fold(enum_set!(), |a, &&b| a | b);
    let regs = regs.iter().choose_multiple(r, num_reg).iter().fold(enum_set!(), |a, &&b| a | b);
    mods | regs
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KeyEv {
    pub key: KCSet,
    pub press: bool,
}

impl KeyEv {
    pub fn new(key: KCSet, press: bool) -> Self {
        Self { key, press }
    }

    pub fn press(key: KCSet) -> Self {
        Self::new(key, true)
    }

    pub fn release(key: KCSet) -> Self {
        Self::new(key, false)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysEv {
    pub phys: u32,
    pub press: bool,
}

impl PhysEv {
    pub fn new(phys: u32, press: bool) -> Self {
        Self { phys, press }
    }

    pub fn press(phys: u32) -> Self {
        Self::new(phys, true)
    }

    pub fn release(phys: u32) -> Self {
        Self::new(phys, false)
    }
}

// Based on QMK keycodes.
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Ord, PartialOrd, EnumSetType, EnumIter, EnumString, Hash, Display)]
pub enum KC {
    // Numbers:
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Navigation:
    Enter,
    Esc,
    Backspace,
    Tab,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDn,
    Up,
    Down,
    Left,
    Right,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,
    App,

    // Media:
    MediaMute,
    MediaVolUp,
    MediaVolDown,
    MediaPrev,
    MediaNext,
    MediaPlayPause,
    MediaStop,

    // Symbols:
    Minus,        // - and _
    Equals,       // = and +
    LeftBracket,  // [ and {
    RightBracket, // ] and }
    Backslash,    // \ and |
    Semicolon,    // ; and :
    Quote,        // ' and ""
    Grave,        // ` and ~
    Comma,        // , and <
    Dot,          // . and >
    Slash,        // / and ?

    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // F keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Mod:
    Ctrl,
    Shift,
    Alt,
    Super,
}

impl KC {
    pub fn is_mod(&self) -> bool {
        [KC::Ctrl, KC::Shift, KC::Alt, KC::Super].contains(self)
    }
}
