advent_of_code::solution!(1);

fn get_sorted_sides(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines = input.lines();

    for line in lines {
        let parts = line.split_once("   ").unwrap();

        left.push(parts.0.parse::<u32>().unwrap());
        right.push(parts.1.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = get_sorted_sides(input);

    Some(left.iter().zip(right).map(|(&l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut similarity = 0;

    let (left, right) = get_sorted_sides(input);

    let mut right_iter = right.iter().peekable();

    for left_num in left {
        let mut occurs = 0;

        while let Some(&right_num) = right_iter.next_if(|&&right_num| right_num <= left_num) {
            if left_num == right_num {
                occurs += 1;
            }
        }

        similarity += left_num * occurs;
    }

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
