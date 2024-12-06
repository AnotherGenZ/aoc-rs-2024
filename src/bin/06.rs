use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&u8> for Orientation {
    type Error = ();

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        use Orientation::*;
        match value {
            b'^' => Ok(Up),
            b'v' => Ok(Down),
            b'<' => Ok(Left),
            b'>' => Ok(Right),
            _ => Err(()),
        }
    }
}

fn is_open(grid: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    let max_x = grid[0].len();
    let max_y = grid.len();

    if x >= max_x || y >= max_y {
        return true;
    }

    if grid[y][x] == b'#' {
        return false;
    }

    true
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
    orientation: Orientation,
}

impl Position {
    fn step(&mut self, grid: &Vec<&[u8]>) {
        use Orientation::*;

        match self.orientation {
            Up => {
                if is_open(&grid, self.x, self.y.wrapping_sub(1)) {
                    self.y = self.y.wrapping_sub(1);
                } else {
                    self.orientation = Right;
                }
            }
            Down => {
                if is_open(&grid, self.x, self.y + 1) {
                    self.y += 1;
                } else {
                    self.orientation = Left;
                }
            }
            Left => {
                if is_open(&grid, self.x.wrapping_sub(1), self.y) {
                    self.x = self.x.wrapping_sub(1);
                } else {
                    self.orientation = Up;
                }
            }
            Right => {
                if is_open(&grid, self.x + 1, self.y) {
                    self.x += 1;
                } else {
                    self.orientation = Down;
                }
            }
        }
    }
}

impl TryFrom<(usize, usize, &u8)> for Position {
    type Error = ();

    fn try_from(value: (usize, usize, &u8)) -> Result<Self, Self::Error> {
        let position = Position {
            x: value.0,
            y: value.1,
            orientation: value.2.try_into()?,
        };

        Ok(position)
    }
}

fn parse_grid(input: &str) -> Vec<&[u8]> {
    input
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>()
}

fn get_start_pos(grid: &Vec<&[u8]>) -> Position {
    grid.iter()
        .enumerate()
        .find_map(|(r_idx, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c_idx, char)| (c_idx, r_idx, char).try_into().ok())
        })
        .expect("Guard missing??")
}

fn get_guard_positions(grid: &Vec<&[u8]>) -> (Position, HashSet<(usize, usize)>) {
    let max_x = grid[0].len();
    let max_y = grid.len();

    let mut current_pos = get_start_pos(&grid);
    let starting_pos = current_pos;
    let mut guard_positions: HashSet<(usize, usize)> =
        HashSet::from([(starting_pos.x, starting_pos.y)]);

    loop {
        current_pos.step(&grid);

        if (0..max_x).contains(&current_pos.x) && (0..max_y).contains(&current_pos.y) {
            guard_positions.insert((current_pos.x, current_pos.y));
        } else {
            break;
        }
    }

    (starting_pos, guard_positions)
}

fn is_loop(grid: &Vec<&[u8]>, starting_pos: &Position) -> bool {
    let max_x = grid[0].len();
    let max_y = grid.len();

    let mut current_pos = *starting_pos;
    let mut guard_positions: HashSet<(usize, usize, Orientation)> =
        HashSet::from([(current_pos.x, current_pos.y, current_pos.orientation)]);

    loop {
        current_pos.step(&grid);

        if (0..max_x).contains(&current_pos.x) && (0..max_y).contains(&current_pos.y) {
            if !guard_positions.insert((current_pos.x, current_pos.y, current_pos.orientation)) {
                return true;
            }
        } else {
            break;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let (_, guard_positions) = get_guard_positions(&grid);

    Some(guard_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let (starting_position, guard_path) = get_guard_positions(&grid);

    Some(
        guard_path
            .par_iter()
            .filter(|(x, y)| {
                let mut grid = grid.clone();
                let mut new_row = Vec::from(grid[*y]);
                new_row[*x] = b'#';
                grid[*y] = &new_row;

                is_loop(&grid, &starting_position)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
