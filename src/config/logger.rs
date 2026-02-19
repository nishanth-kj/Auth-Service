use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::time::SystemTime;

pub fn setup_logger(env: &str) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    let level = if env == "PRODUCTION" {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file("auth-service.log")?)
        .apply()?;
    Ok(())
}
