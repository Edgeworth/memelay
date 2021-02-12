use crate::constants::Constants;
use crate::models::count_map::CountMap;
use crate::types::{KeyEv, Kc};
use derive_more::Display;
use enumset::enum_set;
use smallvec::SmallVec;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "count: {}", kcm)]
pub struct KeyAutomata {
    // TODO: Is this still needed?
    kcm: CountMap<Kc>,
}

impl Default for KeyAutomata {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyAutomata {
    pub fn new() -> Self {
        Self { kcm: CountMap::new() }
    }

    pub fn kc_counts(&self) -> &CountMap<Kc> {
        &self.kcm
    }

    pub fn event(&mut self, kev: KeyEv, cnst: &Constants) -> Option<SmallVec<[KeyEv; 4]>> {
        let mut evs = SmallVec::new();

        for kc in kev.kcset {
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
    use crate::types::KcSet;
    use lazy_static::lazy_static;

    const SUPER: KcSet = enum_set!(Kc::Super);
    const CTRL: KcSet = enum_set!(Kc::Ctrl);
    const C: KcSet = enum_set!(Kc::C);
    const CTRL_C: KcSet = enum_set!(Kc::C | Kc::Ctrl);
    lazy_static! {
        static ref CNST: Constants = Constants { max_mod_pressed: 5, ..Default::default() };
    }

    #[test]
    fn regular_then_mod_then_release() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::press(SUPER), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(SUPER)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(SUPER), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(SUPER)])
        );
    }

    #[test]
    fn regular_letter() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
    }

    #[test]
    fn multiple_taps_separate() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
    }

    #[test]
    fn multiple_taps_interleaved() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(CTRL), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(CTRL)])
        );
        assert_eq!(
            ks.event(KeyEv::press(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(CTRL), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(CTRL)])
        );
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(CTRL_C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(CTRL), KeyEv::press(C)])
        );
        assert_eq!(
            ks.event(KeyEv::release(CTRL_C), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(CTRL), KeyEv::release(C)])
        );
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeyAutomata::new();
        assert_eq!(
            ks.event(KeyEv::press(SUPER), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::press(SUPER)])
        );
        assert_eq!(
            ks.event(KeyEv::release(SUPER), &CNST).unwrap(),
            SmallVec::from_buf([KeyEv::release(SUPER)])
        );
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeyAutomata::new();
        ks.event(KeyEv::release(SUPER), &CNST).unwrap();
    }
}
