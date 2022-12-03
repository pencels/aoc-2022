use std::collections::HashMap;

use maplit::hashmap;
use lazy_static::lazy_static;
use std::fs;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<i32> for Choice {
    type Error = ();

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => return Err(())
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Result {
    Win,
    Loss,
    Draw,
}

lazy_static! {
    static ref OPPONENT_CHOICES: HashMap<&'static str, Choice> = hashmap! {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
    };

    static ref MY_CHOICES: HashMap<&'static str, Choice> = hashmap! {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
    };

    static ref CHOICE_SCORE: HashMap<Choice, i32> = hashmap! {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    static ref RESULT_SCORE: HashMap<Result, i32> = hashmap! {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Loss => 0,
    };

    static ref DESIRED_RESULT: HashMap<&'static str, Result> = hashmap! {
        "X" => Result::Loss,
        "Y" => Result::Draw,
        "Z" => Result::Win,
    };
}

fn get_result(my_choice: Choice, opponent_choice: Choice) -> Result {
    if my_choice == opponent_choice {
        Result::Draw
    } else if (my_choice as i32 + 1) % 3 == opponent_choice as i32 {
        Result:: Loss
    } else {
        Result:: Win
    }
}

fn get_score(my_choice: Choice, result: Result) -> i32 {
    CHOICE_SCORE.get(&my_choice).unwrap() + RESULT_SCORE.get(&result).unwrap()
}

fn parse_lines<O, F>(path: &str, parse_line: F) -> Vec<O>
    where F: Fn(&str) -> O
{
    let file = fs::read_to_string(path).expect(&format!("{} not found", path));
    file.lines().into_iter().map(parse_line).collect()
}

fn get_choices(path: &str) -> Vec<(Choice, Choice)> {
    parse_lines(path, |line: &str| {
        let mut choices = line.split(" ");
        let opp = OPPONENT_CHOICES.get(choices.next().unwrap()).unwrap();
        let my = MY_CHOICES.get(choices.next().unwrap()).unwrap();
        (*opp, *my)
    })
}

fn get_choices_and_results(path: &str) -> Vec<(Choice, Result)> {
    parse_lines(path, |line| {
        let mut tokens = line.split(" ");
        let opp = OPPONENT_CHOICES.get(tokens.next().unwrap()).unwrap();
        let result = DESIRED_RESULT.get(tokens.next().unwrap()).unwrap();
        (*opp, *result)
    })
}

fn get_choice_for_result(opponent_choice: Choice, result: Result) -> Choice {
    match result {
        Result::Draw => opponent_choice,
        Result::Loss => Choice::try_from((opponent_choice as i32 + 2) % 3).unwrap(),
        Result::Win => Choice::try_from((opponent_choice as i32 + 1) % 3).unwrap(),
    }
}

fn main() {
    let mut score = 0;
    let choices = get_choices("input.txt");
    for (opp, my) in choices {
        let result = get_result(my, opp);
        score += get_score(my, result);
    }
    println!("score is {}", score);

    let choices_and_results = get_choices_and_results("input.txt");
    let mut score = 0;
    for (opp, result) in choices_and_results {
        let my = get_choice_for_result(opp, result);
        score += get_score(my, result);
    }
    println!("new score is {}", score);
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn get_loss() {
        let choice = get_choice_for_result(Choice::Rock, Result::Loss);
        assert_eq!(choice, Choice::Scissors);
    }
}