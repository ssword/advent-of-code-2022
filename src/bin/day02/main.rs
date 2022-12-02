use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

pub fn read_num_records<T: AsRef<Path>, U: FromStr>(pathname: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    read_to_string(pathname)
        .expect("unable to open file")
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .filter_map(|num| num.parse::<U>().ok())
                .collect::<Vec<U>>()
        })
        .collect()
}

fn main() {
    let elves_calories_stat = read_num_records("data/input01.txt");
    let mut elves_calories = elves_calories_stat
        .iter()
        .map(|r| r.iter().sum::<u32>())
        .collect::<Vec<u32>>();

    elves_calories.sort_by(|a, b| b.cmp(a));

    println!("The max calorie is {:?}", elves_calories.first().unwrap());

    elves_calories.sort();
    elves_calories.reverse();
    println!(
        "The total calories from top three elves are {:?}",
        elves_calories.iter().take(3).sum::<u32>()
    );
}
