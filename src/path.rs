use crate::constants::Constants;
use crate::ga::util::{combine_cost, combine_fitness};
use crate::layout_eval::LayoutCfg;
use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::Model;
use crate::types::{KeyEv, PhysEv};
use derive_more::Display;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::usize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Display)]
#[display(fmt = "Node, corpus idx {}:\n  qmk: {}", idx, qmk)]
struct Node<'a> {
    pub qmk: QmkModel<'a>, // Currently have this keyboard state.
    pub idx: usize, // Processed this much of the corpus (transformed to countmaps of keycodes).
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { qmk: QmkModel::new(layout), idx: 0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathFinder<'a> {
    layout_cfg: &'a LayoutCfg,
    kevs: &'a [KeyEv],
    cnst: &'a Constants,
    l: &'a Layout,
}

impl<'a> PathFinder<'a> {
    pub fn new(
        layout_cfg: &'a LayoutCfg,
        kevs: &'a [KeyEv],
        cnst: &'a Constants,
        l: &'a Layout,
    ) -> Self {
        Self { layout_cfg, kevs, cnst, l }
    }

    fn phys_cost(&self, pev: &[PhysEv]) -> f64 {
        pev.iter().fold(0.0, |c, pev| c + self.layout_cfg.cost[pev.phys as usize])
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn try_pevs<'b>(&self, mut n: Node<'b>, pevs: &[PhysEv]) -> Option<Node<'b>> {
        let mut kevs_qmk = Vec::new();
        for &pev in pevs.iter() {
            kevs_qmk.extend(n.qmk.event(pev, &self.cnst)?);
        }
        for ev in kevs_qmk.into_iter() {
            if n.idx < self.kevs.len() && ev != self.kevs[n.idx] {
                return None;
            }
            n.idx += 1;
        }

        Some(n)
    }

    pub fn path_fitness(&self) -> u128 {
        let mut q: PriorityQueue<Node<'_>, OrderedFloat<f64>> = PriorityQueue::new();
        let mut seen: HashSet<Node<'_>> = HashSet::new();
        let mut best = (0, OrderedFloat(0.0));

        let st = Node::new(self.l);
        q.push(st.clone(), OrderedFloat(0.0));
        let mut cnt = 0;
        while let Some((n, d)) = q.pop() {
            let d = -d;
            seen.insert(n.clone());
            cnt += 1;

            // println!("cost: {}, dijk: {}, seen: {}", -d, n, seen.len());
            // Look for getting furthest through corpus, then for lowest cost.
            if n.idx > best.0 || (n.idx == best.0 && d < best.1) {
                best = (n.idx, d)
            }
            if n.idx >= self.kevs.len() {
                break;
            }
            // Try pressing and releasing physical keys.
            for pevs in n.qmk.key_ev_edges(self.kevs[n.idx]).into_iter() {
                let next = n.clone();
                if let Some(next) = self.try_pevs(next, &pevs) {
                    if seen.contains(&next) {
                        continue;
                    }
                    let cost = d + OrderedFloat(self.phys_cost(&pevs));
                    q.push_increase(next, -cost);
                }
            }
        }
        // Typing all corpus is top priority, then cost to do so.
        // println!("asdf {} {}, {}", best.0 as u128, self.events.len() as u128, cnt);
        // println!("evs: {:?}", self.events);

        let fitness = combine_fitness(0, best.0 as u128, self.kevs.len() as u128);
        combine_cost(fitness, best.1.into_inner() as u128, self.kevs.len() as u128 * 1000)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn path_fitness() {}
}
