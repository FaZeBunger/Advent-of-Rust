use std::fs;

mod linked_list;

type Matrix = Vec<Vec<char>>;
type Coord = (usize, usize);

const MAX_SIZE: usize = 100000000;

fn get_surrounding(matrix: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<linked_list::Node> {
    use crate::linked_list::Node;
    let mut surrounding = Vec::new();

    if x > 0 {
        let node = Node::new(matrix[y][x - 1], (x - 1, y));
        surrounding.push(node);
    }
    if x < matrix[y].len() - 1 {
        let node = Node::new(matrix[y][x + 1], (x + 1, y));
        surrounding.push(node);
    }
    if y > 0 {
        let node = Node::new(matrix[y - 1][x], (x, y - 1));
        surrounding.push(node);
    }
    if y < matrix.len() - 1 {
        let node = Node::new(matrix[y + 1][x], (x, y + 1));
        surrounding.push(node);
    }

    surrounding
}

fn create_matrix() -> Matrix {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_char(matrix: &Matrix, c: char) -> Option<(usize, usize)> {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == c {
                return Some((x, y));
            }
        }
    }
    None
}

fn traverse(
    matrix: &Matrix,
    node: linked_list::Node,
    mut _depth: usize,
    seen: &mut Vec<Coord>,
    curr_shortest: &mut usize,
) {
    if node.value == 'E' {
        return;
    }

    seen.push(node.coords);

    // I want to spawn a new thread for each surrounding node so that
    // each can run concurrently.
    //
    // I need to make sure that I don't have too many threads, unless
    // rust is creating green threads. If it isn't, too many threads
    // would cause the program to crash.
    //
    // There is no guarantee that by rust not using system threads, and instead
    // it's own, would not create crashes or errors.
    //
    // Steps
    //  1. For each node, create a new thread, cloning the current information
    //     into the thread, so that each thread can act independently.
    //
    //  2. Create a mutex of a vector of paths that each thread can access.
    //     If another thread has seen or is currently on that path, then we skip
    //     out of the loop for that path and start a new one.
    //
    //  3. Create a mutex for the current shortest length.
    //     If the path we found is not the shortest, or is longer than the shortest
    //     that a thread has just found, we should stop looking at that path and start
    //     looking for a new path until all have been exhausted.
    //
    //  4. Return whatever value is in the mutex currently.


    for next_node in get_surrounding(&matrix, node.coords) {
        if node.can_move(next_node.value) {
            if seen.contains(&next_node.coords) {
                continue;
            }

            if _depth >= *curr_shortest {
                return;
            }

            if next_node.value == 'E' {
                if _depth < *curr_shortest {
                    println!("Current Shortest: {}", _depth);
                    *curr_shortest = _depth;
                }
                return;
            }

            _depth += 1;
            traverse(&matrix, next_node, _depth, seen, curr_shortest);
            _depth -= 1;
            seen.pop();
        }
    }

    return;
}

fn part_1() {
    use linked_list::Node;

    let depth = 1;
    let matrix = create_matrix();
    let start_index = find_char(&create_matrix(), 'S').unwrap();
    let start_node = Node::new('S', start_index);
    let mut seen = Vec::new();
    let mut curr_shortest = MAX_SIZE;

    traverse(&matrix, start_node, depth, &mut seen, &mut curr_shortest);

    println!("Found Length: {}", curr_shortest);
}

fn main() {
    part_1();
}
