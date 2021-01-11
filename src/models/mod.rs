use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{PhysEv, KC};

mod count_map;
pub mod key_automata;
pub mod layout;
pub mod qmk;
#[cfg(test)]
mod tests;
pub mod us;

pub trait Model {
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Option<Vec<CountMap<KC>>>;
}
