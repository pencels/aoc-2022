#![feature(once_cell)]

use std::{fs, sync::LazyLock};
use regex::Regex;

static MOVE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new() -> Stacks {
        Stacks {
            stacks: Vec::new()
        }
    }

    fn push(&mut self, num: usize, c: char) {
        if self.stacks.len() < num {
            self.stacks.resize_with(num, Vec::new);
        }
        self.stacks[num - 1].push(c);
    }

    fn pop(&mut self, num: usize) -> Option<char> {
        self.stacks.get_mut(num - 1).and_then(|stack| stack.pop())
    }

    fn lift(&mut self, num: usize, quantity: usize) -> Vec<char> {
        self.stacks.get_mut(num - 1).map(|stack| {
            stack.drain(stack.len() - quantity..stack.len()).collect()
        }).unwrap_or(Vec::new())
    }

    fn place(&mut self, num: usize, stuff: &[char]) {
        self.stacks.get_mut(num - 1).map(|stack| stack.extend_from_slice(stuff));
    }

    fn peek(&self) -> String {
        let mut s = String::new();
        for stack in &self.stacks {
            if let Some(c) = stack.last() {
                s.push(*c);
            }
        }
        s
    }
}

fn read_stacks(path: &str) -> Stacks {
    let contents = fs::read_to_string(path).unwrap();

    let lines: Vec<_> = contents.lines().take_while(|line| !line.is_empty()).collect();

    let mut stacks = Stacks::new();
    for line in lines.iter().rev().skip(1) {
        for (i, chunk ) in line.as_bytes().chunks(4).enumerate() {
            let chunk = std::str::from_utf8(chunk).unwrap();
            let mut matches = chunk.matches(char::is_alphabetic);
            match matches.nth(0).and_then(|c| c.chars().nth(0)) {
                Some(c) => stacks.push(i + 1, c),
                _ => {}
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn read_moves(path: &str) -> Vec<Move> {
    let contents = fs::read_to_string(path).unwrap();

    contents.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(parse_move)
        .collect()
}

fn parse_move(line: &str) -> Move {
    match MOVE_REGEX.captures(line) {
        Some(captures) => {
            Move {
                quantity: captures[1].parse().unwrap(),
                src: captures[2].parse().unwrap(),
                dst: captures[3].parse().unwrap(),
            }
        },
        _ => panic!("invlaid line: {}", line),
    }
}

fn main() {
    let mut stacks = read_stacks("input.txt");
    let mut other_stacks = stacks.clone();
    let moves = read_moves("input.txt");
    
    for m in moves {
        // Do single moves
        for _ in 0..m.quantity {
            match stacks.pop(m.src) {
                Some(c) => stacks.push(m.dst, c),
                _ => {}
            }
        }

        // Do chunky moves
        let chunk = other_stacks.lift(m.src, m.quantity);
        other_stacks.place(m.dst, &chunk);
    }

    println!("top of stacks: {}", stacks.peek());
    println!("top of stacks after chunky moves: {}", other_stacks.peek());
}
