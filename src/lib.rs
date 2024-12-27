pub mod template;

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

/// Assuming zero coordinate of the board is at top left corner
#[derive(PartialEq, Clone)]
pub struct Board {
    pub cells: Vec<Vec<char>>,
}

pub trait Bounded {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_in_bounds(&self, coord: &Coord) -> bool;
}

pub trait Searchable {
    fn find(&self, char: char) -> Option<Coord>;
    
    fn count(&self, char: char) -> u32;
}

impl Board {
    pub fn new(cells: Vec<Vec<char>>) -> Board {
        return Board { cells };
    }
    
    pub fn from(input: &str) -> Board {
        return Board::new(input.lines().map(|line| line.chars().collect()).collect());
    }

    pub fn at(&self, coord: &Coord) -> Option<char> {
        if self.is_in_bounds(coord) {
            return Some(self.cells[coord.x as usize][coord.y as usize]);
        };

        return None;
    }

    pub fn mutate(&mut self, coord: &Coord, value: char) {
        self.cells[coord.x as usize][coord.y as usize] = value;
    }
}

impl Bounded for Board {
    fn width(&self) -> usize {
        if self.cells.len() == 0 {
            return 0;
        }
        
        return self.cells[0].len();
    }

    fn height(&self) -> usize {
        return self.cells.len();
    }
    
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        return coord.x >= 0 
            && coord.y >= 0
            && coord.x < (self.height() as i32) 
            && coord.y < (self.width() as i32);
    }
}

impl Searchable for Board {
    fn find(&self, char: char) -> Option<Coord> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.cells[i][j] == char {
                    return Some(Coord::new(i as i32, j as i32));
                }
            }
        }

        return None;
    }

    fn count(&self, char: char) -> u32 {
        let mut result = 0;
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.cells[i][j] == char {
                    result += 1;
                }
            }
        }
        return result;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        return Ok(());
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        return Coord { x, y };
    }

    pub fn add(&self, vector: &Vector) -> Coord {
        return Coord { x: self.x + vector.x, y: self.y + vector.y };
    }
    
    pub fn subtract(&self, other: &Coord) -> Vector {
        return Vector { x: self.x - other.x, y: self.y - other.y };
    }
}

impl Vector {
    pub const NORTH: Vector = Vector { x: -1, y: 0 };
    pub const SOUTH: Vector = Vector { x: 1, y: 0 };
    pub const WEST: Vector = Vector { x: 0, y: -1 };
    pub const EAST: Vector = Vector { x: 0, y: 1 };
    
    pub fn new(x: i32, y: i32) -> Vector {
        return Vector { x, y };
    }

    pub fn rotate_right(&self) -> Vector {
        match self {
            &Vector::NORTH => Vector::EAST,
            &Vector::EAST => Vector::SOUTH,
            &Vector::SOUTH => Vector::WEST,
            &Vector::WEST => Vector::NORTH,
            _ => panic!("Rotating unsupported vector"),
        }
    }
}

pub fn num_digits(x: u64) -> u32 {
    let mut i = 0;
    let mut x = x;
    while x > 0 {
        x /= 10;
        i += 1;
    }
    return i;
}
