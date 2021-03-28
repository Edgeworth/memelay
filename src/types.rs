use num_enum::IntoPrimitive;
use strum_macros::{Display as StrumDisplay, EnumString};

impl Default for Kc {
    fn default() -> Self {
        Self::None
    }
}

// Based on QMK keycodes.
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    EnumString,
    Hash,
    StrumDisplay,
    IntoPrimitive,
)]
#[repr(i8)]
pub enum Kc {
    None,

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

    // Symbols - shifted numbers:
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "!")]
    Exclamation,
    #[strum(serialize = "@")]
    AtSign,
    #[strum(serialize = "#")]
    Hash,
    #[strum(serialize = "$")]
    DollarSign,
    #[strum(serialize = "%")]
    PercentSign,
    #[strum(serialize = "^")]
    Caret,
    #[strum(serialize = "&")]
    Ampersand,
    #[strum(serialize = "*")]
    Asterisk,
    #[strum(serialize = "(")]
    LeftParen,

    // Symbols - non-shifted:
    #[strum(serialize = "-")]
    Minus, //
    #[strum(serialize = "=")]
    Equals,
    #[strum(serialize = "[")]
    LeftBracket,
    #[strum(serialize = "]")]
    RightBracket,
    #[strum(serialize = "\\")]
    Backslash,
    #[strum(serialize = "`")]
    Grave,
    #[strum(serialize = "'")]
    Quote,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "/")]
    Slash,

    // Symbols - shifted
    #[strum(serialize = "_")]
    Underscore,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,
    #[strum(serialize = "|")]
    Bar,
    #[strum(serialize = "~")]
    Tilde,
    #[strum(serialize = "\"")]
    DoubleQuote,
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = "<")]
    LeftAngle,
    #[strum(serialize = ">")]
    RightAngle,
    #[strum(serialize = "?")]
    QuestionMark,

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

pub const QWERTY: [Kc; 30] = [
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

pub const COLEMAK_DHM: [Kc; 30] = [
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
