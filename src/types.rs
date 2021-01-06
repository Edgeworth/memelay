use crate::prelude::*;
use enumset::{EnumSet, EnumSetType};
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

#[derive(Debug, Ord, PartialOrd, EnumSetType, EnumIter, Display)]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Super,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key {
    pub kc: KeyCode,
    pub md: EnumSet<Modifier>,
}

// Based on QMK keycodes.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, EnumIter, Display)]
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

    // Modifiers:
    Ctrl,
    Shift,
    Alt,
    Super,
    App,

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
