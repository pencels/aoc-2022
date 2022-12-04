#![feature(iter_array_chunks)]

use std::{fs, collections::HashSet};

fn get_item_priority(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        item as i32 - 'a' as i32 + 1
    } else {
        item as i32 - 'A' as i32 + 27
    }
}

fn get_rucksacks(file: &str) -> Vec<&str> {
    file.lines().collect()
}

fn get_rucksack_compartments(file: &str) -> Vec<(&str, &str)> {
    file.lines().map(|line| {
        line.split_at(line.len() / 2)
    }).collect()
}

fn find_common_item(strings: &[&str]) -> char {
    *strings
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|mut acc, other| {
            acc.retain(|e| other.contains(e));
            acc
        })
        .unwrap().iter().nth(0).unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let priority_sum: i32 = get_rucksack_compartments(&file)
        .iter()
        .map(|(l, r)| find_common_item(&[l, r]))
        .map(|i| get_item_priority(i))
        .sum();

    let group_priority_sum: i32 = get_rucksacks(&file)
        .chunks(3)
        .map(|chunk| find_common_item(chunk))
        .map(|i| get_item_priority(i))
        .sum();

    println!("sum is {}", priority_sum);
    println!("group priority sum is {}", group_priority_sum);
}
