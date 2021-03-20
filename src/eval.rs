use crate::ingest::{load_keys, load_layout_cfg};
use crate::layout::Layout;
use crate::types::Kc;
use crate::Args;
use eyre::Result;
use memega::ops::fitness::count_different;
use memega::ops::mutation::{mutate_gen, mutate_rate, mutate_swap};
use memega::Evaluator;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutCfg {
    pub layout: String,
    pub cost: Vec<u64>,
}

impl LayoutCfg {
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

    pub fn num_physical(&self) -> usize {
        self.cost.len()
    }
}

#[derive(Debug, Clone)]
pub struct LayoutEval {
    pub layout_cfg: LayoutCfg,
    pub keys: Vec<Kc>,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let layout_cfg = load_layout_cfg(&args.cfg_path)?;
        let keys = load_keys(&args.data_path)?;
        Ok(Self { layout_cfg, keys })
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
        let mut fitness = 0.0;
        for kc in self.keys.iter() {
            if s.keys.contains(kc) {
                fitness += 1.0;
            }
        }
        fitness
    }

    fn distance(&self, s1: &Layout, s2: &Layout) -> f64 {
        count_different(&s1.keys, &s2.keys) as f64
    }
}
