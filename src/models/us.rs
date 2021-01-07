use crate::models::keyset::KeySet;
use crate::models::layer::{Layer, Layout};
use crate::models::{Model, Transition};
use crate::prelude::*;
use crate::types::{Key, KeyEv, PhysEv};
use lazy_static::lazy_static;

lazy_static! {
    // static ref LAYER: Layer = Layer::new();
    static ref LAYOUT: Layout = Layout::new();
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct USModel {
    layout: &'static Layout,
    ks: KeySet,
}

impl USModel {
    pub fn new() -> Self {
        Self { layout: &LAYOUT, ks: KeySet::new() }
    }

    fn get_key(&self, phys: u32) -> Key {
        self.layout.layers[0].keys[phys as usize]
    }
}

impl Model for USModel {
    type M = USModel;

    fn valid(&mut self, pev: PhysEv) -> bool {
        let kev = KeyEv::new(self.get_key(pev.phys), pev.count);
        self.ks.valid(kev)
    }

    fn event(&mut self, pev: PhysEv) -> Transition<USModel> {
        let mut ns = self.clone();
        let kev = KeyEv::new(self.get_key(pev.phys), pev.count);
        let events = ns.ks.key_event(kev);
        Transition::new(ns, &events)
    }
}
