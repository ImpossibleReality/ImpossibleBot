use env_logger::builder;
use log::{Level, LevelFilter};
use std::env;
use std::io::Write;

pub fn logger_init() {
    builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{format}{} [{}]\u{001b}[0m - {}",
                record.level(),
                record.target(),
                record.args(),
                format = level_format(record.level())
            )
        })
        .init();
}

fn level_format(level: Level) -> &'static str {
    match level {
        Level::Error => "\u{001b}[1;4;31m",
        Level::Warn => "\u{001b}[1;4;33m",
        Level::Info => "\u{001b}[1;34m",
        Level::Debug => "\u{001b}",
        Level::Trace => "\u{001b}[2m",
    }
}
