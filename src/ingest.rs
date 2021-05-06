use std::fs;
use std::path::Path;
use std::str::FromStr;

use eyre::{eyre, Result, WrapErr};

use crate::eval::Histograms;
use crate::model::Model;
use crate::types::Kc;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum State {
    Layout,
    Keys,
    Fixed,
    UnigramCost,
    BigramCost,
    Row,
    Hand,
    Finger,
}

pub fn load_seeds<P: AsRef<Path>>(layout_path: P) -> Result<Vec<Vec<Kc>>> {
    let mut keys = Vec::new();
    let mut layouts = Vec::new();
    for i in fs::read_to_string(layout_path)?.lines() {
        if i.is_empty() {
            layouts.push(keys.clone());
            keys.clear();
        }
        for kc in i.split(char::is_whitespace) {
            if kc.is_empty() {
                continue;
            }
            keys.push(Kc::from_str(kc).wrap_err(eyre!("could not find {}", kc))?);
        }
    }
    if !keys.is_empty() {
        layouts.push(keys);
    }
    for v in layouts.iter() {
        if v.len() != layouts[0].len() {
            return Err(eyre!("not all layouts the same size"));
        }
    }
    Ok(layouts)
}

pub fn load_model<P: AsRef<Path>>(cfg_path: P) -> Result<Model> {
    let mut state = State::Layout;
    let mut layout = String::new();
    let mut keys = Vec::new();
    let mut fixed = Vec::new();
    let mut unigram_cost = Vec::new();
    let mut bigram_cost = [[[0.0; 5]; 4]; 4];
    let mut bigram_idx = 0;
    let mut row = Vec::new();
    let mut hand = Vec::new();
    let mut finger = Vec::new();
    for i in fs::read_to_string(cfg_path)?.lines() {
        let mut updated = true;
        if i.starts_with("layout") {
            state = State::Layout;
        } else if i.starts_with("keys") {
            state = State::Keys;
        } else if i.starts_with("fixed") {
            state = State::Fixed;
        } else if i.starts_with("unigram_cost") {
            state = State::UnigramCost;
        } else if i.starts_with("bigram_cost") {
            state = State::BigramCost;
        } else if i.starts_with("row") {
            state = State::Row;
        } else if i.starts_with("hand") {
            state = State::Hand;
        } else if i.starts_with("finger") {
            state = State::Finger;
        } else {
            updated = false;
        }

        if updated {
            continue;
        }
        if state == State::Layout {
            layout += i;
            layout.push('\n');
            continue;
        }
        for s in i.split(char::is_whitespace) {
            if s.is_empty() {
                continue;
            }
            match state {
                State::Layout => {}
                State::Keys => keys.push(Kc::from_str(&s).unwrap()),
                State::Fixed => fixed.push(Kc::from_str(&s).unwrap_or_default()),
                State::UnigramCost => unigram_cost.push(s.parse::<f64>().unwrap()),
                State::BigramCost => {
                    bigram_cost[bigram_idx / 5 / 4][bigram_idx / 5 % 4][bigram_idx % 5] =
                        s.parse::<f64>().unwrap();
                    bigram_idx += 1;
                }
                State::Row => row.push(s.parse::<i32>().unwrap()),
                State::Hand => hand.push(s.parse::<i32>().unwrap()),
                State::Finger => finger.push(s.parse::<i32>().unwrap()),
            };
        }
    }
    assert_eq!(bigram_idx, 80, "missing bigram costs");

    Ok(Model { layout, universe: keys, fixed, unigram_cost, bigram_cost, row, hand, finger })
}

pub fn load_histograms<P: AsRef<Path>>(
    unigrams_path: P,
    bigrams_path: P,
    trigrams_path: P,
) -> Result<Histograms> {
    let mut unigrams: Vec<(Kc, f64)> = Vec::new();
    for i in fs::read_to_string(unigrams_path)?.lines().skip(1) {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 2 {
            return Err(eyre!("weird unigrams line: {}", i));
        }
        let (kcstr, count) = (items[0], items[1].parse::<f64>()?);
        let kc = Kc::from_str(kcstr)?;
        unigrams.push((kc, count));
    }

    let mut bigrams: Vec<((Kc, Kc), f64)> = Vec::new();
    for i in fs::read_to_string(bigrams_path)?.lines().skip(1) {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 3 {
            return Err(eyre!("weird unigrams line: {}", i));
        }
        let (kcstr1, kcstr2, count) = (items[0], items[1], items[2].parse::<f64>()?);
        let kc1 = Kc::from_str(kcstr1)?;
        let kc2 = Kc::from_str(kcstr2)?;
        bigrams.push(((kc1, kc2), count));
    }

    let mut trigrams: Vec<((Kc, Kc, Kc), f64)> = Vec::new();
    for i in fs::read_to_string(trigrams_path)?.lines().skip(1) {
        let items = i.split(char::is_whitespace).collect::<Vec<_>>();
        if items.len() != 4 {
            return Err(eyre!("weird unigrams line: {}", i));
        }
        let (kcstr1, kcstr2, kcstr3, count) =
            (items[0], items[1], items[2], items[3].parse::<f64>()?);
        let kc1 = Kc::from_str(kcstr1)?;
        let kc2 = Kc::from_str(kcstr2)?;
        let kc3 = Kc::from_str(kcstr3)?;
        trigrams.push(((kc1, kc2, kc3), count));
    }


    Ok(Histograms { unigrams, bigrams, trigrams })
}
