use crate::env::Constants;
use crate::models::count_map::CountMap;
use crate::types::{PhysEv, KC};

mod count_map;
pub mod key_automata;
pub mod layout;
pub mod qmk;
pub mod us;

pub trait Model {
    fn valid(&mut self, pev: PhysEv, cnst: &Constants) -> bool;
    fn event(&mut self, pev: PhysEv, cnst: &Constants) -> Vec<CountMap<KC>>;
}
