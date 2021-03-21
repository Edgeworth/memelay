use crate::eval::Params;
use crate::layout::Layout;
use crate::types::Kc;
use eyre::{eyre, Result, WrapErr};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum State {
    Layout,
    Cost,
    Row,
    Hand,
    Finger,
}

pub fn load_layout<P: AsRef<Path>>(layout_path: P) -> Result<Layout> {
    const SKIP: &str = "|\\";
    let mut keys = Vec::new();
    for i in fs::read_to_string(layout_path)?.lines() {
        for kc in i.split(|c: char| c.is_whitespace() || SKIP.contains(c)) {
            if kc.is_empty() {
                continue;
            }
            keys.push(Kc::from_str(kc).wrap_err(eyre!("could not find {}", kc))?);
        }
    }
    Ok(Layout::new(keys))
}

pub fn load_params<P: AsRef<Path>>(cfg_path: P) -> Result<Params> {
    const ALLOWED: &str = "RLpmitr-.0123456789X";
    let mut state = State::Layout;
    let mut layout = String::new();
    let mut cost = Vec::new();
    let mut row = Vec::new();
    let mut hand = Vec::new();
    let mut finger = Vec::new();
    for i in fs::read_to_string(cfg_path)?.lines() {
        let mut updated = true;
        if i.starts_with("layout") {
            state = State::Layout;
        } else if i.starts_with("cost") {
            state = State::Cost;
        } else if i.starts_with("row") {
            state = State::Row;
        } else if i.starts_with("hand") {
            state = State::Hand;
        } else if i.starts_with("finger") {
            state = State::Finger;
        } else {
            updated = false;
        }

        if updated || i.starts_with('#') {
            continue;
        }
        if state == State::Layout {
            layout += i;
            layout.push('\n');
            continue;
        }
        for item in i.split(char::is_whitespace) {
            let filtered: String = item.chars().filter(|&c| ALLOWED.contains(c)).collect();
            if filtered.is_empty() {
                continue;
            }
            match state {
                State::Layout => {}
                State::Cost => cost.push(filtered.parse::<f64>().unwrap()),
                State::Row => row.push(filtered.parse::<i32>().unwrap()),
                State::Hand => hand.push(filtered.parse::<i32>().unwrap()),
                State::Finger => finger.push(filtered.parse::<i32>().unwrap()),
            };
        }
    }

    Ok(Params { layout, cost, row, hand, finger })
}

pub fn load_keys<P: AsRef<Path>>(path: P) -> Result<Vec<Kc>> {
    let mut keys = Vec::new();
    for i in fs::read_to_string(path)?.lines() {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 3 {
            return Err(eyre!("weird corpus line: {}", i));
        }
        let (_, kcstr, _pressed) = (items[0], items[1], items[2] == "1");
        let kc = match kcstr {
            "0" => Kc::Num0,
            "1" => Kc::Num1,
            "2" => Kc::Num2,
            "3" => Kc::Num3,
            "4" => Kc::Num4,
            "5" => Kc::Num5,
            "6" => Kc::Num6,
            "7" => Kc::Num7,
            "8" => Kc::Num8,
            "9" => Kc::Num9,
            "A" => Kc::A,
            "B" => Kc::B,
            "C" => Kc::C,
            "D" => Kc::D,
            "E" => Kc::E,
            "F" => Kc::F,
            "G" => Kc::G,
            "H" => Kc::H,
            "I" => Kc::I,
            "J" => Kc::J,
            "K" => Kc::K,
            "L" => Kc::L,
            "M" => Kc::M,
            "N" => Kc::N,
            "O" => Kc::O,
            "P" => Kc::P,
            "Q" => Kc::Q,
            "R" => Kc::R,
            "S" => Kc::S,
            "T" => Kc::T,
            "U" => Kc::U,
            "V" => Kc::V,
            "W" => Kc::W,
            "X" => Kc::X,
            "Y" => Kc::Y,
            "Z" => Kc::Z,
            "F1" => Kc::F1,
            "F2" => Kc::F2,
            "F3" => Kc::F3,
            "F4" => Kc::F4,
            "F5" => Kc::F5,
            "F6" => Kc::F6,
            "F7" => Kc::F7,
            "F8" => Kc::F8,
            "F9" => Kc::F9,
            "F10" => Kc::F10,
            "F11" => Kc::F11,
            "F12" => Kc::F12,
            "ENTER" => Kc::Enter,
            "ESC" => Kc::Esc,
            "BACKSPACE" => Kc::Backspace,
            "TAB" => Kc::Tab,
            "SPACE" => Kc::Space,
            "INS" => Kc::Insert,
            "DEL" => Kc::Delete,
            "HOME" => Kc::Home,
            "END" => Kc::End,
            "PGUP" => Kc::PageUp,
            "PGDN" => Kc::PageDn,
            "UARROW" => Kc::Up,
            "DARROW" => Kc::Down,
            "LARROW" => Kc::Left,
            "RARROW" => Kc::Right,
            "NUMLOCK" => Kc::NumLock,
            "SCROLLLOCK" => Kc::ScrollLock,
            "PSSR" => Kc::MediaVolDown,
            "PABR" => Kc::Pause,
            "MENU" => Kc::App,
            "MINUS" => Kc::Minus,
            "EQUAL" => Kc::Equals,
            "LBRACE" => Kc::LeftBracket,
            "RBRACE" => Kc::RightBracket,
            "BACKSLASH" => Kc::Backslash,
            "SEMICOLON" => Kc::Semicolon,
            "APOSTROPHE" => Kc::Quote,
            "GRAVE" => Kc::Grave,
            "COMMA" => Kc::Comma,
            "DOT" => Kc::Dot,
            "SLASH" => Kc::Slash,
            "LALT" => Kc::Alt,
            "LCTRL" => Kc::Ctrl,
            "LSHIFT" => Kc::Shift,
            "LSUPER" => Kc::Super,
            "RALT" => Kc::Alt,
            "RCTRL" => Kc::Ctrl,
            "RSHIFT" => Kc::Shift,
            "RSUPER" => Kc::Super,
            _ => return Err(eyre!("unrecognised corpus key {}", kcstr)),
        };
        keys.push(kc);
    }
    Ok(keys)
}
