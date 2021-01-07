use crate::prelude::*;
use enumset::{enum_set, EnumSet, EnumSetType};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::seq::IteratorRandom;
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

#[derive(Debug, Ord, PartialOrd, EnumSetType, EnumIter, Hash, Display)]
pub enum Mod {
    Ctrl,
    Shift,
    Alt,
    Super,
}

pub fn rand_mod<R: rand::Rng + ?Sized>(r: &mut R) -> EnumSet<Mod> {
    Mod::iter().filter(|_| r.gen_bool(0.5)).fold(EnumSet::new(), |a, b| a | b)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key {
    pub kc: KeyCode,
    pub md: EnumSet<Mod>,
}

impl Key {
    pub const fn new(kc: KeyCode, md: EnumSet<Mod>) -> Self {
        Self { kc, md }
    }

    pub fn with_mods(mut self, md: EnumSet<Mod>) -> Self {
        self.md |= md;
        self
    }
}

impl Distribution<Key> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, r: &mut R) -> Key {
        Key { kc: r.gen(), md: rand_mod(r) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct KeyEv {
    pub key: Key,
    pub count: i32,
}

impl KeyEv {
    pub fn new(key: Key, count: i32) -> Self {
        Self { key, count }
    }

    pub fn press(key: Key) -> Self {
        Self::new(key, 1)
    }

    pub fn release(key: Key) -> Self {
        Self::new(key, -1)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysEv {
    pub phys: u32,
    pub count: i32,
}

impl PhysEv {
    pub fn new(phys: u32, count: i32) -> Self {
        Self { phys, count }
    }

    pub fn press(phys: u32) -> Self {
        Self::new(phys, 1)
    }

    pub fn release(phys: u32) -> Self {
        Self::new(phys, -1)
    }
}

// Based on QMK keycodes.
#[derive(Debug, Ord, PartialOrd, EnumSetType, EnumIter, Hash, Display)]
pub enum KeyCode {
    // Misc:
    None,
    Transparent,

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
}

impl Distribution<KeyCode> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, r: &mut R) -> KeyCode {
        KeyCode::iter().choose(r).unwrap()
    }
}
