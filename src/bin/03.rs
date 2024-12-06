use regex::Regex;

advent_of_code::solution!(3);

fn simple_multiply(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures_iter = re.captures_iter(input);
    let mut sum: u32 = 0;
    for capture in captures_iter {
        let mul = capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap();
        sum += mul;
    }
    return sum;
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(simple_multiply(input));
}



pub fn part_two(input: &str) -> Option<u32> {
    let re_newline = Regex::new(r"\n").unwrap();
    let cow_newline = re_newline.replace_all(input, "");
    
    dbg!(&input);
    
    let re_middle = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let cow_middle = re_middle.replace_all(&cow_newline, "");

    dbg!(&cow_middle);
    
    let re_end = Regex::new(r"don't\(\).*$").unwrap();
    let cow_end = re_end.replace_all(&cow_middle, "");
    
    dbg!(&cow_end);
    
    return Some(simple_multiply(&cow_end));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_part_two_donts() {
        let actual = part_two("mul(1,1)don't()mul(1,2)do()don't()mul(1,3)");
        assert_eq!(actual, Some(1));
    }

    #[test]
    fn test_part_two_only_donts() {
        let actual = part_two("don't()mul(1,1)don't()mul(1,2)don't()mul(1,3)");
        assert_eq!(actual, Some(0));
    }

    #[test]
    fn test_part_two_many_dos() {
        let actual = part_two("do()mul(1,1)don't()mul(1,2)do()mul(1,4)don't()mul(1,3)do()mul(1,5)");
        assert_eq!(actual, Some(10));
    }

    #[test]
    fn test_part_two_next_do_dont() {
        let actual = part_two("do()don't()do()mul(1,1)don't()mul(1,2)do()don't()mul(1,4)don't()mul(1,3)do()mul(1,5)");
        assert_eq!(actual, Some(6));
    }

    #[test]
    fn test_part_two_multiline() {
        let actual = part_two("do()don't()do()mul(1,1)don't()mul(1,2)\ndo()don't()mul(1,4)don't()mul(1,3)mul(1,5)");
        assert_eq!(actual, Some(1));
    }
}
