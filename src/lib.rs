pub mod template;

pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub struct Vector {
    pub x: i32,
    pub y: i32,
}

pub type Crossword = Vec<Vec<char>>;

pub trait Addressable {
    fn at(&self, coord: &Coord) -> Option<char>;
}

pub trait Bounded {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_in_bounds(&self, coord: &Coord) -> bool;
}

impl Addressable for Crossword {
    fn at(&self, coord: &Coord) -> Option<char> {
        if self.is_in_bounds(coord) {
            return Some(self[coord.x as usize][coord.y as usize]);
        };
        
        return None;
    }
}

impl Bounded for Crossword {
    fn width(&self) -> usize {
        if self.len() == 0 {
            return 0;
        }
        
        return self[0].len();
    }

    fn height(&self) -> usize {
        return self.len();
    }
    
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        return coord.x >= 0 
            && coord.y >= 0
            && coord.x < (self.height() as i32) 
            && coord.y < (self.width() as i32);
    }
}

pub trait Arithmetical {
    fn add(&self, vector: &Vector) -> Coord;
}

impl Arithmetical for Coord {
    fn add(&self, vector: &Vector) -> Coord {
        return Coord { x: self.x + vector.x, y: self.y + vector.y };
    }
}
