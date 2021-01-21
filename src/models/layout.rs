use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{rand_kcset, KCSet, KCSetExt};
use derive_more::Display;
use enumset::enum_set;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "{:?}", keys)]
pub struct Layer {
    pub keys: Vec<KCSet>,
}

impl Layer {
    pub fn new(keys: &[KCSet]) -> Self {
        Self { keys: keys.to_vec() }
    }

    pub fn rand_with_size(len: usize, cnst: &Constants) -> Self {
        let mut r = rand::thread_rng();
        Self { keys: (0..len).map(|_| rand_kcset(&mut r, cnst)).collect() }
    }

    pub fn num_physical(&self) -> usize {
        self.keys.len()
    }

    pub fn normalise(&mut self, cnst: &Constants) {
        // Remove same keys and excess mod keys.
        let mut cm: CountMap<KCSet> = CountMap::new();
        let mut keys = Vec::new();
        let mut mod_count = 0;
        for mut kcset in self.keys.iter().copied() {
            let mods = kcset.mods();
            if !mods.is_empty() {
                mod_count += 1;
                if mod_count > cnst.max_phys_mod_per_layer {
                    kcset.remove_all(mods);
                }
            }
            if cm.adjust_count(kcset, true) <= cnst.max_phys_duplicate_per_layer as i32 {
                keys.push(kcset);
            } else {
                keys.push(enum_set!());
            }
        }
        self.keys = keys;
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    pub fn from_layers(layers: &[Layer]) -> Self {
        Self { layers: layers.to_vec() }
    }

    pub fn rand_with_size(size: usize, num_layers: usize, cnst: &Constants) -> Self {
        let mut l = Layout::new();
        for _ in 0..num_layers {
            l = l.with_layer(Layer::rand_with_size(size, cnst));
        }
        l.normalise(&cnst);
        l
    }

    pub fn with_layer(mut self, l: Layer) -> Self {
        self.layers.push(l);
        self
    }

    pub fn num_physical(&self) -> usize {
        self.layers.get(0).map(|x| x.num_physical()).unwrap_or(0)
    }

    // Gets rid of useless layout features.
    pub fn normalise(&mut self, cnst: &Constants) {
        for layer in self.layers.iter_mut() {
            layer.normalise(cnst);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KC;
    use lazy_static::lazy_static;

    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);
    const C: KCSet = enum_set!(KC::C);
    lazy_static! {
        static ref CNST: Constants =
            Constants { max_phys_duplicate_per_layer: 1, ..Default::default() };
    }

    #[test]
    fn normalise_mod() {
        let mut l = Layer::new(&[CTRL_C]);
        l.normalise(&CNST);
        assert_eq!(l, Layer::new(&[C]));
    }

    #[test]
    fn normalise_same() {
        let mut l = Layer::new(&[C, C]);
        l.normalise(&CNST);
        assert_eq!(l, Layer::new(&[C, enum_set!()]));
    }

    #[test]
    fn normalise_mod_same() {
        let mut l = Layer::new(&[CTRL_C, C]);
        l.normalise(&CNST);
        assert_eq!(l, Layer::new(&[C, enum_set!()]));
    }
}
