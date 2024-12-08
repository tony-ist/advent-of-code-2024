use advent_of_code::{Addressable, Arithmetical, Coord, Crossword, Vector};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let crossword: Crossword = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let height = crossword.len();
    let width = crossword[0].len();
    let mut count = 0;

    for i in 0..height {
        for j in 0..width {
            count += count_xmas(&crossword, &Coord{ x: i as i32, y: j as i32 });
        }
    }

    return Some(count);
}

fn count_xmas(crossword: &Crossword, coord: &Coord) -> u32 {
    let directions = [
      Vector { x: -1, y: -1 },  
      Vector { x: -1, y: 0 },  
      Vector { x: -1, y: 1 },  
      Vector { x: 0, y: -1 },  
      Vector { x: 0, y: 1 },  
      Vector { x: 1, y: -1 },  
      Vector { x: 1, y: 0 },  
      Vector { x: 1, y: 1 },  
    ];
    
    let result = directions
        .map(|dir| check_direction(crossword, coord, &dir))
        .map(|has_xmas| if has_xmas { 1 } else { 0 })
        .iter().sum();

    return result;
}

fn check_direction(crossword: &Crossword, coord: &Coord, direction: &Vector) -> bool {
    if crossword.at(coord) != Some('X') {
        return false;
    }
    
    let m_coord = coord.add(&direction);
    if crossword.at(&m_coord) != Some('M') {
        return false;
    }
    
    let a_coord = m_coord.add(&direction);
    if crossword.at(&a_coord) != Some('A') {
        return false;
    }
    
    let s_coord = a_coord.add(&direction);
    if crossword.at(&s_coord) != Some('S') {
        return false;
    }
    
    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
