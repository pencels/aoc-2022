use std::fs;

struct Assignment(i32, i32);

impl Assignment {
    fn start(&self) -> i32 {
        self.0
    }

    fn end(&self) -> i32 {
        self.1
    }
}

fn parse_assignment(s: &str) -> Assignment {
    let (lhs, rhs) = s.split_once('-').unwrap();
    Assignment(lhs.parse().unwrap(), rhs.parse().unwrap())
}

fn read_assignments(path: &str) -> Vec<(Assignment, Assignment)> {
    let file = fs::read_to_string(path).unwrap();

    file.lines().map(|line| {
        let (lhs, rhs) = line.split_once(',').unwrap();
        (parse_assignment(lhs), parse_assignment(rhs))
    }).collect()
}

fn contains(lhs: &Assignment, rhs: &Assignment) -> bool {
    lhs.start() <= rhs.start() && lhs.end() >= rhs.end()
}

fn contains_symmetric(lhs: &Assignment, rhs: &Assignment) -> bool {
    contains(lhs, rhs) || contains(rhs, lhs)
}

fn overlaps(lhs: &Assignment, rhs: &Assignment) -> bool {
    rhs.start() <= lhs.end() && rhs.end() >= lhs.start() || lhs.start() <= rhs.end() && lhs.end() >= rhs.start()
}

fn main() {
    let assignments = read_assignments("input.txt");
    let contained = assignments
        .iter()
        .filter(|(l, r)| contains_symmetric(l, r))
        .count();
    println!("contained {}", contained);

    let overlaps = assignments
        .iter()
        .filter(|(l, r)| overlaps(l, r))
        .count();
    println!("overlaps {}", overlaps);
}
