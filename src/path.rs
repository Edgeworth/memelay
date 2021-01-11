use crate::constants::Constants;
use crate::ga::util::{combine_cost, combine_fitness};
use crate::layout_eval::LayoutCfg;
use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::us::USModel;
use crate::models::Model;
use crate::types::PhysEv;
use derive_more::Display;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::collections::HashSet;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "Node, corpus idx {}:\n  qmk: {}\n  us: {}", idx, qmk, us)]
struct Node<'a> {
    pub qmk: QmkModel<'a>, // Currently have this keyboard state.
    pub us: USModel,
    pub idx: usize, // Processed this much of the corpus.
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { qmk: QmkModel::new(layout), us: USModel::new(), idx: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathFinder<'a> {
    layout_cfg: &'a LayoutCfg,
    corpus: &'a [PhysEv],
    cnst: &'a Constants,
    l: &'a Layout,
}

impl<'a> PathFinder<'a> {
    pub fn new(
        layout_cfg: &'a LayoutCfg,
        corpus: &'a [PhysEv],
        cnst: &'a Constants,
        l: &'a Layout,
    ) -> Self {
        Self { layout_cfg, corpus, cnst, l }
    }

    fn phys_cost(&self, pev: PhysEv) -> f64 {
        self.layout_cfg.cost[pev.phys as usize]
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn try_event<'b>(&self, mut n: Node<'b>, pev: PhysEv) -> Option<Node<'b>> {
        let mut events_qmk = n.qmk.event(pev, &self.cnst)?;
        while !events_qmk.is_empty() && n.idx < self.corpus.len() {
            // If we get a stray release which causes US model to fail, ignore and skip it.
            if let Some(mut events_us) = n.us.event(self.corpus[n.idx], &self.cnst) {
                while !events_us.is_empty() && !events_qmk.is_empty() {
                    if events_us[0] != events_qmk[0] {
                        return None;
                    }
                    events_us.remove(0);
                    events_qmk.remove(0);
                }
            }
            n.idx += 1;
        }
        if events_qmk.is_empty() {
            Some(n)
        } else {
            None
        }
    }

    pub fn path_fitness(&self) -> u128 {
        let mut q: PriorityQueue<Node<'_>, OrderedFloat<f64>> = PriorityQueue::new();
        let mut seen: HashSet<Node<'_>> = HashSet::new();
        let mut best = (0, OrderedFloat(0.0));
        // let mut reg_cumsum = Vec::new();

        let st = Node::new(self.l);
        q.push(st.clone(), OrderedFloat(0.0));
        while let Some((n, d)) = q.pop() {
            let d = -d;
            seen.insert(n.clone());

            // println!("cost: {}, dijk: {}, seen: {}", -d, n, seen.len());
            // Look for getting furthest through corpus, then for lowest cost.
            if n.idx > best.0 || (n.idx == best.0 && d < best.1) {
                best = (n.idx, d)
            }
            if n.idx >= self.corpus.len() {
                break;
            }
            // Try pressing and releasing physical keys.
            for &press in &[true, false] {
                for i in 0..self.l.num_physical() {
                    let next = n.clone();
                    let pev = PhysEv::new(i, press);
                    if let Some(next) = self.try_event(next, pev) {
                        if seen.contains(&next) {
                            continue;
                        }
                        let cost = d + OrderedFloat(self.phys_cost(pev));
                        q.push_increase(next, -cost);
                    }
                }
            }
        }
        // Typing all corpus is top priority, then cost to do so.
        // println!(
        //     "asdf {} {} {}, stuck on: {:?}",
        //     best.0 as u128,
        //     st.start_idx,
        //     block_size as u128,
        //     st.us.get_key(self.corpus[best.0 as usize].phys)
        // );

        let fitness = combine_fitness(0, best.0 as u128, self.corpus.len() as u128);
        let fitness =
            combine_cost(fitness, best.1.into_inner() as u128, self.corpus.len() as u128 * 1000);
        fitness
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn path_fitness() {}
}
