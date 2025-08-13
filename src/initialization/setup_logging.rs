use colored::Colorize;
use env_logger::Builder;
use std::io::Write;

/// Initializes the global logger for the application.
///
/// Sets up colored output, info-level filtering, and a custom format for log messages.
/// This should be called once at startup before any logging occurs.
///
/// # Example
/// ```rust
/// setup_logging().await;
/// ```
pub async fn setup_logging() {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            // Use colored output for log levels and message formatting
            let _style = buf.default_level_style(record.level());
            writeln!(buf, "{}: {}", record.level(), format!("{}", record.args()))
        })
        .init();

    log::info!("{}", "Logger initialized successfully".green());
}
