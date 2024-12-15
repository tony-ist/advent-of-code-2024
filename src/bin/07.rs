use itertools::{Itertools, MultiProduct};
use std::slice::Iter;
use advent_of_code::num_digits;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let (results, operands_list) = extract_results_and_operands(input);

    let mut answer = 0;
    
    for (i, operands) in operands_list.iter().enumerate() {
        if is_solvable_2_op(results[i], operands) {
            answer += results[i];
        }
    }

    return Some(answer);
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

const ALL_OPERATORS: [Operator; 3] = [
    Operator::Add,
    Operator::Multiply,
    Operator::Concat,
];

struct BinStack {
    stack: u32,
}

impl BinStack {
    fn new(value: u32) -> Self {
        Self { stack: value }
    }
    
    fn pop(&mut self) -> Operator {
        let result = self.stack & 1;
        self.stack >>= 1;
        return match result {
            0 => Operator::Add,
            _ => Operator::Multiply,
        };
    }
}

fn is_solvable_2_op(result: u64, operands: &Vec<u32>) -> bool {
    let operators_len = operands.len() - 1;
    let mut op_order: u32 = 0;
    
    for _ in 0..2u32.pow(operators_len as u32) {
        let calculated = calculate_2_op(operands, op_order);
        
        if calculated == result {
            return true;
        }
        
        op_order += 1;
    }
    
    return false;
}

fn calculate_2_op(operands: &Vec<u32>, op_order: u32) -> u64 {
    let mut stack = BinStack::new(op_order);
    let mut accumulator: u64 = operands[0] as u64;

    for i in 1..operands.len() {
        let operator = stack.pop();

        match operator {
            Operator::Add => {
                accumulator += operands[i] as u64;
            },
            Operator::Multiply => {
                accumulator *= operands[i] as u64;
            },
            Operator::Concat => {
                panic!("Concat operator is not supported in this function")
            },
        };
    }
    
    return accumulator;
}

pub fn part_two(input: &str) -> Option<u64> {
    let (results, operands_list) = extract_results_and_operands(input);

    let mut answer = 0;

    for (i, operands) in operands_list.iter().enumerate() {
        if is_solvable_3_op(results[i], operands) {
            answer += results[i];
        }
    }

    return Some(answer);
}

fn extract_results_and_operands(input: &str) -> (Vec<u64>, Vec<Vec<u32>>) {
    let results: Vec<u64> = input
        .lines()
        .map(|line| line.split(':').nth(0).unwrap().parse::<u64>().unwrap())
        .collect();
    let operands_list: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    (results, operands_list)
}

fn is_solvable_3_op(result: u64, operands: &Vec<u32>) -> bool {
    let operators_len = operands.len() - 1;
    let cartesian_product = all_operator_permutations(operators_len);
    
    for permutation in cartesian_product {
        let calculated = calculate_3_op(operands, &permutation);
        
        if calculated == result {
            return true;
        }
    }
    
    return false;
}

fn all_operator_permutations<'a>(len: usize) -> MultiProduct<Iter<'a, Operator>> {
    return (0..len).map(|_| ALL_OPERATORS.iter()).multi_cartesian_product();
}

fn calculate_3_op(operands: &Vec<u32>, permutation: &Vec<&Operator>) -> u64 {
    let mut accumulator: u64 = operands[0] as u64;

    for i in 1..operands.len() {
        let operator = permutation[i - 1];
        
        match operator {
            Operator::Add => {
                accumulator += operands[i] as u64;
            },
            Operator::Multiply => {
                accumulator *= operands[i] as u64;
            },
            Operator::Concat => {
                accumulator = concat_u64(accumulator, operands[i] as u64);
            },
        };
    }

    return accumulator;
}

fn concat_u64(a: u64, b: u64) -> u64 {
    return a * 10u64.pow(num_digits(b)) + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
    
    #[test]
    fn test_calculate_simple() {
        let operands = vec![1, 2, 3, 4, 5];
        let result = calculate_2_op(&operands, 0b1010);
        assert_eq!(result, 65);
    }

    #[test]
    fn test_calculate_input_0b10() {
        let operands = vec![81, 40, 27];
        let result = calculate_2_op(&operands, 0b10);
        assert_eq!(result, 3267);
    }

    #[test]
    fn test_calculate_input_0b01() {
        let operands = vec![81, 40, 27];
        let result = calculate_2_op(&operands, 0b01);
        assert_eq!(result, 3267);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat_u64(123, 456), 123456);
    }
    
    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 0);
        assert_eq!(num_digits(5), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(55), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(100), 3);
        assert_eq!(num_digits(555), 3);
        assert_eq!(num_digits(999), 3);
    }

    #[test]
    fn test_all_op_permutations() {
        let mut actual = all_operator_permutations(2);
        assert_eq!(actual.next(), Some(vec![&Operator::Add, &Operator::Add]));
        assert_eq!(actual.next(), Some(vec![&Operator::Add, &Operator::Multiply]));
        assert_eq!(actual.next(), Some(vec![&Operator::Add, &Operator::Concat]));
        assert_eq!(actual.next(), Some(vec![&Operator::Multiply, &Operator::Add]));
        assert_eq!(actual.next(), Some(vec![&Operator::Multiply, &Operator::Multiply]));
        assert_eq!(actual.next(), Some(vec![&Operator::Multiply, &Operator::Concat]));
        assert_eq!(actual.next(), Some(vec![&Operator::Concat, &Operator::Add]));
        assert_eq!(actual.next(), Some(vec![&Operator::Concat, &Operator::Multiply]));
        assert_eq!(actual.next(), Some(vec![&Operator::Concat, &Operator::Concat]));

        assert_eq!(actual.next(), None);
    }
}
