use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();

    if input.is_empty() {
        return Some(0);
    }

    let regex = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))").expect("Regex failed to compile");

    Some(regex.captures_iter(input).fold(0, |sum, captures| {
        let a = captures[2].parse::<u32>().unwrap();
        let b = captures[3].parse::<u32>().unwrap();

        sum + a * b
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim();

    if input.is_empty() {
        return Some(0);
    }

    let regex = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))")
        .expect("Regex failed to compile");

    let mut sum = 0;
    let mut enabled = true;

    for captures in regex.captures_iter(input) {
        let action = &captures[0];

        if action.starts_with("mul") {
            if enabled {
                let a = captures[4].parse::<u32>().unwrap();
                let b = captures[5].parse::<u32>().unwrap();

                sum += a * b;
            }
        } else {
            enabled = !action.starts_with("don't");
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
