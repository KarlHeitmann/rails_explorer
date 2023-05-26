// use log::{trace, LevelFilter, SetLoggerError};
// use log::{trace, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    // filter::threshold::ThresholdFilter,
};

fn setup_logger() {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or(String::new());
    let level = match log_level.as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    };
    let file_path = "./log";

    // Build a stderr logger.
    let _stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                // .build(log::LevelFilter::Trace),
                .build(level),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    // let _handle = log4rs::init_config(config)?;
    let _handle = log4rs::init_config(config).unwrap();
}

fn main() {
    setup_logger();

    log::error!("Program starts...");
    log::warn!("Program starts...");
    log::info!("Program starts...");
    log::debug!("Program starts...");
    log::trace!("Program starts...");

    println!("Hello, world!");

}
