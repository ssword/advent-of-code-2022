use std::fs;
use std::io::{BufRead, BufReader, Result as io_result};

fn main() -> io_result<()> {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let mut elves_calories_stat: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in input_string.lines() {
        if line.is_empty() {
            elves_calories_stat.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }

    elves_calories_stat.push(sum);

    println!(
        "The max calory is {:?}",
        elves_calories_stat.iter().max().unwrap()
    );

    elves_calories_stat.sort();
    elves_calories_stat.reverse();
    println!(
        "The total calories from top three elves are {:?}",
        elves_calories_stat[..3].iter().sum::<u32>()
    );

    Ok(())
}
