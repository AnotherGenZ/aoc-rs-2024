use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn sort_pages(ordering_rules: &HashMap<&str, HashSet<&str>>, a: &str, b: &str) -> Ordering {
    let a_after = ordering_rules.get(a);
    let b_after = ordering_rules.get(b);

    if a_after.is_some_and(|a_after_pages| a_after_pages.contains(b)) {
        Ordering::Less
    } else if b_after.is_some_and(|b_after_pages| b_after_pages.contains(a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn are_pages_sorted(ordering_rules: &HashMap<&str, HashSet<&str>>, a: &str, b: &str) -> bool {
    match sort_pages(ordering_rules, a, b) {
        Ordering::Less | Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ordering_rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut sum = 0;

    let lines = input.lines();

    let mut passed_separator = false;

    for line in lines {
        if line == "" {
            passed_separator = true;
            continue;
        }

        if !passed_separator {
            ordering_rules
                .entry(&line[0..=1])
                .or_default()
                .insert(&line[3..=4]);
        } else {
            let pages = line.split(",").collect::<Vec<_>>();

            let is_sorted = pages.is_sorted_by(|a, b| are_pages_sorted(&ordering_rules, a, b));

            if !is_sorted {
                continue;
            }

            sum += pages[pages.len() / 2].parse::<u32>().unwrap();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ordering_rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut sum = 0;

    let lines = input.lines();

    let mut passed_separator = false;

    for line in lines {
        if line == "" {
            passed_separator = true;
            continue;
        }

        if !passed_separator {
            ordering_rules
                .entry(&line[0..=1])
                .or_default()
                .insert(&line[3..=4]);
        } else {
            let mut pages = line.split(",").collect::<Vec<_>>();

            let is_sorted = pages.is_sorted_by(|a, b| are_pages_sorted(&ordering_rules, a, b));

            if !is_sorted {
                pages.sort_by(|a, b| sort_pages(&ordering_rules, a, b));

                sum += pages[pages.len() / 2].parse::<u32>().unwrap();
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
