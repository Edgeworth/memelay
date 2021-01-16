use enumset::enum_set;

use crate::layout_eval::LayoutCfg;
use crate::models::layout::{Layer, Layout};
use crate::models::us::US_LAYER;
use crate::prelude::*;
use crate::types::{Finger, KCSet, PhysEv, KC};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum State {
    Layout,
    Cost,
    Finger,
}

pub fn load_layout<P: AsRef<Path>>(layout_path: P) -> Result<Layout> {
    const SKIP: &str = "|\\/";
    let mut keys = Vec::new();
    let mut l = Layout::new();
    for i in fs::read_to_string(layout_path)?.lines() {
        if i.starts_with("layer") {
            if !keys.is_empty() {
                l = l.with_layer(Layer::new(&keys));
                keys.clear();
            }
            continue;
        }
        for item in i.split(|c: char| c.is_whitespace() || SKIP.contains(c)) {
            if item.is_empty() {
                continue;
            }
            let mut kcset = enum_set!();
            for kc in item.split('+') {
                if kc != "-" {
                    kcset |= KC::from_str(kc).wrap_err(eyre!("could not find {}", kc))?;
                }
            }
            keys.push(kcset);
        }
    }
    l = l.with_layer(Layer::new(&keys));

    if l.layers.windows(2).any(|a| a[0].keys.len() != a[1].keys.len()) {
        Err(eyre!("not all layers have the same size"))
    } else {
        Ok(l)
    }
}

pub fn load_layout_cfg<P: AsRef<Path>>(cfg_path: P) -> Result<LayoutCfg> {
    const ALLOWED: &str = "RLPMIT-.0123456789X";
    let mut state = State::Layout;
    let mut layout = String::new();
    let mut cost = Vec::new();
    let mut fing = Vec::new();
    for i in fs::read_to_string(cfg_path)?.lines() {
        let mut updated = true;
        match i.trim() {
            "layout" => state = State::Layout,
            "cost" => state = State::Cost,
            "finger" => state = State::Finger,
            _ => updated = false,
        }
        if updated {
            continue;
        }
        if state == State::Layout {
            layout += i;
            layout.push('\n');
        } else {
            for item in i.split(char::is_whitespace) {
                let filtered: String = item.chars().filter(|&c| ALLOWED.contains(c)).collect();
                if filtered.is_empty() {
                    continue;
                }
                match state {
                    State::Cost => cost.push(filtered.parse::<f64>().unwrap()),
                    State::Finger => fing.push(Finger::from_str(&filtered).unwrap()),
                    State::Layout => {}
                };
            }
        }
    }

    if cost.len() != fing.len() {
        Err(eyre!("{} costs does not match {} fingers", cost.len(), fing.len()))
    } else {
        Ok(LayoutCfg { layout, cost, fing })
    }
}

pub fn load_corpus<P: AsRef<Path>>(corpus_path: P) -> Result<Vec<PhysEv>> {
    let mut corpus = Vec::new();
    for i in fs::read_to_string(corpus_path)?.lines() {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 2 {
            return Err(eyre!("weird corpus line: {}", i));
        }
        let (kc, pressed) = (items[0], items[1] == "1");
        let kcset = KCSet::from(match kc {
            "0" => KC::Num0,
            "1" => KC::Num1,
            "2" => KC::Num2,
            "3" => KC::Num3,
            "4" => KC::Num4,
            "5" => KC::Num5,
            "6" => KC::Num6,
            "7" => KC::Num7,
            "8" => KC::Num8,
            "9" => KC::Num9,
            "A" => KC::A,
            "B" => KC::B,
            "C" => KC::C,
            "D" => KC::D,
            "E" => KC::E,
            "F" => KC::F,
            "G" => KC::G,
            "H" => KC::H,
            "I" => KC::I,
            "J" => KC::J,
            "K" => KC::K,
            "L" => KC::L,
            "M" => KC::M,
            "N" => KC::N,
            "O" => KC::O,
            "P" => KC::P,
            "Q" => KC::Q,
            "R" => KC::R,
            "S" => KC::S,
            "T" => KC::T,
            "U" => KC::U,
            "V" => KC::V,
            "W" => KC::W,
            "X" => KC::X,
            "Y" => KC::Y,
            "Z" => KC::Z,
            "F1" => KC::F1,
            "F2" => KC::F2,
            "F3" => KC::F3,
            "F4" => KC::F4,
            "F5" => KC::F5,
            "F6" => KC::F6,
            "F7" => KC::F7,
            "F8" => KC::F8,
            "F9" => KC::F9,
            "F10" => KC::F10,
            "F11" => KC::F11,
            "F12" => KC::F12,
            "ENTER" => KC::Enter,
            "ESC" => KC::Esc,
            "BACKSPACE" => KC::Backspace,
            "TAB" => KC::Tab,
            "SPACE" => KC::Space,
            "INS" => KC::Insert,
            "DEL" => KC::Delete,
            "HOME" => KC::Home,
            "END" => KC::End,
            "PGUP" => KC::PageUp,
            "PGDN" => KC::PageDn,
            "UARROW" => KC::Up,
            "DARROW" => KC::Down,
            "LARROW" => KC::Left,
            "RARROW" => KC::Right,
            "NUMLOCK" => KC::NumLock,
            "SCROLLLOCK" => KC::ScrollLock,
            "PSSR" => KC::MediaVolDown,
            "PABR" => KC::Pause,
            "MENU" => KC::App,
            "MINUS" => KC::Minus,
            "EQUAL" => KC::Equals,
            "LBRACE" => KC::LeftBracket,
            "RBRACE" => KC::RightBracket,
            "BACKSLASH" => KC::Backslash,
            "SEMICOLON" => KC::Semicolon,
            "APOSTROPHE" => KC::Quote,
            "GRAVE" => KC::Grave,
            "COMMA" => KC::Comma,
            "DOT" => KC::Dot,
            "SLASH" => KC::Slash,
            "LALT" => KC::Alt,
            "LCTRL" => KC::Ctrl,
            "LSHIFT" => KC::Shift,
            "LSUPER" => KC::Super,
            "RALT" => KC::Alt,
            "RCTRL" => KC::Ctrl,
            "RSHIFT" => KC::Shift,
            _ => return Err(eyre!("unrecognised corpus key {}", kc)),
        });
        let index = US_LAYER.keys.iter().position(|&x| x == kcset).unwrap();
        corpus.push(PhysEv::new(index, pressed));
    }
    Ok(corpus)
}
