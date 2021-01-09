use crate::prelude::*;
use crate::types::{KCSet, KCSetExt, KeyEv, KC};
use enumset::enum_set;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct KeyAutomata {
    kc: Vec<(KC, i32)>,
    pending_update: bool,
}

fn press_to_count(press: bool) -> i32 {
    if press {
        1
    } else {
        -1
    }
}

impl KeyAutomata {
    pub fn new() -> Self {
        Self { kc: Vec::new(), pending_update: false }
    }

    pub fn valid(&mut self, kev: KeyEv) -> bool {
        let count = press_to_count(kev.press);
        for kc in kev.key.iter() {
            if self.get_count(kc) + count < 0 {
                return false;
            }
        }
        true
    }

    fn get_count(&self, kc: KC) -> i32 {
        self.kc.iter().find(|x| x.0 == kc).map(|x| x.1).unwrap_or(0)
    }

    fn adjust_count(&mut self, kc: KC, press: bool) -> i32 {
        let count = press_to_count(press);
        if let Some(kv) = self.kc.iter_mut().find(|x| x.0 == kc) {
            kv.1 += count;
            kv.1
        } else {
            self.kc.push((kc, count));
            count
        }
    }

    fn kcset(&self) -> KCSet {
        let mut kcset = enum_set!();
        self.kc.iter().filter(|x| x.1 > 0).for_each(|x| kcset |= x.0);
        kcset
    }

    pub fn key_event(&mut self, kev: KeyEv) -> Vec<KCSet> {
        // Rules for events:
        //   1. Letter keys are always immediate
        //   2. Multiple key presses of the same keycode don't generate new state.
        //   3. Sending mod information is delayed for as long as possible.
        let mut evs = Vec::new();

        let prev = self.kcset();
        for kc in kev.key {
            if self.adjust_count(kc, kev.press) < 0 {
                panic!(eyre!("keycode released too many times"));
            }
        }

        let kcset = self.kcset();
        // Send a state update unless it was only extra modifier keys pressed,
        // in which case we can wait.
        let mods_released = !kcset.mods().is_superset(prev.mods());
        if mods_released && self.pending_update {
            evs.push(prev);
        }
        self.pending_update = false;

        if mods_released || kcset.regular() != prev.regular() {
            evs.push(kcset);
        } else if kcset != prev {
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

    #[test]
    fn regular_letter() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.key_event(KeyEv::press(C)), [C]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [NONE]);
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeyAutomata::new();
        assert!(ks.key_event(KeyEv::press(CTRL)).is_empty());
        assert_eq!(ks.key_event(KeyEv::press(C)), [CTRL_C]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [CTRL]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL)), [NONE]);
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.key_event(KeyEv::press(CTRL_C)), [CTRL_C]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL_C)), [NONE]);
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeyAutomata::new();
        assert!(ks.key_event(KeyEv::press(SUPER)).is_empty());
        assert_eq!(ks.key_event(KeyEv::release(SUPER)), [SUPER, NONE]);
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeyAutomata::new();
        ks.key_event(KeyEv::release(SUPER));
    }
}
