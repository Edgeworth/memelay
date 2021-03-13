use crate::layout_eval::LayoutCfg;
use crate::models::layout::{Layer, Layout};
use crate::models::us::US_LAYER;
use crate::types::{Finger, Kc, KcSet, PhysEv};
use enumset::enum_set;
use eyre::{eyre, Result, WrapErr};
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
                    kcset |= Kc::from_str(kc).wrap_err(eyre!("could not find {}", kc))?;
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
                    State::Cost => cost.push(filtered.parse::<u64>().unwrap()),
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
        let kcset = KcSet::from(match kc {
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
            _ => return Err(eyre!("unrecognised corpus key {}", kc)),
        });
        let index = US_LAYER.keys.iter().position(|&x| x == kcset).unwrap();
        corpus.push(PhysEv::new(index, pressed));
    }
    Ok(corpus)
}
