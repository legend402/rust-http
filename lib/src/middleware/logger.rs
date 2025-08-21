use env_logger::fmt::Color;
use env_logger::Env;
use chrono::Local;
use log::LevelFilter;
use std::io::{Write};

pub fn init_logger() {
    let env = Env::default().filter_or("info", "debug");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };
            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.args())
        })
        .filter(None, LevelFilter::Debug)
        .init();
}