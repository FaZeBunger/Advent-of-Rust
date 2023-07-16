use std::fs;

struct Pair {
    first: Section,
    second: Section,
}

struct Section {
    start: u16,
    end: u16,
}

fn parse() -> Vec<Pair> {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut pairs: Vec<Pair> = Vec::new();

    for line in input.lines() {
        let (range_1, range_2) = line.split_once(',').unwrap();

        let (start, end) = range_1.split_once('-').unwrap();
        let section_1 = Section {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        };

        let (start, end) = range_2.split_once('-').unwrap();
        let section_2 = Section {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        };

        let new_pair = Pair {
            first: section_1,
            second: section_2,
        };
        pairs.push(new_pair);
    }
    pairs
}

impl Pair {
    fn contains(&self) -> bool {
        if (self.first.start <= self.second.start) && (self.first.end >= self.second.end) {
            return true;
        } else if (self.second.start <= self.first.start) && (self.second.end >= self.first.end) {
            return true;
        }

        return false;
    }

    fn overlaps(&self) -> bool {
        if (self.second.start <= self.first.start) && (self.first.start <= self.second.end) {
            return true;
        } else if (self.second.start <= self.first.end) && (self.first.end <= self.second.end) {
            return true;
        } else if (self.first.start <= self.second.start) && (self.second.start <= self.first.end) {
            return true;
        } else if (self.first.start <= self.second.end) && (self.second.end <= self.first.end) {
            return true;
        }
        return false;
    }
}

fn part_1() {
    let pairs: Vec<Pair> = parse();

    let mut contains = 0;

    for pair in pairs {
        if pair.contains() {
            contains += 1;
        }
    }
    println!("Part 1: {}", contains);
}

fn part_2() {
    let pairs: Vec<Pair> = parse();

    let mut overlaps = 0;

    for pair in pairs {
        if pair.overlaps() {
            overlaps += 1;
        }
    }
    println!("Part 2: {}", overlaps);
}

fn main() {
    part_1();
    part_2();
}
