use itertools::{repeat_n, Itertools};
use std::collections::{HashMap, HashSet};
use std::ops::Range;

advent_of_code::solution!(8);

type Coord = (i32, i32);
struct Grid {
    x_bounds: Range<i32>,
    y_bounds: Range<i32>,
}

impl Grid {
    fn new(grid: &[Vec<char>]) -> Self {
        let max_x = grid[0].len() as i32;
        let max_y = grid.len() as i32;

        Grid {
            x_bounds: 0..max_x,
            y_bounds: 0..max_y,
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.x_bounds.contains(&coord.0) && self.y_bounds.contains(&coord.1)
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<Coord>> {
    let mut antenna_positions: HashMap<char, Vec<Coord>> = HashMap::new();

    grid.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(c_idx, &cell)| {
            if cell == '.' {
                return;
            }

            antenna_positions
                .entry(cell)
                .or_default()
                .push((c_idx as i32, row_idx as i32));
        })
    });

    antenna_positions
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let antenna_positions = get_antennas(&grid);
    let grid = Grid::new(&grid);

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in antenna_positions.values() {
        if coords.len() <= 1 {
            continue;
        }
        
        repeat_n(coords, 2)
            .multi_cartesian_product()
            .for_each(|pair| {
                let (a_x, a_y) = *pair[0];
                let (b_x, b_y) = *pair[1];

                if a_x == b_x && a_y == b_y {
                    return;
                }

                let dx = a_x - b_x;
                let dy = a_y - b_y;

                for coord in [(a_x + dx, a_y + dy), (b_x + -dx, b_y + -dy)] {
                    if grid.contains(&coord) {
                        antinodes.insert(coord);
                    }
                }
            });
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let antenna_positions = get_antennas(&grid);
    let grid = Grid::new(&grid);

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in antenna_positions.values() {
        if coords.len() <= 1 {
            continue;
        }

        repeat_n(coords, 2)
            .multi_cartesian_product()
            .for_each(|pair| {
                let (a_x, a_y) = *pair[0];
                let (b_x, b_y) = *pair[1];

                if a_x == b_x && a_y == b_y {
                    return;
                }

                let dx = a_x - b_x;
                let dy = a_y - b_y;

                antinodes.insert(*pair[0]);

                for d in 1.. {
                    let coord = (a_x + dx * d, a_y + dy * d);

                    if !grid.contains(&coord) {
                        break;
                    }

                    antinodes.insert(coord);
                }
            });
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
