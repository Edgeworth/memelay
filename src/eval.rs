use crate::ingest::{load_keys, load_params};
use crate::layout::Layout;
use crate::types::Kc;
use crate::Args;
use eyre::Result;
use memega::ops::fitness::count_different;
use memega::ops::mutation::{mutate_gen, mutate_rate, mutate_swap};
use memega::Evaluator;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Params {
    pub layout: String,
    pub cost: Vec<f64>,
    pub row: Vec<i32>,
    pub hand: Vec<i32>,
    pub finger: Vec<i32>,
}

impl Params {
    pub fn format(&self, l: &Layout) -> String {
        let mut s = String::new();
        let mut idx = 0;
        for c in self.layout.chars() {
            if c == 'X' {
                s += &format!("{:?}", l.keys[idx]);
                idx += 1;
            } else {
                s.push(c);
            }
        }
        s.push('\n');
        s
    }
}

#[derive(Debug, Clone)]
pub struct LayoutEval {
    pub params: Params,
    pub keys: Vec<Kc>,
    pub match_keys: Vec<Kc>,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let params = load_params(&args.params_path)?;
        let keys = load_keys(&args.data_path)?;
        let match_keys = vec![
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
            Kc::Scolon,
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
        Ok(Self { params, keys, match_keys })
    }
}

impl Evaluator for LayoutEval {
    type Genome = Layout;
    const NUM_CROSSOVER: usize = 1;
    const NUM_MUTATION: usize = 2;

    fn crossover(&self, _s1: &mut Layout, _s2: &mut Layout, idx: usize) {
        match idx {
            0 => {} // Do nothing.
            _ => panic!("unknown crossover strategy"),
        };
    }

    fn mutate(&self, s: &mut Layout, rate: f64, idx: usize) {
        match idx {
            0 => {
                // Mutate random available key.
                mutate_rate(&mut s.keys, rate, |_| mutate_gen());
            }
            1 => {
                // Swap random key
                mutate_swap(&mut s.keys, rate);
            }
            _ => panic!("unknown mutation strategy"),
        }
    }

    fn fitness(&self, s: &Layout) -> f64 {
        // Indexed by row jump length.
        const SAME_FING: &[f64] = &[2.5, 3.0, 4.0];
        const PINKY_RING: &[f64] = &[0.5, 1.0, 1.5];
        const RING_MID: &[f64] = &[0.1, 0.2, 0.3];
        let mut cost = 0.0;

        let mut prev = Kc::None;
        for &kc in self.keys.iter() {
            // Finger penalties
            if let Some(curi) = s.keys.iter().position(|&v| v == kc) {
                cost += self.params.cost[curi];

                // Bigram penalities
                if let Some(previ) = s.keys.iter().position(|&v| v == prev) {
                    // Model from https://colemakmods.github.io/mod-dh/compare.html
                    let cfing = self.params.finger[curi];
                    let pfing = self.params.finger[previ];
                    let crow = self.params.row[curi];
                    let prow = self.params.row[previ];
                    let chand = self.params.hand[curi];
                    let phand = self.params.hand[previ];
                    let same_hand = chand == phand;
                    let same_fing = same_hand && cfing == pfing;
                    let pinky_ring =
                        same_hand && (cfing == 3 && pfing == 2 || cfing == 2 && pfing == 3);
                    let ring_mid =
                        same_hand && (cfing == 2 && pfing == 1 || cfing == 1 && pfing == 2);
                    let jump_len = (crow - prow).abs() as usize;

                    if same_fing {
                        cost += SAME_FING[jump_len];
                    }
                    if pinky_ring {
                        cost += PINKY_RING[jump_len];
                    }
                    if ring_mid {
                        cost += RING_MID[jump_len];
                    }
                }
            }
            prev = kc;
        }


        // Tie-breaking: similarity to qwerty:
        cost += count_different(&s.keys, &self.match_keys) as f64;

        1.0 / (cost + 1.0)
    }

    fn distance(&self, s1: &Layout, s2: &Layout) -> f64 {
        count_different(&s1.keys, &s2.keys) as f64
    }
}
