use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn read_words_from_file(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    println!("-> Reading input from: {}", path.display());
    let content = fs::read_to_string(path)?;
    let words: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
    Ok(words)
}

fn benchmark_set(words: Vec<String>) -> (HashSet<String>, f64) {
    let start_time = Instant::now();

    let mut word_set: HashSet<String> = HashSet::new();
    for word in words {
        word_set.insert(word);
    }

    let duration = start_time.elapsed();
    println!("\n--- Set Benchmark (Unique Words) ---");
    println!("Time taken: {:.4} seconds", duration.as_secs_f64());

    (word_set, duration.as_secs_f64())
}

fn benchmark_hashmap(words: Vec<String>) -> (HashMap<String, u32>, f64) {
    let start_time = Instant::now();

    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    let duration = start_time.elapsed();
    println!("\n--- Hash Map Benchmark (Word Frequencies) ---");
    println!("Time taken: {:.4} seconds", duration.as_secs_f64());

    (word_counts, duration.as_secs_f64())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path_to_file>");
        eprintln!("Example: cargo run -- ./bible.txt");
        return Ok(());
    }

    let file_path = PathBuf::from(&args[1]);

    let words = read_words_from_file(&file_path)?;
    println!("Successfully read {} words.", words.len());

    let (word_set, set_time) = benchmark_set(words.clone());
    println!("Result: Found {} unique words.", word_set.len());

    let (word_counts, map_time) = benchmark_hashmap(words);
    println!(
        "Result: Found {} unique words with frequency counts.",
        word_counts.len()
    );

    println!("\n========================================");
    println!("        BENCHMARK SUMMARY");
    println!("========================================");
    println!("Set (HashSet) Time: {:.4} seconds", set_time);
    println!("Hash Map (HashMap) Time: {:.4} seconds", map_time);

    if map_time < set_time {
        println!("\nConclusion: The Hash Map approach was faster for this task.");
    } else if set_time < map_time {
        println!("\nConclusion: The Set approach was faster for this task.");
    } else {
        println!("\nConclusion: Both approaches took approximately the same time.");
    }

    Ok(())
}
