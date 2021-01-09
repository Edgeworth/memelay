use crate::types::{KCSet, PhysEv};

pub mod key_automata;
pub mod layer;
pub mod qmk;
pub mod us;

pub trait Model {
    fn valid(&mut self, pev: PhysEv) -> bool;
    fn event(&mut self, pev: PhysEv) -> Vec<KCSet>;
}
