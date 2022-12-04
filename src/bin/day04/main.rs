use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

fn read_rucksack(pathname: &str) -> Vec<u32> {
    let mut common_items: Vec<char> = Vec::new();
    let mut rucksake_info = read_to_string(pathname).expect("unable to open file");
    for line in rucksake_info.lines() {
        let mut rucksake: Vec<char> = line.chars().collect();
        let rucksake_len = rucksake.len() / 2;
        let (compartment1, compartment2) = rucksake.split_at_mut(rucksake_len);
        for item in compartment1 {
            if compartment2.contains(item) {
                common_items.push(*item);
                break;
            }
        }
    }
    common_items
        .iter()
        .map(|s| match s {
            'A'..='Z' => *s as u32 - 'A' as u32 + 27,
            'a'..='z' => *s as u32 - 'a' as u32 + 1,
            _ => 0,
        })
        .collect::<Vec<u32>>()
}

fn find_common_types(compartment1: &[char], compartment2: &[char]) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::new();
    for item in compartment1 {
        if compartment2.contains(item) {
            common_items.push(*item);
        }
    }
    common_items.clone()
}

fn find_badges(pathname: &str) -> Vec<u32> {
    let mut badges: Vec<char> = Vec::new();
    let mut rucksake_info = read_to_string(pathname)
        .expect("unable to open file")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for group in rucksake_info.chunks(3) {
        let (group1, group2, group3) = (group[0].clone(), group[1].clone(), group[2].clone());
        let badge = find_common_types(&group1, &group2);
        let common_badge = find_common_types(&badge, &group3);
        badges.push(common_badge[0]);
    }
    badges
        .iter()
        .map(|s| match s {
            'A'..='Z' => *s as u32 - 'A' as u32 + 27,
            'a'..='z' => *s as u32 - 'a' as u32 + 1,
            _ => 0,
        })
        .collect::<Vec<u32>>()
}

fn find_fully_contained_assignments(pathname: &str) -> Vec<u32> {
    let mut overlays: Vec<u32> = Vec::new();

    let record = read_to_string(&pathname).expect("unable to open file");

    for line in record.lines() {
        let (elf1, elf2) = line.split_once(",").unwrap();
        let (elf1_low, elf1_high) = elf1
            .split_once("-")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();
        let (elf2_low, elf2_high) = elf2
            .split_once("-")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();

        if (elf1_low <= elf2_low && elf2_high <= elf1_high)
            | (elf2_low <= elf1_low && elf1_high <= elf2_high)
        {
            overlays.push(1);
        } else {
            overlays.push(0);
        }
    }
    overlays
}

fn find_overlay_assignments(pathname: &str) -> Vec<u32> {
    let mut overlays: Vec<u32> = Vec::new();

    let record = read_to_string(&pathname).expect("unable to open file");

    for line in record.lines() {
        let (elf1, elf2) = line.split_once(",").unwrap();
        let (elf1_low, elf1_high) = elf1
            .split_once("-")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();
        let (elf2_low, elf2_high) = elf2
            .split_once("-")
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();

        if (elf1_high <= elf2_high && elf2_low <= elf1_high)
            | (elf2_high <= elf1_high && elf1_low <= elf2_high)
        {
            overlays.push(1);
        } else {
            overlays.push(0);
        }
    }
    overlays
}

fn main() {
    let round_records: Vec<u32> = find_fully_contained_assignments("./data/input04.prod");
    println!(
        "assignment pairs one fully contain the other: {:?}",
        round_records.iter().sum::<u32>()
    );
    let overlay_records: Vec<u32> = find_overlay_assignments("./data/input04.prod");
    println!("{:?}", overlay_records);
    println!(
        "assignment pairs overlay: {:?}",
        overlay_records.iter().sum::<u32>()
    );
}
