#![feature(let_chains)]

use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::cmp::PartialEq;
use std::ops::{Add, Index, Range};

advent_of_code::solution!(12);

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Coord(i32, i32);

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<OffsetLocation> for Coord {
    fn from(value: OffsetLocation) -> Self {
        match value {
            OffsetLocation::Above => Coord(0, -1),
            OffsetLocation::Below => Coord(0, 1),
            OffsetLocation::Left => Coord(-1, 0),
            OffsetLocation::Right => Coord(1, 0),
        }
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
    x_bounds: Range<i32>,
    y_bounds: Range<i32>,
}

impl<T> Grid<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        let max_x = grid[0].len() as i32;
        let max_y = grid.len() as i32;

        Grid {
            data: grid,
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
    sides: usize,
    perimeter: usize,
    area: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
enum OffsetLocation {
    Above,
    Below,
    Left,
    Right,
}

impl OffsetLocation {
    fn right(&self) -> Self {
        match self {
            OffsetLocation::Above => OffsetLocation::Right,
            OffsetLocation::Below => OffsetLocation::Left,
            OffsetLocation::Left => OffsetLocation::Above,
            OffsetLocation::Right => OffsetLocation::Below,
        }
    }

    fn left(&self) -> Self {
        match self {
            OffsetLocation::Above => OffsetLocation::Left,
            OffsetLocation::Below => OffsetLocation::Right,
            OffsetLocation::Left => OffsetLocation::Below,
            OffsetLocation::Right => OffsetLocation::Above,
        }
    }
}

const OFFSETS: [OffsetLocation; 4] = [
    OffsetLocation::Above,
    OffsetLocation::Below,
    OffsetLocation::Left,
    OffsetLocation::Right,
];

fn explore_region(
    grid: &Grid<char>,
    explored: &mut FxHashSet<Coord>,
    region: &mut Region,
    fences: &mut Option<FxHashSet<(OffsetLocation, Coord)>>,
    coord: Coord,
) {
    if explored.contains(&coord) {
        return;
    }

    let region_plant = grid[coord];

    region.area += 1;
    explored.insert(coord);

    for offset in OFFSETS {
        let offset_coord = coord + offset.into();

        if grid.contains(&offset_coord) && grid[offset_coord] == region_plant {
            explore_region(grid, explored, region, fences, offset_coord)
        } else {
            region.perimeter += 1;

            if let Some(fences) = fences {
                fences.insert((offset, offset_coord));
            }
        }
    }
}

fn find_regions(grid: &Grid<char>, find_sides: bool) -> Vec<Region> {
    let mut explored: FxHashSet<Coord> = FxHashSet::default();
    let mut regions = Vec::new();

    for j in 0..grid.y_bounds.end {
        for i in 0..grid.x_bounds.end {
            if !explored.contains(&(i, j).into()) {
                let mut region = Region {
                    sides: 0,
                    perimeter: 0,
                    area: 0,
                };

                let mut fences = if find_sides {
                    Some(FxHashSet::default())
                } else {
                    None
                };

                explore_region(grid, &mut explored, &mut region, &mut fences, (i, j).into());

                if let Some(fences) = &mut fences {
                    let mut sides = 0;

                    while !fences.is_empty() {
                        let (fence_d, fence_coord) = *fences.iter().next().unwrap();

                        let mut fence = fence_coord;
                        let step = fence_d.right().into();
                        while fences.remove(&(fence_d, fence)) {
                            fence = fence + step;
                        }

                        let step = fence_d.left().into();
                        let mut fence = fence_coord + step;
                        while fences.remove(&(fence_d, fence)) {
                            fence = fence + step;
                        }

                        sides += 1;
                    }

                    region.sides = sides;
                }

                regions.push(region);
            }
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
    let grid = parse_grid(input);

    let regions = find_regions(&grid, true);
    let price = regions
        .iter()
        .map(|region| (region.area * region.sides) as u32)
        .sum();

    Some(price)
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
