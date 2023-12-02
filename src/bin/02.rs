advent_of_code::solution!(2);

struct Counts {
    red: u32,
    green: u32,
    blue: u32,
}

impl Counts {
    fn lt(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let base = Counts {
        red: 12,
        green: 13,
        blue: 14,
    };
    Some(
        input
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .enumerate()
            .map(|(i, line)| {
                if line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(";")
                    .map(|cubes| {
                        let mut counts = Counts {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        for cube in cubes.split(", ") {
                            let mut parts = cube.trim().split(" ");
                            let count = parts.next().unwrap().parse::<u32>().unwrap();
                            let color = parts.next().unwrap();
                            match color {
                                "red" => counts.red = count,
                                "green" => counts.green = count,
                                "blue" => counts.blue = count,
                                _ => panic!("Unknown color: {}", color),
                            }
                        }

                        counts
                    })
                    .all(|counts| counts.lt(&base))
                {
                    (i + 1) as u32
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| {
                if line.trim() == "" {
                    return 0;
                }
                let mut max = Counts {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                let counts: Vec<_> = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(";")
                    .map(|cubes| {
                        let mut counts = Counts {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        for cube in cubes.split(", ") {
                            let mut parts = cube.trim().split(" ");
                            let count = parts.next().unwrap().parse::<u32>().unwrap();
                            let color = parts.next().unwrap();
                            match color {
                                "red" => counts.red = count,
                                "green" => counts.green = count,
                                "blue" => counts.blue = count,
                                _ => panic!("Unknown color: {}", color),
                            }
                        }

                        counts
                    })
                    .collect();
                for counts in counts {
                    if counts.red > max.red {
                        max.red = counts.red;
                    }
                    if counts.green > max.green {
                        max.green = counts.green;
                    }
                    if counts.blue > max.blue {
                        max.blue = counts.blue;
                    }
                }

                max.red * max.green * max.blue
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
