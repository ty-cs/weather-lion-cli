use std::io;

use clap::Command;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use weather_lion::commands::temperature::get_temperature;

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
    /// Get real-time air temperature
    #[clap(name = "temp")]
    Temperature,
    /// Get real-time air quality
    #[clap(name = "aqi")]
    AQI,
    /// Get all data
    #[clap(name = "a")]
    All,
    /// Print shell completions to stdout
    Completions { shell: Option<Shell> },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
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
        Commands::Completions { shell } => match *shell {
            Some(shell) => {
                let mut cmd = Cli::command();
                eprintln!("Generating completion file for {}...", shell);
                print_completions(shell, &mut cmd);
            }
            None => {
                println!("No shell specified!")
            }
        },
        Commands::Temperature => {
            get_temperature()?;
        }
        Commands::AQI => {
            println!("AQI");
        }  Commands::All => {
            get_24hr_weather()?;
            println!();
            get_2hr_weather()?;
            println!();
            get_temperature()?;


        }
    }
    Ok(())
}
