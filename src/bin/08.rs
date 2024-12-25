use std::collections::{HashMap, HashSet};
use advent_of_code::{Board, Bounded, Coord, Vector};

advent_of_code::solution!(8);

#[derive(Debug)]
struct AntennaMap(HashMap<char, Vec<Coord>>);

impl AntennaMap {
    fn new() -> AntennaMap {
        return AntennaMap(HashMap::new());
    }
    
    fn from(board: &Board) -> AntennaMap {
        let mut antennas = AntennaMap::new();

        for x in 0..board.height() {
            for y in 0..board.width() {
                let coord = Coord { x: x as i32, y: y as i32 };
                let character = board.at(&coord).unwrap();
                if character != '.' {
                    antennas.insert(character, &coord);
                }
            }
        }
        
        return antennas;
    }
    
    fn insert(&mut self, c: char, coord: &Coord) {
        match self.0.get_mut(&c) {
            Some(vec) => vec.push(coord.clone()),
            None => { self.0.insert(c, vec![coord.clone()]); },
        }
    }
    
    fn pairs(&self) -> Vec<(Coord, Coord)> {
        let mut result = Vec::new();
        
        for (_, coords) in self.0.iter() {
            for coord in coords {
                for other_coord in coords {
                    if coord != other_coord {
                        result.push((coord.clone(), other_coord.clone()));
                    }
                }
            }
        }
        
        return result;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::from(input);
    let antennas = AntennaMap::from(&board);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for pair in antennas.pairs() {
        let pair_antinodes = get_antinodes(&board, &pair);
        pair_antinodes.iter().for_each(|antinode| { antinodes.insert(antinode.clone()); });
    }

    return Some(antinodes.len() as u32);
}

fn get_antinodes(board: &Board, pair: &(Coord, Coord)) -> Vec<Coord> {
    let vector1 = Vector::new(pair.0.x - pair.1.x, pair.0.y - pair.1.y);
    let vector2 = Vector::new(pair.1.x - pair.0.x, pair.1.y - pair.0.y);
    let mut result = Vec::new();
    
    let antinode1 = pair.0.add(&vector1);
    let antinode2 = pair.1.add(&vector2);
    
    if board.is_in_bounds(&antinode1) {
        result.push(antinode1);
    }
    
    if board.is_in_bounds(&antinode2) {
        result.push(antinode2);
    }
    
    return result;
}

fn get_resonant_antinodes(board: &Board, pair: &(Coord, Coord)) -> Vec<Coord> {
    let vector1 = Vector::new(pair.0.x - pair.1.x, pair.0.y - pair.1.y);
    let vector2 = Vector::new(pair.1.x - pair.0.x, pair.1.y - pair.0.y);
    let mut result = Vec::new();
    
    let mut antinode = pair.0.clone();
    
    while board.is_in_bounds(&antinode) {
        result.push(antinode.clone());
        antinode = antinode.add(&vector1);
    }

    let mut antinode = pair.1.clone();

    while board.is_in_bounds(&antinode) {
        result.push(antinode.clone());
        antinode = antinode.add(&vector2);
    }

    return result;
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::from(input);
    let antennas = AntennaMap::from(&board);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for pair in antennas.pairs() {
        let resonant_antinodes = get_resonant_antinodes(&board, &pair);
        resonant_antinodes.iter().for_each(|antinode| { antinodes.insert(antinode.clone()); });
    }

    return Some(antinodes.len() as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
    
    #[test]
    fn test_get_antinodes() {
        let board = Board::new(vec![vec!['.', '.', '.'], vec!['.', '1', '.'], vec!['.', '.', '1']]);
        let pair = (Coord::new(1, 1), Coord::new(2, 2));
        let actual = get_antinodes(&board, &pair);
        assert_eq!(actual, vec![Coord::new(0, 0)]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
