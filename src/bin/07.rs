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
                let num_r_digits = r.checked_ilog10().unwrap_or(0) + 1;

                l * 10_u64.pow(num_r_digits) + r
            }
        }
    }
}

trait Digits<T> {
    fn num_digits(&self) -> u32;
    fn get_first_digits(&self, n_remove: u32) -> T;
    fn compare_digits(&self, other: T, n: u32) -> bool;
}

impl Digits<u64> for u64 {
    fn num_digits(&self) -> u32 {
        self.checked_ilog10().unwrap_or(0) + 1
    }
    fn get_first_digits(&self, n_remove: u32) -> u64 {
        let mask = 10u64.pow(n_remove);
        (self - (self % mask)) / mask
    }

    fn compare_digits(&self, mut other: u64, n: u32) -> bool {
        let mut l = *self;

        for _ in 0..n {
            if (l % 10) != (other % 10) {
                return false;
            }

            l /= 10;
            other /= 10;
        }

        true
    }
}

fn solvable(test_value: u64, operands: &mut [u64], operators: &[Operator]) -> bool {
    let num_operands = operands.len();

    if num_operands == 2 {
        return operators
            .iter()
            .any(|op| op.apply(operands[1], operands[0]) == test_value);
    }

    operators.iter().any(|op| {
        let first = operands[0];
        let operands = &mut operands[1..num_operands];

        let new_test_value = match op {
            Operator::Add => test_value - first,
            Operator::Multiply => {
                if test_value % first != 0 {
                    return false;
                }

                test_value / first
            }
            Operator::Concatenate => {
                let num_digits = first.num_digits();

                if !test_value.compare_digits(first, num_digits) {
                    return false;
                }

                test_value.get_first_digits(num_digits)
            }
        };

        solvable(new_test_value, operands, operators)
    })
}

fn sum_results(input: &str, operators: &[Operator]) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (val, nums) = line.split_once(":").expect("Missing colon");
            let test_value = val.parse::<u64>().unwrap();
            let mut operands = nums
                .trim()
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            operands.reverse();

            solvable(test_value, &mut operands, operators).then_some(test_value)
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
