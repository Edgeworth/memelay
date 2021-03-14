use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt, Default, PartialEq)]
pub struct Constants {
    #[structopt(short, long, default_value = "100", help = "Population size for GA")]
    pub pop_size: usize,

    #[structopt(
        short,
        long,
        default_value = "10000",
        help = "Number of generations to run for GA"
    )]
    pub runs: usize,

    // Runtime search restrictions:
    // Layout restrictions:
    #[structopt(
        long,
        default_value = "20",
        help = "Maximum number of physical keys with mod keycodes"
    )]
    pub max_phys_mod: usize,

    #[structopt(
        long,
        default_value = "2",
        help = "Maximum number of physical keys with identical keycode sets"
    )]
    pub max_phys_dup: usize,

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
}
