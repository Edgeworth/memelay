use crate::models::count_map::CountMap;
use crate::types::{KCSet, KC};

pub fn kcm(kcset: KCSet) -> CountMap<KC> {
    let kcm = CountMap::new();
    merge_kcm(kcm, kcset)
}

pub fn merge_kcm(mut kcm: CountMap<KC>, kcset: KCSet) -> CountMap<KC> {
    for kc in kcset {
        kcm.adjust_count(kc, true);
    }
    kcm
}
