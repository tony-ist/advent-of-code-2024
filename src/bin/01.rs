use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let nested_list: Vec<Vec<&str>> = input
        .split('\n')
        .map(|pair| pair.split("   ").collect::<Vec<&str>>())
        .collect()
    ;
    let mut left_list: Vec<u32> = nested_list
        .iter()
        .map(|l| l[0].parse::<u32>().unwrap())
        .collect()
    ;
    let mut right_list: Vec<u32> = nested_list
        .iter()
        .map(|l| l[1].parse::<u32>().unwrap())
        .collect()
    ;
    
    left_list.sort();
    right_list.sort();
    
    let mut result = 0u32;
    
    for i in 0..left_list.len() {
        result += left_list[i].abs_diff(right_list[i]); 
    }
    
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let nested_list: Vec<Vec<&str>> = input
        .split('\n')
        .map(|pair| pair.split("   ").collect::<Vec<&str>>())
        .collect()
        ;
    let left_list: Vec<u32> = nested_list
        .iter()
        .map(|l| l[0].parse::<u32>().unwrap())
        .collect()
        ;
    let right_list: Vec<u32> = nested_list
        .iter()
        .map(|l| l[1].parse::<u32>().unwrap())
        .collect()
        ;

    let mut left_freq: HashMap<u32, u32> = HashMap::new();
    let mut right_freq: HashMap<u32, u32> = HashMap::new();

    left_list.iter().for_each(|x| { left_freq.insert(*x, 0); } );
    right_list.iter().for_each(|x| { right_freq.insert(*x, 0); } );
    
    left_list.iter().for_each(|x| *left_freq.get_mut(x).unwrap() += 1 );
    right_list.iter().for_each(|x| *right_freq.get_mut(x).unwrap() += 1 );
    
    let mut result = 0u32;

    for i in 0..left_list.len() {
        let current_x = left_list[i];
        let freq = right_freq.get(&current_x).unwrap_or(&0u32);
        result += freq * current_x;
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
