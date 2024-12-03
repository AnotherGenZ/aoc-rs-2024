advent_of_code::solution!(2);

#[derive(PartialEq, Clone)]
enum Pattern {
    Unknown,
    Increasing,
    Decreasing,
}

fn evaluate_pair(a: i32, b: i32, pattern: &mut Pattern) -> Option<bool> {
    let mut has_error = false;

    match b - a {
        diff if diff > 0 => {
            match pattern {
                Pattern::Unknown => *pattern = Pattern::Increasing,
                Pattern::Increasing => {}
                Pattern::Decreasing => {
                    has_error = true;
                }
            }

            if !(1..=3).contains(&diff) {
                has_error = true;
            }
        }
        diff if diff < 0 => {
            match pattern {
                Pattern::Unknown => *pattern = Pattern::Decreasing,
                Pattern::Increasing => {
                    has_error = true;
                }
                Pattern::Decreasing => {}
            }

            if diff.abs() < 1 || diff.abs() > 3 {
                has_error = true;
            }
        }
        _ => has_error = true,
    }

    has_error.then_some(false)
}

fn evaluate_report(levels: &Vec<i32>) -> bool {
    let mut pattern = Pattern::Unknown;

    let windowed = levels.windows(2);

    for w in windowed {
        let a = w[0];
        let b = w[1];

        if evaluate_pair(a, b, &mut pattern).is_some() {
            return false;
        }
    }

    true
}

fn parse_report(report: &str) -> Vec<i32> {
    report
        .split(" ")
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.trim().split('\n');

    Some(
        reports
            .filter(|report| {
                let levels = parse_report(report);

                evaluate_report(&levels)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.trim().split('\n');

    Some(
        reports
            .filter(|report| {
                let levels = parse_report(report);

                match evaluate_report(&levels) {
                    true => true,
                    false => {
                        for i in 0..levels.len() {
                            let levels: Vec<i32> = levels
                                .iter()
                                .enumerate()
                                .filter(|&(j, _)| i != j)
                                .map(|(_, e)| *e)
                                .collect();

                            if evaluate_report(&levels) {
                                return true;
                            }
                        }

                        false
                    }
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
