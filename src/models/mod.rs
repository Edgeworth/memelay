use crate::types::{KeyEv, PhysEv};

pub mod keyset;
pub mod layer;
pub mod qmk;
pub mod us;

pub trait Model {
    type M: Model;

    fn valid(&mut self, pev: PhysEv) -> bool;
    fn event(&mut self, pev: PhysEv) -> Transition<Self::M>;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Transition<M: Model> {
    pub s: M,
    pub kev: Vec<KeyEv>,
}

impl<M: Model> Transition<M> {
    pub fn new(s: M, kev: &[KeyEv]) -> Self {
        Self { s, kev: kev.to_vec() }
    }
}
