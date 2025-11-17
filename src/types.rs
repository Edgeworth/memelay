use num_enum::IntoPrimitive;
use strum::{Display as StrumDisplay, EnumString};

impl Default for Kc {
    fn default() -> Self {
        Self::None
    }
}

// Based on QMK keycodes.
#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_kc_default() {
        assert_eq!(Kc::default(), Kc::None);
    }

    #[test]
    fn test_kc_ordering() {
        assert!(Kc::None < Kc::Num0);
        assert!(Kc::A < Kc::B);
        assert_eq!(Kc::A, Kc::A);
    }

    #[test]
    fn test_kc_from_str_letters() {
        assert_eq!(Kc::from_str("a").unwrap(), Kc::A);
        assert_eq!(Kc::from_str("z").unwrap(), Kc::Z);
        assert_eq!(Kc::from_str("m").unwrap(), Kc::M);
    }

    #[test]
    fn test_kc_from_str_numbers() {
        assert_eq!(Kc::from_str("0").unwrap(), Kc::Num0);
        assert_eq!(Kc::from_str("5").unwrap(), Kc::Num5);
        assert_eq!(Kc::from_str("9").unwrap(), Kc::Num9);
    }

    #[test]
    fn test_kc_from_str_symbols() {
        assert_eq!(Kc::from_str("-").unwrap(), Kc::Minus);
        assert_eq!(Kc::from_str("=").unwrap(), Kc::Equals);
        assert_eq!(Kc::from_str("[").unwrap(), Kc::LeftBracket);
        assert_eq!(Kc::from_str("]").unwrap(), Kc::RightBracket);
        assert_eq!(Kc::from_str("\\").unwrap(), Kc::Backslash);
        assert_eq!(Kc::from_str("`").unwrap(), Kc::Grave);
        assert_eq!(Kc::from_str("'").unwrap(), Kc::Quote);
        assert_eq!(Kc::from_str(";").unwrap(), Kc::Semicolon);
        assert_eq!(Kc::from_str(",").unwrap(), Kc::Comma);
        assert_eq!(Kc::from_str(".").unwrap(), Kc::Dot);
        assert_eq!(Kc::from_str("/").unwrap(), Kc::Slash);
    }

    #[test]
    fn test_kc_from_str_shifted_symbols() {
        assert_eq!(Kc::from_str("_").unwrap(), Kc::Underscore);
        assert_eq!(Kc::from_str("+").unwrap(), Kc::Plus);
        assert_eq!(Kc::from_str("{").unwrap(), Kc::LeftBrace);
        assert_eq!(Kc::from_str("}").unwrap(), Kc::RightBrace);
        assert_eq!(Kc::from_str("|").unwrap(), Kc::Bar);
        assert_eq!(Kc::from_str("~").unwrap(), Kc::Tilde);
        assert_eq!(Kc::from_str("\"").unwrap(), Kc::DoubleQuote);
        assert_eq!(Kc::from_str(":").unwrap(), Kc::Colon);
        assert_eq!(Kc::from_str("<").unwrap(), Kc::LeftAngle);
        assert_eq!(Kc::from_str(">").unwrap(), Kc::RightAngle);
        assert_eq!(Kc::from_str("?").unwrap(), Kc::QuestionMark);
        assert_eq!(Kc::from_str("!").unwrap(), Kc::Exclamation);
        assert_eq!(Kc::from_str("@").unwrap(), Kc::AtSign);
        assert_eq!(Kc::from_str("#").unwrap(), Kc::Hash);
        assert_eq!(Kc::from_str("$").unwrap(), Kc::DollarSign);
        assert_eq!(Kc::from_str("%").unwrap(), Kc::PercentSign);
        assert_eq!(Kc::from_str("^").unwrap(), Kc::Caret);
        assert_eq!(Kc::from_str("&").unwrap(), Kc::Ampersand);
        assert_eq!(Kc::from_str("*").unwrap(), Kc::Asterisk);
        assert_eq!(Kc::from_str("(").unwrap(), Kc::LeftParen);
        assert_eq!(Kc::from_str(")").unwrap(), Kc::RightParen);
    }

    #[test]
    fn test_kc_from_str_invalid() {
        assert!(Kc::from_str("invalid").is_err());
        assert!(Kc::from_str("").is_err());
        assert!(Kc::from_str("abc").is_err());
    }

    #[test]
    fn test_kc_display() {
        assert_eq!(Kc::A.to_string(), "a");
        assert_eq!(Kc::Num0.to_string(), "0");
        assert_eq!(Kc::Minus.to_string(), "-");
        assert_eq!(Kc::LeftBracket.to_string(), "[");
    }

    #[test]
    fn test_kc_into_i8() {
        let val: i8 = Kc::None.into();
        assert_eq!(val, 0);
        let val: i8 = Kc::Num0.into();
        assert_eq!(val, 1);
    }

    #[test]
    fn test_kc_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Kc::A);
        set.insert(Kc::B);
        set.insert(Kc::A);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&Kc::A));
        assert!(set.contains(&Kc::B));
        assert!(!set.contains(&Kc::C));
    }

    #[test]
    fn test_qwerty_constant_length() {
        assert_eq!(QWERTY.len(), 30);
    }

    #[test]
    fn test_qwerty_constant_values() {
        assert_eq!(QWERTY[0], Kc::Q);
        assert_eq!(QWERTY[9], Kc::P);
        assert_eq!(QWERTY[10], Kc::A);
        assert_eq!(QWERTY[19], Kc::Semicolon);
        assert_eq!(QWERTY[20], Kc::Z);
        assert_eq!(QWERTY[29], Kc::Slash);
    }

    #[test]
    fn test_colemak_dhm_constant_length() {
        assert_eq!(COLEMAK_DHM.len(), 30);
    }

    #[test]
    fn test_colemak_dhm_constant_values() {
        assert_eq!(COLEMAK_DHM[0], Kc::Q);
        assert_eq!(COLEMAK_DHM[2], Kc::F);
        assert_eq!(COLEMAK_DHM[10], Kc::A);
        assert_eq!(COLEMAK_DHM[11], Kc::R);
        assert_eq!(COLEMAK_DHM[29], Kc::Slash);
    }

    #[test]
    fn test_qwerty_no_duplicates() {
        use std::collections::HashSet;
        let set: HashSet<_> = QWERTY.iter().collect();
        assert_eq!(set.len(), QWERTY.len());
    }

    #[test]
    fn test_colemak_dhm_no_duplicates() {
        use std::collections::HashSet;
        let set: HashSet<_> = COLEMAK_DHM.iter().collect();
        assert_eq!(set.len(), COLEMAK_DHM.len());
    }

    #[test]
    fn test_kc_copy_clone() {
        let kc1 = Kc::A;
        let kc2 = kc1;
        let kc3 = kc1.clone();
        assert_eq!(kc1, kc2);
        assert_eq!(kc1, kc3);
    }

    #[test]
    fn test_kc_debug() {
        let kc = Kc::A;
        let debug_str = format!("{:?}", kc);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_all_f_keys() {
        assert_eq!(Kc::from_str("F1").unwrap(), Kc::F1);
        assert_eq!(Kc::from_str("F12").unwrap(), Kc::F12);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(Kc::from_str("(").unwrap(), Kc::LeftParen);
        assert_eq!(Kc::from_str(")").unwrap(), Kc::RightParen);
    }

    #[test]
    fn test_brackets_and_braces() {
        assert_eq!(Kc::from_str("[").unwrap(), Kc::LeftBracket);
        assert_eq!(Kc::from_str("]").unwrap(), Kc::RightBracket);
        assert_eq!(Kc::from_str("{").unwrap(), Kc::LeftBrace);
        assert_eq!(Kc::from_str("}").unwrap(), Kc::RightBrace);
    }

    #[test]
    fn test_all_numbers() {
        for i in 0..=9 {
            let s = i.to_string();
            let kc = Kc::from_str(&s);
            assert!(kc.is_ok(), "Failed to parse number {}", i);
        }
    }

    #[test]
    fn test_all_letters_lowercase() {
        for c in b'a'..=b'z' {
            let s = (c as char).to_string();
            let kc = Kc::from_str(&s);
            assert!(kc.is_ok(), "Failed to parse letter {}", c as char);
        }
    }

    #[test]
    fn test_partial_ord() {
        assert!(Kc::A.partial_cmp(&Kc::B).is_some());
        assert!(Kc::A < Kc::B);
        assert!(Kc::B > Kc::A);
    }
}
