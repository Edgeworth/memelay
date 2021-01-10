use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::models::key_automata::KeyAutomata;
use crate::models::layout::Layout;
use crate::models::Model;
use crate::types::{KCSet, KeyEv, PhysEv, KC};
use derive_more::Display;

// TODO: model multiple active layers.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "layer: {}, phys: {}, key state: {}", layer, phys, ks)]
pub struct QmkModel<'a> {
    pub layout: &'a Layout, // TODO: undo layout
    phys: CountMap<u32>,
    layer: usize, // Current active layer.
    ks: KeyAutomata,
    idle_count: usize,
}

impl<'a> QmkModel<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout, phys: CountMap::new(), layer: 0, ks: KeyAutomata::new(), idle_count: 0 }
    }

    fn get_key(&self, phys: u32) -> KCSet {
        self.layout.layers[self.layer].keys[phys as usize]
    }
}

impl<'a> Model for QmkModel<'a> {
    fn valid(&mut self, pev: PhysEv, cnst: &Constants) -> bool {
        // Limit number pressed to 4.
        if self.phys.num_pressed() > cnst.max_phys_pressed {
            return false;
        }
        // Limit idle to 4.
        if self.idle_count > cnst.max_phys_idle {
            return false;
        }
        let key = self.get_key(pev.phys);
        // Don't press keys that don't do anything.
        if key.is_empty() {
            return false;
        }
        let peek = self.phys.peek_adjust(pev.phys, pev.press);
        let kev = KeyEv::new(key, pev.press);
        // Don't allow pressing the same physical key multiple times.
        self.ks.valid(kev, cnst) && (peek == 0 || peek == 1)
    }

    fn event(&mut self, pev: PhysEv, _cnst: &Constants) -> Vec<CountMap<KC>> {
        self.phys.adjust_count(pev.phys, pev.press);
        let kev = self.ks.key_event(KeyEv::new(self.get_key(pev.phys), pev.press));
        if kev.is_empty() {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }
        kev
    }
}
