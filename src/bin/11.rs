use advent_of_code::num_digits;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(' ').map(|stone_str| stone_str.parse().unwrap()).collect();
    return Some(blink(&stones, 25));
}

fn blink_recursively(stone: u64, times: u64) -> u64 {
    if times == 0 {
        return 1;
    }

    return blink_recursively(stone, times - 1) + blink_recursively(stone, times - 1);

    if stone == 0 {
        return vec![1];
    }

    return match is_num_digits_even(stone) {
        true => split(stone),
        false => vec![stone * 2024],
    }
}

fn blink(stones: &Vec<u64>, times: u64) -> u64 {
    let mut stones = stones.clone();
    for i in 0..times {
        println!("Iterated {} times. Stones len is {}", i+1, stones.len());
        stones = stones.iter().map(|stone| blink_once(*stone)).flatten().collect();
        // print_u64_slice(&stones[..min(stones.len(), 15)]);
        // print_u64_slice(&stones);
    }
    return stones.len() as u64;
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
        assert_eq!(result, None);
    }
}
