use crate::eval::{Histograms, Params};
use crate::layout::Layout;
use crate::types::Kc;
use eyre::{eyre, Result, WrapErr};
use std::collections::HashMap;
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

pub fn load_layouts<P: AsRef<Path>>(layout_path: P) -> Result<Vec<Layout>> {
    const SKIP: &str = "|\\";
    let mut keys = Vec::new();
    let mut layouts = Vec::new();
    for i in fs::read_to_string(layout_path)?.lines() {
        if i.is_empty() {
            layouts.push(Layout::new(keys.clone()));
            keys.clear();
        }
        for kc in i.split(|c: char| c.is_whitespace() || SKIP.contains(c)) {
            if kc.is_empty() {
                continue;
            }
            keys.push(Kc::from_str(kc).wrap_err(eyre!("could not find {}", kc))?);
        }
    }
    if !keys.is_empty() {
        layouts.push(Layout::new(keys));
    }
    for v in layouts.iter() {
        if v.keys.len() != layouts[0].keys.len() {
            return Err(eyre!("not all layouts the same size"));
        }
    }
    Ok(layouts)
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

pub fn load_histograms<P: AsRef<Path>>(unigrams_path: P, bigrams_path: P) -> Result<Histograms> {
    let mut unigrams: HashMap<Kc, f64> = HashMap::new();
    let mut bigrams: HashMap<(Kc, Kc), f64> = HashMap::new();
    for i in fs::read_to_string(unigrams_path)?.lines().skip(1) {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 2 {
            return Err(eyre!("weird unigrams line: {}", i));
        }
        let (kcstr, count) = (items[0], items[1].parse::<f64>()?);
        let kc = Kc::from_str(kcstr)?;
        unigrams.insert(kc, count).expect_none("duplicate unigram");
    }

    for i in fs::read_to_string(bigrams_path)?.lines().skip(1) {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 3 {
            return Err(eyre!("weird unigrams line: {}", i));
        }
        let (kcstr1, kcstr2, count) = (items[0], items[1], items[2].parse::<f64>()?);
        let kc1 = Kc::from_str(kcstr1)?;
        let kc2 = Kc::from_str(kcstr2)?;
        bigrams.insert((kc1, kc2), count).expect_none("duplicate bigram");
    }

    Ok(Histograms { unigrams, bigrams })
}
