use std::collections::{BTreeMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts = line
                .split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .collect::<Vec<&str>>();
            assert!(parts.len() == 2);
            let winners: HashSet<u32> =
                HashSet::from_iter(parts[0].split_whitespace().map(|s| s.parse().unwrap()));

            let num_winners = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .filter(|n| winners.contains(n))
                .count() as u32;

            // println!("{} -> {}", line, num_winners);

            if num_winners == 0 {
                0
            } else {
                2_u32.pow(num_winners - 1)
            }
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    let iter = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .enumerate();

    for (i, line) in iter {
        let count = counts.entry(i).or_insert(0);
        *count += 1;

        let parts = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .collect::<Vec<&str>>();
        assert!(parts.len() == 2);
        let winners: HashSet<u32> =
            HashSet::from_iter(parts[0].split_whitespace().map(|s| s.parse().unwrap()));

        let num_winners = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .filter(|n| winners.contains(n))
            .count();

        let current_card_count = *counts.get(&i).unwrap();
        for j in 1_usize..=num_winners {
            let next_card = counts.entry(i + j).or_insert(0);
            *next_card += current_card_count;
        }
    }

    let sum: usize = counts.values().sum();
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
