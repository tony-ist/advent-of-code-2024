advent_of_code::solution!(4);

#[derive()]
struct Coord {
    x: usize,
    y: usize,
}

type Vector = Coord;

pub fn part_one(input: &str) -> Option<u32> {
    let crossword: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    
    let height = crossword.len();
    let width = crossword[0].len();
    let mut count = 0;
    
    for i in 0..height {
        for j in 0..width {
            if crossword[i][j] != 'X' {
                continue;
            }
            
            count += count_xmas(&crossword, Coord{ x: i, y: j });
        }
    }
    
    return Some(count);
}

fn count_xmas(crossword: &Vec<Vec<char>>, coord: Coord) -> u32 {
    
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
