advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut first_digit = ' ';
                let mut last_digit = ' ';

                for digit in line.chars() {
                    if digit.is_numeric() {
                        if first_digit == ' ' {
                            first_digit = digit;
                        }
                        last_digit = digit;
                    }
                }
                let number = format!("{}{}", first_digit, last_digit);
                number.parse::<u32>().unwrap()
            })
            .sum(),
    )
}

const NUMBER_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(|line| {
            let mut first_digit: u32 = 0;
            let mut last_digit: u32 = 0;

            let mut set_digit = |digit: u32| {
                if first_digit == 0 {
                    first_digit = digit;
                }
                last_digit = digit;
            };

            for i in 0..line.len() {
                let c = &line[i..i + 1];
                match c {
                    "0" => set_digit(0),
                    "1" => set_digit(1),
                    "2" => set_digit(2),
                    "3" => set_digit(3),
                    "4" => set_digit(4),
                    "5" => set_digit(5),
                    "6" => set_digit(6),
                    "7" => set_digit(7),
                    "8" => set_digit(8),
                    "9" => set_digit(9),
                    _ => (),
                }
                for word in NUMBER_NAMES.iter() {
                    if line[i..].starts_with(word) {
                        match *word {
                            "one" => set_digit(1),
                            "two" => set_digit(2),
                            "three" => set_digit(3),
                            "four" => set_digit(4),
                            "five" => set_digit(5),
                            "six" => set_digit(6),
                            "seven" => set_digit(7),
                            "eight" => set_digit(8),
                            "nine" => set_digit(9),
                            _ => (),
                        }
                    }
                }
            }

            (first_digit * 10) + last_digit
        })
        .sum();
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
