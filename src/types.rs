use crate::constants::Constants;
use derive_more::Display;
use enumset::{enum_set, EnumSet, EnumSetType};
use ga::ops::sampling::rws;
use rand::seq::IteratorRandom;
use smallvec::SmallVec;
use strum::IntoEnumIterator;
use strum_macros::{Display as StrumDisplay, EnumIter, EnumString};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, EnumString, StrumDisplay)]
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
    fn reg(&self) -> KCSet;
    fn mods(&self) -> KCSet;
    fn layers(&self) -> KCSet;
}

pub type KCSet = EnumSet<KC>;

impl KCSetExt for KCSet {
    fn reg(&self) -> KCSet {
        self.iter().filter(|x| !x.is_mod()).collect()
    }

    fn mods(&self) -> KCSet {
        self.iter().filter(|x| x.is_mod()).collect()
    }

    fn layers(&self) -> KCSet {
        self.iter().filter(|x| x.is_layer()).collect()
    }
}

pub fn rand_kcset(cnst: &Constants) -> KCSet {
    let mut r = rand::thread_rng();
    let num_mod = rws(&cnst.num_mod_assigned_weights).unwrap();
    let num_reg = rws(&cnst.num_reg_assigned_weights).unwrap();
    let mods = KC::iter().filter(|k| k.is_mod()).collect::<SmallVec<[KC; 4]>>();
    let regs = KC::iter().filter(|k| !k.is_mod()).collect::<SmallVec<[KC; 2]>>();
    let mods =
        mods.iter().choose_multiple(&mut r, num_mod).iter().fold(enum_set!(), |a, &&b| a | b);
    let regs =
        regs.iter().choose_multiple(&mut r, num_reg).iter().fold(enum_set!(), |a, &&b| a | b);
    mods | regs
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "KeyEv({:?}, {})", kcset, press)]
pub struct KeyEv {
    pub kcset: KCSet,
    pub press: bool,
}

impl KeyEv {
    pub fn new(kcset: KCSet, press: bool) -> Self {
        Self { kcset, press }
    }

    pub fn press(kcset: KCSet) -> Self {
        Self::new(kcset, true)
    }

    pub fn release(kcset: KCSet) -> Self {
        Self::new(kcset, false)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "({}, {})", phys, press)]
pub struct PhysEv {
    pub phys: usize,
    pub press: bool,
}

impl PhysEv {
    pub fn new(phys: usize, press: bool) -> Self {
        Self { phys, press }
    }

    pub fn press(phys: usize) -> Self {
        Self::new(phys, true)
    }

    pub fn release(phys: usize) -> Self {
        Self::new(phys, false)
    }
}

// Based on QMK keycodes.
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Ord, PartialOrd, EnumSetType, EnumIter, EnumString, Hash, StrumDisplay)]
pub enum KC {
    // Mod - these come first on purpose, to make sure e.g. Ctrl-C is generated as Ctrl then C.
    Ctrl,
    Shift,
    Alt,
    Super,

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

    // Layer control:
    Layer0,
    Layer1,
}

impl KC {
    pub fn is_mod(&self) -> bool {
        [KC::Ctrl, KC::Shift, KC::Alt, KC::Super].contains(self)
    }

    pub fn is_layer(&self) -> bool {
        self.layer_num().is_some()
    }

    pub fn layer_num(&self) -> Option<usize> {
        match self {
            KC::Layer0 => Some(0),
            KC::Layer1 => Some(1),
            _ => None,
        }
    }
}
