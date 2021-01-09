use crate::constants::{MAX_MODS_PER_LAYER, MAX_SAME};
use crate::models::count_map::CountMap;
use crate::types::{rand_kcset, KCSet, KCSetExt};
use derive_more::Display;
use enumset::enum_set;
use rand::seq::IteratorRandom;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "{:?}", keys)]
pub struct Layer {
    pub keys: Vec<KCSet>,
}

impl Layer {
    pub fn new(keys: &[KCSet]) -> Self {
        Self { keys: keys.to_vec() }
    }

    pub fn rand_with_size(len: usize) -> Self {
        let mut r = rand::thread_rng();
        Self { keys: (0..len).map(|_| rand_kcset(&mut r)).collect() }
    }

    pub fn num_physical(&self) -> usize {
        self.keys.len()
    }

    pub fn normalise(&mut self, max_mods_per_layer: i32, max_same: i32) {
        let mut r = rand::thread_rng();
        let mods =
            self.keys.iter_mut().filter(|kcset| !kcset.mods().is_empty()).collect::<Vec<_>>();
        // Remove excess mod keys.
        let remove_count = mods.len() as i32 - max_mods_per_layer;
        if remove_count > 0 {
            let to_remove = mods.into_iter().choose_multiple(&mut r, remove_count as usize);
            for kcset in to_remove {
                kcset.remove_all(kcset.mods());
            }
        }

        // Remove same keys.
        let mut cm: CountMap<KCSet> = CountMap::new();
        let mut keys = Vec::new();
        for &kcset in self.keys.iter() {
            if cm.adjust_count(kcset, true) <= max_same {
                keys.push(kcset);
            } else {
                keys.push(enum_set!());
            }
        }
        self.keys = keys;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Layout {
    pub layers: Vec<Layer>,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.layers.iter().enumerate() {
            writeln!(f, "  Layer {}: {}", i, v)?;
        }
        Ok(())
    }
}

impl Layout {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn with_layer(mut self, l: Layer) -> Self {
        self.layers.push(l);
        self
    }

    pub fn num_physical(&self) -> usize {
        self.layers.get(0).map(|x| x.num_physical()).unwrap_or(0)
    }

    // Gets rid of useless layout features.
    pub fn normalise(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.normalise(MAX_MODS_PER_LAYER as i32, MAX_SAME as i32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KC;

    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);
    const C: KCSet = enum_set!(KC::C);

    #[test]
    fn normalise_mod() {
        let mut l = Layer::new(&[CTRL_C]);
        l.normalise(0, 1);
        assert_eq!(l, Layer::new(&[C]));
    }

    #[test]
    fn normalise_same() {
        let mut l = Layer::new(&[C, C]);
        l.normalise(0, 1);
        assert_eq!(l, Layer::new(&[C, enum_set!()]));
    }

    #[test]
    fn normalise_mod_same() {
        let mut l = Layer::new(&[CTRL_C, C]);
        l.normalise(0, 1);
        assert_eq!(l, Layer::new(&[C, enum_set!()]));
    }
}
