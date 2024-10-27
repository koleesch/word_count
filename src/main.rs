use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path).unwrap();

    // use larger buffer ti reduce I/O-operations
    let reader = BufReader::with_capacity(1024 * 1024, file);

    let mut word_counts: BTreeMap<String, usize> = BTreeMap::new();

    // Process in larger chunks to allow for more idle time between processing
    for chunk in reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .chunks(10000)
    {
        // high cpu activity: DVFS might increase cpu frequency
        process_chunk(chunk, &mut word_counts);

        // Potential place for a short sleep to allow cpu to enter a lower power state
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    for (word, count) in word_counts {
        println!("{}: {}", word, count);
    }
}

/// Process a chunk of lines, updating the word counts
///
/// # Arguments
///
/// * `chunk` - A slice of strings representing the lines to process
/// * `word_counts` - A mutable reference to a `BTreeMap` to store the word counts
fn process_chunk(chunk: &[String], word_counts: &mut BTreeMap<String, usize>) {
    for line in chunk {
        for word in line.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_chunk() {
        // Arrange
        let chunk = vec![
            String::from("The quick brown fox jumps over the lazy dog"),
            String::from("The dog barks"),
        ];
        let mut word_counts = BTreeMap::new();

        // Act
        process_chunk(&chunk, &mut word_counts);

        // Assert
        assert_eq!(word_counts.get("The"), Some(&2));
        assert_eq!(word_counts.get("the"), Some(&1));
        assert_eq!(word_counts.get("quick"), Some(&1));
        assert_eq!(word_counts.get("brown"), Some(&1));
        assert_eq!(word_counts.get("fox"), Some(&1));
        assert_eq!(word_counts.get("jumps"), Some(&1));
        assert_eq!(word_counts.get("over"), Some(&1));
        assert_eq!(word_counts.get("lazy"), Some(&1));
        assert_eq!(word_counts.get("dog"), Some(&2));
        assert_eq!(word_counts.get("barks"), Some(&1));
        assert_eq!(word_counts.len(), 10); // Total unique words
    }

    #[test]
    fn test_process_chunk_empty() {
        // Arrange
        let chunk: Vec<String> = Vec::new();
        let mut word_counts = BTreeMap::new();

        // Act
        process_chunk(&chunk, &mut word_counts);

        // Assert
        assert!(word_counts.is_empty());
    }

    #[test]
    fn test_process_chunk_case_insensitive() {
        // Arrange
        let chunk = vec![
            String::from("The THE the").to_lowercase(),
            String::from("Dog dog DOG").to_lowercase(),
        ];
        let mut word_counts = BTreeMap::new();

        // Act
        process_chunk(&chunk, &mut word_counts);

        // Assert
        assert_eq!(word_counts.get("the"), Some(&3));
        assert_eq!(word_counts.get("dog"), Some(&3));
        assert_eq!(word_counts.len(), 2);
    }

    #[test]
    fn test_process_chunk_sorted_keys() {
        // Arrange
        let chunk = vec![
            String::from("zebra apple cat"),
            String::from("dog elephant bear"),
        ];
        let mut word_counts = BTreeMap::new();

        // Act
        process_chunk(&chunk, &mut word_counts);

        // Assert
        let keys: Vec<_> = word_counts.keys().collect();
        assert_eq!(keys, vec!["apple", "bear", "cat", "dog", "elephant", "zebra"]);
    }
}