use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kbd::constants::Constants;
use kbd::ingest::{load_corpus, load_layout, load_layout_cfg};
use kbd::eval::LayoutCfg;
use kbd::models::compute_kevs;
use kbd::models::layout::Layout;
use kbd::models::us::UsModel;
use kbd::path::{PathFinder, PathResult};
use kbd::types::KeyEv;

struct PathFinderCfg<'a> {
    layout_cfg: &'a LayoutCfg,
    kevs: &'a [KeyEv],
    cnst: &'a Constants,
    l: &'a Layout,
}

fn compute_path(cfg: &PathFinderCfg<'_>) -> PathResult {
    PathFinder::new(cfg.layout_cfg, cfg.kevs, cfg.cnst, cfg.l).path()
}

fn path(c: &mut Criterion) {
    let layout_cfg = load_layout_cfg("data/moonlander.cfg").unwrap();
    let corpus = load_corpus("data/bench.data").unwrap();
    let cnst = Default::default();
    let kevs = compute_kevs(UsModel::new(), &corpus, &cnst);
    let l = load_layout("data/bench.layout").unwrap();
    let cfg = PathFinderCfg { layout_cfg: &layout_cfg, kevs: &kevs, cnst: &cnst, l: &l };
    c.bench_with_input(BenchmarkId::new("path", "bench data"), &cfg, |b, cfg| {
        b.iter(|| compute_path(cfg));
    });
}

criterion_group!(benches, path);
criterion_main!(benches);
