use colored::Colorize;
use log::Level;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        match record.metadata().level() {
            Level::Error => println!("{}", format!("{}", record.args()).red()),
            Level::Warn => println!("{}", format!("{}", record.args()).yellow()),
            Level::Info => println!("{}", format!("{}", record.args())),
            Level::Debug => println!("{}", format!("{}", record.args()).blue()),
            Level::Trace => println!("{}", format!("{}", record.args()).cyan()),
        }
    }

    fn flush(&self) {}
}
