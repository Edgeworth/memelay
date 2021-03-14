use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{Kc, KcSet, KcSetExt, KeyEv, PhysEv};
use derive_more::Display;
use smallvec::SmallVec;

#[derive(Debug, Clone, Display)]
#[display(fmt = "phys {} ks {}", phys, ks)]
pub struct QmkModel<'a> {
    l: &'a Layout,
    phys: CountMap<usize>, // Holds # of times physical key pressed.
    ks: KeyAutomata,
}

impl Eq for QmkModel<'_> {}

impl PartialEq for QmkModel<'_> {
    fn eq(&self, o: &Self) -> bool {
        self.l == o.l && self.phys == o.phys && self.ks == o.ks
    }
}

impl std::hash::Hash for QmkModel<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Small set of things to hash that should distinguish the majority of cases.
        self.phys.hash(state);
    }
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { l: layout, phys: CountMap::new(), ks: KeyAutomata::new() }
    }

    pub fn key_ev_edges(&self, kev: KeyEv) -> SmallVec<[SmallVec<[PhysEv; 8]>; 8]> {
        let mut edges = SmallVec::new();
        for (phys, &kcset) in self.l.keys.iter().enumerate() {
            // Only try pressing this key if it makes progress to |kev| without pressing other stuff.
            if kev.kcset.is_superset(kcset) && !kcset.is_empty() {
                let pev = PhysEv::new(phys, kev.press);
                let mut v = SmallVec::new();
                v.push(pev);
                edges.push(v);
            }
        }
        edges
    }
}

impl<'a> Model for QmkModel<'a> {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<SmallVec<[KeyEv; 4]>> {
        if !(0..=1).contains(&self.phys.adjust_count(pev.phys, pev.press)) {
            return None; // Don't allow pressing the same physical key multiple times.
        }

        let kev = KeyEv::new(self.l.keys[pev.phys as usize], pev.press);
        self.ks.event(kev, cnst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KcSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const NONE: KcSet = enum_set!();
    const SUPER: KcSet = enum_set!(Kc::Super);
    const CTRL: KcSet = enum_set!(Kc::Ctrl);
    const A: KcSet = enum_set!(Kc::A);
    const C: KcSet = enum_set!(Kc::C);

    lazy_static! {
        static ref CNST: Constants = Default::default();
        static ref LAYOUT: Layout = Layout::new(vec![SUPER, CTRL, C, A, NONE]);
    }

    #[test]
    fn regular_letter() {
        let mut m = QmkModel::new(&LAYOUT);
        assert_eq!(
            m.event(PhysEv::new(2, true), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
    }

    #[test]
    fn kev_edges() {
        let m = QmkModel::new(&LAYOUT);
        assert_eq!(
            m.key_ev_edges(KeyEv::new(C, true)),
            SmallVec::from_buf([SmallVec::from_buf([PhysEv::new(2, true)])])
        );
        assert_eq!(
            m.key_ev_edges(KeyEv::new(A, true)),
            SmallVec::from_buf([SmallVec::from_buf([
                PhysEv::new(3, true),
                PhysEv::new(3, false),
                PhysEv::new(2, true)
            ])])
        );
        // May return invalid edges however.
        assert_eq!(
            m.key_ev_edges(KeyEv::new(C, false)),
            SmallVec::from_buf([SmallVec::from_buf([PhysEv::new(2, false)])])
        );
    }
    #[test]
    fn kev_edges_does_not_use_none_keys() {
        let m = QmkModel::new(&LAYOUT);
        assert_eq!(
            m.key_ev_edges(KeyEv::new(C, true)),
            SmallVec::from_buf([SmallVec::from_buf([PhysEv::new(2, true)])])
        );
    }
}
