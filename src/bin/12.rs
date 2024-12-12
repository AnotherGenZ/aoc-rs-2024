use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
use std::ops::{Add, Index, Range};

advent_of_code::solution!(12);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Coord(i32, i32);

impl Coord {
    // fn swap(&self) -> Coord {
    //     (self.1, self.0).into()
    // }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Grid<T> {
    pub data: Vec<Vec<T>>,
    size: usize,
    x_bounds: Range<i32>,
    y_bounds: Range<i32>,
}

impl<T> Grid<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        let max_x = grid[0].len() as i32;
        let max_y = grid.len() as i32;

        Grid {
            data: grid,
            size: (max_x * max_y) as usize,
            x_bounds: 0..max_x,
            y_bounds: 0..max_y,
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.x_bounds.contains(&coord.0) && self.y_bounds.contains(&coord.1)
    }
}

impl<T: Copy> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        if self.x_bounds.contains(&index.0) && self.y_bounds.contains(&index.1) {
            return &self.data[index.1 as usize][index.0 as usize];
        }

        panic!("Coordinate out of bounds");
    }
}

fn parse_grid(input: &str) -> Grid<char> {
    Grid::new(
        input
            .lines()
            .map(|row| row.chars().collect_vec())
            .collect_vec(),
    )
}

#[derive(Debug)]
struct Region {
    // sides: usize,
    perimeter: usize,
    area: usize,
}

const OFFSETS: [Coord; 4] = [Coord(-1, 0), Coord(0, -1), Coord(1, 0), Coord(0, 1)];

// #[derive(Debug)]
// enum Location {
//     Above,
//     Below,
//     Left,
//     Right,
// }

fn explore_region(grid: &Grid<char>, start: Coord, find_sides: bool) -> (Region, BTreeSet<Coord>) {
    let region_plant = grid[start];

    let mut perimeter = 0;

    let mut plots: BTreeSet<Coord> = BTreeSet::default();
    let mut boundary_plots = BTreeSet::default();
    let mut queue = VecDeque::from_iter([start]);

    while let Some(pos) = queue.pop_front() {
        if plots.contains(&pos) {
            continue;
        }

        if grid.contains(&pos) && grid[pos] == region_plant {
            plots.insert(pos);

            for offset in OFFSETS {
                queue.push_back(pos + offset);
            }
        } else {
            boundary_plots.insert(pos);
            perimeter += 1;
        }
    }

    let mut region = Region {
        // sides: 0,
        area: plots.len(),
        perimeter,
    };

    // if region_plant == 'R' {
    //     // println!("{boundary_plots:?}");
    // }

    // if find_sides {
    //     let mut sides = 0;
    //
    //     let visited_y = BTreeSet::from_iter(visited.iter().map((|coord| coord.swap())));
    //
    //     let mut prev_y: Option<i32> = None;
    //     for coord in &visited {
    //         if grid[*coord] == 'R' {
    //             println!("{coord:?}");
    //         }
    //
    //         if let Some(prev_y) = prev_y {
    //             if coord.1 != prev_y + 1 {
    //                 sides += 1;
    //             }
    //         }
    //
    //         prev_y = Some(coord.1)
    //     }
    //
    //     println!("{region_plant}: {sides}");
    //
    //     region.sides = Some(sides);
    // }

    (region, plots)
}

fn find_regions(grid: &Grid<char>, find_sides: bool) -> Vec<Region> {
    let mut explored: BTreeSet<Coord> = BTreeSet::new();
    let mut regions = Vec::new();

    let mut start_row = 0;

    let mut start = (0, 0).into();
    while (explored.len() < grid.size) {
        let (region, region_plots) = explore_region(grid, start, find_sides);

        explored.extend(region_plots);
        regions.push(region);

        'start: for j in start_row..grid.y_bounds.end {
            for i in 0..grid.x_bounds.end {
                if (!explored.contains(&(i, j).into())) {
                    start = (i, j).into();
                    break 'start;
                }
            }

            start_row += 1;
        }
    }

    regions
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let regions = find_regions(&grid, false);
    let price = regions
        .iter()
        .map(|region| (region.area * region.perimeter) as u32)
        .sum();

    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let grid = parse_grid(input);
    // 
    // let regions = find_regions(&grid, true);
    // let price = regions
    //     .iter()
    //     .map(|region| (region.area * region.sides) as u32)
    //     .sum();
    // 
    // Some(price)
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
