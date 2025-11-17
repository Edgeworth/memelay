use derive_more::Display;

use crate::types::Kc;

#[must_use]
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display("{:?}", keys)]
pub struct Layout {
    pub keys: Vec<Kc>,
}

impl Layout {
    pub fn new(keys: Vec<Kc>) -> Self {
        Self { keys }
    }

    pub fn size(&self) -> usize {
        self.keys.len()
    }
}

pub const QWERTY_KEYS: [Kc; 30] = [
    Kc::Q,
    Kc::W,
    Kc::E,
    Kc::R,
    Kc::T,
    Kc::Y,
    Kc::U,
    Kc::I,
    Kc::O,
    Kc::P,
    Kc::A,
    Kc::S,
    Kc::D,
    Kc::F,
    Kc::G,
    Kc::H,
    Kc::J,
    Kc::K,
    Kc::L,
    Kc::Semicolon,
    Kc::Z,
    Kc::X,
    Kc::C,
    Kc::V,
    Kc::B,
    Kc::N,
    Kc::M,
    Kc::Comma,
    Kc::Dot,
    Kc::Slash,
];

pub const COLEMAK_DHM_KEYS: [Kc; 30] = [
    Kc::Q,
    Kc::W,
    Kc::F,
    Kc::P,
    Kc::B,
    Kc::J,
    Kc::L,
    Kc::U,
    Kc::Y,
    Kc::Semicolon,
    Kc::A,
    Kc::R,
    Kc::S,
    Kc::T,
    Kc::G,
    Kc::M,
    Kc::N,
    Kc::E,
    Kc::I,
    Kc::O,
    Kc::Z,
    Kc::X,
    Kc::C,
    Kc::D,
    Kc::V,
    Kc::K,
    Kc::H,
    Kc::Comma,
    Kc::Dot,
    Kc::Slash,
];

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_layout_new() {
        let keys = vec![Kc::A, Kc::B, Kc::C];
        let layout = Layout::new(keys.clone());
        assert_eq!(layout.keys, keys);
    }

    #[test]
    fn test_layout_size() {
        let layout = Layout::new(vec![Kc::A, Kc::B, Kc::C]);
        assert_eq!(layout.size(), 3);
    }

    #[test]
    fn test_layout_size_empty() {
        let layout = Layout::new(vec![]);
        assert_eq!(layout.size(), 0);
    }

    #[test]
    fn test_layout_default() {
        let layout = Layout::default();
        assert_eq!(layout.keys, Vec::<Kc>::new());
        assert_eq!(layout.size(), 0);
    }

    #[test]
    fn test_layout_equality() {
        let layout1 = Layout::new(vec![Kc::A, Kc::B]);
        let layout2 = Layout::new(vec![Kc::A, Kc::B]);
        let layout3 = Layout::new(vec![Kc::B, Kc::A]);
        assert_eq!(layout1, layout2);
        assert_ne!(layout1, layout3);
    }

    #[test]
    fn test_layout_ordering() {
        let layout1 = Layout::new(vec![Kc::A, Kc::B]);
        let layout2 = Layout::new(vec![Kc::A, Kc::C]);
        let layout3 = Layout::new(vec![Kc::B, Kc::A]);
        assert!(layout1 < layout2);
        assert!(layout1 < layout3);
    }

    #[test]
    fn test_layout_clone() {
        let layout1 = Layout::new(vec![Kc::A, Kc::B, Kc::C]);
        let layout2 = layout1.clone();
        assert_eq!(layout1, layout2);
    }

    #[test]
    fn test_layout_display() {
        let layout = Layout::new(vec![Kc::A, Kc::B, Kc::C]);
        let display_str = format!("{}", layout);
        assert!(display_str.contains("A"));
        assert!(display_str.contains("B"));
        assert!(display_str.contains("C"));
    }

    #[test]
    fn test_qwerty_keys_length() {
        assert_eq!(QWERTY_KEYS.len(), 30);
    }

    #[test]
    fn test_qwerty_keys_values() {
        assert_eq!(QWERTY_KEYS[0], Kc::Q);
        assert_eq!(QWERTY_KEYS[10], Kc::A);
        assert_eq!(QWERTY_KEYS[20], Kc::Z);
        assert_eq!(QWERTY_KEYS[29], Kc::Slash);
    }

    #[test]
    fn test_colemak_dhm_keys_length() {
        assert_eq!(COLEMAK_DHM_KEYS.len(), 30);
    }

    #[test]
    fn test_colemak_dhm_keys_values() {
        assert_eq!(COLEMAK_DHM_KEYS[0], Kc::Q);
        assert_eq!(COLEMAK_DHM_KEYS[2], Kc::F);
        assert_eq!(COLEMAK_DHM_KEYS[10], Kc::A);
        assert_eq!(COLEMAK_DHM_KEYS[29], Kc::Slash);
    }

    #[test]
    fn test_qwerty_keys_no_duplicates() {
        use std::collections::HashSet;
        let set: HashSet<_> = QWERTY_KEYS.iter().collect();
        assert_eq!(set.len(), QWERTY_KEYS.len());
    }

    #[test]
    fn test_colemak_dhm_keys_no_duplicates() {
        use std::collections::HashSet;
        let set: HashSet<_> = COLEMAK_DHM_KEYS.iter().collect();
        assert_eq!(set.len(), COLEMAK_DHM_KEYS.len());
    }

    #[test]
    fn test_layout_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let layout1 = Layout::new(vec![Kc::A, Kc::B]);
        let layout2 = Layout::new(vec![Kc::A, Kc::B]);
        let layout3 = Layout::new(vec![Kc::B, Kc::A]);
        set.insert(layout1.clone());
        set.insert(layout2);
        set.insert(layout3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_layout_with_single_key() {
        let layout = Layout::new(vec![Kc::A]);
        assert_eq!(layout.size(), 1);
        assert_eq!(layout.keys[0], Kc::A);
    }

    #[test]
    fn test_layout_with_many_keys() {
        let keys: Vec<Kc> = vec![Kc::A; 100];
        let layout = Layout::new(keys.clone());
        assert_eq!(layout.size(), 100);
        assert_eq!(layout.keys, keys);
    }

    #[test]
    fn test_qwerty_vs_colemak_different() {
        assert_ne!(QWERTY_KEYS, COLEMAK_DHM_KEYS);
    }

    #[test]
    fn test_qwerty_vs_colemak_some_same() {
        let qwerty_set: std::collections::HashSet<_> = QWERTY_KEYS.iter().collect();
        let colemak_set: std::collections::HashSet<_> = COLEMAK_DHM_KEYS.iter().collect();
        let intersection: Vec<_> = qwerty_set.intersection(&colemak_set).collect();
        assert!(!intersection.is_empty());
    }

    #[test]
    fn test_layout_debug() {
        let layout = Layout::new(vec![Kc::A, Kc::B]);
        let debug_str = format!("{:?}", layout);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_layout_partial_eq_with_default() {
        let layout1 = Layout::default();
        let layout2 = Layout::new(vec![]);
        assert_eq!(layout1, layout2);
    }
}
