use crate::constants::Constants;
use crate::ga::util::{combine_cost, combine_fitness};
use crate::layout_eval::LayoutCfg;
use crate::models::count_map::CountMap;
use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::us::USModel;
use crate::models::Model;
use crate::types::{KCSetExt, PhysEv, KC};
use derive_more::Display;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::usize;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
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
    events: Vec<CountMap<KC>>,
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
        let mut events = Vec::new();
        let mut us = USModel::new();
        for &pev in corpus.iter() {
            // If we get a stray release which causes US model to fail, ignore and skip it.
            events.extend(us.event(pev, cnst).unwrap_or_default());
        }
        Self { layout_cfg, events, cnst, l }
    }

    fn phys_cost(&self, pev: PhysEv) -> f64 {
        self.layout_cfg.cost[pev.phys as usize]
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn try_event<'b>(&self, mut n: Node<'b>, pev: PhysEv) -> Option<Node<'b>> {
        let mut events_qmk = n.qmk.event(pev, &self.cnst)?;
        for ev in events_qmk.into_iter() {
            if ev != self.events[n.idx] {
                return None;
            }
            n.idx += 1;
        }
        Some(n)
    }

    // fn compute_cumulative(&self) -> Vec<usize> {
    //     let mut reg_presses = Vec::new();
    //     let us = USModel::new();
    //     for prev in self.events.iter() {
    //         // Assumes that a physical key will always map to the same key event.
    //         let reg_count = us.get_key(prev.phys).reg().len();
    //         reg_presses.push(reg_count);
    //     }
    //     let c = reg_presses
    //         .iter()
    //         .rev()
    //         .scan(0, |c, cnt| {
    //             *c += cnt;
    //             Some(*c)
    //         })
    //         .collect::<Vec<_>>();
    //     let mut c: Vec<usize> = c.iter().rev().copied().collect();
    //     c.push(0);
    //     c
    // }

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
            if n.idx >= self.events.len() {
                break;
            }
            // Try pressing and releasing physical keys.
            for &press in &[true, false] {
                for i in 0..self.l.num_physical() {
                    let next = n.clone();
                    let pev = PhysEv::new(i, press);
                    if let Some(next) = self.try_event(next, pev) {
                        // Assume that every press of a mod key will generate an event upon release.
                        // TODO: this is not valid in case US model does e.g. ctrl, shift, alt and
                        // wants to build up mods?

                        // let qmk_mod_count = next.qmk.kc_counts().mods();
                        // let us_mod_count = next.us.kc_counts().mods();
                        // if qmk_mod_count != us_mod_count && qmk_mod_count.is_superset(&us_mod_count)
                        // {
                        //     continue;
                        // }
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
        // println!("asdf {} {}, {}", best.0 as u128, self.events.len() as u128, cnt);

        let fitness = combine_fitness(0, best.0 as u128, self.events.len() as u128);
        let fitness =
            combine_cost(fitness, best.1.into_inner() as u128, self.events.len() as u128 * 1000);
        fitness
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn path_fitness() {}
}
