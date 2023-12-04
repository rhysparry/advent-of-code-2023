use advent_of_code_2023::days::{day1, day2, day3};
use advent_of_code_2023::io::Source;
use advent_of_code_2023::Solver;
use anyhow::Context;
use clap::Parser;
use log::{info, Level};
use std::ops::RangeInclusive;
use thiserror::Error;

fn source_value_parser(value: &str) -> Result<Source, String> {
    match Source::try_from(value) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The day to run
    #[arg(value_parser = day_in_range)]
    day: u8,
    /// The input file to use
    #[arg(long, short, value_parser = source_value_parser, default_value = "-")]
    input: Source,
    /// The log level to use
    #[arg(long, default_value = "info")]
    log_level: Level,
}

fn print(input: &Source) -> anyhow::Result<()> {
    info!("Reading input from {}", input);
    let input = input
        .read_string()
        .with_context(|| format!("Failed to read input: {input}"))?;
    println!("{}", input);
    Ok(())
}

const DAY_RANGE: RangeInclusive<usize> = 0..=3;

fn day_in_range(value: &str) -> Result<u8, String> {
    let day: usize = value
        .parse()
        .map_err(|e| format!("Invalid day: {} ({})", value, e))?;
    if DAY_RANGE.contains(&day) {
        Ok(day as u8)
    } else {
        Err(format!(
            "Invalid day: {}. Must be in the range {}-{}",
            day,
            DAY_RANGE.start(),
            DAY_RANGE.end()
        ))
    }
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Invalid day: {0}")]
    InvalidDay(u8),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

fn main() -> Result<(), ApplicationError> {
    let cli = Cli::parse();
    simple_logger::init_with_level(cli.log_level).context("Failed to initialize logger")?;
    match cli.day {
        0 => print(&cli.input)?,
        1 => day1::CalibrationSolver
            .run(&cli.input)
            .with_context(|| "Day 1 failed")?,
        2 => day2::GameSolver::default()
            .run(&cli.input)
            .with_context(|| "Day 2 failed")?,
        3 => day3::GearRatioSolver
            .run(&cli.input)
            .with_context(|| "Day 3 failed")?,
        _ => return Err(ApplicationError::InvalidDay(cli.day)),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
