use crate::constants::{MAX_IDLE, MAX_PRESS};
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layer::{Layer, Layout};
use crate::models::Model;
use crate::types::{rand_kcset, KCSet, KeyEv, PhysEv, KC};
use crate::Env;
use derive_more::Display;
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
        const MUTATE_KEY_PROB: f32 = 0.95;
        let mut r = rand::thread_rng();
        let layer_idx = r.gen_range(0..p1.layers.len());
        let key_idx = r.gen_range(0..p1.layers[layer_idx].keys.len());

        let mut l = if r.gen::<f32>() < crossover_rate {
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
            if r.gen::<f32>() < MUTATE_KEY_PROB {
                // Mutate random key.
                l.layers[layer_idx].keys[key_idx] = rand_kcset(&mut r);
            } else {
                // Swap random layer.
                let swap_idx = r.gen_range(0..p1.layers.len());
                l.layers.swap(layer_idx, swap_idx);
            }
            Some(l)
        };
        if let Some(l) = &mut l {
            l.normalise();
        }
        l
    }

    fn distance(p1: &Layout, p2: &Layout, env: Arc<RwLock<Env>>) -> f32 {
        let env = env.read().unwrap();
        let mut dist = 0.0;
        let layer_min = p1.layers.len().min(p2.layers.len());
        let layer_max = p1.layers.len().max(p2.layers.len());
        dist += ((layer_max - layer_min) * env.num_physical()) as f64; // Different # layers is a big difference.
        for i in 0..layer_min {
            for j in 0..p1.layers[i].keys.len() {
                if p1.layers[i].keys[j] != p2.layers[i].keys[j] {
                    dist += 1.0;
                }
            }
        }
        // Radiate adjusts distance in +- 0.1 increments. Divide by 500 here so
        // it's approximately in that range.
        dist as f32 / 500.0
    }
    fn base(env: &mut Env) -> Layout {
        let mut l = Layout::new().with_layer(Layer::rand_with_size(env.num_physical()));
        l.normalise();
        l
    }
}

// TODO: model multiple active layers.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "layer: {}, phys: {}, key state: {}", layer, phys, ks)]
pub struct QmkModel<'a> {
    pub layout: &'a Layout, // TODO: undo layout
    phys: CountMap<u32>,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
    idle_count: usize,
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout, phys: CountMap::new(), layer: 0, ks: KeyAutomata::new(), idle_count: 0 }
    }

    fn get_key(&self, phys: u32) -> KCSet {
        self.layout.layers[self.layer].keys[phys as usize]
    }
}

impl<'a> Model for QmkModel<'a> {
    fn valid(&mut self, pev: PhysEv) -> bool {
        // Limit number pressed to 4.
        if self.phys.num_pressed() > MAX_PRESS {
            return false;
        }
        // Limit idle to 4.
        if self.idle_count > MAX_IDLE {
            return false;
        }
        let key = self.get_key(pev.phys);
        // Don't press keys that don't do anything.
        if key.is_empty() {
            return false;
        }
        let peek = self.phys.peek_adjust(pev.phys, pev.press);
        let kev = KeyEv::new(key, pev.press);
        // Don't allow pressing the same physical key multiple times.
        self.ks.valid(kev) && (peek == 0 || peek == 1)
    }

    fn event(&mut self, pev: PhysEv) -> Vec<CountMap<KC>> {
        self.phys.adjust_count(pev.phys, pev.press);
        let kev = self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.press));
        if kev.is_empty() {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }
        kev
    }
}
