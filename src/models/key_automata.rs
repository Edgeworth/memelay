use crate::models::count_map::CountMap;
use crate::prelude::*;
use crate::types::{KCSet, KCSetExt, KeyEv, KC};
use derive_more::Display;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "count: {}, pending: {}", kcm, pending_update)]
pub struct KeyAutomata {
    kcm: CountMap<KC>,
    pending_update: bool,
}

impl KeyAutomata {
    pub fn new() -> Self {
        Self { kcm: CountMap::new(), pending_update: false }
    }

    pub fn valid(&mut self, kev: KeyEv) -> bool {
        kev.key.iter().all(|x| self.kcm.peek_adjust(x, kev.press) >= 0)
    }

    pub fn key_event(&mut self, kev: KeyEv) -> Vec<CountMap<KC>> {
        // Rules for events:
        //   1. Letter keys are always immediate
        //   2. Multiple key presses of the same keycode don't generate new state.
        //   3. Sending mod information is delayed for as long as possible.
        let mut evs = Vec::new();

        let prev = self.kcm.clone();
        for kc in kev.key {
            if self.kcm.adjust_count(kc, kev.press) < 0 {
                panic!(eyre!("keycode released too many times"));
            }
        }

        // Send a state update unless it was only extra modifier keys pressed,
        // in which case we can wait.
        let mods_released = !self.kcm.mods().is_superset(&prev.mods());
        if mods_released && self.pending_update {
            evs.push(prev.clone());
        }
        self.pending_update = false;

        if mods_released || self.kcm.regular() != prev.regular() {
            evs.push(self.kcm.clone());
        } else if self.kcm != prev {
            self.pending_update = true;
        }

        evs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enumset::enum_set;

    const NONE: KCSet = enum_set!();
    const SUPER: KCSet = enum_set!(KC::Super);
    const CTRL: KCSet = enum_set!(KC::Ctrl);
    const C: KCSet = enum_set!(KC::C);
    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);

    fn kcm(kcset: KCSet) -> CountMap<KC> {
        let kcm = CountMap::new();
        merge_kcm(kcm, kcset)
    }

    fn merge_kcm(mut kcm: CountMap<KC>, kcset: KCSet) -> CountMap<KC> {
        for kc in kcset {
            kcm.adjust_count(kc, true);
        }
        kcm
    }

    #[test]
    fn regular_letter() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.key_event(KeyEv::press(C)), [kcm(C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [kcm(NONE)]);
    }

    #[test]
    fn multiple_taps_separate() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.key_event(KeyEv::press(C)), [kcm(C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [kcm(NONE)]);
        assert_eq!(ks.key_event(KeyEv::press(C)), [kcm(C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [kcm(NONE)]);
    }

    #[test]
    fn multiple_taps_interleaved() {
        let mut ks = KeyAutomata::new();
        let one = kcm(C);
        let two = merge_kcm(one.clone(), C);
        assert_eq!(ks.key_event(KeyEv::press(C)), [one.clone()]);
        assert_eq!(ks.key_event(KeyEv::press(C)), [two]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [one]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [kcm(NONE)]);
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeyAutomata::new();
        assert!(ks.key_event(KeyEv::press(CTRL)).is_empty());
        assert_eq!(ks.key_event(KeyEv::press(C)), [kcm(CTRL_C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [kcm(CTRL)]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL)), [kcm(NONE)]);
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.key_event(KeyEv::press(CTRL_C)), [kcm(CTRL_C)]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL_C)), [kcm(NONE)]);
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeyAutomata::new();
        assert!(ks.key_event(KeyEv::press(SUPER)).is_empty());
        assert_eq!(ks.key_event(KeyEv::release(SUPER)), [kcm(SUPER), kcm(NONE)]);
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeyAutomata::new();
        ks.key_event(KeyEv::release(SUPER));
    }
}
