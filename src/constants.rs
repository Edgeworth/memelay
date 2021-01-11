use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt, Default, PartialEq)]
pub struct Constants {
    #[structopt(short, long, default_value = "100", help = "Population size for GA")]
    pub pop_size: usize,

    #[structopt(short, long, default_value = "100", help = "Number of generations to run for GA")]
    pub runs: usize,

    #[structopt(long, default_value = "20000", help = "Batch size for GA fitness")]
    pub batch_size: usize,

    #[structopt(long, default_value = "1", help = "Number of batches to run for GA fitness")]
    pub batch_num: usize,

    #[structopt(long, help = "Print GA debug info.")]
    pub debug: bool,

    // Runtime search restrictions:
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
        default_value = "1",
        help = "Maximum number of duplicate mod keycodes pressed"
    )]
    pub max_mod_pressed: usize,

    // Layout restrictions:
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

    // Roulette distributions for controlling randomness in various places:
    #[structopt(
        long,
        default_value = "10,90",
        use_delimiter = true,
        help = "Weight to assign k regular keycodes to a key, where k is in the index."
    )]
    pub num_reg_assigned_weights: Vec<f64>,

    #[structopt(
        long,
        default_value = "100,4,3,2,1",
        use_delimiter = true,
        help = "Weight to assign k mod keycodes to a key, where k is in the index."
    )]
    pub num_mod_assigned_weights: Vec<f64>,

    #[structopt(
        long,
        default_value = "1,10",
        use_delimiter = true,
        help = "Weights to roulette each crossover strategy."
    )]
    pub crossover_strat_weights: Vec<f64>,

    #[structopt(
        long,
        default_value = "20,5,1,20",
        use_delimiter = true,
        help = "Weights to roulette each mutate strategy."
    )]
    pub mutate_strat_weights: Vec<f64>,
}
