use std::char::from_digit;
use std::fmt::{Display, Formatter, Write};

advent_of_code::solution!(9);

struct Drive(Vec<Option<u32>>);

#[derive(PartialEq, Debug)]
struct File {
    index: usize,
    len: usize,
    id: u32,
}

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

    fn defragment_whole_files(&mut self) {
        let mut maybe_rightmost_file = self.search_rightmost_file(self.0.len() - 1);
        while let Some(rightmost_file) = maybe_rightmost_file {
            if rightmost_file.index == 0 {
                return;
            }
            
            let maybe_space_index = self.space_index(rightmost_file.len, rightmost_file.index);
            if let Some(space_index) = maybe_space_index {
                self.move_file(&rightmost_file, space_index);
            }
            maybe_rightmost_file = self.search_rightmost_file(rightmost_file.index - 1);
        }
    }
    
    fn search_rightmost_file(&self, index: usize) -> Option<File> {
        let maybe_rightmost_file_index = self.rightmost_file_index(index);
        
        if let Some(rightmost_file_index) = maybe_rightmost_file_index {
            let rightmost_file_length = self.file_length(rightmost_file_index);
            let rightmost_file_id = self.0[rightmost_file_index].unwrap();
            return Some(File::new(rightmost_file_index, rightmost_file_length, rightmost_file_id));
        }
        
        return None;
    }
    
    fn space_index(&self, size: usize, border_index: usize) -> Option<usize> {
        let mut i = 0;

        while i < self.0.len() && i < border_index {
            while self.0[i].is_some() {
                i += 1;
            }
            
            let space_start = i;
            
            while i < self.0.len() && i < border_index  && self.0[i].is_none() {
                i += 1;
            }

            if i - space_start >= size {
                return Some(space_start);
            }
        }
        
        return None;
    }

    fn move_file(&mut self, file: &File, space_index: usize) {
        for i in 0..file.len {
            self.0[space_index + i] = self.0[file.index + i];
            self.0[file.index + i] = None;
        }
    }

    fn max_file_index(&self) -> usize {
        for i in (0..self.0.len()).rev() {
            if self.0[i].is_some() {
                return i;
            }
        }
        return 0;
    }
    
    fn max_file_id(&self) -> u32 {
        let mut max_id = 0;
        for &maybe_id in self.0.iter() {
            if let Some(id) = maybe_id {
                max_id = id.max(max_id);
            }
        }
        return max_id;
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

    fn rightmost_file_index(&self, index: usize) -> Option<usize> {
        let right_file_side = self.skip_space(index)?;
        
        if right_file_side == 0 {
            return Some(0);
        }
        
        let mut left_index = right_file_side - 1;
        let mut right_index = right_file_side;
        
        while self.0[left_index] == self.0[right_index] {
            if left_index == 0 {
                return Some(0);
            }
            
            left_index -= 1;
            right_index -= 1;
        }
        
        return Some(right_index);
    }

    fn file_length(&self, index: usize) -> usize {
        if self.0[index].is_none() {
            return 0;
        }
        
        let mut i = index;
        while i < self.0.len() && self.0[i] == Some(self.0[index].unwrap()) {
            i += 1;
        }
        return i - index;
    }

    fn skip_space(&self, index: usize) -> Option<usize> {
        let mut i = index.min(self.0.len() - 1);
        while i > 0 && self.0[i].is_none() {
            i -= 1;
        }
        return match self.0[i] {
            Some(_) => Some(i),
            None => None,
        };
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

impl File {
    fn new(index: usize, len: usize, id: u32) -> File {
        return File { index, len, id };
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

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = read_disk_map(input);
    let mut drive = Drive::from(&disk_map);
    drive.defragment_whole_files();
    let result = drive.checksum();
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_search_rightmost_file_1() {
        test_search_rightmost_file("11", 1, &Some(File::new(0, 1, 0)));
    }

    #[test]
    fn test_search_rightmost_file_2() {
        test_search_rightmost_file("01", 1, &None);
    }

    #[test]
    fn test_search_rightmost_file_3() {
        test_search_rightmost_file("2222", 7, &Some(File::new(4, 2, 1)));
    }

    #[test]
    fn test_search_rightmost_file_4() {
        test_search_rightmost_file("222", 7, &Some(File::new(4, 2, 1)));
    }

    fn test_search_rightmost_file(disk_map: &str, index: usize, expected: &Option<File>) {
        let drive = Drive::from(&read_disk_map(disk_map));
        let actual = drive.search_rightmost_file(index);
        assert_eq!(&actual, expected);
    }
    
    #[test]
    fn test_rightmost_file_index_1() {
        let drive = Drive::from(&read_disk_map("1"));
        let actual = drive.rightmost_file_index(0);
        assert_eq!(actual, Some(0));
    }

    #[test]
    fn test_rightmost_file_index_2() {
        let drive = Drive::from(&read_disk_map("1111")); // 0.1.
        let actual = drive.rightmost_file_index(3);
        assert_eq!(actual, Some(2));
    }

    #[test]
    fn test_rightmost_file_index_3() {
        let drive = Drive::from(&read_disk_map("1111")); // 0.1.
        let actual = drive.rightmost_file_index(10);
        assert_eq!(actual, Some(2));
    }

    #[test]
    fn test_rightmost_file_index_4() {
        let drive = Drive::from(&read_disk_map("01"));
        let actual = drive.rightmost_file_index(10);
        assert_eq!(actual, None);
    }

    #[test]
    fn test_rightmost_file_index_5() {
        let drive = Drive::from(&read_disk_map("2"));
        let actual = drive.rightmost_file_index(1);
        assert_eq!(actual, Some(0));
    }

    #[test]
    fn test_skip_space() {
        let drive = Drive::from(&read_disk_map("11"));
        let actual = drive.skip_space(1);
        assert_eq!(actual, Some(0));
    }

    #[test]
    fn test_file_length_1() {
        let drive = Drive::from(&read_disk_map("21"));
        let actual = drive.file_length(0);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_file_length_2() {
        let drive = Drive::from(&read_disk_map("2"));
        let actual = drive.file_length(0);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_file_length_none() {
        let drive = Drive::from(&read_disk_map("02"));
        let actual = drive.file_length(0);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_space_index_1() {
        let drive = Drive::from(&read_disk_map("1122")); // 0.11..
        let actual = drive.space_index(2, 10);
        assert_eq!(actual, Some(4));
    }

    #[test]
    fn test_space_index_2() {
        let drive = Drive::from(&read_disk_map("1122")); // 0.11..
        let actual = drive.space_index(1, 10);
        assert_eq!(actual, Some(1));
    }

    #[test]
    fn test_space_index_3() {
        let drive = Drive::from(&read_disk_map("112233")); // 0.11..
        let actual = drive.space_index(2, 10);
        assert_eq!(actual, Some(4));
    }

    #[test]
    fn test_space_index_border() {
        let drive = Drive::from(&read_disk_map("1122")); // 0.11..
        let actual = drive.space_index(2, 4);
        assert_eq!(actual, None);
    }
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
