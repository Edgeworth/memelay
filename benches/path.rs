use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use designer::constants::Constants;
use designer::ingest::{load_corpus, load_layout, load_layout_cfg};
use designer::layout_eval::LayoutCfg;
use designer::models::compute_kevs;
use designer::models::layout::Layout;
use designer::models::us::USModel;
use designer::path::PathFinder;
use designer::types::KeyEv;

struct PathFinderCfg<'a> {
    layout_cfg: &'a LayoutCfg,
    kevs: &'a [KeyEv],
    cnst: &'a Constants,
    l: &'a Layout,
}

fn compute_path(cfg: &PathFinderCfg<'_>) -> u128 {
    PathFinder::new(cfg.layout_cfg, cfg.kevs, cfg.cnst, cfg.l).path_fitness()
    // panic!("asdf")
}

fn path(c: &mut Criterion) {
    let layout_cfg = load_layout_cfg("data/moonlander.cfg").unwrap();
    let corpus = load_corpus("data/bench.data").unwrap();
    let cnst = Constants {
        max_phys_pressed: 4,
        max_phys_idle: 2,
        max_mod_pressed: 1,
        ..Default::default()
    };
    let kevs = compute_kevs(USModel::new(), &corpus, &cnst);
    let l = load_layout("data/bench.layout").unwrap();
    let cfg = PathFinderCfg { layout_cfg: &layout_cfg, kevs: &kevs, cnst: &cnst, l: &l };
    c.bench_with_input(BenchmarkId::new("path", "bench data"), &cfg, |b, cfg| {
        b.iter(|| compute_path(cfg));
    });
}

criterion_group!(benches, path);
criterion_main!(benches);