use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{KeyEv, PhysEv, KC};

pub mod count_map;
pub mod key_automata;
pub mod layout;
pub mod qmk;
pub mod us;

pub trait Model {
    // Takes a phys event and returns a vector of key events.
    // Model will not be valid to use if it returns None.
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<Vec<KeyEv>>;
    // Return the countmap of keys currently pressed.
    fn kc_counts(&self) -> &CountMap<KC>;
}
