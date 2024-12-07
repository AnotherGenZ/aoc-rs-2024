use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use std::iter::successors;

advent_of_code::solution!(7);

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, l: u64, r: u32) -> u64 {
        use Operator::*;

        match self {
            Add => l + (r as u64),
            Multiply => l * (r as u64),
            Concatenate => {
                let num_r_digits = successors(Some(r), |&n| (n >= 10).then(|| n / 10)).count();

                l * 10_u64.pow(num_r_digits as u32) + (r as u64)
            }
        }
    }
}

fn sum_results(input: &str, operators: &[Operator]) -> u64 {
    input
        .lines()
        .par_bridge()
        .filter_map(|line| {
            let (val, nums) = line.split_once(":").expect("Missing colon");
            let test_value = val.parse::<u64>().unwrap();
            let operands = nums
                .trim()
                .split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let num_slots = operands.len() - 1;
            let permutations = repeat_n(operators, num_slots).multi_cartesian_product();

            for permutation in permutations {
                let result = permutation
                    .iter()
                    .enumerate()
                    .fold(operands[0] as u64, |acc, (op_idx, op)| {
                        op.apply(acc, operands[op_idx + 1])
                    });

                if result == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(sum_results(input, &[Operator::Add, Operator::Multiply]))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sum_results(
        input,
        &[Operator::Add, Operator::Multiply, Operator::Concatenate],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
