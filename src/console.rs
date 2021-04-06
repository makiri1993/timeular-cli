use colored::Colorize;
use log::{Level, Metadata, Record};

pub static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    println!(
                        "{} ❌ - {}",
                        record.level().as_str().red().bold(),
                        record.args()
                    );
                }
                Level::Warn => {
                    println!(
                        "{} ⚠️ - {}",
                        record.level().as_str().red().bold(),
                        record.args()
                    );
                }
                Level::Info => {
                    println!(
                        "{} 💡 - {}",
                        record.level().as_str().yellow().bold(),
                        record.args()
                    );
                }
                Level::Debug => {
                    println!(
                        "{} 🤖 - {}",
                        record.level().as_str().red().bold(),
                        record.args()
                    );
                }
                Level::Trace => {
                    println!(
                        "{} 🔎 - {}",
                        record.level().as_str().red().bold(),
                        record.args()
                    );
                }
            }
        }
    }

    fn flush(&self) {}
}
