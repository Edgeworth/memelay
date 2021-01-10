use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::us::USModel;
use crate::models::Model;
use crate::types::PhysEv;
use crate::Env;
use derive_more::Display;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use radiate::Problem;
use rand::Rng;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Fitness {
    env: Env,
}

impl Fitness {
    pub fn new(env: Env) -> Self {
        Self { env }
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn unify<'a>(&self, mut n: Node<'a>, pev: PhysEv) -> Option<Node<'a>> {
        let corpus = &self.env.corpus;

        let mut events_qmk = n.qmk.event(pev, &self.env.cnst);
        while !events_qmk.is_empty() && n.corpus_idx < corpus.len() {
            // If we get a stray release, ignore and skip it.
            if n.us.valid(corpus[n.corpus_idx], &self.env.cnst) {
                let mut events_us = n.us.event(corpus[n.corpus_idx], &self.env.cnst);
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
        self.env.layout_cfg.cost[pev.phys as usize]
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
            if n.corpus_idx - n.start_idx >= block_size - 1 {
                break;
            }
            // Try pressing and releasing physical keys.
            for &press in &[true, false] {
                for i in 0..l.num_physical() {
                    let mut next = n.clone();
                    let pev = PhysEv::new(i as u32, press);
                    if !next.qmk.valid(pev, &self.env.cnst) {
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

impl Problem<Layout> for Fitness {
    fn empty() -> Self {
        Self::new(Env::default())
    }

    fn solve(&self, l: &mut Layout) -> f32 {
        let block_size = self.env.cnst.batch_size.min(self.env.corpus.len());
        let mut shortest_path_cost_avg = 0.0;
        let mut r = rand::thread_rng();
        for _ in 0..self.env.cnst.batch_num {
            let start_idx = r.gen_range(0..=(self.env.corpus.len() - block_size));
            shortest_path_cost_avg += self.path_fitness(l, start_idx, block_size);
        }
        // println!("DONE: {} {}", best.0, st.start_idx);
        let mut fitness = shortest_path_cost_avg / self.env.cnst.batch_num as f64;
        fitness = fitness * 1000.0 - self.layout_cost(l);
        fitness as f32
    }
}
