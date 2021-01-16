use crate::constants::Constants;
use crate::ga::util::{combine_cost, crossover_vec};
use crate::ga::{Cfg, Evaluator};
use crate::ingest::{load_corpus, load_layout_cfg};
use crate::models::layout::Layout;
use crate::path::PathFinder;
use crate::prelude::*;
use crate::types::{rand_kcset, Finger, PhysEv};
use crate::Args;
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutCfg {
    pub layout: String,
    pub cost: Vec<f64>,
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
    type T = Layout;
    type F = u128;

    fn reproduce(&self, cfg: &Cfg, a: &Layout, b: &Layout) -> Layout {
        let mut r = rand::thread_rng();
        let layer_idx = r.gen_range(0..a.layers.len());
        let key_idx = r.gen_range(0..a.layers[layer_idx].keys.len());
        let layer_idx2 = r.gen_range(0..a.layers.len());
        let key_idx2 = r.gen_range(0..a.layers[layer_idx2].keys.len());

        let mut l = if r.gen::<f64>() < cfg.xover_rate {
            let idx = WeightedAliasIndex::new(self.cnst.crossover_strat_weights.clone()).unwrap();
            let mut l = Layout::new();

            match idx.sample(&mut r) {
                0 => {
                    // Crossover on layer level.
                    let crosspoint = r.gen_range(0..a.layers.len());
                    l.layers = crossover_vec(&a.layers, &b.layers, crosspoint);
                }
                1 => {
                    // Crossover on keys level;
                    l.layers = a.layers.clone();
                    l.layers[layer_idx].keys = crossover_vec(
                        &a.layers[layer_idx].keys,
                        &b.layers[layer_idx].keys,
                        key_idx,
                    );
                }
                _ => panic!("unknown crossover strategy"),
            }
            l
        } else {
            let idx = WeightedAliasIndex::new(self.cnst.mutate_strat_weights.clone()).unwrap();
            let mut l = a.clone();

            match idx.sample(&mut r) {
                0 => {
                    // Mutate random available key.
                    let avail = l.layers[layer_idx].keys.iter_mut().filter(|k| !k.is_empty());
                    if let Some(kcset) = avail.choose(&mut r) {
                        *kcset = rand_kcset(&mut r, &self.cnst);
                    }
                }
                1 => {
                    // Mutate random empty key.
                    let empty = l.layers[layer_idx].keys.iter_mut().filter(|k| k.is_empty());
                    if let Some(kcset) = empty.choose(&mut r) {
                        *kcset = rand_kcset(&mut r, &self.cnst);
                    }
                }
                2 => {
                    // Swap random layer.
                    let swap_idx = r.gen_range(0..a.layers.len());
                    l.layers.swap(layer_idx, swap_idx);
                }
                3 => {
                    // Swap random key
                    let tmp = l.layers[layer_idx].keys[key_idx];
                    l.layers[layer_idx].keys[key_idx] = l.layers[layer_idx2].keys[key_idx2];
                    l.layers[layer_idx2].keys[key_idx2] = tmp;
                }
                _ => panic!("unknown mutation strategy"),
            }
            l
        };
        l.normalise(&self.cnst);
        l
    }

    fn fitness(&self, _: &Cfg, a: &Layout) -> u128 {
        let block_size = self.cnst.batch_size.min(self.corpus.len());
        let mut shortest_path_cost_avg = 0;
        let mut r = rand::thread_rng();
        for _ in 0..self.cnst.batch_num {
            let start_idx = r.gen_range(0..=(self.corpus.len() - block_size));
            shortest_path_cost_avg += PathFinder::new(
                &self.layout_cfg,
                &self.corpus[start_idx..(start_idx + block_size)],
                &self.cnst,
                a,
            )
            .path_fitness();
        }
        let fitness = shortest_path_cost_avg / self.cnst.batch_num as u128;
        combine_cost(fitness, self.layout_cost(a), 1000)
    }

    fn distance(&self, _: &Cfg, a: &Layout, b: &Layout) -> f64 {
        let mut dist = 0.0;
        let layer_min = a.layers.len().min(b.layers.len());
        let layer_max = a.layers.len().max(b.layers.len());
        // Different # layers is a big difference.
        dist += ((layer_max - layer_min) * self.layout_cfg.num_physical()) as f64;
        for i in 0..layer_min {
            for j in 0..a.layers[i].keys.len() {
                if a.layers[i].keys[j] != b.layers[i].keys[j] {
                    dist += 1.0;
                }
            }
        }
        // TODO: this divide by 500.0
        dist / 500.0
    }
}
