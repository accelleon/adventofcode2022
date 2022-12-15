use std::{collections::HashSet, hash::Hash};

fn calc_prio(input: &str) -> i32 {
    // 1 line represents 2 inventories
    // find the character that appears in both inventories
    // a - z are 1-26
    // A - Z are 27-52

    // Map lines into an inventory outcome
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Split in half, to hashsets
            let (left, right) = line.split_at(line.len() / 2);
            let left: HashSet<char> = left.chars().collect();
            let right: HashSet<char> = right.chars().collect();

            // Return the priority of the intersection
            left.intersection(&right).fold(0, |acc, &c| {
                acc + match c {
                    'a'..='z' => c as i32 - 'a' as i32 + 1,
                    'A'..='Z' => c as i32 - 'A' as i32 + 27,
                    _ => 0,
                }
            })
        })
        .sum()
}

fn calc_badge(input: &str) -> i32 {
    // Each 3 lines represents 1 group
    // Find the item that appears in all 3 inventories
    // a - z are 1-26
    // A - Z are 27-52

    // Map lines into an inventory outcome
    input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|line| HashSet::from_iter(line.chars()))
                .fold(None, |acc: Option<HashSet<_>>, set| {
                    match acc {
                        Some(acc) => Some(acc.intersection(&set).cloned().collect()),
                        None => Some(set),
                    }
                })
                .unwrap()
                .iter()
                .fold(0, |acc, &c| {
                    acc + match c {
                        'a'..='z' => c as i32 - 'a' as i32 + 1,
                        'A'..='Z' => c as i32 - 'A' as i32 + 27,
                        _ => 0,
                    }
                })
        })
        .sum()
}

fn main() {
    // read in input.txt
    let input = include_str!("input.txt");
    println!("{}", calc_prio(&input));
    println!("{}", calc_badge(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_prio() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(calc_prio(input), 157);
    }

    #[test]
    fn test_calc_badge() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(calc_badge(input), 70);
    }
}
