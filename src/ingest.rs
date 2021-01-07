use crate::fitness::Fitness;
use crate::prelude::*;
use crate::types::{Finger, Key, KeyCode, KeyEv, Mod};
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
            "0" => Key::new(KeyCode::Num0, enum_set!()),
            "1" => Key::new(KeyCode::Num1, enum_set!()),
            "2" => Key::new(KeyCode::Num2, enum_set!()),
            "3" => Key::new(KeyCode::Num3, enum_set!()),
            "4" => Key::new(KeyCode::Num4, enum_set!()),
            "5" => Key::new(KeyCode::Num5, enum_set!()),
            "6" => Key::new(KeyCode::Num6, enum_set!()),
            "7" => Key::new(KeyCode::Num7, enum_set!()),
            "8" => Key::new(KeyCode::Num8, enum_set!()),
            "9" => Key::new(KeyCode::Num9, enum_set!()),
            "A" => Key::new(KeyCode::A, enum_set!()),
            "B" => Key::new(KeyCode::B, enum_set!()),
            "C" => Key::new(KeyCode::C, enum_set!()),
            "D" => Key::new(KeyCode::D, enum_set!()),
            "E" => Key::new(KeyCode::E, enum_set!()),
            "F" => Key::new(KeyCode::F, enum_set!()),
            "G" => Key::new(KeyCode::G, enum_set!()),
            "H" => Key::new(KeyCode::H, enum_set!()),
            "I" => Key::new(KeyCode::I, enum_set!()),
            "J" => Key::new(KeyCode::J, enum_set!()),
            "K" => Key::new(KeyCode::K, enum_set!()),
            "L" => Key::new(KeyCode::L, enum_set!()),
            "M" => Key::new(KeyCode::M, enum_set!()),
            "N" => Key::new(KeyCode::N, enum_set!()),
            "O" => Key::new(KeyCode::O, enum_set!()),
            "P" => Key::new(KeyCode::P, enum_set!()),
            "Q" => Key::new(KeyCode::Q, enum_set!()),
            "R" => Key::new(KeyCode::R, enum_set!()),
            "S" => Key::new(KeyCode::S, enum_set!()),
            "T" => Key::new(KeyCode::T, enum_set!()),
            "U" => Key::new(KeyCode::U, enum_set!()),
            "V" => Key::new(KeyCode::V, enum_set!()),
            "W" => Key::new(KeyCode::W, enum_set!()),
            "X" => Key::new(KeyCode::X, enum_set!()),
            "Y" => Key::new(KeyCode::Y, enum_set!()),
            "Z" => Key::new(KeyCode::Z, enum_set!()),
            "F1" => Key::new(KeyCode::F1, enum_set!()),
            "F2" => Key::new(KeyCode::F2, enum_set!()),
            "F3" => Key::new(KeyCode::F3, enum_set!()),
            "F4" => Key::new(KeyCode::F4, enum_set!()),
            "F5" => Key::new(KeyCode::F5, enum_set!()),
            "F6" => Key::new(KeyCode::F6, enum_set!()),
            "F7" => Key::new(KeyCode::F7, enum_set!()),
            "F8" => Key::new(KeyCode::F8, enum_set!()),
            "F9" => Key::new(KeyCode::F9, enum_set!()),
            "F10" => Key::new(KeyCode::F10, enum_set!()),
            "F11" => Key::new(KeyCode::F11, enum_set!()),
            "F12" => Key::new(KeyCode::F12, enum_set!()),
            "ENTER" => Key::new(KeyCode::Enter, enum_set!()),
            "ESC" => Key::new(KeyCode::Esc, enum_set!()),
            "BACKSPACE" => Key::new(KeyCode::Backspace, enum_set!()),
            "TAB" => Key::new(KeyCode::Tab, enum_set!()),
            "SPACE" => Key::new(KeyCode::Space, enum_set!()),
            "INS" => Key::new(KeyCode::Insert, enum_set!()),
            "DEL" => Key::new(KeyCode::Delete, enum_set!()),
            "HOME" => Key::new(KeyCode::Home, enum_set!()),
            "END" => Key::new(KeyCode::End, enum_set!()),
            "PGUP" => Key::new(KeyCode::PageUp, enum_set!()),
            "PGDN" => Key::new(KeyCode::PageDn, enum_set!()),
            "UARROW" => Key::new(KeyCode::Up, enum_set!()),
            "DARROW" => Key::new(KeyCode::Down, enum_set!()),
            "LARROW" => Key::new(KeyCode::Left, enum_set!()),
            "RARROW" => Key::new(KeyCode::Right, enum_set!()),
            "NUMLOCK" => Key::new(KeyCode::NumLock, enum_set!()),
            "SCROLLLOCK" => Key::new(KeyCode::ScrollLock, enum_set!()),
            "PSSR" => Key::new(KeyCode::MediaVolDown, enum_set!()),
            "PABR" => Key::new(KeyCode::Pause, enum_set!()),
            "MENU" => Key::new(KeyCode::App, enum_set!()),
            "MINUS" => Key::new(KeyCode::Minus, enum_set!()),
            "EQUAL" => Key::new(KeyCode::Equals, enum_set!()),
            "LBRACE" => Key::new(KeyCode::LeftBracket, enum_set!()),
            "RBRACE" => Key::new(KeyCode::RightBracket, enum_set!()),
            "BACKSLASH" => Key::new(KeyCode::Backslash, enum_set!()),
            "SEMICOLON" => Key::new(KeyCode::Semicolon, enum_set!()),
            "APOSTROPHE" => Key::new(KeyCode::Quote, enum_set!()),
            "GRAVE" => Key::new(KeyCode::Grave, enum_set!()),
            "COMMA" => Key::new(KeyCode::Comma, enum_set!()),
            "DOT" => Key::new(KeyCode::Dot, enum_set!()),
            "SLASH" => Key::new(KeyCode::Slash, enum_set!()),
            "LALT" => Key::new(KeyCode::None, enum_set!(Mod::Alt)),
            "LCTRL" => Key::new(KeyCode::None, enum_set!(Mod::Ctrl)),
            "LSHIFT" => Key::new(KeyCode::None, enum_set!(Mod::Shift)),
            "LSUPER" => Key::new(KeyCode::None, enum_set!(Mod::Super)),
            "RALT" => Key::new(KeyCode::None, enum_set!(Mod::Alt)),
            "RCTRL" => Key::new(KeyCode::None, enum_set!(Mod::Ctrl)),
            "RSHIFT" => Key::new(KeyCode::None, enum_set!(Mod::Shift)),
            _ => return Err(eyre!("unrecognised corpus key {}", kc)),
        };
        let count = if pressed { 1 } else { -1 };
        corpus.push(KeyEv::new(key, count));
    }
    if cost.len() != fing.len() {
        Err(eyre!("{} costs does not match {} fingers", cost.len(), fing.len()))
    } else {
        Ok(Fitness::new(cost, fing, corpus))
    }
}
