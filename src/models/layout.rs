use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{rand_kcset, KcSet, KcSetExt};
use derive_more::Display;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "{:?}", keys)]
pub struct Layout {
    pub keys: Vec<KcSet>,
}

impl Layout {
    pub fn new(keys: Vec<KcSet>) -> Self {
        Self { keys }
    }

    pub fn rand_with_size(len: usize, cnst: &Constants) -> Self {
        Self { keys: (0..len).map(|_| rand_kcset(cnst)).collect() }
    }

    pub fn num_physical(&self) -> usize {
        self.keys.len()
    }

    // Gets rid of useless layout features.
    pub fn normalise(&mut self, cnst: &Constants) {
        // Remove same keys and excess mod keys.
        let mut cm: CountMap<KcSet> = CountMap::new();
        let mut mod_count = 0;
        for kcset in self.keys.iter_mut() {
            let mods = kcset.mods();
            if !mods.is_empty() {
                mod_count += 1;
                if mod_count > cnst.max_phys_mod {
                    kcset.remove_all(mods);
                }
            }
            if cm.adjust_count(*kcset, true) > cnst.max_phys_dup as i32 {
                kcset.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Kc;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const CTRL_C: KcSet = enum_set!(Kc::C | Kc::Ctrl);
    const C: KcSet = enum_set!(Kc::C);
    lazy_static! {
        static ref CNST: Constants = Constants { max_phys_dup: 1, ..Default::default() };
    }

    #[test]
    fn normalise_mod() {
        let mut l = Layout::new(vec![CTRL_C]);
        l.normalise(&CNST);
        assert_eq!(l, Layout::new(vec![C]));
    }

    #[test]
    fn normalise_same() {
        let mut l = Layout::new(vec![C, C]);
        l.normalise(&CNST);
        assert_eq!(l, Layout::new(vec![C, enum_set!()]));
    }

    #[test]
    fn normalise_mod_same() {
        let mut l = Layout::new(vec![CTRL_C, C]);
        l.normalise(&CNST);
        assert_eq!(l, Layout::new(vec![C, enum_set!()]));
    }
}
