advent_of_code::solution!(2);

fn is_safe(input: &Vec<u32>) -> bool {
    if [0usize, 1usize].contains(&input.len()) {
        return true;
    }

    let is_ascending = input[0] < input[1];
    let is_descending = !is_ascending;

    if is_ascending {
        for i in 1..input.len() {
            let delta = (input[i] as i32) - (input[i-1] as i32);
            if !is_safe_delta(delta) {
                return false;
            }
        }
    }

    if is_descending {
        for i in 1..input.len() {
            let delta = (input[i-1] as i32) - (input[i] as i32);
            if !is_safe_delta(delta) {
                return false;
            }
        }
    }


    return true;
}

fn is_safe_delta(delta: i32) -> bool {
    return 1 <= delta && delta <= 3;
}

fn is_tolerable(input: &Vec<u32>) -> bool {
    for i in 0..input.len() {
        let mut sublist = input.clone();
        sublist.remove(i);
        if is_safe(&sublist) {
            return true;
        }
    }
    
    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let nested_list: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| line
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect())
        .collect()
        ;

    let result = nested_list.iter().map(|x| match is_safe(x) {
        true => 1,
        false => 0,
    }).sum();

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let nested_list: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| line
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect())
        .collect()
        ;

    let result = nested_list.iter().map(|x| match is_tolerable(x) {
        true => 1,
        false => 0,
    }).sum();

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
