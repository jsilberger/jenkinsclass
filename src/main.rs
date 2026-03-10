use clap::Parser;
use fortune::{FORTUNES, print_fortune};
use rand::{RngExt, SeedableRng, rngs::StdRng};

#[derive(Debug, Parser)]
#[command(name = "fortune", version, about = "Print a random developer fortune")]
 struct Cli {
    /// Recipient name shown in the formatted output.
    #[arg(long, default_value = "you")]
    name: String,

    /// RNG seed for deterministic fortune selection.
    #[arg(long)]
    seed: Option<u64>,

    /// List all available fortunes and exit.
    #[arg(long)]
    list: bool,
}

/// Picks a random fortune and prints it for the requested recipient.
fn main() {
    let cli = Cli::parse();

    if cli.list {
        for fortune in &*FORTUNES {
            println!("{fortune}");
        }

        return;
    }

    if FORTUNES.is_empty() {
        eprintln!("No fortunes are available.");
        std::process::exit(1);
    }

    let index = if let Some(seed) = cli.seed {
        let mut rng = StdRng::seed_from_u64(seed);
        rng.random_range(0..FORTUNES.len())
    } else {
        let mut rng = rand::rng();
        rng.random_range(0..FORTUNES.len())
    };

    let output = print_fortune(FORTUNES[index], &cli.name);
    println!("{output}");
}
