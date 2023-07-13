use std::fs;

fn parse() -> Vec<u32> {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut elves: Vec<u32> = Vec::new();

    for elf in input.split("\n\n") {
        let mut total_calories: u32 = 0;
        for line in elf.lines() {
            let int_line: u32 = line.parse().unwrap();
            total_calories += int_line;
        }
        elves.push(total_calories);
    }
    elves.sort();
    elves
}

fn part_1() {
    let elf_list = parse();
    println!("{}", elf_list.last().unwrap())
}

fn part_2() {
    let mut elf_list = parse();
    let mut total_calories = 0;
    for _ in 0..3 {
        total_calories += elf_list.pop().unwrap();
    }
    println!("{total_calories}");
}

fn main() {
    part_1();
    part_2();
}
