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
    fn apply(&self, l: u64, r: u64) -> u64 {
        use Operator::*;

        match self {
            Add => l + r,
            Multiply => l * r,
            Concatenate => {
                let num_r_digits = successors(Some(r), |&n| (n >= 10).then(|| n / 10)).count();

                l * 10_u64.pow(num_r_digits as u32) + r
            }
        }
    }
}

fn solvable(test_value: u64, operands: &mut [u64], operators: &[Operator]) -> bool {
    let num_operands = operands.len();

    if num_operands == 2 {
        return operators
            .iter()
            .any(|op| op.apply(operands[0], operands[1]) == test_value);
    }

    operators.iter().any(|op| {
        let first = operands[0];
        let second = operands[1];
        let operands = &mut operands[1..num_operands];
        operands[0] = op.apply(first, second);

        if solvable(test_value, operands, operators) {
            return true;
        }

        operands[0] = second;

        false
    })
}

fn sum_results(input: &str, operators: &[Operator]) -> u64 {
    input
        .lines()
        .par_bridge()
        .filter_map(|line| {
            let (val, nums) = line.split_once(":").expect("Missing colon");
            let test_value = val.parse::<u64>().unwrap();
            let mut operands = nums
                .trim()
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            if solvable(test_value, &mut operands, operators) {
                return Some(test_value);
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
