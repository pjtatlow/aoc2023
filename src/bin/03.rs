use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = vec![];
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        });
    let mut numbers: HashSet<(usize, usize, usize, u32)> = HashSet::new();
    let mut total = 0;

    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            let c = row[j];
            if c.is_digit(10) {
                let mut left = j;
                while left > 0 && row[left - 1].is_digit(10) {
                    left -= 1;
                }
                let mut right = j;
                while right < row.len() - 1 && row[right + 1].is_digit(10) {
                    right += 1;
                }

                let s: String = row[left..=right].iter().collect();
                let n: u32 = s.parse().unwrap();
                let key = (i, left, right, n);
                if numbers.contains(&key) {
                    continue;
                }
                numbers.insert(key);

                let mut positions = vec![];
                let left_boundary = if left > 0 { left - 1 } else { 0 };
                for x in left_boundary..=right + 1 {
                    if x >= row.len() {
                        continue;
                    }
                    if i > 0 {
                        positions.push((i - 1, x));
                    }
                    if i < grid.len() - 1 {
                        positions.push((i + 1, x));
                    }
                    if x < left || x > right {
                        positions.push((i, x));
                    }
                }

                for (y, x) in positions {
                    let c = grid[y][x];
                    if !(c.is_digit(10) || c == '.') {
                        total += n;
                        break;
                    }
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = vec![];
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        });

    let mut numbers: HashSet<(usize, usize, usize, u32)> = HashSet::new();
    let mut star_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..grid.len() {
        let row = &grid[i];
        for j in 0..row.len() {
            let c = row[j];
            if c.is_digit(10) {
                let mut left = j;
                while left > 0 && row[left - 1].is_digit(10) {
                    left -= 1;
                }
                let mut right = j;
                while right < row.len() - 1 && row[right + 1].is_digit(10) {
                    right += 1;
                }

                let s: String = row[left..=right].iter().collect();
                let n: u32 = s.parse().unwrap();
                let key = (i, left, right, n);
                if numbers.contains(&key) {
                    continue;
                }
                numbers.insert(key);

                let mut positions = vec![];
                let left_boundary = if left > 0 { left - 1 } else { 0 };
                for x in left_boundary..=right + 1 {
                    if x >= row.len() {
                        continue;
                    }
                    if i > 0 {
                        positions.push((i - 1, x));
                    }
                    if i < grid.len() - 1 {
                        positions.push((i + 1, x));
                    }
                    if x < left || x > right {
                        positions.push((i, x));
                    }
                }

                for (y, x) in positions {
                    let c = grid[y][x];
                    if c == '*' {
                        if !star_numbers.contains_key(&(y, x)) {
                            star_numbers.insert((y, x), vec![]);
                        }
                        star_numbers.get_mut(&(y, x)).unwrap().push(n);
                        break;
                    }
                }
            }
        }
    }
    let mut total: u32 = 0;
    for (_, n) in star_numbers {
        if n.len() == 2 {
            total += n[0] * n[1];
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
