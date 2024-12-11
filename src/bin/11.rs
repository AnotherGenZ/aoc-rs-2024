use rustc_hash::FxHashMap;
use std::iter::Map;
use std::str::SplitWhitespace;

advent_of_code::solution!(11);

enum StoneAction {
    Replace(u64),
    Split((u64, u64)),
}

fn blink_action(stone: u64) -> StoneAction {
    if stone == 0 {
        return StoneAction::Replace(1);
    }

    let num_digits = stone.ilog10() + 1;

    if num_digits % 2 == 0 {
        let power = 10u64.pow(num_digits / 2);
        let right = stone % power;
        let left = stone / power;

        return StoneAction::Split((left, right));
    }

    StoneAction::Replace(2024 * stone)
}

fn blink(stone: u64, i: usize, seen_stones: &mut FxHashMap<(usize, u64), usize>) -> usize {
    if i == 0 {
        return 1;
    } else if let Some(cached) = seen_stones.get(&(i, stone)) {
        return *cached;
    }

    let num_stones = match blink_action(stone) {
        StoneAction::Replace(new_value) => blink(new_value, i - 1, seen_stones),
        StoneAction::Split((left, right)) => blink(left, i - 1, seen_stones) + blink(right, i - 1, seen_stones),
    };

    seen_stones.insert((i, stone), num_stones);
    num_stones
}

fn parse_stones(input: &str) -> Map<SplitWhitespace, fn(&str) -> u64> {
    input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().expect("Stone number is u64"))
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones = parse_stones(input);
    let mut seen_stones = FxHashMap::default();

    let total_stones = stones.map(|stone| blink(stone, 25, &mut seen_stones)).sum();

    Some(total_stones)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = parse_stones(input);
    let mut seen_stones = FxHashMap::default();

    let total_stones = stones.map(|stone| blink(stone, 75, &mut seen_stones)).sum();

    Some(total_stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
