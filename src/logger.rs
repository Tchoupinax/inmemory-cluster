use ansi_term::Color;
use env_logger::{Builder, Env};
use std::io::Write;

pub fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("debug"))
        .format(|buf, record| {
            let level_style = match record.level() {
                log::Level::Error => Color::Red.bold(),
                log::Level::Warn => Color::Yellow.bold(),
                log::Level::Info => Color::Green.normal(),
                log::Level::Debug => Color::Blue.normal(),
                log::Level::Trace => Color::Blue.normal(),
            };

            writeln!(
                buf,
                "{} {} - {}",
                buf.timestamp_millis(),
                level_style.paint(record.level().to_string()),
                record.args()
            )
        })
        .init();
}
