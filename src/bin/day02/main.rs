use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum RockPaperScissors {
    ROCK,
    PAPER,
    SCISSORS,
    None,
}

fn read_round_records(pathname: &str) -> Vec<Vec<RockPaperScissors>> {
    read_to_string(pathname)
        .expect("unable to open file")
        .lines()
        .map(|s| {
            s.split(" ")
                .filter_map(|choice| Some(parse_choice(&choice)))
                .collect()
        })
        .collect()
}

fn parse_choice(choice: &str) -> RockPaperScissors {
    match choice {
        "A" | "X" => RockPaperScissors::ROCK,
        "B" | "Y" => RockPaperScissors::PAPER,
        "C" | "Z" => RockPaperScissors::SCISSORS,
        _ => RockPaperScissors::None,
    }
}

fn parse_round(round: Vec<RockPaperScissors>) -> u32 {
    match *round {
        [RockPaperScissors::ROCK, RockPaperScissors::ROCK] => 4,
        [RockPaperScissors::ROCK, RockPaperScissors::PAPER] => 8,
        [RockPaperScissors::ROCK, RockPaperScissors::SCISSORS] => 3,
        [RockPaperScissors::PAPER, RockPaperScissors::ROCK] => 1,
        [RockPaperScissors::PAPER, RockPaperScissors::PAPER] => 5,
        [RockPaperScissors::PAPER, RockPaperScissors::SCISSORS] => 9,
        [RockPaperScissors::SCISSORS, RockPaperScissors::ROCK] => 7,
        [RockPaperScissors::SCISSORS, RockPaperScissors::PAPER] => 2,
        [RockPaperScissors::SCISSORS, RockPaperScissors::SCISSORS] => 6,
        _ => 0,
    }
}

fn main() {
    let round_records: Vec<Vec<RockPaperScissors>> =
        read_round_records("../../../data/input02.prod");

    let total_score: u32 = round_records
        .iter()
        .map(|round| parse_round(round.to_vec()))
        .sum();

    // println!("{:?}", round_records);
    println!("{:?}", total_score);
}
