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
    #[structopt(
        long,
        default_value = "3",
        help = "Maximum number of physical keys to press on average to make one key event"
    )]
    pub max_phys_per_kev: usize,

    // TODO: Can remove these since we always output a keyev ?
    #[structopt(
        long,
        default_value = "4",
        help = "Maximum number of physical keys to press at a time"
    )]
    pub max_phys_pressed: usize,

    // Value of 2 allows e.g. ctrl + shift to be pressed, then a letter key, but not
    // three modifiers.
    #[structopt(
        long,
        default_value = "2",
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
}
