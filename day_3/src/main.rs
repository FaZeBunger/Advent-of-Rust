use std::fs;

fn parse() -> Vec<String> {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let mut out: Vec<String> = Vec::new();

    for line in input.lines() {
        out.push(line.to_string())
    }

    out
}

fn get_index(item: &char) -> u32 {
    if item.is_lowercase() {
        item.clone() as u32 - 96
    } else {
        item.clone() as u32 - 38
    }
}

fn part_1() {
    let rucksacks: Vec<String> = parse();
    let mut items: Vec<char> = Vec::new();

    for sack in rucksacks {
        let (first, second) = sack.split_at(sack.len() / 2);

        for char in first.chars() {
            if second.contains(char) {
                items.push(char);
                break;
            }
        }
    }

    let items = items.iter().map(|item| get_index(item));
    let s: u32 = items.sum();
    println!("Total: {s}");
}

fn part_2() {
    let rucksacks: Vec<String> = parse();
    let length: usize = rucksacks.len();
    let mut badges: Vec<char> = Vec::new();

    for idx in (0..length).step_by(3) {
        let sack_1 = rucksacks[idx].clone();
        let sack_2 = rucksacks[idx + 1].clone();
        let sack_3 = rucksacks[idx + 2].clone();

        for char in sack_1.chars() {
            if sack_2.contains(char) && sack_3.contains(char) {
                badges.push(char);
                break
            }
        }
    }

    let badges = badges.iter().map(|badge| get_index(badge));
    let s: u32 = badges.sum();

    println!("Part 2: {s}");
}

fn main() {
    part_1();
    part_2();
}

