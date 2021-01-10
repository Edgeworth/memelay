use crate::ingest::{load_corpus, load_layout_cfg};
use crate::models::layer::Layout;
use crate::prelude::*;
use crate::types::{Finger, PhysEv};
use crate::Args;
use radiate::Envionment;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt, Default, PartialEq)]
pub struct Constants {
    #[structopt(short, long, default_value = "100", help = "Population size for GA")]
    pub pop_size: usize,

    #[structopt(short, long, default_value = "100", help = "Number of generations to run for GA")]
    pub runs: usize,

    #[structopt(long, default_value = "100", help = "Batch size for GA fitness")]
    pub batch_size: usize,

    #[structopt(long, default_value = "10", help = "Number of batches to run for GA fitness")]
    pub batch_num: usize,

    #[structopt(
        long,
        default_value = "4",
        help = "Maximum number of physical keys to press at a time"
    )]
    pub max_phys_pressed: usize,

    #[structopt(
        long,
        default_value = "4",
        help = "Maximum number of physical key-strokes without generating any keycodes"
    )]
    pub max_phys_idle: usize,

    #[structopt(
        long,
        default_value = "20",
        help = "Maximum number of physical keys with mod keycodes per layer"
    )]
    pub max_phys_mod_per_layer: usize,

    #[structopt(
        long,
        default_value = "2",
        help = "Maximum number of physical keys with identical keycode sets per layer"
    )]
    pub max_phys_duplicate_per_layer: usize,

    #[structopt(
        long,
        default_value = "1",
        help = "Maximum number of duplicate mod keycodes pressed"
    )]
    pub max_mod_pressed: usize,

    // Roulette distributions for controlling randomness in various places:
    #[structopt(
        long,
        default_value = "30,70",
        use_delimiter = true,
        help = "Weight to assign k regular keycodes to a key, where k is in the index."
    )]
    pub num_reg_assigned_weights: Vec<f32>,

    #[structopt(
        long,
        default_value = "70,4,4,2,2",
        use_delimiter = true,
        help = "Weight to assign k mod keycodes to a key, where k is in the index."
    )]
    pub num_mod_assigned_weights: Vec<f32>,

    #[structopt(
        long,
        default_value = "1,10",
        use_delimiter = true,
        help = "Weights to roulette each crossover strategy."
    )]
    pub crossover_strat_weights: Vec<f32>,

    #[structopt(
        long,
        default_value = "10,1,20",
        use_delimiter = true,
        help = "Weights to roulette each mutate strategy."
    )]
    pub mutate_strat_weights: Vec<f32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayoutCfg {
    pub layout: String,
    pub cost: Vec<f64>,
    pub fing: Vec<Finger>,
}

impl LayoutCfg {
    pub fn format_solution(&self, l: &Layout) -> String {
        let mut s = String::new();
        for (i, layer) in l.layers.iter().enumerate() {
            s += &format!("Layer {}\n", i);
            let mut idx = 0;
            for c in self.layout.chars() {
                if c == 'X' {
                    let mut kstr = format!("{:?}", layer.keys[idx]);
                    kstr.retain(|c| !r"() ".contains(c));
                    let kstr = kstr.replace("EnumSet", "");
                    s += &kstr;
                    idx += 1;
                } else {
                    s.push(c);
                }
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub layout_cfg: LayoutCfg,
    pub corpus: Vec<PhysEv>,
    pub cnst: Constants,
}

impl Env {
    pub fn from_args(args: Args) -> Result<Self> {
        let layout_cfg = load_layout_cfg(&args.cfg_path)?;
        let corpus = load_corpus(&args.corpus_path)?;
        Ok(Self { layout_cfg, corpus, cnst: args.cnst })
    }

    pub fn num_physical(&self) -> usize {
        self.layout_cfg.cost.len()
    }
}

impl Envionment for Env {}
impl Default for Env {
    fn default() -> Self {
        Self {
            layout_cfg: Default::default(),
            corpus: Default::default(),
            cnst: Default::default(),
        }
    }
}
