use std::{borrow::BorrowMut, fs};

#[derive(Debug, Clone, Copy)]
enum Val {
    Num(u128),
    Old,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Val),
    Mul(Val),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    divisible_by: u128,
    true_dest: u128,
    false_dest: u128,
}

impl Monkey {
    fn do_turn(&mut self, value: u128) -> u128 {
        let out = match &self.operation {
            Operation::Add(val) => match val {
                Val::Num(num) => value + num,
                Val::Old => value + value,
            },
            Operation::Mul(val) => match val {
                Val::Num(num) => value * num,
                Val::Old => value * value,
            },
        };
        out
    }
}

fn parse_input() -> Vec<Monkey> {
    let input =
        fs::read_to_string("/home/eb/rust/Advent of Code/Advent-of-Rust/day_11/src/input.txt")
            .expect("Unable to read file");
    let mut monkeys = Vec::new();

    let parts = input.split("\n\n");
    for part in parts {
        let mut lines = part.lines();
        let line = lines.next();
        let mut line = match line {
            Some(val) => val,
            None => continue,
        };

        line = lines.next().unwrap();

        // Parse items
        let (_, items) = line.split_once(": ").unwrap();
        line = lines.next().unwrap();

        // Parse Operation
        let (_, operation) = line.split_once("= old ").unwrap();
        let mut operation = operation.split_whitespace();

        let operation = match operation.next().unwrap() {
            "+" => Operation::Add(match operation.next().unwrap() {
                "old" => Val::Old,
                num => Val::Num(num.parse().unwrap()),
            }),
            "*" => Operation::Mul(match operation.next().unwrap() {
                "old" => Val::Old,
                num => Val::Num(num.parse().unwrap()),
            }),
            _ => panic!("Unknown operation"),
        };

        // Parse Divisible by
        line = lines.next().unwrap();
        let (_, divisible_by) = line.split_once("divisible by ").unwrap();
        let divisible_by: u128 = divisible_by.parse().unwrap();

        // Parse destinations
        line = lines.next().unwrap();
        let true_dest = line.split_whitespace().nth(5).unwrap().parse().unwrap();

        line = lines.next().unwrap();
        let false_dest = line.split_whitespace().nth(5).unwrap().parse().unwrap();

        // Push monkey
        monkeys.push(Monkey {
            items: items
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation,
            divisible_by,
            true_dest,
            false_dest,
        });
    }

    monkeys
}

fn part_1() {
    let mut monkeys = parse_input();
    let mut inspections = vec![0 as u128; monkeys.len()];
    let common_multiple: u128 = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let items: Vec<u128> = monkeys[idx].items.drain(..).collect();
            let mut monkey = monkeys[idx].clone();

            for old in items {
                inspections[idx] += 1;
                let new = monkey.do_turn(old);

                let new = new % common_multiple;

                let idx = if new % monkey.divisible_by == 0 {
                    monkey.true_dest as usize
                } else {
                    monkey.false_dest as usize
                };

                let recipient = &mut monkeys[idx];
                recipient.items.push(new);
            }
        }
    }
    inspections.sort();
    println!(
        "{:?}",
        inspections.pop().unwrap() * inspections.pop().unwrap()
    );
}

fn main() {
    part_1();
}
