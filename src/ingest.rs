use std::fs;
use std::path::Path;
use std::str::FromStr;

use eyre::{Result, WrapErr, eyre};

use crate::eval::{Histograms, KeyState};
use crate::model::Model;
use crate::types::Kc;

#[must_use]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum ParseStage {
    Layout,
    Keys,
    Fixed,
    UnigramCost,
    BigramCost,
    Row,
    Hand,
    Finger,
}

pub fn load_seeds<P: AsRef<Path>>(layout_path: P) -> Result<Vec<KeyState>> {
    let mut keys = Vec::new();
    let mut layouts = Vec::new();
    for i in fs::read_to_string(layout_path)?.lines() {
        if i.is_empty() {
            layouts.push(KeyState(keys.clone()));
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
        layouts.push(KeyState(keys));
    }
    for v in &layouts {
        if v.len() != layouts[0].len() {
            return Err(eyre!("not all layouts the same size"));
        }
    }
    Ok(layouts)
}

pub fn load_model<P: AsRef<Path>>(cfg_path: P) -> Result<Model> {
    let mut state = ParseStage::Layout;
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
            state = ParseStage::Layout;
        } else if i.starts_with("keys") {
            state = ParseStage::Keys;
        } else if i.starts_with("fixed") {
            state = ParseStage::Fixed;
        } else if i.starts_with("unigram_cost") {
            state = ParseStage::UnigramCost;
        } else if i.starts_with("bigram_cost") {
            state = ParseStage::BigramCost;
        } else if i.starts_with("row") {
            state = ParseStage::Row;
        } else if i.starts_with("hand") {
            state = ParseStage::Hand;
        } else if i.starts_with("finger") {
            state = ParseStage::Finger;
        } else {
            updated = false;
        }

        if updated {
            continue;
        }
        if state == ParseStage::Layout {
            layout += i;
            layout.push('\n');
            continue;
        }
        for s in i.split(char::is_whitespace) {
            if s.is_empty() {
                continue;
            }
            match state {
                ParseStage::Layout => {}
                ParseStage::Keys => keys.push(Kc::from_str(s)?),
                ParseStage::Fixed => fixed.push(Kc::from_str(s).unwrap_or_default()),
                ParseStage::UnigramCost => unigram_cost.push(s.parse::<f64>()?),
                ParseStage::BigramCost => {
                    bigram_cost[bigram_idx / 5 / 4][bigram_idx / 5 % 4][bigram_idx % 5] =
                        s.parse::<f64>()?;
                    bigram_idx += 1;
                }
                ParseStage::Row => row.push(s.parse::<i32>()?),
                ParseStage::Hand => hand.push(s.parse::<i32>()?),
                ParseStage::Finger => finger.push(s.parse::<i32>()?),
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_seeds_single_layout() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file, "d e f").unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts.len(), 1);
        assert_eq!(layouts[0].0, vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    }

    #[test]
    fn test_load_seeds_multiple_layouts() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "d e f").unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts.len(), 2);
        assert_eq!(layouts[0].0, vec![Kc::A, Kc::B, Kc::C]);
        assert_eq!(layouts[1].0, vec![Kc::D, Kc::E, Kc::F]);
    }

    #[test]
    fn test_load_seeds_whitespace_handling() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a  b   c").unwrap();
        writeln!(file, "  d e f  ").unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts.len(), 1);
        assert_eq!(layouts[0].0, vec![Kc::A, Kc::B, Kc::C, Kc::D, Kc::E, Kc::F]);
    }

    #[test]
    fn test_load_seeds_empty_lines_at_end() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "d e f").unwrap();
        writeln!(file).unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts.len(), 2);
    }

    #[test]
    fn test_load_seeds_invalid_keycode() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a invalid c").unwrap();
        file.flush().unwrap();

        let result = load_seeds(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_seeds_different_sizes() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "d e").unwrap();
        file.flush().unwrap();

        let result = load_seeds(file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not all layouts the same size"));
    }

    #[test]
    fn test_load_seeds_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts.len(), 0);
    }

    #[test]
    fn test_load_histograms_unigrams() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        writeln!(unigrams_file, "a 100.5").unwrap();
        writeln!(unigrams_file, "b 50.25").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let hist = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        )
        .unwrap();

        assert_eq!(hist.unigrams.len(), 2);
        assert_eq!(hist.unigrams[0], (Kc::A, 100.5));
        assert_eq!(hist.unigrams[1], (Kc::B, 50.25));
    }

    #[test]
    fn test_load_histograms_bigrams() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        writeln!(bigrams_file, "a b 10.5").unwrap();
        writeln!(bigrams_file, "c d 20.25").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let hist = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        )
        .unwrap();

        assert_eq!(hist.bigrams.len(), 2);
        assert_eq!(hist.bigrams[0], ((Kc::A, Kc::B), 10.5));
        assert_eq!(hist.bigrams[1], ((Kc::C, Kc::D), 20.25));
    }

    #[test]
    fn test_load_histograms_trigrams() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        writeln!(trigrams_file, "a b c 5.5").unwrap();
        writeln!(trigrams_file, "d e f 7.25").unwrap();
        trigrams_file.flush().unwrap();

        let hist = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        )
        .unwrap();

        assert_eq!(hist.trigrams.len(), 2);
        assert_eq!(hist.trigrams[0], ((Kc::A, Kc::B, Kc::C), 5.5));
        assert_eq!(hist.trigrams[1], ((Kc::D, Kc::E, Kc::F), 7.25));
    }

    #[test]
    fn test_load_histograms_invalid_unigram_format() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        writeln!(unigrams_file, "a").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let result = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_load_histograms_invalid_bigram_format() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        writeln!(bigrams_file, "a b").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let result = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_load_histograms_invalid_trigram_format() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        writeln!(trigrams_file, "a b c").unwrap();
        trigrams_file.flush().unwrap();

        let result = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_load_histograms_invalid_number() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        writeln!(unigrams_file, "a notanumber").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let result = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_load_model_basic() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "layout").unwrap();
        writeln!(file, "X X X").unwrap();
        writeln!(file, "keys").unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file, "fixed").unwrap();
        writeln!(file, "None None None").unwrap();
        writeln!(file, "unigram_cost").unwrap();
        writeln!(file, "1.0 2.0 3.0").unwrap();
        writeln!(file, "bigram_cost").unwrap();
        for _ in 0..80 {
            write!(file, "0.0 ").unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file, "row").unwrap();
        writeln!(file, "0 0 0").unwrap();
        writeln!(file, "hand").unwrap();
        writeln!(file, "0 0 1").unwrap();
        writeln!(file, "finger").unwrap();
        writeln!(file, "0 1 0").unwrap();
        file.flush().unwrap();

        let model = load_model(file.path()).unwrap();
        assert_eq!(model.universe, vec![Kc::A, Kc::B, Kc::C]);
        assert_eq!(model.fixed, vec![Kc::None, Kc::None, Kc::None]);
        assert_eq!(model.unigram_cost, vec![1.0, 2.0, 3.0]);
        assert_eq!(model.row, vec![0, 0, 0]);
        assert_eq!(model.hand, vec![0, 0, 1]);
        assert_eq!(model.finger, vec![0, 1, 0]);
    }

    #[test]
    fn test_load_model_with_fixed_keys() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "layout").unwrap();
        writeln!(file, "X X").unwrap();
        writeln!(file, "keys").unwrap();
        writeln!(file, "a b").unwrap();
        writeln!(file, "fixed").unwrap();
        writeln!(file, "a None").unwrap();
        writeln!(file, "unigram_cost").unwrap();
        writeln!(file, "1.0 2.0").unwrap();
        writeln!(file, "bigram_cost").unwrap();
        for _ in 0..80 {
            write!(file, "0.0 ").unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file, "row").unwrap();
        writeln!(file, "0 0").unwrap();
        writeln!(file, "hand").unwrap();
        writeln!(file, "0 0").unwrap();
        writeln!(file, "finger").unwrap();
        writeln!(file, "0 1").unwrap();
        file.flush().unwrap();

        let model = load_model(file.path()).unwrap();
        assert_eq!(model.fixed, vec![Kc::A, Kc::None]);
    }

    #[test]
    fn test_load_model_layout_preservation() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "layout").unwrap();
        writeln!(file, "X X X").unwrap();
        writeln!(file, "Y Y Y").unwrap();
        writeln!(file, "keys").unwrap();
        writeln!(file, "a b c").unwrap();
        writeln!(file, "fixed").unwrap();
        writeln!(file, "None None None").unwrap();
        writeln!(file, "unigram_cost").unwrap();
        writeln!(file, "1.0 2.0 3.0").unwrap();
        writeln!(file, "bigram_cost").unwrap();
        for _ in 0..80 {
            write!(file, "0.0 ").unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file, "row").unwrap();
        writeln!(file, "0 0 0").unwrap();
        writeln!(file, "hand").unwrap();
        writeln!(file, "0 0 1").unwrap();
        writeln!(file, "finger").unwrap();
        writeln!(file, "0 1 0").unwrap();
        file.flush().unwrap();

        let model = load_model(file.path()).unwrap();
        assert!(model.layout.contains("X X X"));
        assert!(model.layout.contains("Y Y Y"));
    }

    #[test]
    fn test_histograms_clone() {
        let hist = Histograms {
            unigrams: vec![(Kc::A, 1.0)],
            bigrams: vec![((Kc::A, Kc::B), 2.0)],
            trigrams: vec![((Kc::A, Kc::B, Kc::C), 3.0)],
        };
        let cloned = hist.clone();
        assert_eq!(hist.unigrams, cloned.unigrams);
        assert_eq!(hist.bigrams, cloned.bigrams);
        assert_eq!(hist.trigrams, cloned.trigrams);
    }

    #[test]
    fn test_load_seeds_only_whitespace_lines() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "   ").unwrap();
        writeln!(file, "\t\t").unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert!(layouts.is_empty() || layouts[0].0.is_empty());
    }

    #[test]
    fn test_load_seeds_mixed_whitespace() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "a\tb\t c").unwrap();
        file.flush().unwrap();

        let layouts = load_seeds(file.path()).unwrap();
        assert_eq!(layouts[0].0, vec![Kc::A, Kc::B, Kc::C]);
    }

    #[test]
    fn test_load_histograms_empty_files() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let hist = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        )
        .unwrap();

        assert_eq!(hist.unigrams.len(), 0);
        assert_eq!(hist.bigrams.len(), 0);
        assert_eq!(hist.trigrams.len(), 0);
    }

    #[test]
    fn test_load_histograms_multiple_entries() {
        let mut unigrams_file = NamedTempFile::new().unwrap();
        writeln!(unigrams_file, "header").unwrap();
        for i in 0..10 {
            writeln!(unigrams_file, "a {}.0", i).unwrap();
        }
        unigrams_file.flush().unwrap();

        let mut bigrams_file = NamedTempFile::new().unwrap();
        writeln!(bigrams_file, "header").unwrap();
        bigrams_file.flush().unwrap();

        let mut trigrams_file = NamedTempFile::new().unwrap();
        writeln!(trigrams_file, "header").unwrap();
        trigrams_file.flush().unwrap();

        let hist = load_histograms(
            unigrams_file.path(),
            bigrams_file.path(),
            trigrams_file.path(),
        )
        .unwrap();

        assert_eq!(hist.unigrams.len(), 10);
    }

    #[test]
    fn test_load_model_whitespace_handling() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "layout").unwrap();
        writeln!(file, "X X X").unwrap();
        writeln!(file, "keys").unwrap();
        writeln!(file, "  a   b   c  ").unwrap();
        writeln!(file, "fixed").unwrap();
        writeln!(file, "None None None").unwrap();
        writeln!(file, "unigram_cost").unwrap();
        writeln!(file, "1.0  2.0  3.0").unwrap();
        writeln!(file, "bigram_cost").unwrap();
        for _ in 0..80 {
            write!(file, "0.0 ").unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file, "row").unwrap();
        writeln!(file, "0 0 0").unwrap();
        writeln!(file, "hand").unwrap();
        writeln!(file, "0 0 1").unwrap();
        writeln!(file, "finger").unwrap();
        writeln!(file, "0 1 0").unwrap();
        file.flush().unwrap();

        let model = load_model(file.path()).unwrap();
        assert_eq!(model.universe, vec![Kc::A, Kc::B, Kc::C]);
    }

    #[test]
    fn test_parse_stage_values() {
        let stages = [
            ParseStage::Layout,
            ParseStage::Keys,
            ParseStage::Fixed,
            ParseStage::UnigramCost,
            ParseStage::BigramCost,
            ParseStage::Row,
            ParseStage::Hand,
            ParseStage::Finger,
        ];

        for stage in &stages {
            let debug_str = format!("{:?}", stage);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_histograms_debug() {
        let hist = Histograms {
            unigrams: vec![],
            bigrams: vec![],
            trigrams: vec![],
        };
        let debug_str = format!("{:?}", hist);
        assert!(!debug_str.is_empty());
    }
}
