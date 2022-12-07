use std::{collections::HashSet, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn is_distinct(window: &[u8]) -> bool {
    window.iter().collect::<HashSet<_>>().len() == window.len()
}

fn find_marker_start(sequence: &[u8], size: usize) -> usize {
    size + sequence
        .windows(size)
        .take_while(|window| !is_distinct(window))
        .count()
}

fn main() -> Result<()> {
    let sequence = fs::read("input.txt")?;
    for size in [4, 14] {
        println!("len {} marker found after {} chars", size, find_marker_start(&sequence, size));
    }
    Ok(())
}