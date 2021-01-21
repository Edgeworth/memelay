use crate::constants::Constants;
use crate::ingest::{load_corpus, load_layout_cfg};
use crate::models::compute_kevs;
use crate::models::layout::Layout;
use crate::models::us::USModel;
use crate::path::PathFinder;
use crate::types::{rand_kcset, Finger, PhysEv};
use crate::Args;
use eyre::Result;
use ga::util::{combine_cost, combine_fitness, crossover_kpx};
use ga::{Cfg, Evaluator};
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use smallvec::smallvec;
use smallvec::SmallVec;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutCfg {
    pub layout: String,
    pub cost: Vec<u64>,
    pub fing: Vec<Finger>,
}

impl LayoutCfg {
    pub fn format(&self, l: &Layout) -> String {
        let mut s = String::new();
        for (i, layer) in l.layers.iter().enumerate() {
            s += &format!("layer {}\n", i);
            let mut idx = 0;
            for c in self.layout.chars() {
                if c == 'X' {
                    let mut kstr = format!("{:?}", layer.keys[idx]);
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
        }
        s
    }

    pub fn num_physical(&self) -> usize {
        self.cost.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutEval {
    pub layout_cfg: LayoutCfg,
    pub corpus: Vec<PhysEv>,
    pub cnst: Constants,
}

impl LayoutEval {
    pub fn from_args(args: &Args) -> Result<Self> {
        let layout_cfg = load_layout_cfg(&args.cfg_path)?;
        let corpus = load_corpus(&args.corpus_path)?;
        Ok(Self { layout_cfg, corpus, cnst: args.cnst.clone() })
    }

    fn layout_cost(&self, l: &Layout) -> u128 {
        // Penalise more layers.
        let mut cost = l.layers.len() as u128;
        for layer in l.layers.iter() {
            for kcset in layer.keys.iter() {
                // Penalise more keys.
                cost += kcset.len() as u128;
            }
        }
        cost
    }
}

impl Evaluator for LayoutEval {
    type State = Layout;
    type Fitness = u128;

    fn crossover(&self, _: &Cfg, s1: &Layout, s2: &Layout) -> SmallVec<[Layout; 2]> {
        let mut r = rand::thread_rng();
        let lidx = r.gen_range(0..s1.layers.len());
        let kidx = r.gen_range(0..s1.layers[lidx].keys.len());
        let mut c1 = s1.clone();
        let mut c2 = s2.clone();
        let idx = WeightedAliasIndex::new(self.cnst.crossover_strat_weights.clone()).unwrap();
        match idx.sample(&mut r) {
            0 => {
                // Crossover on layer level.
                let xpoint = r.gen_range(0..s1.layers.len());
                (c1.layers, c2.layers) =
                    crossover_kpx(s1.layers.clone(), s2.layers.clone(), &[xpoint]);
            }
            1 => {
                // Crossover on keys level;
                (c1.layers[lidx].keys, c2.layers[lidx].keys) = crossover_kpx(
                    s1.layers[lidx].keys.clone(),
                    s2.layers[lidx].keys.clone(),
                    &[kidx],
                );
            }
            _ => panic!("unknown crossover strategy"),
        };
        c1.normalise(&self.cnst);
        c2.normalise(&self.cnst);
        smallvec![c1, c2]
    }

    fn mutate(&self, _: &Cfg, s: &mut Layout) {
        let mut r = rand::thread_rng();
        let lidx = r.gen_range(0..s.layers.len());
        let kidx = r.gen_range(0..s.layers[lidx].keys.len());
        let lidx2 = r.gen_range(0..s.layers.len());
        let kidx2 = r.gen_range(0..s.layers[lidx2].keys.len());

        let idx = WeightedAliasIndex::new(self.cnst.mutate_strat_weights.clone()).unwrap();
        match idx.sample(&mut r) {
            0 => {
                // Mutate random available key.
                let avail = s.layers[lidx].keys.iter_mut().filter(|k| !k.is_empty());
                if let Some(kcset) = avail.choose(&mut r) {
                    *kcset = rand_kcset(&mut r, &self.cnst);
                }
            }
            1 => {
                // Mutate random empty key.
                let empty = s.layers[lidx].keys.iter_mut().filter(|k| k.is_empty());
                if let Some(kcset) = empty.choose(&mut r) {
                    *kcset = rand_kcset(&mut r, &self.cnst);
                }
            }
            2 => {
                // Swap random layer.
                let swap_idx = r.gen_range(0..s.layers.len());
                s.layers.swap(lidx, swap_idx);
            }
            3 => {
                // Swap random key
                let tmp = s.layers[lidx].keys[kidx];
                s.layers[lidx].keys[kidx] = s.layers[lidx2].keys[kidx2];
                s.layers[lidx2].keys[kidx2] = tmp;
            }
            _ => panic!("unknown mutation strategy"),
        }
        s.normalise(&self.cnst);
    }

    fn fitness(&self, _: &Cfg, s: &Layout) -> u128 {
        let mut path_cost_mean = 0;
        let mut r = rand::thread_rng();
        let block_size = self.cnst.batch_size.min(self.corpus.len());
        let start_idx = r.gen_range(0..=(self.corpus.len() - block_size));
        for _ in 0..self.cnst.batch_num {
            let kevs = compute_kevs(
                USModel::new(),
                &self.corpus[start_idx..(start_idx + block_size)],
                &self.cnst,
            );
            let res = PathFinder::new(&self.layout_cfg, &kevs, &self.cnst, s).path();
            let fitness = combine_fitness(0, res.kevs_found as u128, kevs.len() as u128);
            let fitness = combine_cost(fitness, res.cost as u128, kevs.len() as u128 * 1000000);
            path_cost_mean += fitness;
        }
        let fitness = path_cost_mean / self.cnst.batch_num as u128;
        combine_cost(fitness, self.layout_cost(s), 1000)
    }

    fn distance(&self, _: &Cfg, s1: &Layout, s2: &Layout) -> f64 {
        let mut dist = 0.0;
        let layer_min = s1.layers.len().min(s2.layers.len());
        let layer_max = s1.layers.len().max(s2.layers.len());
        // Different # layers is a big difference.
        dist += ((layer_max - layer_min) * self.layout_cfg.num_physical()) as f64;
        for i in 0..layer_min {
            for j in 0..s1.layers[i].keys.len() {
                if s1.layers[i].keys[j] != s2.layers[i].keys[j] {
                    dist += 1.0;
                }
            }
        }
        // TODO: this divide by 500.0
        dist / 500.0
    }
}
