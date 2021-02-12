use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{KeyEv, PhysEv, Kc};
use smallvec::SmallVec;

pub mod count_map;
pub mod key_automata;
pub mod layout;
pub mod qmk;
pub mod us;

pub trait Model {
    // Takes a phys event and returns a vector of key events.
    // Model will not be valid to use if it returns None.
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<SmallVec<[KeyEv; 4]>>;
    // Return the countmap of keys currently pressed.
    fn kc_counts(&self) -> &CountMap<Kc>;
}

pub fn compute_kevs<T: Model>(mut model: T, pevs: &[PhysEv], cnst: &Constants) -> Vec<KeyEv> {
    let mut kevs = Vec::new();
    for &pev in pevs.iter() {
        // If we get a stray release which causes model to fail, ignore and skip it.
        kevs.extend(model.event(pev, cnst).unwrap_or_default());
    }
    kevs
}
