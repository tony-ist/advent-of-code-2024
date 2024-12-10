use std::mem::swap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Vec<u32>> = input_parts[0]
        .split('\n')
        .map(|line| line
            .split('|')
            .map(|num_str| num_str.parse().unwrap())
            .collect())
        .collect();

    let updates: Vec<Vec<u32>> = input_parts[1]
        .split('\n')
        .map(|line| line
            .split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect())
        .collect();

    let mut result = 0;

    for update in updates {
        if is_ordered_correctly(&update, &rules) {
            result += middle(&update);
        }
    }

    return Some(result);
}

fn middle(x: &Vec<u32>) -> u32 {
    return x[x.len()/2];
}

fn is_ordered_correctly(update: &Vec<u32>, rules: &Vec<Vec<u32>>) -> bool {
    return get_violated_indices(update, rules).is_none();
}

fn adheres_to_rules(lhs: u32, rhs: u32, rules: &Vec<Vec<u32>>) -> bool {
    for rule in rules {
        if rule[0] == rhs && rule[1] == lhs {
            return false;
        }
    }
    
    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Vec<u32>> = input_parts[0]
        .split('\n')
        .map(|line| line
            .split('|')
            .map(|num_str| num_str.parse().unwrap())
            .collect())
        .collect();

    let updates: Vec<Vec<u32>> = input_parts[1]
        .split('\n')
        .map(|line| line
            .split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect())
        .collect();

    dbg!(&updates);
    let mut result = 0;

    for update in updates {
        dbg!(&update);
        
        let maybe_middle = get_middle_of_fixed_update(&update, &rules);
        
        dbg!(&maybe_middle);
        
        if let Some(middle) = maybe_middle {
            result += middle;
        }
    }

    return Some(result);
}

/// Returns `None` if update is correct, otherwise fixes it by reordering 
/// and returns `Some` with middle element.
/// Panics if cannot fix the update.
fn get_middle_of_fixed_update(update: &Vec<u32>, rules: &Vec<Vec<u32>>) -> Option<u32> {
    if is_ordered_correctly(update, rules) {
        return None;
    }
    
    let mut update_clone = update.clone();
    
    for i in 0..(update_clone.len() - 1) {
        for j in (i + 1)..update_clone.len() {
            update_clone.swap(i, j);
            
            if (i == 1 && j == 4) {
                println!("{:?}", update_clone);
            }
            
            if is_ordered_correctly(&update_clone, rules) {
                return Some(middle(&update_clone));
            } 
            
            update_clone.swap(i, j);
        }
    }
    
    panic!("Correct modification of update was not found")
}

fn get_violated_indices(update: &Vec<u32>, rules: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            if !adheres_to_rules(update[i], update[j], rules) {
                return Some((i, j));
            }
        }
    }
    
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
    
    #[test]
    fn test_get_middle_of_fixed_update() {
        let rules = vec![
            vec![47, 53],
            vec![97, 13],
            vec![97, 61],
            vec![97, 47],
            vec![75, 29],
            vec![61, 13],
            vec![75, 53],
            vec![29, 13],
            vec![97, 29],
            vec![53, 29],
            vec![61, 53],
            vec![97, 53],
            vec![61, 29],
            vec![47, 13],
            vec![75, 47],
            vec![97, 75],
            vec![47, 61],
            vec![75, 61],
            vec![47, 29],
            vec![75, 13],
            vec![53, 13],
        ];
        
        let update = vec![97,13,75,29,47];
        let actual = get_middle_of_fixed_update(&update, &rules);
        
        assert_eq!(actual, Some(47));
    }
}
