use std::collections::HashSet;
use advent_of_code::{Addressable, Board, Bounded, Coord, Searchable, Vector};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board::new(input.lines().map(|line| line.chars().collect()).collect());
    let initial_coord = board.find('^').unwrap();
    
    board.mutate(&initial_coord, '.');
    trace_board(&mut board, &initial_coord, &Vector::UP);
    
    println!("{}", &board);
    
    return Some(board.count('o'));
}

fn make_step(board: &mut Board, guard_coord: &mut Coord, direction: &mut Vector) {
    board.mutate(&guard_coord, 'o');
    
    let next_coord = guard_coord.add(&direction);
    
    match board.at(&next_coord) {
        Some(x) => {
            match x {
                '.' | 'o' => {
                    *guard_coord = next_coord;
                },
                '#' => {
                    *direction = direction.rotate_right();
                },
                _ => panic!("Invalid character: {}", x.to_string()),
            }
        }
        None => {
            *guard_coord = next_coord;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut empty_board = Board::new(input.lines().map(|line| line.chars().collect()).collect());
    let initial_coord = empty_board.find('^').unwrap();
    empty_board.mutate(&initial_coord, '.');
    
    let mut direction = Vector::UP;
    let mut result = 0;
    let mut traced_board = empty_board.clone();
    
    trace_board(&mut traced_board, &initial_coord, &mut direction);
    
    for i in 0..traced_board.height() {
        for j in 0..traced_board.width() {
            if traced_board.at(&Coord::new(i as i32, j as i32)).unwrap() != 'o' {
                continue;
            }

            let mut mutated_board = empty_board.clone();

            mutated_board.mutate(&Coord::new(i as i32, j as i32), '#');
            
            if has_loop(&mutated_board, &initial_coord, &Vector::UP) {
                result += 1;
            }
        }
    }
    
    return Some(result);
}

fn trace_board(board: &mut Board, coord: &Coord, direction: &Vector) {
    let mut direction = direction.clone();
    let mut coord = coord.clone();
    
    while board.is_in_bounds(&coord) {
        make_step(board, &mut coord, &mut direction);
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct DirectedCoord {
    coord: Coord,
    direction: Vector,
} 

fn has_loop(board: &Board, coord: &Coord, direction: &Vector) -> bool {
    let mut set: HashSet<DirectedCoord> = HashSet::new();
    
    let mut current_board = board.clone();
    let mut current_coord = coord.clone();
    let mut current_direction = direction.clone();
    
    while board.is_in_bounds(&current_coord) {
        set.insert(DirectedCoord{ coord: current_coord.clone(), direction: current_direction.clone()});
        
        make_step(&mut current_board, &mut current_coord, &mut current_direction);
        
        let next_directed_coord = DirectedCoord{ coord: current_coord.clone(), direction: current_direction.clone()};
        
        if set.contains(&next_directed_coord) {
            return true;
        }
    }
    
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    
    #[test]
    fn test_has_loop_1_1() {
        let board = Board::new(vec![vec!['^']]);
        let actual = has_loop(&board, &Coord::new(0, 0), &Vector::UP);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_has_loop_3_3() {
        let board = Board::new(vec![
            vec!['.','#','.'],
            vec!['#','^','#'],
            vec!['.','#','.'],
        ]);
        let actual = has_loop(&board, &Coord::new(1, 1), &Vector::UP);
        assert_eq!(actual, true);
    }
}
