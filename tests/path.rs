use kbd::ingest::{load_corpus, load_layout, load_layout_cfg};
use kbd::models::compute_kevs;
use kbd::models::us::UsModel;
use kbd::path::PathFinder;
use eyre::Result;

#[test]
fn test_alnum_layout_can_path() -> Result<()> {
    let layout_cfg = load_layout_cfg("data/moonlander.cfg")?;
    let corpus = load_corpus("data/alnum.data")?;
    let cnst = Default::default();
    let kevs = compute_kevs(UsModel::new(), &corpus, &cnst);
    let l = load_layout("data/alnum.layout")?;
    let res = PathFinder::new(&layout_cfg, &kevs, &cnst, &l).path();
    assert_eq!(res.kevs_found, kevs.len());
    Ok(())
}
