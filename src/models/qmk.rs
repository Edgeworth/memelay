use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{Kc, KcSet, KcSetExt, KeyEv, PhysEv};
use derive_more::Display;
use smallvec::SmallVec;
use vec_map::VecMap;

type LayerAdjMap = VecMap<VecMap<SmallVec<[PhysEv; 8]>>>;

// TODO: model multiple active layers.
#[derive(Debug, Clone, Display)]
#[display(
    fmt = "layer {}, idle {}, phys {}, cached {:?}, ks {}",
    layer,
    idle_count,
    phys,
    cached_key,
    ks
)]
pub struct QmkModel<'a> {
    layout: &'a Layout,
    phys: CountMap<usize>, // Holds # of times physical key pressed.
    // Holds KCSet initially used when a physical key was pressed. Needed for layers.
    cached_key: VecMap<KcSet>,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
    idle_count: usize, // Number of physical keys we have pressed without producing any key codes
    layer_adj: LayerAdjMap, // How to get from layer a -> b.
}

impl Eq for QmkModel<'_> {}

impl PartialEq for QmkModel<'_> {
    fn eq(&self, o: &Self) -> bool {
        self.layout == o.layout
            && self.phys == o.phys
            && self.cached_key == o.cached_key
            && self.layer == o.layer
            && self.ks == o.ks
            && self.idle_count == o.idle_count
    }
}

impl std::hash::Hash for QmkModel<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Small set of things to hash that should distinguish the majority of cases.
        self.phys.hash(state);
        self.layer.hash(state);
        self.idle_count.hash(state);
    }
}

struct LayerDfs<'a> {
    layout: &'a Layout,
    path: SmallVec<[PhysEv; 8]>,
    layer_adj: VecMap<SmallVec<[PhysEv; 8]>>,
    seen: VecMap<()>,
}

impl<'a> LayerDfs<'a> {
    fn new(layout: &'a Layout) -> Self {
        Self { layout, path: SmallVec::new(), layer_adj: VecMap::new(), seen: VecMap::new() }
    }

    fn dfs(&mut self, layer: usize) {
        if self.seen.insert(layer, ()).is_some() {
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
                self.path.push(PhysEv::new(phys, false));
                self.dfs(l);
                self.path.pop();
                self.path.pop();
            }
        }
    }
}

impl<'a> QmkModel<'a> {
    fn compute_layer_adj(l: &Layout) -> LayerAdjMap {
        let mut layer_adj = VecMap::new();
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
            cached_key: VecMap::new(),
            layer: 0,
            ks: KeyAutomata::new(),
            idle_count: 0,
            layer_adj: Self::compute_layer_adj(layout),
        }
    }

    pub fn key_ev_edges(&self, kev: KeyEv) -> SmallVec<[SmallVec<[PhysEv; 8]>; 8]> {
        let mut edges = SmallVec::new();
        for (lid, layer) in self.layout.layers.iter().enumerate() {
            for (phys, &kcset) in layer.keys.iter().enumerate() {
                // Only try pressing this key if it makes progress to |kev| without pressing other stuff.
                if kev.kcset.is_superset(kcset) && !kcset.is_empty() {
                    let pev = PhysEv::new(phys, kev.press);
                    if lid == self.layer {
                        let mut v = SmallVec::new();
                        v.push(pev);
                        edges.push(v);
                    } else if let Some(adj) =
                        self.layer_adj.get(self.layer).and_then(|adj| adj.get(lid))
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

    fn get_kcset(&mut self, pev: PhysEv) -> (Option<usize>, KcSet) {
        let mut kcset = if pev.press {
            let kcset = self.layout.layers[self.layer].keys[pev.phys as usize];
            self.cached_key.insert(pev.phys, kcset);
            kcset
        } else {
            self.cached_key.remove(pev.phys).unwrap()
        };
        let mut layer = None;
        // Filter layer stuff here, since it is never sent, just handled by QMK.
        // TODO: layer limit here.
        if kcset.remove(Kc::Layer0) {
            layer = Some(0);
        }
        if kcset.remove(Kc::Layer1) && self.layout.layers.len() >= 2 {
            layer = Some(1);
        }
        (layer, kcset)
    }
}

impl<'a> Model for QmkModel<'a> {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<SmallVec<[KeyEv; 4]>> {
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

    fn kc_counts(&self) -> &CountMap<Kc> {
        self.ks.kc_counts()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::layout::Layer;
    use crate::types::KcSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const NONE: KcSet = enum_set!();
    const SUPER: KcSet = enum_set!(Kc::Super);
    const CTRL: KcSet = enum_set!(Kc::Ctrl);
    const A: KcSet = enum_set!(Kc::A);
    const C: KcSet = enum_set!(Kc::C);
    const LAYER0: KcSet = enum_set!(Kc::Layer0);
    const LAYER1: KcSet = enum_set!(Kc::Layer1);

    lazy_static! {
        static ref CNST: Constants = Constants {
            max_mod_pressed: 5,
            max_phys_pressed: 4,
            max_phys_idle: 5,
            ..Default::default()
        };
        static ref LAYOUT: Layout = Layout::new()
            .with_layer(Layer::new(&[SUPER, CTRL, C, LAYER1]))
            .with_layer(Layer::new(&[CTRL, NONE, A, LAYER0]));
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
    fn switch_layer() {
        let mut m = QmkModel::new(&LAYOUT);
        assert_eq!(m.event(PhysEv::new(3, true), &CNST).unwrap(), SmallVec::from_buf([]));
        assert_eq!(
            m.event(PhysEv::new(2, true), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(A)])
        );
        assert_eq!(m.event(PhysEv::new(3, false), &CNST).unwrap(), SmallVec::from_buf([]));
        assert_eq!(
            m.event(PhysEv::new(2, false), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(A)])
        );
        assert_eq!(m.event(PhysEv::new(3, true), &CNST).unwrap(), SmallVec::from_buf([]));
        assert_eq!(m.event(PhysEv::new(3, false), &CNST).unwrap(), SmallVec::from_buf([]));
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
        let layout = Layout::new()
            .with_layer(Layer::new(&[SUPER, CTRL, C, LAYER1, NONE]))
            .with_layer(Layer::new(&[CTRL, NONE, A, LAYER0]));
        let m = QmkModel::new(&layout);
        assert_eq!(
            m.key_ev_edges(KeyEv::new(C, true)),
            SmallVec::from_buf([SmallVec::from_buf([PhysEv::new(2, true)])])
        );
    }
}
