use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{KeyEv, KC};
use derive_more::Display;
use enumset::enum_set;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "count: {}", kcm)]
pub struct KeyAutomata {
    // TODO: Is this still needed?
    kcm: CountMap<KC>,
}

impl KeyAutomata {
    pub fn new() -> Self {
        Self { kcm: CountMap::new() }
    }

    pub fn kc_counts(&self) -> &CountMap<KC> {
        &self.kcm
    }

    pub fn event(&mut self, kev: KeyEv, cnst: &Constants) -> Option<Vec<KeyEv>> {
        let mut evs = Vec::new();

        for kc in kev.key {
            let count = self.kcm.adjust_count(kc, kev.press);
            if kc.is_mod() && count > cnst.max_mod_pressed as i32 {
                return None;
            }
            if count < 0 {
                return None;
            }
            evs.push(KeyEv::new(enum_set!(kc), kev.press));
        }

        Some(evs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KCSet;
    use enumset::enum_set;
    use lazy_static::lazy_static;

    const SUPER: KCSet = enum_set!(KC::Super);
    const CTRL: KCSet = enum_set!(KC::Ctrl);
    const C: KCSet = enum_set!(KC::C);
    const CTRL_C: KCSet = enum_set!(KC::C | KC::Ctrl);
    lazy_static! {
        static ref CNST: Constants = Constants { max_mod_pressed: 5, ..Default::default() };
    }

    #[test]
    fn regular_then_mod_then_release() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::press(SUPER), &CNST).unwrap(), [KeyEv::press(SUPER)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
        assert_eq!(ks.event(KeyEv::release(SUPER), &CNST).unwrap(), [KeyEv::release(SUPER)]);
    }

    #[test]
    fn regular_letter() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
    }

    #[test]
    fn multiple_taps_separate() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
    }

    #[test]
    fn multiple_taps_interleaved() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(CTRL), &CNST).unwrap(), [KeyEv::press(CTRL)]);
        assert_eq!(ks.event(KeyEv::press(C), &CNST).unwrap(), [KeyEv::press(C)]);
        assert_eq!(ks.event(KeyEv::release(C), &CNST).unwrap(), [KeyEv::release(C)]);
        assert_eq!(ks.event(KeyEv::release(CTRL), &CNST).unwrap(), [KeyEv::release(CTRL)]);
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(CTRL_C), &CNST).unwrap(),
            [KeyEv::press(CTRL), KeyEv::press(C)]
        );
        assert_eq!(
            ks.event(KeyEv::release(CTRL_C), &CNST).unwrap(),
            [KeyEv::release(CTRL), KeyEv::release(C)]
        );
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeyAutomata::new();
        assert_eq!(ks.event(KeyEv::press(SUPER), &CNST).unwrap(), [KeyEv::press(SUPER)]);
        assert_eq!(ks.event(KeyEv::release(SUPER), &CNST).unwrap(), [KeyEv::release(SUPER)]);
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeyAutomata::new();
        ks.event(KeyEv::release(SUPER), &CNST).unwrap();
    }
}
