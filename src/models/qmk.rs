use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{KCSet, KCSetExt, KeyEv, PhysEv, KC};
use derive_more::Display;
use enumset::enum_set;
use std::collections::HashMap;
use vec_collections::{VecMap, VecSet};

type LayerAdjMap = VecMap<[(usize, HashMap<usize, Vec<PhysEv>>); 10]>;

// TODO: model multiple active layers.
#[derive(Debug, Clone, Eq, PartialEq, Display)]
#[display(fmt = "layer: {}, phys: {}, key state: {}", layer, phys, ks)]
pub struct QmkModel<'a> {
    layout: &'a Layout,
    phys: CountMap<usize>, // Holds # of times physical key pressed.
    // Holds KCSet initially used when a physical key was pressed. Needed for layers.
    cached_key: VecMap<[(usize, KCSet); 10]>,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
    idle_count: usize,
    layer_adj: LayerAdjMap, // How to get from layer a -> b.
}

impl std::hash::Hash for QmkModel<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.phys.hash(state);
        self.cached_key.hash(state);
        self.cached_key.hash(state);
        self.layer.hash(state);
        self.ks.hash(state);
        self.idle_count.hash(state);
    }
}

struct LayerDfs<'a> {
    layout: &'a Layout,
    path: Vec<PhysEv>,
    layer_adj: HashMap<usize, Vec<PhysEv>>,
    seen: VecSet<[usize; 10]>,
}

impl<'a> LayerDfs<'a> {
    fn new(layout: &'a Layout) -> Self {
        Self { layout, path: Vec::new(), layer_adj: HashMap::new(), seen: VecSet::empty() }
    }

    fn dfs(&mut self, layer: usize) {
        if !self.seen.insert(layer) {
            return;
        }
        // TODO: Use minimum cost path to get between layers - need to run dijkstra.
        self.layer_adj.insert(layer, self.path.clone()).unwrap_none();
        for (phys, kcset) in self.layout.layers[layer].keys.iter().enumerate() {
            let l = kcset.layers();
            if l.len() > 1 {
                panic!("key with multiple assigned layer transitions: {:?}", kcset);
            }
            for layer in l.iter() {
                let l = layer.layer_num().unwrap();
                // TODO: Layer transition types, momentary etc.
                self.path.push(PhysEv::new(phys, true));
                self.dfs(l);
                self.path.pop();
            }
        }
    }
}

impl<'a> QmkModel<'a> {
    fn compute_layer_adj(l: &Layout) -> LayerAdjMap {
        let mut layer_adj = VecMap::empty();
        for layer in 0..l.layers.len() {
            let mut dfs = LayerDfs::new(l);
            dfs.dfs(layer);
            layer_adj.insert(layer, dfs.layer_adj);
        }
        layer_adj
    }

    pub fn new(layout: &'a Layout) -> Self {
        Self {
            layout,
            phys: CountMap::new(),
            cached_key: VecMap::empty(),
            layer: 0,
            ks: KeyAutomata::new(),
            idle_count: 0,
            layer_adj: Self::compute_layer_adj(layout),
        }
    }

    pub fn key_ev_edges(&self, kev: KeyEv) -> Vec<Vec<PhysEv>> {
        let mut edges = Vec::new();
        for (lid, layer) in self.layout.layers.iter().enumerate() {
            for (phys, &kcset) in layer.keys.iter().enumerate() {
                // Only try pressing this key if it makes progress to |kev| without pressing other stuff.
                if kev.kcset.is_superset(kcset) {
                    let pev = PhysEv::new(phys, kev.press);
                    if lid == self.layer {
                        edges.push(vec![pev]);
                    } else if let Some(adj) =
                        self.layer_adj.get(&self.layer).and_then(|adj| adj.get(&phys))
                    {
                        let mut pevs = adj.clone();
                        pevs.push(pev);
                        edges.push(pevs);
                    }
                }
            }
        }
        edges
    }

    fn get_kcset(&mut self, pev: PhysEv) -> (Option<usize>, KCSet) {
        let mut kcset = if pev.press {
            let kcset = self.layout.layers[self.layer].keys[pev.phys as usize];
            self.cached_key.insert(pev.phys, kcset);
            kcset
        } else {
            // TODO: Implement remove in upstream crate.
            self.cached_key.insert(pev.phys, enum_set!()).unwrap()
        };
        let mut layer = None;
        // Filter layer stuff here, since it is never sent, just handled by QMK.
        // TODO: layer limit here.
        if kcset.remove(KC::Layer0) {
            layer = Some(0);
        }
        if kcset.remove(KC::Layer1) && self.layout.layers.len() >= 2 {
            layer = Some(1);
        }
        (layer, kcset)
    }
}

impl<'a> Model for QmkModel<'a> {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<Vec<KeyEv>> {
        if !(0..=1).contains(&self.phys.adjust_count(pev.phys, pev.press)) {
            return None; // Don't allow pressing the same physical key multiple times.
        }

        if self.phys.num_pressed() > cnst.max_phys_pressed {
            return None; // Limit number pressed to 4.
        }

        let (layer, kcset) = self.get_kcset(pev);
        if kcset.is_empty() && layer.is_none() {
            return None; // Don't press keys that don't do anything.
        }

        let kev = self.ks.event(KeyEv::new(kcset, pev.press), cnst)?;
        if kev.is_empty() {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }

        if let Some(layer) = layer {
            self.layer = layer;
            self.idle_count = 0; // Count switching layers as resetting idle.
        }

        if self.idle_count > cnst.max_phys_idle {
            return None; // Limit idle.
        }

        Some(kev)
    }

    fn kc_counts(&self) -> &CountMap<KC> {
        self.ks.kc_counts()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::layout::Layer;
    use crate::types::KCSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const SUPER: KCSet = enum_set!(KC::Super);
    const CTRL: KCSet = enum_set!(KC::Ctrl);
    const A: KCSet = enum_set!(KC::A);
    const C: KCSet = enum_set!(KC::C);
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
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), [KeyEv::press(C)]);
    }

    #[test]
    fn switch_layer() {
        let mut m = QmkModel::new(&LAYOUT);
        assert_eq!(m.event(PhysEv::new(3, true), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(1, true), &CNST).unwrap(), [KeyEv::press(A)]);
        assert_eq!(m.event(PhysEv::new(3, false), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(1, false), &CNST).unwrap(), [KeyEv::release(A)]);
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(2, false), &CNST).unwrap(), []);
        assert_eq!(m.event(PhysEv::new(2, true), &CNST).unwrap(), [KeyEv::press(C)]);
    }

    #[test]
    fn kev_edges() {
        let m = QmkModel::new(&LAYOUT);
        assert_eq!(m.key_ev_edges(KeyEv::new(C, true)), [[PhysEv::new(2, true)]]);
        assert_eq!(
            m.key_ev_edges(KeyEv::new(A, true)),
            [[PhysEv::new(3, true), PhysEv::new(1, true)]]
        );
        // May return invalid edges however.
        assert_eq!(m.key_ev_edges(KeyEv::new(C, false)), [[PhysEv::new(2, false)]]);
    }
}
