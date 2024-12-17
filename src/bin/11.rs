use std::collections::HashMap;
use advent_of_code::num_digits;

advent_of_code::solution!(11);

type Memo = HashMap<(u64, u64), u64>;

fn memoize(memo: &mut Memo, stone: u64, times: u64, result: u64) {
    memo.insert((stone, times), result);
}

fn read_memo(memo: &Memo, stone: u64, times: u64) -> Option<&u64> {
    return memo.get(&(stone, times));
} 

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(' ').map(|stone_str| stone_str.parse().unwrap()).collect();
    return Some(blink(&stones, 25));
}

fn blink_recursively(stone: u64, times: u64, memo: &mut Memo) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(memo_result) = read_memo(memo, stone, times) {
        return *memo_result;
    }
    
    let blink_result = blink_once(stone);
    
    let answer = match blink_result.len() {
        1 => blink_recursively(blink_result[0], times - 1, memo),
        2 => blink_recursively(blink_result[0], times - 1, memo) 
            + blink_recursively(blink_result[1], times - 1, memo),
        _ => panic!("Blink result has length {}", blink_result.len()),
    };
    
    memoize(memo, stone, times, answer);
    
    return answer;
}

fn blink(stones: &Vec<u64>, times: u64) -> u64 {
    let mut memo = Memo::new();
    return stones.iter().map(|stone| blink_recursively(*stone, times, &mut memo)).sum();
}

fn print_u64_slice(vec: &[u64]) {
    for element in vec {
        print!("{}, ", element);
    }
    println!();
}

fn blink_once(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    
    return match is_num_digits_even(stone) {
        true => split(stone),
        false => vec![stone * 2024],
    }
}

fn split(number: u64) -> Vec<u64> {
    let num_digits = num_digits(number);
    let divider = 10u64.pow(num_digits / 2);
    return vec![number / divider, number % divider];
}

fn is_num_digits_even(number: u64) -> bool {
    return num_digits(number) % 2 == 0;
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(' ').map(|stone_str| stone_str.parse().unwrap()).collect();
    return Some(blink(&stones, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
    
    #[test]
    fn test_blink_1() {
        test_blink(&vec![0, 2, 12], 1, 4);
    }

    #[test]
    fn test_blink_2() {
        test_blink(&vec![0, 2, 12], 2, 5);
    }

    #[test]
    fn test_blink_n_10() {
        for i in 0..10 {
            println!("Blinking for stone {}", i);
            blink(&vec![i], 10);
        }
    }
    
    fn test_blink(stones: &Vec<u64>, times: u64, expected: u64) {
        assert_eq!(blink(stones, times), expected);
    }
    
    #[test]
    fn test_split() {
        assert_eq!(split(12), vec![1, 2]);
        assert_eq!(split(1234), vec![12, 34]);
        assert_eq!(split(12345678), vec![1234, 5678]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
