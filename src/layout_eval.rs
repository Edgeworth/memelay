use crate::constants::Constants;
use crate::ga::runner::Ctx;
use crate::ga::util::crossover_vec;
use crate::ga::Evaluator;
use crate::ingest::{load_corpus, load_layout_cfg};
use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::us::USModel;
use crate::models::Model;
use crate::prelude::*;
use crate::types::{rand_kcset, Finger, PhysEv};
use crate::Args;
use derive_more::Display;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
use std::collections::HashSet;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "Node, corpus idx {}:\n  qmk: {}\n  us: {}", corpus_idx, qmk, us)]
pub struct Node<'a> {
    pub qmk: QmkModel<'a>, // Currently have this keyboard state.
    pub us: USModel,
    pub start_idx: usize,
    pub corpus_idx: usize, // Processed this much of the corpus.
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout, corpus_idx: usize) -> Self {
        Self { qmk: QmkModel::new(layout), us: USModel::new(), start_idx: corpus_idx, corpus_idx }
    }
}

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
            s += &format!("Layer {}\n", i);
            let mut idx = 0;
            for c in self.layout.chars() {
                if c == 'X' {
                    let mut kstr = format!("{:?}", layer.keys[idx]);
                    kstr.retain(|c| !r"() ".contains(c));
                    let kstr = kstr.replace("EnumSet", "");
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutEval {
    pub layout_cfg: LayoutCfg,
    pub corpus: Vec<PhysEv>,
    pub cnst: Constants,
}

impl LayoutEval {
    pub fn from_args(args: Args) -> Result<Self> {
        let layout_cfg = load_layout_cfg(&args.cfg_path)?;
        let corpus = load_corpus(&args.corpus_path)?;
        Ok(Self { layout_cfg, corpus, cnst: args.cnst })
    }

    pub fn num_physical(&self) -> usize {
        self.layout_cfg.cost.len()
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn unify<'a>(&self, mut n: Node<'a>, pev: PhysEv) -> Option<Node<'a>> {
        let corpus = &self.corpus;

        let mut events_qmk = n.qmk.event(pev, &self.cnst);
        while !events_qmk.is_empty() && n.corpus_idx < corpus.len() {
            // If we get a stray release, ignore and skip it.
            if n.us.valid(corpus[n.corpus_idx], &self.cnst) {
                let mut events_us = n.us.event(corpus[n.corpus_idx], &self.cnst);
                while !events_us.is_empty() && !events_qmk.is_empty() {
                    if events_us[0] != events_qmk[0] {
                        return None;
                    }
                    events_us.remove(0);
                    events_qmk.remove(0);
                }
            }
            n.corpus_idx += 1;
        }
        if events_qmk.is_empty() {
            Some(n)
        } else {
            None
        }
    }

    fn phys_cost(&self, _: &Layout, pev: PhysEv) -> f64 {
        self.layout_cfg.cost[pev.phys as usize]
    }

    fn layout_cost(&self, l: &Layout) -> f64 {
        // Penalise more layers.
        let mut cost = l.layers.len() as f64;
        for layer in l.layers.iter() {
            for kcset in layer.keys.iter() {
                // Penalise more keys.
                cost += kcset.len() as f64;
            }
        }
        cost
    }

    fn path_fitness(&self, l: &Layout, start_idx: usize, block_size: usize) -> f64 {
        let mut q: PriorityQueue<OrderedFloat<f64>, Node<'_>> = PriorityQueue::new();
        let mut seen: HashSet<Node<'_>> = HashSet::new();
        let mut best = (0, OrderedFloat(0.0));
        let st = Node::new(l, start_idx);
        q.push(OrderedFloat(0.0), st.clone());
        while let Some((d, n)) = q.pop() {
            let d = -d;
            seen.insert(n.clone());

            // println!("cost: {}, dijk: {}, seen: {}", -d, n, seen.len());
            // Look for getting furthest through corpus, then for lowest cost.
            if n.corpus_idx > best.0 || (n.corpus_idx == best.0 && d < best.1) {
                best = (n.corpus_idx, d)
            }
            if n.corpus_idx - n.start_idx >= block_size {
                break;
            }
            // Try pressing and releasing physical keys.
            for &press in &[true, false] {
                for i in 0..l.num_physical() {
                    let mut next = n.clone();
                    let pev = PhysEv::new(i as u32, press);
                    if !next.qmk.valid(pev, &self.cnst) {
                        continue;
                    }

                    if let Some(next) = self.unify(next, pev) {
                        if seen.contains(&next) {
                            continue;
                        }

                        let cost = d + OrderedFloat(self.phys_cost(l, pev));
                        q.push_increase(-cost, next);
                    }
                }
            }
        }
        let mut fitness = (best.0 - st.start_idx) as f64; // Typing all corpus is top priority.
        fitness = fitness * 10000.0 - best.1.into_inner(); // Next is minimising cost.
        fitness
    }
}

impl Evaluator for LayoutEval {
    type T = Layout;
    type F = f64;

    fn reproduce(&mut self, ctx: &Ctx, a: &Layout, b: &Layout) -> Layout {
        let mut r = rand::thread_rng();
        let layer_idx = r.gen_range(0..a.layers.len());
        let key_idx = r.gen_range(0..a.layers[layer_idx].keys.len());
        let layer_idx2 = r.gen_range(0..a.layers.len());
        let key_idx2 = r.gen_range(0..a.layers[layer_idx2].keys.len());

        let mut l = if r.gen::<f64>() < ctx.xover_rate {
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
                    if let Some(key) = avail.choose(&mut r) {
                        *key = rand_kcset(&mut r, &self.cnst);
                    }
                }
                1 => {
                    // Mutate random empty key.
                    let empty = l.layers[layer_idx].keys.iter_mut().filter(|k| k.is_empty());
                    if let Some(key) = empty.choose(&mut r) {
                        *key = rand_kcset(&mut r, &self.cnst);
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

    fn fitness(&mut self, _ctx: &Ctx, a: &Layout) -> f64 {
        let block_size = self.cnst.batch_size.min(self.corpus.len());
        let mut shortest_path_cost_avg = 0.0;
        let mut r = rand::thread_rng();
        for _ in 0..self.cnst.batch_num {
            let start_idx = r.gen_range(0..=(self.corpus.len() - block_size));
            shortest_path_cost_avg += self.path_fitness(a, start_idx, block_size);
        }
        // println!("DONE: {} {}", best.0, st.start_idx);
        let mut fitness = shortest_path_cost_avg / self.cnst.batch_num as f64;
        fitness = fitness * 1000.0 - self.layout_cost(a);
        fitness
    }

    fn distance(&mut self, _ctx: &Ctx, a: &Layout, b: &Layout) -> f64 {
        let mut dist = 0.0;
        let layer_min = a.layers.len().min(b.layers.len());
        let layer_max = a.layers.len().max(b.layers.len());
        dist += ((layer_max - layer_min) * self.num_physical()) as f64; // Different # layers is a big difference.
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
