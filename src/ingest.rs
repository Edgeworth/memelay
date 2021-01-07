use crate::fitness::Fitness;
use crate::models::us::US_LAYER;
use crate::prelude::*;
use crate::types::{Finger, Key, KeyEv, Mod, PhysEv, KC};
use enumset::enum_set;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn fitness_from_file<P: AsRef<Path>>(cfg_path: P, corpus_path: P) -> Result<Fitness> {
    const ALLOWED: &str = "RLPMIT-.0123456789";
    let mut cost = Vec::new();
    let mut fing = Vec::new();
    for i in fs::read_to_string(cfg_path)?.split(char::is_whitespace) {
        let s: String = i.chars().filter(|&c| ALLOWED.contains(c)).collect();
        if !s.is_empty() {
            if let Ok(c) = s.parse::<f64>() {
                cost.push(c);
            } else if let Ok(f) = Finger::from_str(&s) {
                fing.push(f);
            } else {
                return Err(eyre!("unknown cfg value {}", s));
            }
        }
    }
    let mut corpus = Vec::new();
    for i in fs::read_to_string(corpus_path)?.lines() {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 2 {
            return Err(eyre!("weird corpus line: {}", i));
        }
        let (kc, pressed) = (items[0], items[1] == "1");
        let key = match kc {
            "0" => Key::new(KC::Num0, enum_set!()),
            "1" => Key::new(KC::Num1, enum_set!()),
            "2" => Key::new(KC::Num2, enum_set!()),
            "3" => Key::new(KC::Num3, enum_set!()),
            "4" => Key::new(KC::Num4, enum_set!()),
            "5" => Key::new(KC::Num5, enum_set!()),
            "6" => Key::new(KC::Num6, enum_set!()),
            "7" => Key::new(KC::Num7, enum_set!()),
            "8" => Key::new(KC::Num8, enum_set!()),
            "9" => Key::new(KC::Num9, enum_set!()),
            "A" => Key::new(KC::A, enum_set!()),
            "B" => Key::new(KC::B, enum_set!()),
            "C" => Key::new(KC::C, enum_set!()),
            "D" => Key::new(KC::D, enum_set!()),
            "E" => Key::new(KC::E, enum_set!()),
            "F" => Key::new(KC::F, enum_set!()),
            "G" => Key::new(KC::G, enum_set!()),
            "H" => Key::new(KC::H, enum_set!()),
            "I" => Key::new(KC::I, enum_set!()),
            "J" => Key::new(KC::J, enum_set!()),
            "K" => Key::new(KC::K, enum_set!()),
            "L" => Key::new(KC::L, enum_set!()),
            "M" => Key::new(KC::M, enum_set!()),
            "N" => Key::new(KC::N, enum_set!()),
            "O" => Key::new(KC::O, enum_set!()),
            "P" => Key::new(KC::P, enum_set!()),
            "Q" => Key::new(KC::Q, enum_set!()),
            "R" => Key::new(KC::R, enum_set!()),
            "S" => Key::new(KC::S, enum_set!()),
            "T" => Key::new(KC::T, enum_set!()),
            "U" => Key::new(KC::U, enum_set!()),
            "V" => Key::new(KC::V, enum_set!()),
            "W" => Key::new(KC::W, enum_set!()),
            "X" => Key::new(KC::X, enum_set!()),
            "Y" => Key::new(KC::Y, enum_set!()),
            "Z" => Key::new(KC::Z, enum_set!()),
            "F1" => Key::new(KC::F1, enum_set!()),
            "F2" => Key::new(KC::F2, enum_set!()),
            "F3" => Key::new(KC::F3, enum_set!()),
            "F4" => Key::new(KC::F4, enum_set!()),
            "F5" => Key::new(KC::F5, enum_set!()),
            "F6" => Key::new(KC::F6, enum_set!()),
            "F7" => Key::new(KC::F7, enum_set!()),
            "F8" => Key::new(KC::F8, enum_set!()),
            "F9" => Key::new(KC::F9, enum_set!()),
            "F10" => Key::new(KC::F10, enum_set!()),
            "F11" => Key::new(KC::F11, enum_set!()),
            "F12" => Key::new(KC::F12, enum_set!()),
            "ENTER" => Key::new(KC::Enter, enum_set!()),
            "ESC" => Key::new(KC::Esc, enum_set!()),
            "BACKSPACE" => Key::new(KC::Backspace, enum_set!()),
            "TAB" => Key::new(KC::Tab, enum_set!()),
            "SPACE" => Key::new(KC::Space, enum_set!()),
            "INS" => Key::new(KC::Insert, enum_set!()),
            "DEL" => Key::new(KC::Delete, enum_set!()),
            "HOME" => Key::new(KC::Home, enum_set!()),
            "END" => Key::new(KC::End, enum_set!()),
            "PGUP" => Key::new(KC::PageUp, enum_set!()),
            "PGDN" => Key::new(KC::PageDn, enum_set!()),
            "UARROW" => Key::new(KC::Up, enum_set!()),
            "DARROW" => Key::new(KC::Down, enum_set!()),
            "LARROW" => Key::new(KC::Left, enum_set!()),
            "RARROW" => Key::new(KC::Right, enum_set!()),
            "NUMLOCK" => Key::new(KC::NumLock, enum_set!()),
            "SCROLLLOCK" => Key::new(KC::ScrollLock, enum_set!()),
            "PSSR" => Key::new(KC::MediaVolDown, enum_set!()),
            "PABR" => Key::new(KC::Pause, enum_set!()),
            "MENU" => Key::new(KC::App, enum_set!()),
            "MINUS" => Key::new(KC::Minus, enum_set!()),
            "EQUAL" => Key::new(KC::Equals, enum_set!()),
            "LBRACE" => Key::new(KC::LeftBracket, enum_set!()),
            "RBRACE" => Key::new(KC::RightBracket, enum_set!()),
            "BACKSLASH" => Key::new(KC::Backslash, enum_set!()),
            "SEMICOLON" => Key::new(KC::Semicolon, enum_set!()),
            "APOSTROPHE" => Key::new(KC::Quote, enum_set!()),
            "GRAVE" => Key::new(KC::Grave, enum_set!()),
            "COMMA" => Key::new(KC::Comma, enum_set!()),
            "DOT" => Key::new(KC::Dot, enum_set!()),
            "SLASH" => Key::new(KC::Slash, enum_set!()),
            "LALT" => Key::new(KC::None, enum_set!(Mod::Alt)),
            "LCTRL" => Key::new(KC::None, enum_set!(Mod::Ctrl)),
            "LSHIFT" => Key::new(KC::None, enum_set!(Mod::Shift)),
            "LSUPER" => Key::new(KC::None, enum_set!(Mod::Super)),
            "RALT" => Key::new(KC::None, enum_set!(Mod::Alt)),
            "RCTRL" => Key::new(KC::None, enum_set!(Mod::Ctrl)),
            "RSHIFT" => Key::new(KC::None, enum_set!(Mod::Shift)),
            _ => return Err(eyre!("unrecognised corpus key {}", kc)),
        };
        let count = if pressed { 1 } else { -1 };
        let index = US_LAYER.keys.iter().position(|&x| x == key).unwrap();
        corpus.push(PhysEv::new(index as u32, count));
    }
    if cost.len() != fing.len() {
        Err(eyre!("{} costs does not match {} fingers", cost.len(), fing.len()))
    } else {
        Ok(Fitness::new(cost, fing, corpus))
    }
}
