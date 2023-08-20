use std::fs;

mod linked_list;

type Matrix = Vec<Vec<char>>;
type Coord = (usize, usize);

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
    paths: &mut Vec<Vec<Coord>>,
) {
    use linked_list::Path;

    if node.value == 'E' {
        return;
    }

    seen.push(node.coords);
    for next_node in get_surrounding(&matrix, node.coords) {
        if node.can_move(next_node.value) {
            _depth += 1;
            if seen.contains(&next_node.coords) {
                continue;
            }
            if next_node.value == 'E' {
                paths.push(seen.clone());
                return;
            }
            traverse(&matrix, next_node, _depth, seen, paths);
            seen.pop();
        }
        _depth -= 1;
    }

    return;
}

fn part_1() {
    use linked_list::Node;

    let depth = 0;
    let matrix = create_matrix();
    let start_index = find_char(&create_matrix(), 'S').unwrap();
    let start_node = Node::new('S', start_index);

    let mut seen = Vec::new();
    let mut paths = Vec::new();

    traverse(&matrix, start_node, depth, &mut seen, &mut paths);

    let shortest_path = paths.iter().min_by_key(|path| path.len()).unwrap();

    println!("Found {} paths", paths.len());
    println!("Shortest path: {:?}", shortest_path);
    println!("Shortest path length: {}", shortest_path.len());
}

fn main() {
    part_1();
}
