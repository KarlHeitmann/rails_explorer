use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::io;


use ratatui::{
    backend::{CrosstermBackend, Backend},

    // backend::{ CrosstermBackend, Backend },
    Terminal
};

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

use crate::ui::App;

mod ui;
mod routes;

/// Resets the terminal.
fn reset_terminal() -> std::result::Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

}

fn setup_logger(file_path: &str) {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or(String::new());
    let level = match log_level.as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        "off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Error,
    };

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

pub fn explorer_wrapper<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    terminal.clear()?;
    let mut app = App::new();
    app.run(terminal)?;
    // app::app(terminal, &mut node_list_state, &mut git_explorer, repo);
    Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger(&std::env::var("LOG_FILE").unwrap_or(String::from("./log.txt"))); // TODO: if env var is not present, don't setup logger

    /*
    // Different kinds of log messages, in hierarchy order.
    log::error!("Program starts...");
    log::warn!("Program starts...");
    log::info!("Program starts...");
    log::debug!("Program starts...");
    log::trace!("Program starts...");
    */

    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode().expect("can run in raw mode");

    let stdout = std::io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    chain_hook();

    explorer_wrapper(&mut terminal);


    reset_terminal();
    disable_raw_mode()?;
    terminal.show_cursor()?;

    // test_info(&repo);


    Ok(())
}
