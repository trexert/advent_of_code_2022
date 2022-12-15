use crate::logger::Logger;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Repeat up to twice for more output.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Use sample input rather than main input.
    #[arg(short, long)]
    pub sample: bool,
}

pub fn startup() -> Cli {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);
    cli
}

static LOGGER: Logger = Logger;
