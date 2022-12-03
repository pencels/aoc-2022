use std::fs;

fn get_bundles(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path)
        .expect(&format!("{} not found", path));
    let mut bundles = Vec::new();
    let mut bundle = Vec::new();

    for line in contents.lines() {
        if line == "" {
            bundles.push(bundle);
            bundle = Vec::new();
        } else {
            let calories: i32 = line.parse().unwrap();
            bundle.push(calories);
        }
    }

    bundles
}

fn sum(slice: &[i32]) -> i32 {
    let mut s = 0;
    for i in slice {
        s += i;
    }
    s
}

fn main() {
    let bundles = get_bundles("input.txt");
    let mut max = 0;
    for bundle in &bundles {
        max = max.max(sum(&bundle));
    }
    println!("max is {}", max);

    let mut top_three = bundles
        .iter()
        .map(|b| sum(b))
        .collect::<Vec<i32>>();
    top_three.sort();
    top_three.reverse();
    let top_three = &top_three[..3];

    println!("top three sum is {}", sum(top_three));
}
