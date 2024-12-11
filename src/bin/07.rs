advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
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

    let mut answer = 0;
    
    for (i, operands) in operands_list.iter().enumerate() {
        if is_solvable(results[i], operands) {
            answer += results[i];
        }
    }
    
    return Some(answer);
}

enum Operator {
    Add,
    Multiply,
}

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

fn is_solvable(result: u64, operands: &Vec<u32>) -> bool {
    let operators_len = operands.len() - 1;
    let mut op_order: u32 = 0;
    
    for _ in 0..2u32.pow(operators_len as u32) {
        let calculated = calculate(operands, op_order);
        
        if calculated == result {
            return true;
        }
        
        op_order += 1;
    }
    
    return false;
}

fn calculate(operands: &Vec<u32>, op_order: u32) -> u64 {
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
        };
    }
    
    return accumulator;
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
        let result = calculate(&operands, 0b1010);
        assert_eq!(result, 65);
    }

    #[test]
    fn test_calculate_input_0b10() {
        let operands = vec![81, 40, 27];
        let result = calculate(&operands, 0b10);
        assert_eq!(result, 3267);
    }

    #[test]
    fn test_calculate_input_0b01() {
        let operands = vec![81, 40, 27];
        let result = calculate(&operands, 0b01);
        assert_eq!(result, 3267);
    }
}
