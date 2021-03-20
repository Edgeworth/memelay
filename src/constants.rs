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
}
