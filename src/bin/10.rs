use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Range;

type HashMap<K, V> = FxHashMap<K, V>;
type HashSet<V> = FxHashSet<V>;

advent_of_code::solution!(10);

type Coord = (i32, i32);
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

const OFFSETS: [Coord; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn search_around(grid: &Grid<char>, coord: Coord, value: char) -> Vec<Coord> {
    let mut next = Vec::with_capacity(4);

    for offset in OFFSETS {
        let new_coord = (coord.0 + offset.0, coord.1 + offset.1);

        if !grid.contains(&new_coord) {
            continue;
        }

        if grid.data[new_coord.1 as usize][new_coord.0 as usize] == value {
            next.push(new_coord);
        }
    }

    next
}

fn find_peaks(
    grid: &Grid<char>,
    trail_nodes: &mut HashMap<Coord, HashSet<Coord>>,
    coord: Coord,
    next_value: char,
) -> HashSet<Coord> {
    let mut peaks: HashSet<Coord> = HashSet::default();
    let next_coords = search_around(grid, coord, next_value);

    if next_value == '9' {
        return HashSet::from_iter(next_coords);
    }

    for next_coord in next_coords.iter() {
        if let Some(trail_peaks) = trail_nodes.get(next_coord) {
            peaks.extend(trail_peaks);
        } else {
            let next_coord_peaks = find_peaks(
                grid,
                trail_nodes,
                *next_coord,
                (next_value as u8 + 1) as char,
            );

            peaks.extend(&next_coord_peaks);
            trail_nodes.insert(*next_coord, next_coord_peaks);
        }
    }

    peaks
}

fn find_trails(
    grid: &Grid<char>,
    trail_nodes: &mut HashMap<Coord, Vec<Coord>>,
    coord: Coord,
    next_value: char,
) -> Vec<Coord> {
    let mut peaks: Vec<Coord> = Vec::new();
    let next_coords = search_around(grid, coord, next_value);

    if next_value == '9' {
        return next_coords;
    }

    for next_coord in next_coords.iter() {
        if let Some(trail_peaks) = trail_nodes.get(next_coord) {
            peaks.extend(trail_peaks);
        } else {
            let next_coord_peaks = find_trails(
                grid,
                trail_nodes,
                *next_coord,
                (next_value as u8 + 1) as char,
            );

            peaks.extend(&next_coord_peaks);
            trail_nodes.insert(*next_coord, next_coord_peaks);
        }
    }

    peaks
}

fn get_trailheads(grid: &Grid<char>) -> impl Iterator<Item = Coord> + use<'_> {
    grid.data.iter().enumerate().flat_map(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(c_idx, &c)| (c == '0').then_some((c_idx as i32, row_idx as i32)))
    })
}

fn get_grid(input: &str) -> Grid<char> {
    Grid::new(
        input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_grid(input);

    let mut trail_nodes: HashMap<Coord, HashSet<Coord>> = HashMap::default();
    let trailheads = get_trailheads(&grid);

    let mut trailheads_score = 0;

    for trailhead in trailheads {
        let peaks = find_peaks(&grid, &mut trail_nodes, trailhead, (b'0' + 1) as char);

        trailheads_score += peaks.len() as u32;
    }

    Some(trailheads_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_grid(input);

    let mut trail_nodes: HashMap<Coord, Vec<Coord>> = HashMap::default();
    let trailheads = get_trailheads(&grid);

    let mut trailheads_rating = 0;

    for trailhead in trailheads {
        let trails = find_trails(&grid, &mut trail_nodes, trailhead, (b'0' + 1) as char);

        trailheads_rating += trails.len() as u32;
    }

    Some(trailheads_rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
