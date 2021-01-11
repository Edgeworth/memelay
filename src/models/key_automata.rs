use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{KeyEv, KC};
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

    pub fn kc_counts(&self) -> &CountMap<KC> {
        &self.kcm
    }

    pub fn event(&mut self, kev: KeyEv, cnst: &Constants) -> Option<Vec<CountMap<KC>>> {
        // Rules for events:
        //   1. Letter keys are always immediate
        //   2. Multiple key presses of the same keycode don't generate new state.
        //   3. Sending mod information is delayed for as long as possible.
        let mut evs = Vec::new();

        let prev = self.kcm.clone();
        let mut mods_released = false;
        let mut reg_changed = false;
        for kc in kev.key {
            let count = self.kcm.adjust_count(kc, kev.press);
            if kc.is_mod() && count > cnst.max_mod_pressed as i32 {
                return None;
            }
            if count < 0 {
                return None;
            }
            if kc.is_mod() && !kev.press {
                mods_released = true;
            }
            if !kc.is_mod() {
                reg_changed = true;
            }
        }

        // Send a state update unless it was only extra modifier keys pressed,
        // in which case we can wait.
        if mods_released && self.pending_update {
            evs.push(prev.clone());
        }
        self.pending_update = false;

        if mods_released || reg_changed {
            evs.push(self.kcm.clone());
        } else if self.kcm != prev {
            self.pending_update = true;
        }

        Some(evs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tests::{kcm, merge_kcm};
    use crate::types::KCSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const NONE: KCSet = enum_set!();
    const SUPER: KCSet = enum_set!(KC::Super);
    const CTRL: KCSet = enum_set!(KC::Ctrl);
    const C: KCSet = enum_set!(KC::C);
    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);
    lazy_static! {
        static ref CNST: Constants = Constants { max_mod_pressed: 5, ..Default::default() };
    }

    #[test]
    fn regular_letter() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [kcm(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [kcm(NONE)]);
    }

    #[test]
    fn multiple_taps_separate() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [kcm(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [kcm(NONE)]);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [kcm(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [kcm(NONE)]);
    }

    #[test]
    fn multiple_taps_interleaved() {
        let mut ks = KeyAutomata::new();
        let one = kcm(C);
        let two = merge_kcm(one.clone(), C);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [one.clone()]);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [two]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [one]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [kcm(NONE)]);
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(CTRL), &CNST).unwrap(), []);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [kcm(CTRL_C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [kcm(CTRL)]);
        assert_eq!(ks.event(KeyEv::release(CTRL), &CNST).unwrap(), [kcm(NONE)]);
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(CTRL_C), &CNST).unwrap(), [kcm(CTRL_C)]);
        assert_eq!(ks.event(KeyEv::release(CTRL_C), &CNST).unwrap(), [kcm(NONE)]);
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeyAutomata::new();
        assert!(ks.event(KeyEv::press(SUPER), &CNST).unwrap().is_empty());
        assert_eq!(ks.event(KeyEv::release(SUPER), &CNST).unwrap(), [kcm(SUPER), kcm(NONE)]);
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeyAutomata::new();
        ks.event(KeyEv::release(SUPER), &CNST).unwrap();
    }
}
