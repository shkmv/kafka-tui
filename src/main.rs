use std::io;
use std::path::PathBuf;

use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use kafka_tui::app::App;

#[derive(Parser, Debug)]
#[command(
    name = "kafka-tui",
    author,
    version,
    about = "A Terminal User Interface for Apache Kafka",
    long_about = None
)]
struct Args {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Broker addresses (comma-separated)
    #[arg(short, long)]
    brokers: Option<String>,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Suppress rdkafka's internal logging to stderr
    // SAFETY: We set this before any threads are spawned
    unsafe {
        std::env::set_var("RDKAFKA_LOG_LEVEL", "0");
    }

    // Setup logging
    setup_logging(args.verbose)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Hide cursor
    terminal.hide_cursor()?;

    // Run application
    let result = run_app(&mut terminal, args).await;

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Handle any errors from the app
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    _args: Args,
) -> anyhow::Result<()> {
    let mut app = App::new();

    // If brokers were provided via CLI, we could auto-connect here
    // For now, just start the app normally

    app.run(terminal).await?;

    Ok(())
}

fn setup_logging(verbosity: u8) -> anyhow::Result<()> {
    let log_level = match verbosity {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    // For now, log to a file in the data directory
    let log_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kafka-tui")
        .join("logs");

    std::fs::create_dir_all(&log_dir)?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "kafka-tui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_max_level(log_level)
        .init();

    tracing::info!("Kafka TUI starting, log level: {:?}", log_level);

    Ok(())
}
