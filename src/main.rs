use std::path::PathBuf;
use clap::{Parser, Subcommand};
use weather_lion::commands::weather_forecast::{get_24hr_weather, get_2hr_weather};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    // #[clap(short, long, parse(from_occurrences))]
    // debug: usize,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get 2hr weather
    #[clap(name = "2")]
    Two,
    #[clap(name = "24")]
    /// Get 24hr weather
    OneDay,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Two => {
            get_2hr_weather()?;
        }
        Commands::OneDay => {
            get_24hr_weather()?;
        }
    }
    Ok(())
}
