use clap::{Args, Parser, Subcommand, ValueEnum};
use ftt::{
    define::{Block, FttMap},
    error::Result,
    generate::generate,
    search::search,
    simulate::{simulate, Direction},
};
use std::{fs, io, path::PathBuf, time::Instant};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a map
    Generate {
        #[arg(value_enum, default_value_t = Preset::Easy)]
        preset: Preset,
        #[command(flatten)]
        config: Option<Config>,
        /// The seed
        #[arg(long)]
        seed: Option<u64>,
        #[command(flatten)]
        output: Output,
        #[command(flatten)]
        pipe: Pipe,
    },
    /// Search for a possible solution
    Search {
        /// Map to search
        #[command(flatten)]
        map: Map,
        /// Maximum steps to limit
        #[arg(long)]
        max_steps: Option<u8>,
    },
    /// Simulate a solution
    Simulate {
        /// Map to simulate in
        #[command(flatten)]
        map: Map,
        /// Steps
        steps: String,
    },
}

#[derive(Args)]
struct Config {
    /// The width of the map
    #[arg(long)]
    #[arg(value_parser = clap::value_parser!(u8).range(4..))]
    width: Option<u8>,
    /// The height of the map
    #[arg(long)]
    #[arg(value_parser = clap::value_parser!(u8).range(3..))]
    height: Option<u8>,
    /// The blocks
    #[arg(long)]
    blocks: Option<String>,
    /// Minimum steps to limit
    #[arg(long)]
    min_steps: Option<u8>,
    /// Maximum steps to limit
    #[arg(long)]
    max_steps: Option<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Preset {
    Easy,
    Normal,
}

#[derive(Args)]
#[group(multiple = false)]
struct Output {
    /// Output in JSON format
    #[arg(long)]
    json: bool,
    /// Save to file
    #[arg(long)]
    file: Option<PathBuf>,
}

#[derive(Args)]
struct Pipe {
    /// Simulate the map with inputs
    #[arg(long)]
    simulate: bool,
    /// Search in the map
    #[arg(long)]
    search: bool,
}

#[derive(Args)]
#[group(multiple = false)]
struct Map {
    /// Map is JSON format
    #[arg(long)]
    json: Option<String>,
    /// Load from file
    #[arg(long)]
    file: Option<PathBuf>,
}

fn parse_map(map: Map) -> Result<FttMap> {
    let json = match map.json {
        Some(value) => value,
        None => fs::read_to_string(map.file.unwrap())?,
    };
    Ok(serde_json::from_str(&json.replace("'", "\""))?)
}

struct GenerateConfig {
    width: usize,
    height: usize,
    blocks: Vec<(Block, f64)>,
    min_steps: usize,
    max_steps: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            preset,
            config,
            seed,
            output,
            pipe,
        } => {
            let mut generate_config = match preset {
                Preset::Easy => GenerateConfig {
                    width: 8,
                    height: 6,
                    blocks: vec![
                        (Block::Wall, 2.0 / 15.0),
                        (Block::Piston, 1.0 / 15.0),
                    ],
                    min_steps: 4,
                    max_steps: 10,
                },
                Preset::Normal => GenerateConfig {
                    width: 12,
                    height: 9,
                    blocks: vec![
                        (Block::Wall, 0.1),
                        (Block::Piston, 0.1),
                        (Block::Sand, 0.1),
                        (Block::Cobweb, 0.1),
                    ],
                    min_steps: 4,
                    max_steps: 13,
                },
            };

            if let Some(config) = config {
                if let Some(width) = config.width {
                    generate_config.width = width as usize;
                }
                if let Some(height) = config.height {
                    generate_config.height = height as usize;
                }
                if let Some(blocks) = config.blocks {
                    generate_config.blocks =
                        serde_json::from_str(&blocks.replace("'", "\""))?
                };
                if let Some(min_steps) = config.min_steps {
                    generate_config.min_steps = min_steps as usize;
                }
                if let Some(max_steps) = config.max_steps {
                    generate_config.max_steps = max_steps as usize;
                }
            }

            let GenerateConfig {
                width,
                height,
                blocks,
                min_steps,
                max_steps,
            } = generate_config;

            let mut maps = 0;
            let (mut ftt, seed, steps) = loop {
                maps += 1;
                let (ftt, seed) = generate(width, height, &blocks, seed)?;
                // println!("{ftt}");
                let steps = search(ftt.clone(), max_steps, false);
                if steps.clone().unwrap_or_default().len() >= min_steps {
                    break (ftt, seed, steps);
                }
            };

            if output.json || output.file.is_some() {
                let json = serde_json::to_string(&ftt)?;
                let json = format!("//seed: {seed}\n{json}");
                match output.file {
                    Some(file) => fs::write(file, json)?,
                    None => println!("{json}"),
                }
            } else {
                println!(
                    "maps: {maps} \tseed: {seed} \t steps: {}",
                    match steps.clone() {
                        Some(steps) => steps.len().to_string(),
                        None => "Unknown".to_string(),
                    },
                );
                println!("{ftt}");
            }

            if pipe.search {
                let steps = steps
                    .clone()
                    .or_else(|| search(ftt.clone(), max_steps, true));
                match steps.clone() {
                    Some(steps) => println!("A possible solution: {steps:?}"),
                    None => println!("No solution!"),
                }
            }

            if pipe.simulate {
                let mut len = 0;
                let timer = Instant::now();
                loop {
                    let mut input = String::new();
                    let _ = io::stdin().read_line(&mut input)?;
                    for char in input.trim().to_lowercase().chars() {
                        simulate(&mut ftt, Direction::try_from(char)?);
                        len += 1;
                    }
                    println!("\r{ftt}");
                    if ftt.player == ftt.target {
                        break;
                    }
                }
                println!(
                    "Solved in {:?} with {}/{} steps!",
                    timer.elapsed(),
                    len,
                    steps.unwrap().len(),
                );
            }
        }

        Commands::Search { map, max_steps } => {
            let map = parse_map(map)?;
            let max_steps = max_steps.unwrap_or(u8::MAX);
            let steps = search(map, max_steps as usize, true);
            match steps {
                Some(steps) => println!("A possible solution: {steps:?}"),
                None => println!("No solution!"),
            }
        }

        Commands::Simulate { map, steps } => {
            let mut ftt = parse_map(map)?;
            for char in steps.chars() {
                simulate(&mut ftt, Direction::try_from(char)?);
            }
            println!("{ftt}");
        }
    }

    Ok(())
}
