use crate::models::key_automata::KeyAutomata;
use crate::models::layer::Layout;
use crate::models::Model;
use crate::prelude::*;
use crate::types::{KCSet, KeyEv, PhysEv};
use crate::Env;
use radiate::Genome;
use std::sync::{Arc, RwLock};

impl Genome<Layout, Env> for Layout {
    fn crossover(
        parent_one: &Layout,
        parent_two: &Layout,
        env: Arc<RwLock<Env>>,
        crossover_rate: f32,
    ) -> Option<Layout> {
        Some(parent_one.clone())
    }

    fn distance(one: &Layout, two: &Layout, _: Arc<RwLock<Env>>) -> f32 {
        1.0
    }

    fn base(_: &mut Env) -> Layout {
        Layout::new()
    }
}

// TODO: model multiple active layers.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct QmkModel<'a> {
    layout: &'a Layout,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout, layer: 0, ks: KeyAutomata::new() }
    }

    fn get_key(&self, phys: u32) -> KCSet {
        self.layout.layers[self.layer].keys[phys as usize]
    }
}

impl<'a> Model for QmkModel<'a> {
    fn valid(&mut self, pev: PhysEv) -> bool {
        let kev = KeyEv::new(self.get_key(pev.phys), pev.press);
        self.ks.valid(kev)
    }

    fn event(&mut self, pev: PhysEv) -> Vec<KCSet> {
        self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.press))
    }
}
