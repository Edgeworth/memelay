use derive_more::{Deref, DerefMut, Display};
use eyre::Result;
use memega::eval::Evaluator;
use memega::ops::crossover::{crossover_cycle, crossover_order, crossover_pmx};
use memega::ops::distance::count_different;
use memega::ops::mutation::{mutate_insert, mutate_inversion, mutate_scramble, mutate_swap};
use rand::Rng;

use crate::ingest::{load_histograms, load_model};
use crate::model::{Model, PENALTY};
use crate::types::{Kc, COLEMAK_DHM};
use crate::Args;

#[must_use]
#[derive(Debug, Clone)]
pub struct Histograms {
    pub unigrams: Vec<(Kc, f64)>,
    pub bigrams: Vec<((Kc, Kc), f64)>,
    pub trigrams: Vec<((Kc, Kc, Kc), f64)>,
}

#[must_use]
#[derive(Debug, Clone)]
pub struct LayoutEval {
    pub model: Model,
    pub match_keys: Vec<Kc>,
    pub hist: Histograms,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let model = load_model(&args.model_path)?;
        let hist = load_histograms(&args.unigrams_path, &args.bigrams_path, &args.trigrams_path)?;
        Ok(Self { model, hist, match_keys: COLEMAK_DHM.to_vec() })
    }
}

#[must_use]
#[derive(Debug, Display, Deref, DerefMut, Hash, Clone, PartialEq, Eq, PartialOrd)]
#[display(fmt = "{_0:?}")]
pub struct KeyState(pub Vec<Kc>);

impl Evaluator for LayoutEval {
    type State = KeyState;
    const NUM_CROSSOVER: usize = 4;
    const NUM_MUTATION: usize = 4;

    fn crossover(&self, s1: &mut Self::State, s2: &mut Self::State, idx: usize) {
        // Crossover without touching fixed keys.
        let mut unfixed1 = self.model.without_fixed(s1);
        let mut unfixed2 = self.model.without_fixed(s2);
        match idx {
            0 => {} // Do nothing.
            1 => {
                crossover_pmx(&mut unfixed1, &mut unfixed2);
            }
            2 => {
                crossover_order(&mut unfixed1, &mut unfixed2);
            }
            3 => {
                crossover_cycle(&mut unfixed1, &mut unfixed2);
            }
            _ => panic!("unknown crossover strategy"),
        };
        *s1 = KeyState(self.model.with_fixed(&unfixed1));
        *s2 = KeyState(self.model.with_fixed(&unfixed2));
    }

    fn mutate(&self, s: &mut Self::State, rate: f64, idx: usize) {
        let mut r = rand::thread_rng();
        let mutate = r.gen::<f64>() < rate;
        // Mutate without touching fixed keys.
        let mut unfixed = self.model.without_fixed(s);
        match idx {
            0 => {
                if mutate {
                    mutate_swap(&mut unfixed);
                }
            }
            1 => {
                if mutate {
                    mutate_insert(&mut unfixed);
                }
            }
            2 => {
                if mutate {
                    mutate_scramble(&mut unfixed);
                }
            }
            3 => {
                if mutate {
                    mutate_inversion(&mut unfixed);
                }
            }
            _ => panic!("unknown mutation strategy"),
        }
        *s = KeyState(self.model.with_fixed(&unfixed));
    }

    fn fitness(&self, s: &Self::State, _data: &Self::Data) -> Result<f64> {
        let mut cost = 0.0;

        cost += self.model.unigram_cost(s, &self.hist.unigrams);
        cost += self.model.bigram_cost(s, &self.hist.bigrams);
        cost += self.model.trigram_cost(s, &self.hist.trigrams);

        #[must_use]
        struct Cons {
            a: Kc,
            b: Kc,
            horiz: bool,
            ordered: bool,
        }

        let horiz = [
            Cons { a: Kc::Comma, b: Kc::Dot, horiz: true, ordered: true }, // Keep , and . next to eachother.
            Cons { a: Kc::DoubleQuote, b: Kc::Quote, horiz: false, ordered: false }, // " and ' vert.
            Cons { a: Kc::Asterisk, b: Kc::Ampersand, horiz: false, ordered: false }, // * and & vert.
            Cons { a: Kc::Ampersand, b: Kc::Bar, horiz: false, ordered: false }, // | and & vert.
            Cons { a: Kc::Minus, b: Kc::Plus, horiz: false, ordered: false },    // - and + vert.
        ];
        for Cons { a, b, horiz, ordered } in horiz {
            let apos = s.iter().position(|&v| v == a);
            let bpos = s.iter().position(|&v| v == b);
            if bpos.is_none() || apos.is_none() {
                continue;
            }
            let apos = apos.unwrap();
            let bpos = bpos.unwrap();
            let (ab, ba) = if horiz {
                (apos + 1 == bpos, bpos + 1 == apos)
            } else {
                let abelow = self.model.key_below(apos);
                let bbelow = self.model.key_below(bpos);
                let ab = if let Some(abelow) = abelow { abelow == bpos } else { false };
                let ba = if let Some(bbelow) = bbelow { bbelow == apos } else { false };
                (ab, ba)
            };
            if (!ba || ordered) && !ab {
                cost += PENALTY;
            }
        }

        // Check fixed keys
        for (i, &kc) in self.model.fixed.iter().enumerate() {
            if kc != Kc::None && kc != s[i] {
                cost += PENALTY;
            }
        }

        // Tie-breaking: similarity to given existing layout:
        cost += count_different(s, &self.match_keys) as f64 / 100000.0;

        // 1.0 / (cost + 1.0)
        Ok((-cost).exp())
    }

    fn distance(&self, s1: &Self::State, s2: &Self::State) -> Result<f64> {
        let mut d = 0.0;
        for i in 0..s1.len() {
            d += (i8::from(s1[i]) - i8::from(s2[i])).abs() as f64;
        }
        Ok(d)
    }
}
