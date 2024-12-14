use itertools::Itertools;
use rustc_hash::FxHashSet;

advent_of_code::solution!(14);

enum Quadrant {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn step(&mut self, steps: isize) {
        self.pos.0 = (self.pos.0 + self.velocity.0 * steps).rem_euclid(WIDTH);
        self.pos.1 = (self.pos.1 + self.velocity.1 * steps).rem_euclid(HEIGHT);
    }

    fn get_quadrant(&self) -> Option<Quadrant> {
        let (x, y) = self.pos;

        if x == WIDTH / 2 || y == HEIGHT / 2 {
            return None;
        }

        let left = 0..(WIDTH / 2);
        let top = 0..(HEIGHT / 2);

        Some(if left.contains(&x) {
            if top.contains(&y) {
                Quadrant::TopLeft
            } else {
                Quadrant::BottomLeft
            }
        } else {
            if top.contains(&y) {
                Quadrant::TopRight
            } else {
                Quadrant::BottomRight
            }
        })
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (start_pos, velocity) = value.split_once(" ").unwrap();

        let (px, py) = start_pos
            .split_once("=")
            .unwrap()
            .1
            .split_once(",")
            .unwrap();
        let (vx, vy) = velocity.split_once("=").unwrap().1.split_once(",").unwrap();

        Self {
            pos: (px.parse().unwrap(), py.parse().unwrap()),
            velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}
fn safety_factor(robots: &[Robot]) -> usize {
    let mut quadrants = vec![0; 4];

    for robot in robots {
        if let Some(quad) = robot.get_quadrant() {
            let quad = quad as usize;
            quadrants[quad] += 1;
        }
    }

    quadrants[0..4].iter().fold(1, |acc, quad| acc * quad)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.into()).collect_vec();

    for robot in robots.iter_mut() {
        robot.step(100);
    }

    let safety_factor = safety_factor(&robots);

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.into()).collect_vec();

    let mut step_count = 0;

    loop {
        step_count += 1;

        let mut taken = FxHashSet::default();

        for robot in robots.iter_mut() {
            robot.step(1);

            taken.insert(robot.pos);
        }

        if taken.len() == robots.len() {
            break;
        }
    }

    Some(step_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
