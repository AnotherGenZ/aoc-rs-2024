advent_of_code::solution!(13);

const CONVERSION_ERROR: isize = 10000000000000;

#[derive(Debug)]
struct Machine {
    btn_a: (isize, isize),
    btn_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn needed_tokens(&self, part_2: bool) -> Option<usize> {
        let (ax, ay) = self.btn_a;
        let (bx, by) = self.btn_b;
        let (mut px, mut py) = self.prize;

        if part_2 {
            px += CONVERSION_ERROR;
            py += CONVERSION_ERROR;
        }

        let denom = ax * by - ay * bx;
        let num1 = by * px - bx * py;
        let num2 = ax * py - ay * px;

        if denom == 0 {
            return None;
        }

        if num1 % denom != 0 || num2 % denom != 0 {
            return None;
        }

        let a = num1 / denom;
        let b = num2 / denom;

        if a < 0 || b < 0 {
            return None;
        }

        Some((3 * a + b) as usize)
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lines = value
            .lines()
            .map(|line| line.split_once(":").unwrap().1.split_once(",").unwrap());

        let (a_x, a_y) = lines.next().unwrap();
        let (b_x, b_y) = lines.next().unwrap();
        let (p_x, p_y) = lines.next().unwrap();

        let btn_a = (
            a_x.split_once("+").unwrap().1.parse().unwrap(),
            a_y.split_once("+").unwrap().1.parse().unwrap(),
        );

        let btn_b = (
            b_x.split_once("+").unwrap().1.parse().unwrap(),
            b_y.split_once("+").unwrap().1.parse().unwrap(),
        );

        let prize = (
            p_x.split_once("=").unwrap().1.parse().unwrap(),
            p_y.split_once("=").unwrap().1.parse().unwrap(),
        );

        Self {
            btn_a,
            btn_b,
            prize,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = input.split("\n\n").map(Machine::from);

    let tokens = machines
        .filter_map(|machine| machine.needed_tokens(false))
        .sum();

    Some(tokens)
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = input.split("\n\n").map(Machine::from);

    let tokens = machines
        .filter_map(|machine| machine.needed_tokens(true))
        .sum();

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
