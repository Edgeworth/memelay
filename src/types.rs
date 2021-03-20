use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use strum_macros::{Display as StrumDisplay, EnumIter, EnumString};

impl Distribution<Kc> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> Kc {
        // Create specific subset:
        match r.gen_range(1..=30) {
            1 => Kc::Scolon,
            2 => Kc::Comma,
            3 => Kc::Dot,
            4 => Kc::Slash,
            5 => Kc::A,
            6 => Kc::B,
            7 => Kc::C,
            8 => Kc::D,
            9 => Kc::E,
            10 => Kc::F,
            11 => Kc::G,
            12 => Kc::H,
            13 => Kc::I,
            14 => Kc::J,
            15 => Kc::K,
            16 => Kc::L,
            17 => Kc::M,
            18 => Kc::N,
            19 => Kc::O,
            20 => Kc::P,
            21 => Kc::Q,
            22 => Kc::R,
            23 => Kc::S,
            24 => Kc::T,
            25 => Kc::U,
            26 => Kc::V,
            27 => Kc::W,
            28 => Kc::X,
            29 => Kc::Y,
            30 => Kc::Z,
            _ => panic!("bug"),
        }
    }
}

// Based on QMK keycodes.
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter, EnumString, Hash, StrumDisplay,
)]
pub enum Kc {
    None,

    // Mod
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
    Grave,        // ` and ~
    Quote,        // ' and ""
    Scolon,       // ; and :
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
