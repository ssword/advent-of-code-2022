use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

const SHIP_SLOT: usize = 9;

fn parse_input(pathname: &str, ship: &mut Vec<VecDeque<char>>) -> Vec<Move> {
    let mut instructions: Vec<Move> = Vec::new();
    let stack_string = read_to_string(pathname).expect("unwable to read file");
    for line in stack_string.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        if words[0] == "move" {
            let amount = words[1].parse::<usize>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();
            instructions.push(Move { amount, from, to });
        } else if line.contains('[') {
            for token_pair in line.chars().enumerate() {
                for i in 0..SHIP_SLOT {
                    if ship.len() < SHIP_SLOT + 1 {
                        ship.push(VecDeque::new());
                    }

                    if token_pair.0 == 1 + i * 4 && token_pair.1.is_alphabetic() {
                        ship[i].push_front(token_pair.1);
                    }
                }
            }
        }
    }
    return instructions.clone();
}

fn main() {
    let mut ship: Vec<VecDeque<char>> = Vec::new();
    let instructions = parse_input("./data/input05.prod", &mut ship);
    println!("{:?}", ship);

    for i in instructions {
        println!("{:?}", i);
        let ship_len = ship[i.from - 1].len().clone();
        let ship_crate = ship[i.from - 1].split_off(ship_len - i.amount);
        ship[i.to - 1].extend(ship_crate);
        println!("{:?}", ship)
    }
}
