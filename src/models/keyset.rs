use crate::prelude::*;
use crate::types::{Key, KeyEv, Mod, KC};
use enumset::{enum_set, EnumSet};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct KeySet {
    pub kc: Vec<(KC, i32)>,
    pub md: Vec<(Mod, i32)>,
}

impl KeySet {
    pub fn new() -> Self {
        Self { kc: Vec::new(), md: Vec::new() }
    }

    pub fn mods(&self) -> EnumSet<Mod> {
        let mut mods = enum_set!();
        self.md.iter().filter(|x| x.1 > 0).for_each(|x| mods |= x.0);
        mods
    }

    pub fn valid(&mut self, kev: KeyEv) -> bool {
        for md in kev.key.md.iter() {
            if Self::adjust_count(&mut self.md, md, 0) + kev.count < 0 {
                return false;
            }
        }
        Self::adjust_count(&mut self.kc, kev.key.kc, 0) + kev.count >= 0
    }

    fn adjust_count<T: Eq>(v: &mut Vec<(T, i32)>, key: T, count: i32) -> i32 {
        if let Some(kv) = v.iter_mut().find(|x| x.0 == key) {
            kv.1 += count;
            kv.1
        } else {
            v.push((key, count));
            count
        }
    }

    pub fn key_event(&mut self, kev: KeyEv) -> Vec<KeyEv> {
        // Rules for events:
        //   1. Letter keys are always immediate
        // TODO: Adding mod keys etc may change these rules.
        let mut evs = Vec::new();
        if kev.count == 0 {
            return evs;
        }

        // First update mod status in case we generate a letter key event.
        for md in kev.key.md.iter() {
            let md_count = Self::adjust_count(&mut self.md, md, kev.count);
            if md_count == 0 {
            } else if md_count < 0 {
                panic!(eyre!("mod released too many times"));
            }
        }

        let kc_count = Self::adjust_count(&mut self.kc, kev.key.kc, kev.count);
        if kc_count == 0 {
            let produced_key = kev.key.with_mods(self.mods());
            evs.push(KeyEv::new(produced_key, 1));
            evs.push(KeyEv::new(produced_key, -1));
        } else if kc_count < 0 {
            panic!(eyre!("key released too many times"));
        }

        evs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enumset::enum_set;

    const SUPER: Key = Key::new(KC::None, enum_set!(Mod::Super));
    const CTRL: Key = Key::new(KC::None, enum_set!(Mod::Ctrl));
    const C: Key = Key::new(KC::C, enum_set!());
    const CTRL_C: Key = Key::new(KC::C, enum_set!(Mod::Ctrl));

    #[test]
    fn regular_letter() {
        let mut ks = KeySet::new();
        assert_eq!(ks.key_event(KeyEv::press(C)), [KeyEv::press(C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [KeyEv::release(C)]);
    }

    #[test]
    fn ctrl_c_split() {
        let mut ks = KeySet::new();
        assert!(ks.key_event(KeyEv::press(CTRL)).is_empty());
        assert_eq!(ks.key_event(KeyEv::press(C)), [KeyEv::press(CTRL_C)]);
        assert_eq!(ks.key_event(KeyEv::release(C)), [KeyEv::release(C)]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL)), [KeyEv::release(CTRL)]);
    }

    #[test]
    fn ctrl_c_coalesced() {
        let mut ks = KeySet::new();
        assert_eq!(ks.key_event(KeyEv::press(CTRL_C)), [KeyEv::press(CTRL_C)]);
        assert_eq!(ks.key_event(KeyEv::release(CTRL_C)), [KeyEv::release(CTRL_C)]);
    }

    // Super only is used for e.g opening search bar.
    #[test]
    fn super_only() {
        let mut ks = KeySet::new();
        assert!(ks.key_event(KeyEv::press(SUPER)).is_empty());
        assert_eq!(
            ks.key_event(KeyEv::release(SUPER)),
            [KeyEv::press(SUPER), KeyEv::release(SUPER)]
        );
    }

    #[test]
    #[should_panic]
    fn release_must_follow_press() {
        let mut ks = KeySet::new();
        ks.key_event(KeyEv::release(SUPER));
    }
}
