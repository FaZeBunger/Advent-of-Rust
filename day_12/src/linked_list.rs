#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Path {
    Null,
    Address(Box<Node>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub coords: (usize, usize),
    pub value: char,
    pub paths: Vec<Path>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(value: char, coords: (usize, usize)) -> Node {
        Node {
            coords,
            value,
            paths: Vec::new(),
        }
    }

    pub fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }

    pub fn can_move(&self, direction: char) -> bool {
        let real_value = match self.value {
            'S' => 97,
            'E' => 122,
            _ => self.value as u8,
        };
        let direction = match direction {
            'S' => 97,
            'E' => 122,
            _ => direction as u8,
        };

        if real_value >= direction {
            return true;
        } else if real_value == direction - 1 {
            return true;
        }
        return false;
    }
}
