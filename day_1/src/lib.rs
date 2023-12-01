/// Grab the first and last digit from each line and sum each line's value
pub fn get_total_calibration(input: &mut aoc2023::LinesIter) -> u16 {
    input.map(|line| get_line_calibration(&line.unwrap()))
        .sum()
}

/// Like get_total_calibration, but converts english numbers into digits first
pub fn get_total_calibration_2(input: &mut aoc2023::LinesIter) -> u16 {
    input.map(|line| {
        let line = eng_to_char(&line.unwrap());
        get_line_calibration(&line)
    }).sum()
}

/// Grab the first and last digit of each line and return as a number
fn get_line_calibration(line: &str) -> u16 {
    let digits = line.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();
    let first_num = *digits.first()
        .expect("There should be at least one element") as u8;
    let last_num = *digits.last()
        .expect("There should be at least one element") as u8;
    let calibration = String::from_utf8(vec![first_num, last_num]).unwrap();
    calibration.parse().unwrap()
}

const ENG_CHAR_MAP: [(&str, char);9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

/// Convert all English numbers into integer characters
fn eng_to_char(line: &str) -> String {
    let mut digits: Vec<Option<char>> = Vec::with_capacity(line.len());
    digits.resize(line.len(), None);

    for mapping in ENG_CHAR_MAP {
        // Search for english numbers and save to the digits vector
        for english_num in FindIter::find_iter(line, mapping.0) {
            digits[english_num] = Some(mapping.1);
        }
        
        // Search for integers and save to the digits vector
        for num in FindIter::find_iter(line, &String::from(mapping.1)) {
            digits[num] = Some(mapping.1);
        }
    }

    let digits = digits.into_iter()
        .filter_map(|c| c);
    String::from_iter(digits)
}

struct FindIter<'a, 'b> {
    slice: &'a str,
    pattern: &'b str,
    pointer_idx: usize
}

impl<'a, 'b> Iterator for FindIter<'a, 'b> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.slice[self.pointer_idx..]
            .find(self.pattern) {
            Some(idx) => {
                let match_idx = self.pointer_idx + idx;
                self.pointer_idx = match_idx + 1;
                Some(match_idx)
            }
            None => None,
        }
    }
}

impl<'a, 'b> FindIter<'a, 'b> {
    pub fn find_iter(slice: &'a str, pattern: &'b str) -> Self {
        Self {
            slice,
            pattern,
            pointer_idx: 0
        }
    }
}

#[cfg(test)]
mod day_1 {
    use super::*;

    #[test]
    fn part_1() {
        let mut input = aoc2023::read_as_lines("../inputs/day_1/mock");
        let part_1 = get_total_calibration(&mut input);
        assert_eq!(part_1, 142);
    }

    #[test]
    fn find_iter() {
        let input = "aaaXXaXXXaaXXXaa";
        let finds = FindIter::find_iter(input, "aa")
            .collect::<Vec<usize>>();
        assert_eq!(finds, vec![0,1,9,14]);
    }

    #[test]
    fn mapping() {
        let input = "two3four5";
        let output = eng_to_char(input);
        assert_eq!(output, String::from("2345"));
    }

    #[test]
    fn part_2() {
        let mut input = aoc2023::read_as_lines("../inputs/day_1/mock_2");
        let part_2 = get_total_calibration_2(&mut input);
        assert_eq!(part_2, 281);
    }
}