mod mutator;
mod homophones;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use mutator::TextMutator;
use structopt::StructOpt;

/// A program that deliberately introduces minor errors into text for proofreading practice
#[derive(StructOpt, Debug)]
#[structopt(name = "text-mutator")]
struct Opt {
    /// Input file path (if not provided, will read from stdin)
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file path (if not provided, will write to stdout)
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Mutation rate (0.0-1.0) - higher means more errors
    #[structopt(short, long, default_value = "0.05")]
    rate: f32,

    /// Random seed for reproducible results
    #[structopt(short, long)]
    seed: Option<u64>,

    /// Enable letter swapping mutations
    #[structopt(long)]
    swap_letters: bool,

    /// Enable punctuation removal mutations
    #[structopt(long)]
    remove_punctuation: bool,

    /// Enable homophone replacements (your/you're, their/there/they're, etc.)
    #[structopt(long)]
    homophones: bool,

    /// Enable all mutation types
    #[structopt(long)]
    all_mutations: bool,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    // Set mutation flags
    let swap_letters = opt.swap_letters || opt.all_mutations;
    let remove_punctuation = opt.remove_punctuation || opt.all_mutations;
    let homophones = opt.homophones || opt.all_mutations;

    // If no specific mutations are enabled, default to all
    let (swap_letters, remove_punctuation, homophones) = if !swap_letters && !remove_punctuation && !homophones {
        (true, true, true)
    } else {
        (swap_letters, remove_punctuation, homophones)
    };

    // Read input
    let mut input = String::new();
    match &opt.input {
        Some(path) => {
            input = fs::read_to_string(path)?;
        },
        None => {
            println!("Please enter input:");
            io::stdin().read_line(&mut input)?;
        }
    }

    // Apply mutations
    let mut text_mutator = TextMutator::new(
        opt.rate,
        opt.seed,
        swap_letters,
        remove_punctuation,
        homophones
    );

    let (mutated_text, num_mutations) = text_mutator.mutate(&input);

    // Write output
    match &opt.output {
        Some(path) => {
            fs::write(path, &mutated_text)?;
            eprintln!("Added {} mutations to the text", num_mutations);
        },
        None => {
            io::stdout().write_all(mutated_text.as_bytes())?;
            eprintln!("\n--- Added {} mutations to the text ---", num_mutations);
        }
    }

    Ok(())
}