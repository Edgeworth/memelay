use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{KCSet, KeyEv, PhysEv, KC};
use derive_more::Display;
use vec_map::VecMap;

// TODO: model multiple active layers.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "layer: {}, phys: {}, key state: {}", layer, phys, ks)]
pub struct QmkModel<'a> {
    pub layout: &'a Layout, // TODO: undo layout
    phys: CountMap<usize>,
    // Holds KCSet initially used when a physical key was pressed. Needed for layers.
    cached_key: VecMap<KCSet>,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
    idle_count: usize,
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self {
            layout,
            phys: CountMap::new(),
            cached_key: VecMap::new(),
            layer: 0,
            ks: KeyAutomata::new(),
            idle_count: 0,
        }
    }

    fn get_key(&mut self, pev: PhysEv) -> (Option<usize>, KCSet) {
        let mut kcset = if pev.press {
            let kcset = self.layout.layers[self.layer].keys[pev.phys as usize];
            self.cached_key.insert(pev.phys, kcset);
            kcset
        } else {
            self.cached_key.remove(pev.phys).unwrap()
        };
        let mut layer = None;
        // Filter layer stuff here, since it is never sent, just handled by QMK.
        if kcset.remove(KC::Layer0) {
            layer = Some(0);
        }
        if kcset.remove(KC::Layer1) {
            layer = Some(1);
        }
        (layer, kcset)
    }
}

impl<'a> Model for QmkModel<'a> {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<Vec<CountMap<KC>>> {
        if !(0..=1).contains(&self.phys.adjust_count(pev.phys, pev.press)) {
            return None; // Don't allow pressing the same physical key multiple times.
        }

        if self.phys.num_pressed() > cnst.max_phys_pressed {
            return None; // Limit number pressed to 4.
        }
        if self.idle_count > cnst.max_phys_idle {
            return None; // Limit idle to 4.
        }

        let (layer, key) = self.get_key(pev);
        if key.is_empty() && layer.is_none() {
            return None; // Don't press keys that don't do anything.
        }
        self.layer = layer.unwrap_or(self.layer);

        let kev = self.ks.event(KeyEv::new(key, pev.press), cnst)?;
        if kev.is_empty() {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }
        Some(kev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::layout::Layer;
    use crate::models::tests::kcm;
    use crate::types::KCSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const NONE: KCSet = enum_set!();
    const SUPER: KCSet = enum_set!(KC::Super);
    const CTRL: KCSet = enum_set!(KC::Ctrl);
    const A: KCSet = enum_set!(KC::A);
    const C: KCSet = enum_set!(KC::C);
    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);
    const LAYER0: KCSet = enum_set!(KC::Layer0);
    const LAYER1: KCSet = enum_set!(KC::Layer1);
    lazy_static! {
        static ref CNST: Constants = Constants {
            max_mod_pressed: 5,
            max_phys_pressed: 4,
            max_phys_idle: 5,
            ..Default::default()
        };
        static ref LAYOUT: Layout = Layout::new()
            .with_layer(Layer::new(&[SUPER, CTRL, C, LAYER1]))
            .with_layer(Layer::new(&[CTRL, A, LAYER0]));
    }

    #[test]
    fn regular_letter() {
        let mut m = QmkModel::new(&LAYOUT);
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), [kcm(C)]);
    }

    #[test]
    fn switch_layer() {
        let mut m = QmkModel::new(&LAYOUT);
        assert_eq!(m.event(PhysEv::new(3, true), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(1, true), &CNST).unwrap(), [kcm(A)]);
        assert_eq!(m.event(PhysEv::new(3, false), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(1, false), &CNST).unwrap(), [kcm(NONE)]);
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(2, false), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), [kcm(C)]);
    }
}
