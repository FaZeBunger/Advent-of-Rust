use std::fs;

enum Move {
    Rock,
    Paper,
    Scissors,
    Win,
    Lose,
    Draw,
}

impl Move {
    fn calc_points(&self, enemy: Move) -> u32 {
        let move_points = match self {
            Move::Rock => {
                (match enemy {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                    _ => panic!("Enemy did not match any option"),
                } + 1)
            }
            Move::Paper => {
                (match enemy {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                    _ => panic!("Enemy did not match any option"),
                } + 2)
            }
            Move::Scissors => {
                (match enemy {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                    _ => panic!("Enemy did not match any option"),
                } + 3)
            }
            Move::Win => {
                (match enemy {
                    Move::Rock => 2,
                    Move::Paper => 3,
                    Move::Scissors => 1,
                    _ => panic!("Enemy did not match any option"),
                } + 6)
            }
            Move::Lose => {
                (match enemy {
                    Move::Rock => 3,
                    Move::Paper => 1,
                    Move::Scissors => 2,
                    _ => panic!("Enemy did not match any option"),
                } + 0)
            }
            Move::Draw => {
                (match enemy {
                    Move::Rock => 1,
                    Move::Paper => 2,
                    Move::Scissors => 3,
                    _ => panic!("Enemy did not match any option"),
                } + 3)
            }
        };
        return move_points;
    }
}

fn part_1() {
    let moves: String = parse();
    let mut total_points = 0;

    for line in moves.lines() {
        let (player_1, player_2) = line.split_once(' ').unwrap();

        let player_1 = match player_1 {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Player 1 does not match any field"),
        };
        let player_2 = match player_2 {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Player 1 does not match any field"),
        };

        total_points += player_2.calc_points(player_1);
    }
    println!("Part 1: {total_points}");
}

fn part_2() {
    let moves: String = parse();
    let mut total_points = 0;

    for line in moves.lines() {
        let (player_1, player_2) = line.split_once(' ').unwrap();

        let player_1 = match player_1 {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Player 1 does not match any field"),
        };
        let player_2 = match player_2 {
            "X" => Move::Lose,
            "Y" => Move::Draw,
            "Z" => Move::Win,
            _ => panic!("Player 1 does not match any field"),
        };

        total_points += player_2.calc_points(player_1);
    }
    println!("Part 2: {total_points}");
}

fn parse() -> String {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    input
}

fn main() {
    part_1();
    part_2();
}
