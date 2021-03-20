use eyre::Result;
use kbd::run;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    color_eyre::install()?;
    run()
}
