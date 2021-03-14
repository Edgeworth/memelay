use crate::constants::Constants;
use crate::ingest::{load_corpus, load_layout_cfg};
use crate::models::compute_kevs;
use crate::models::layout::Layout;
use crate::models::us::UsModel;
use crate::path::PathFinder;
use crate::types::{rand_kcset, Finger, KeyEv};
use crate::Args;
use eyre::Result;
use ga::ops::crossover::crossover_kpx;
use ga::ops::fitness::count_different;
use ga::ops::mutation::{mutate_rate, mutate_swap};
use ga::Evaluator;
use rand::Rng;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutCfg {
    pub layout: String,
    pub cost: Vec<u64>,
    pub fing: Vec<Finger>,
}

impl LayoutCfg {
    pub fn format(&self, l: &Layout) -> String {
        let mut s = String::new();
        let mut idx = 0;
        for c in self.layout.chars() {
            if c == 'X' {
                let mut kstr = format!("{:?}", l.keys[idx]);
                kstr.retain(|c| !r"() ".contains(c));
                kstr = kstr.replace("EnumSet", "").replace("|", "+");
                if kstr.is_empty() {
                    kstr = "-".to_owned();
                }
                s += &kstr;
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

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutEval {
    pub layout_cfg: LayoutCfg,
    pub kevs: Vec<KeyEv>,
    pub cnst: Constants,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let layout_cfg = load_layout_cfg(&args.cfg_path)?;
        let corpus = load_corpus(&args.corpus_path)?;
        let kevs = compute_kevs(UsModel::new(), &corpus, &args.cnst);
        Ok(Self { layout_cfg, kevs, cnst: args.cnst.clone() })
    }

    fn layout_cost(&self, l: &Layout) -> f64 {
        // Penalise more keys.
        l.keys.iter().map(|kcset| kcset.len()).sum() as f64
    }
}

impl Evaluator for LayoutEval {
    type Genome = Layout;
    const NUM_CROSSOVER: usize = 2;
    const NUM_MUTATION: usize = 2;

    fn crossover(&self, s1: &mut Layout, s2: &mut Layout, idx: usize) {
        let mut r = rand::thread_rng();
        match idx {
            0 => {} // Do nothing.
            1 => {
                // 2-pt crossover on keys level;
                crossover_kpx(&mut s1.keys, &mut s2.keys, 2);
            }
            _ => panic!("unknown crossover strategy"),
        };
        s1.normalise(&self.cnst);
        s2.normalise(&self.cnst);
    }

    fn mutate(&self, s: &mut Layout, rate: f64, idx: usize) {
        match idx {
            0 => {
                // Mutate random available key.
                mutate_rate(&mut s.keys, rate, |_| rand_kcset(&self.cnst));
            }
            1 => {
                // Swap random key
                mutate_swap(&mut s.keys, rate);
            }
            _ => panic!("unknown mutation strategy"),
        }
        s.normalise(&self.cnst);
    }

    fn fitness(&self, s: &Layout) -> f64 {
        let mut fit = 0.0;
        let res = PathFinder::new(&self.layout_cfg, &self.kevs, &self.cnst, s).path();
        // TODO: Need multi-objective EAs here.
        fit += res.kevs_found as f64 / self.kevs.len() as f64;
        if res.kevs_found == self.kevs.len() {
            fit += 1.0 / (res.cost as f64 + self.layout_cost(s) + 1.0);
        }
        fit
    }

    fn distance(&self, s1: &Layout, s2: &Layout) -> f64 {
        count_different(&s1.keys, &s2.keys) as f64
    }
}
