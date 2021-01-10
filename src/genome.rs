use crate::env::Env;
use crate::models::layout::{Layer, Layout};
use crate::types::rand_kcset;
use radiate::Genome;
use rand::Rng;
use rand_distr::{Distribution, WeightedAliasIndex};
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
        env: Arc<RwLock<Env>>,
        crossover_rate: f32,
    ) -> Option<Layout> {
        let mut r = rand::thread_rng();
        let env = env.read().unwrap();
        let layer_idx = r.gen_range(0..p1.layers.len());
        let key_idx = r.gen_range(0..p1.layers[layer_idx].keys.len());
        let layer_idx2 = r.gen_range(0..p1.layers.len());
        let key_idx2 = r.gen_range(0..p1.layers[layer_idx2].keys.len());

        let mut l = if r.gen::<f32>() < crossover_rate {
            let crossover_idx =
                WeightedAliasIndex::new(env.cnst.crossover_strat_weights.clone()).unwrap();
            let mut l = Layout::new();

            match crossover_idx.sample(&mut r) {
                0 => {
                    // Crossover on layer level.
                    let crosspoint = r.gen_range(0..p1.layers.len());
                    l.layers = crossover_vec(&p1.layers, &p2.layers, crosspoint);
                }
                1 => {
                    // Crossover on keys level;
                    l.layers = p1.layers.clone();
                    l.layers[layer_idx].keys = crossover_vec(
                        &p1.layers[layer_idx].keys,
                        &p2.layers[layer_idx].keys,
                        key_idx,
                    );
                }
                _ => panic!("unknown crossover strategy"),
            }
            Some(l)
        } else {
            let crossover_idx =
                WeightedAliasIndex::new(env.cnst.mutate_strat_weights.clone()).unwrap();
            let mut l = p1.clone();

            match crossover_idx.sample(&mut r) {
                0 => {
                    // Mutate random key.
                    l.layers[layer_idx].keys[key_idx] = rand_kcset(&mut r, &env.cnst);
                }
                1 => {
                    // Swap random layer.
                    let swap_idx = r.gen_range(0..p1.layers.len());
                    l.layers.swap(layer_idx, swap_idx);
                }
                2 => {
                    // Swap random key
                    let tmp = l.layers[layer_idx].keys[key_idx];
                    l.layers[layer_idx].keys[key_idx] = l.layers[layer_idx2].keys[key_idx2];
                    l.layers[layer_idx2].keys[key_idx2] = tmp;
                }
                _ => panic!("unknown crossover strategy"),
            }
            Some(l)
        };
        if let Some(l) = &mut l {
            l.normalise(&env.cnst);
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
        let mut l = Layout::new().with_layer(Layer::rand_with_size(env.num_physical(), &env.cnst));
        l.normalise(&env.cnst);
        l
    }
}
