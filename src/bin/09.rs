use std::char::from_digit;
use std::fmt::{Display, Formatter, Write};

advent_of_code::solution!(9);

struct CyclicCounter(u8);

impl CyclicCounter {
    fn new() -> CyclicCounter {
        return CyclicCounter(0);
    }
    
    fn next(&mut self) -> u8 {
        self.0 += 1;
        
        if self.0 > 9 {
            self.0 = 0;
        }
        
        return self.0;
    }
    
    fn value(&self) -> u8 {
        return self.0;
    }
}

struct Drive(Vec<Option<u32>>);

impl Drive {
    fn from(disk_map: &Vec<u8>) -> Drive {
        let drive_len = disk_map.iter().fold(0, |acc, &x| acc + x as u32);
        let mut drive = Drive(Vec::with_capacity(drive_len as usize));
        let mut is_inserting_files = true;
        let mut counter: u32 = 0;

        for &x in disk_map {
            for _ in 0..x {
                if is_inserting_files {
                    drive.0.push(Some(counter));
                } else {
                    drive.0.push(None);
                }
            }
            if is_inserting_files {
                counter += 1;
            }
            is_inserting_files = !is_inserting_files;
        }

        return drive;
    }

    fn defragment(&mut self) {
        let mut left = 0;
        let mut right = self.0.len() - 1;
        
        while left < right {
            if self.0[left].is_some() {
                left += 1;
                continue;
            }
            
            if self.0[right].is_none() {
                right -= 1;
                continue
            }
            
            self.0.swap(left, right);
        }
    }

    fn checksum(&self) -> u64 {
        let mut result = 0;
        for (i, &maybe_id) in self.0.iter().enumerate() {
            if let Some(id) = maybe_id {
                result += (i as u64) * (id as u64);
            }
        }
        return result;
    }
}

impl Display for Drive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &x in self.0.iter() {
            let c = match x {
                None => '.',
                Some(number) => from_digit(number as u32, 10).unwrap(),
            };
            f.write_char(c)?;
        }
        return Ok(());
    }
}

fn read_disk_map(input: &str) -> Vec<u8> {
    return input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = read_disk_map(input);
    let mut drive = Drive::from(&disk_map);
    drive.defragment();
    let result = drive.checksum();
    return Some(result);
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
        assert_eq!(result, Some(1928));
    }
    
    #[test]
    fn test_fmt_drive() {
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        let drive = Drive::from(&read_disk_map("2333133121414131402"));
        let actual = format!("{}", drive);
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_defragment_drive() {
        let expected = "0099811188827773336446555566..............";
        let mut drive = Drive::from(&read_disk_map("2333133121414131402"));
        drive.defragment();
        let actual = format!("{}", drive);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
