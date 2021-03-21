use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use strum_macros::{Display as StrumDisplay, EnumString};

impl Distribution<Kc> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> Kc {
        // Create specific subset:
        match r.gen_range(1..=30) {
            1 => Kc::Semicolon,
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

impl Default for Kc {
    fn default() -> Self {
        Self::None
    }
}

// Based on QMK keycodes.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumString, Hash, StrumDisplay)]
pub enum Kc {
    None,

    // Mod
    Ctrl,
    Shift,
    Alt,
    Super,

    // Numbers:
    #[strum(serialize = "0")]
    Num0,
    #[strum(serialize = "1")]
    Num1,
    #[strum(serialize = "2")]
    Num2,
    #[strum(serialize = "3")]
    Num3,
    #[strum(serialize = "4")]
    Num4,
    #[strum(serialize = "5")]
    Num5,
    #[strum(serialize = "6")]
    Num6,
    #[strum(serialize = "7")]
    Num7,
    #[strum(serialize = "8")]
    Num8,
    #[strum(serialize = "9")]
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
    #[strum(serialize = "-")]
    Minus, // - and _
    #[strum(serialize = "=")]
    Equals, // = and +
    #[strum(serialize = "[")]
    LeftBracket, // [ and {
    #[strum(serialize = "]")]
    RightBracket, // ] and }
    #[strum(serialize = "\\")]
    Backslash, // \ and |
    #[strum(serialize = "`")]
    Grave, // ` and ~
    #[strum(serialize = "'")]
    Quote, // ' and ""
    #[strum(serialize = ";")]
    Semicolon, // ; and :
    #[strum(serialize = ",")]
    Comma, // , and <
    #[strum(serialize = ".")]
    Dot, // . and >
    #[strum(serialize = "/")]
    Slash, // / and ?

    // Letters
    #[strum(serialize = "a")]
    A,
    #[strum(serialize = "b")]
    B,
    #[strum(serialize = "c")]
    C,
    #[strum(serialize = "d")]
    D,
    #[strum(serialize = "e")]
    E,
    #[strum(serialize = "f")]
    F,
    #[strum(serialize = "g")]
    G,
    #[strum(serialize = "h")]
    H,
    #[strum(serialize = "i")]
    I,
    #[strum(serialize = "j")]
    J,
    #[strum(serialize = "k")]
    K,
    #[strum(serialize = "l")]
    L,
    #[strum(serialize = "m")]
    M,
    #[strum(serialize = "n")]
    N,
    #[strum(serialize = "o")]
    O,
    #[strum(serialize = "p")]
    P,
    #[strum(serialize = "q")]
    Q,
    #[strum(serialize = "r")]
    R,
    #[strum(serialize = "s")]
    S,
    #[strum(serialize = "t")]
    T,
    #[strum(serialize = "u")]
    U,
    #[strum(serialize = "v")]
    V,
    #[strum(serialize = "w")]
    W,
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "y")]
    Y,
    #[strum(serialize = "z")]
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
