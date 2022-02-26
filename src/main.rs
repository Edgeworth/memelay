use eyre::Result;
use memelay::run;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    color_eyre::install()?;
    run()
}
