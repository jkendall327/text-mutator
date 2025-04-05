mod mutator;
mod homophones;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::io::{self, Write};
use mutator::TextMutator;

/// A program that deliberately introduces minor errors into text for proofreading practice

fn main() -> io::Result<()> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting text-mutator");

    // Set mutation flags
    let swap_letters = true;
    let remove_punctuation = true;
    let homophones = true;

    // If no specific mutations are enabled, default to all
    let (swap_letters, remove_punctuation, homophones) = if !swap_letters && !remove_punctuation && !homophones {
        (true, true, true)
    } else {
        (swap_letters, remove_punctuation, homophones)
    };

    // Read input
    let mut input = String::new();
    println!("Please enter input:");
    io::stdin().read_line(&mut input)?;

    // Apply mutations
    let mut text_mutator = TextMutator::new(
        1.0,
        None,
        swap_letters,
        remove_punctuation,
        homophones
    );

    let (mutated_text, num_mutations) = text_mutator.mutate(&input);

    // Write output
    io::stdout().write_all(mutated_text.as_bytes())?;
    eprintln!("\n--- Added {} mutations to the text ---", num_mutations);

    Ok(())
}
