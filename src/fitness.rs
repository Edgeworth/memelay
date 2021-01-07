use crate::models::layer::Layout;
use crate::models::qmk::QmkModel;
use crate::models::Model;
use crate::prelude::*;
use crate::types::{Finger, Key, KeyEv, PhysEv};
use ordered_float::OrderedFloat;
use radiate::Problem;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Node<'a> {
    pub s: QmkModel<'a>,   // Currently have this keyboard state.
    pub corpus_idx: usize, // Processed this much of the corpus.
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { s: QmkModel::new(layout), corpus_idx: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fitness {
    cost: Vec<f64>,
    fing: Vec<Finger>,
    corpus: Vec<KeyEv>,
}

impl Fitness {
    pub fn new(cost: Vec<f64>, fing: Vec<Finger>, corpus: Vec<KeyEv>) -> Self {
        Self { cost, fing, corpus }
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn unify(&self, n: Node<'_>, ev: &[KeyEv]) {
        // let mut v = Vec::new();
        // while v.len() != ev.len() && n.corpus_idx < self.corpus.len() {
        //     v.extend(n.s.event(self.corpus[n.corpus_idx]))
        // }
        // Some(n)
    }
}

impl Problem<Layout> for Fitness {
    fn empty() -> Self {
        Self::new(vec![], vec![], vec![])
    }

    fn solve(&self, l: &mut Layout) -> f32 {
        const MIN: f32 = -1000000000.0;
        let mut q: BTreeSet<(OrderedFloat<f64>, Node<'_>)> = BTreeSet::new();
        let mut dist: HashMap<Node<'_>, f64> = HashMap::new();
        let mut seen: HashSet<Node<'_>> = HashSet::new();
        q.insert((0.0.into(), Node::new(l)));
        while let Some(v) = q.first().cloned() {
            q.remove(&v);
            let mut n = v.1;
            seen.insert(n.clone());

            if n.corpus_idx == self.corpus.len() - 1 {
                return dist[&n] as f32;
            }
            // Try pressing keys.
            // TODO: Need to have 'unification' function between prefix of corpus and returned key
            // events.
            for &count in &[-1, 1] {
                for i in 0..l.num_physical() {
                    let pev = PhysEv::new(i as u32, count);
                    if !n.s.valid(pev) {
                        continue;
                    }
                    let t = n.s.event(pev);
                    // if let Some(next) = self.unify(n.clone(), t.ev) {}
                }
            }

            // Try releasing keys.
        }
        MIN
    }
}
