use itertools::Itertools;

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
        } else if top.contains(&y) {
            Quadrant::TopRight
        } else {
            Quadrant::BottomRight
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
    let mut quadrants = [0; 4];

    for robot in robots {
        if let Some(quad) = robot.get_quadrant() {
            let quad = quad as usize;
            quadrants[quad] += 1;
        }
    }

    quadrants[0..4].iter().product()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.into()).collect_vec();

    for robot in robots.iter_mut() {
        robot.step(100);
    }

    let safety_factor = safety_factor(&robots);

    Some(safety_factor)
}

fn mod_inverse(a: isize, m: isize) -> isize {
    let a = a % m;

    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }

    1
}

fn calculate_variance(robots: &[Robot], x: bool) -> f64 {
    let mean = robots
        .iter()
        .map(|r| (if x { r.pos.0 } else { r.pos.1 }) as f64)
        .sum::<f64>()
        / robots.len() as f64;

    let variance = robots
        .iter()
        .map(|r| {
            let diff = (if x { r.pos.0 } else { r.pos.1 }) as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / robots.len() as f64;

    variance
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.into()).collect_vec();

    let mut bx = 0;
    let mut bx_var = (10 * 100) as f64;
    let mut by = 0;
    let mut by_var = (10 * 1000) as f64;

    for step in 0..HEIGHT {
        for robot in robots.iter_mut() {
            robot.step(1);
        }

        let x_var = calculate_variance(&robots, true);
        let y_var = calculate_variance(&robots, false);

        if x_var < bx_var {
            bx = step;
            bx_var = x_var;
        }

        if y_var < by_var {
            by = step;
            by_var = y_var;
        }
    }

    let step_count =
        (bx + (mod_inverse(WIDTH, HEIGHT) * (by - bx)) * WIDTH).rem_euclid(HEIGHT * WIDTH);

    Some(step_count as u32)
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
