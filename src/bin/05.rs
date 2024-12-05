use std::cmp::Ordering;
use std::collections::HashSet;

advent_of_code::solution!(5);

fn sort_pages(ordering_rules: &HashSet<(u32, u32)>, a: u32, b: u32) -> Ordering {
    if ordering_rules.contains(&(a, b)) {
        Ordering::Less
    } else if ordering_rules.contains(&(b, a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn are_pages_sorted(ordering_rules: &HashSet<(u32, u32)>, a: u32, b: u32) -> bool {
    match sort_pages(ordering_rules, a, b) {
        Ordering::Less | Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ordering_rules: HashSet<(u32, u32)> = HashSet::new();
    let mut sum = 0;

    let lines = input.lines();

    let mut passed_separator = false;

    for line in lines {
        if line.is_empty() {
            passed_separator = true;
            continue;
        }

        if !passed_separator {
            ordering_rules.insert((
                line[0..=1].parse::<u32>().unwrap(),
                line[3..=4].parse::<u32>().unwrap(),
            ));
        } else {
            let pages = line
                .split(",")
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let is_sorted = pages.is_sorted_by(|a, b| are_pages_sorted(&ordering_rules, *a, *b));

            if !is_sorted {
                continue;
            }

            sum += pages[pages.len() / 2];
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ordering_rules: HashSet<(u32, u32)> = HashSet::new();
    let mut sum = 0;

    let lines = input.lines();

    let mut passed_separator = false;

    for line in lines {
        if line.is_empty() {
            passed_separator = true;
            continue;
        }

        if !passed_separator {
            ordering_rules.insert((
                line[0..=1].parse::<u32>().unwrap(),
                line[3..=4].parse::<u32>().unwrap(),
            ));
        } else {
            let mut pages = line
                .split(",")
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let is_sorted = pages.is_sorted_by(|a, b| are_pages_sorted(&ordering_rules, *a, *b));

            if !is_sorted {
                pages.sort_by(|a, b| sort_pages(&ordering_rules, *a, *b));

                sum += pages[pages.len() / 2];
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
