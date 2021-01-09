use crate::models::key_automata::KeyAutomata;
use crate::models::layer::{Layer, Layout};
use crate::models::Model;
use crate::types::{rand_kcset, KCSet, KeyEv, PhysEv};
use crate::Env;
use radiate::Genome;
use rand::Rng;

use std::sync::{Arc, RwLock};

fn crossover_vec<T: Clone>(a: &[T], b: &[T], crosspoint: usize) -> Vec<T> {
    let mut v = Vec::new();
    v.extend(a[..crosspoint].iter().cloned());
    v.extend(b[crosspoint..].iter().cloned());
    v
}

impl Genome<Layout, Env> for Layout {
    fn crossover(
        p1: &Layout,
        p2: &Layout,
        _: Arc<RwLock<Env>>,
        crossover_rate: f32,
    ) -> Option<Layout> {
        let mut r = rand::thread_rng();
        let layer_idx = r.gen_range(0..p1.layers.len());
        let key_idx = r.gen_range(0..p1.layers[layer_idx].keys.len());

        if r.gen::<f32>() < crossover_rate {
            let mut l = Layout::new();
            if r.gen::<bool>() {
                // Crossover on layer level.
                let crosspoint = r.gen_range(0..p1.layers.len());
                l.layers = crossover_vec(&p1.layers, &p2.layers, crosspoint);
            } else {
                // Crossover on keys level;
                l.layers = p1.layers.clone();
                l.layers[layer_idx].keys =
                    crossover_vec(&p1.layers[layer_idx].keys, &p2.layers[layer_idx].keys, key_idx);
            }
            Some(l)
        } else {
            let mut l = p1.clone();
            if r.gen::<bool>() {
                // Mutate random key.
                l.layers[layer_idx].keys[key_idx] = rand_kcset(&mut r);
            } else {
                // Swap random layer.
                let swap_idx = r.gen_range(0..p1.layers.len());
                l.layers.swap(layer_idx, swap_idx);
            }
            Some(l)
        }
    }

    fn distance(_one: &Layout, _two: &Layout, _: Arc<RwLock<Env>>) -> f32 {
        1.0
    }

    fn base(env: &mut Env) -> Layout {
        Layout::new().with_layer(Layer::rand_with_size(env.cost.len()))
    }
}

// TODO: model multiple active layers.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct QmkModel<'a> {
    pub layout: &'a Layout, // TODO: undo layout
    layer: usize,           // Current active layer.
    ks: KeyAutomata,
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout, layer: 0, ks: KeyAutomata::new() }
    }

    fn get_key(&self, phys: u32) -> KCSet {
        self.layout.layers[self.layer].keys[phys as usize]
    }
}

impl<'a> Model for QmkModel<'a> {
    fn valid(&mut self, pev: PhysEv) -> bool {
        // TODO: break Vec() in key automata out into holdmap or something class. use herefor phys
        // keys too. in usmodel as well?
        let kev = KeyEv::new(self.get_key(pev.phys), pev.press);
        self.ks.valid(kev)
    }

    fn event(&mut self, pev: PhysEv) -> Vec<KCSet> {
        self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.press))
    }
}
