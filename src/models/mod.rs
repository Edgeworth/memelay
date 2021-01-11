use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{PhysEv, KC};

pub mod count_map;
pub mod key_automata;
pub mod layout;
pub mod qmk;
#[cfg(test)]
mod tests;
pub mod us;

pub trait Model {
    // Takes a phys event and returns a vector of counts of each keycode. Multiple countmaps may be
    // returned if releasing a key confirms some pending events that were held back. If it returns
    // none, it means the physical event was not valid. The model will not be valid to use after it
    // returns None here.
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<Vec<CountMap<KC>>>;
    // Return the countmap of keys currently pressed.
    fn kc_counts(&self) -> &CountMap<KC>;
}
