use crate::constants::Constants;
use crate::layout_eval::LayoutCfg;
use crate::models::layout::Layout;
use crate::models::qmk::QmkModel;
use crate::models::Model;
use crate::types::{KeyEv, PhysEv};
use derive_more::Display;
use priority_queue::PriorityQueue;
use smallvec::SmallVec;
use std::usize;

#[derive(Debug, Clone, Eq, Display)]
#[display(fmt = "Node(idx({}),  qmk({}))", idx, qmk)]
struct Node<'a> {
    pub qmk: QmkModel<'a>, // Currently have this keyboard state.
    pub idx: usize, // Processed this much of the corpus (transformed to countmaps of keycodes).
    pub cost: u64,
}

impl std::hash::Hash for Node<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.qmk.hash(state);
        self.idx.hash(state);
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, o: &Self) -> bool {
        self.idx == o.idx && self.qmk == o.qmk
    }
}

impl<'a> Node<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { qmk: QmkModel::new(layout), idx: 0, cost: 0 }
    }
}

pub struct PathResult {
    pub kevs_found: usize,
    pub cost: u64,
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

    fn phys_cost(&self, pev: &[PhysEv]) -> u64 {
        pev.iter().fold(0, |c, pev| c + self.layout_cfg.cost[pev.phys as usize])
    }

    // Check that |ev| could be produced from Node by consuming corpus operations.
    fn try_pevs<'b>(&self, mut n: Node<'b>, pevs: &[PhysEv]) -> Option<Node<'b>> {
        let mut kevs_qmk = SmallVec::<[KeyEv; 4]>::new();
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

    pub fn path(&self) -> PathResult {
        let mut q: PriorityQueue<Node<'_>, i64> = PriorityQueue::new();
        let mut best = (0, 0);

        let st = Node::new(self.l);
        q.push(st.clone(), 0);
        let mut cnt = 0;
        while let Some((n, pri)) = q.pop() {
            // We don't use a seen check - the extra hash and clone is expensive.
            // Cost can never decrease and we can't revisit the same state,
            // because we use only try sequences of physical events that progress
            // the key state.

            // Remove finished states to keep access quick.
            q.remove(&n);
            cnt += 1;

            // println!(
            //     "pri: {}, dijk: {}, get to: {}",
            //     pri,
            //     n,
            //     self.kevs
            //         .get(n.idx)
            //         .map(|kev| kev.to_string())
            //         .unwrap_or_else(|| "done".to_owned())
            // );
            // Look for getting furthest through corpus, then for lowest cost.
            if n.idx > best.0 || (n.idx == best.0 && n.cost < best.1) {
                best = (n.idx, n.cost)
            }
            if n.idx >= self.kevs.len() {
                break;
            }

            // Try pressing and releasing physical keys.
            for pevs in n.qmk.key_ev_edges(self.kevs[n.idx]).into_iter() {
                // println!("  try edges: {:?}", pevs);
                let next = n.clone();
                if let Some(mut next) = self.try_pevs(next, &pevs) {
                    next.cost += self.phys_cost(&pevs);
                    // Small heuristic - have to press at least # remaining key event number of
                    // physical key events to finish.
                    let pri = next.cost + self.kevs.len() as u64 - next.idx as u64;
                    q.push_increase(next, -(pri as i64));
                }
            }
        }
        // Typing all corpus is top priority, then cost to do so.
        // println!("asdf len {}, cost {}, tot ev {}, seen {}", best.0, best.1, self.kevs.len(), cnt);
        PathResult { kevs_found: best.0, cost: best.1 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_fitness() {}
}
