use std::fs;

fn parse_input() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read file");
}

fn part_1() {
    let as_list: Vec<char> = parse_input().chars().collect();
    let mut current_idx: usize = 4;

    for window in as_list.windows(4) {
        let mut current_values: Vec<char> = Vec::new();

        for char in window {
            if current_values.contains(char) {
                continue;
            }
            current_values.push(char.clone());
        }
        if current_values == window {
            println!("Part 1: {}", current_idx);
            return;
        }
        current_idx += 1;
    }
}

fn part_2() {
    let as_list: Vec<char> = parse_input().chars().collect();
    let mut current_idx: usize = 14;

    for window in as_list.windows(14) {
        let mut current_values: Vec<char> = Vec::new();

        for char in window {
            if current_values.contains(char) {
                continue;
            }
            current_values.push(char.clone());
        }
        if current_values == window {
            println!("Part 1: {}", current_idx);
            return;
        }
        current_idx += 1;
    }

}

fn main() {
    part_1();
    part_2();
}
