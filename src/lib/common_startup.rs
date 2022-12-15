use crate::logger::Logger;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

pub fn startup() {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);
}

static LOGGER: Logger = Logger;
