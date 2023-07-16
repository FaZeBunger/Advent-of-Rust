use std::fs;

fn create_stacks() -> Vec<Vec<char>> {
    vec![
        vec!['Z', 'J', 'G'],
        vec!['Q', 'L', 'R', 'P', 'W', 'F', 'V', 'C'],
        vec!['F', 'P', 'M', 'C', 'L', 'G', 'R'],
        vec!['L', 'F', 'B', 'W', 'P', 'H', 'M'],
        vec!['G', 'C', 'F', 'S', 'V', 'Q'],
        vec!['W', 'H', 'J', 'Z', 'M', 'Q', 'T', 'L'],
        vec!['H', 'F', 'S', 'B', 'V'],
        vec!['F', 'J', 'Z', 'S'],
        vec!['M', 'C', 'D', 'P', 'F', 'H', 'B', 'T'],
    ]
}

fn convert_to_usize(slice: &str) -> usize {
    slice.trim().parse().unwrap()
}

fn part_1() {
    let mut stacks: Vec<Vec<char>> = create_stacks();
    let instructions: String = fs::read_to_string("input.txt").expect("Unable to read file");

    for line in instructions.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let amount: usize = words[1].trim().parse().unwrap();
        let mut origin: usize = words[3].trim().parse().unwrap();
        let mut dest: usize = words[5].trim().parse().unwrap();

        dest -= 1;
        origin -= 1;

        for _ in 0..amount {
            let item: char = stacks[origin].pop().unwrap();
            stacks[dest].push(item);
        }
    }
    for stack in stacks.clone() {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn part_2() {
    let mut stacks: Vec<Vec<char>> = create_stacks();
    let instructions: String = fs::read_to_string("input.txt").expect("Unable to read file");

    for line in instructions.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let amount: usize = convert_to_usize(words[1]);
        let mut origin: usize = convert_to_usize(words[3]);
        let mut dest: usize = convert_to_usize(words[5]);

        dest -= 1;
        origin -= 1;

        let origin_len: usize = stacks[origin].len();
        let mut to_add: Vec<char> = stacks[origin]
            .drain(origin_len - amount..origin_len)
            .collect();
        stacks[dest].append(&mut to_add);
    }
    for stack in stacks.clone() {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn main() {
    part_1();
    part_2();
}
