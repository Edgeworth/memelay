use crate::models::layer::Layout;
use crate::models::qmk::QmkModel;
use crate::models::us::USModel;
use crate::models::Model;
use crate::types::PhysEv;
use crate::Env;
use derive_more::Display;
use ordered_float::OrderedFloat;
use radiate::Problem;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "Node, corpus idx {}:\n  qmk: {}\n  us: {}", corpus_idx, qmk, us)]
pub struct Node<'a> {
    pub qmk: QmkModel<'a>, // Currently have this keyboard state.
    pub us: USModel,
    pub corpus_idx: usize, // Processed this much of the corpus.
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { qmk: QmkModel::new(layout), us: USModel::new(), corpus_idx: 0 }
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

        let mut events_qmk = n.qmk.event(pev);
        while !events_qmk.is_empty() && n.corpus_idx < corpus.len() {
            let mut events_us = n.us.event(corpus[n.corpus_idx]);
            while !events_us.is_empty() && !events_qmk.is_empty() {
                if events_us[0] != events_qmk[0] {
                    return None;
                }
                events_us.remove(0);
                events_qmk.remove(0);
            }
            n.corpus_idx += 1;
        }
        if events_qmk.is_empty() {
            Some(n)
        } else {
            None
        }
    }
}

impl Problem<Layout> for Fitness {
    fn empty() -> Self {
        Self::new(Env::default())
    }

    fn solve(&self, l: &mut Layout) -> f32 {
        let mut q: BTreeSet<(OrderedFloat<f64>, Node<'_>)> = BTreeSet::new();
        let mut dist: HashMap<Node<'_>, OrderedFloat<f64>> = HashMap::new();
        let mut seen: HashSet<Node<'_>> = HashSet::new();
        let mut best = (0, OrderedFloat(0.0));
        let st = Node::new(l);
        q.insert((OrderedFloat(0.0), st.clone()));
        dist.insert(st, OrderedFloat(0.0));
        while let Some(v) = q.first().cloned() {
            q.remove(&v);
            let n = v.1;
            seen.insert(n.clone());

            // Look for getting furthest through corpus, then for lowest cost.
            if n.corpus_idx > best.0 || (n.corpus_idx == best.0 && dist[&n] < best.1) {
                best = (n.corpus_idx, dist[&n])
            }
            if n.corpus_idx == self.env.corpus.len() - 1 {
                break;
            }
            // Try pressing and releasing physical keys.
            for &press in &[true, false] {
                for i in 0..l.num_physical() {
                    let mut next = n.clone();
                    let pev = PhysEv::new(i as u32, press);
                    if !next.qmk.valid(pev) {
                        continue;
                    }

                    if let Some(next) = self.unify(next, pev) {
                        if seen.contains(&next) {
                            continue;
                        }

                        let cost = v.0 + 1.0.into(); // TODO: Cost function.
                        let d = dist.entry(next.clone()).or_insert(cost);
                        if cost <= *d {
                            *d = cost;
                            q.insert((cost, next));
                        }
                    }
                }
            }
        }
        let fitness = best.0 as f64 * 10000.0 - best.1.into_inner();
        fitness as f32
    }
}
