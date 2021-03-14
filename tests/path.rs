use designer::ingest::{load_corpus, load_layout, load_layout_cfg};
use designer::models::compute_kevs;
use designer::models::us::UsModel;
use designer::path::PathFinder;
use eyre::Result;

#[test]
fn test_all_keys_can_path() -> Result<()> {
    let layout_cfg = load_layout_cfg("data/moonlander.cfg")?;
    let corpus = load_corpus("data/all_keys.data")?;
    let cnst = Default::default();
    let kevs = compute_kevs(UsModel::new(), &corpus, &cnst);
    let l = load_layout("data/all_keys.layout")?;
    let res = PathFinder::new(&layout_cfg, &kevs, &cnst, &l).path();
    assert_eq!(res.kevs_found, kevs.len());
    Ok(())
}
