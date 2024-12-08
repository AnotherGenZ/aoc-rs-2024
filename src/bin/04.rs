advent_of_code::solution!(4);

fn safe_get(grid: &[&[u8]], x: usize, y: usize) -> u8 {
    *grid.get(y).and_then(|row| row.get(x)).unwrap_or(&b'.')
}

fn count_xmas(grid: &[&[u8]], x: usize, y: usize) -> u32 {
    [
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
    .iter()
    .filter(|(dx, dy)| {
        (1..=3).all(|i| {
            let (cx, cy) = (
                (x as i32).wrapping_add(dx * i) as usize,
                (y as i32).wrapping_add(dy * i) as usize,
            );
            safe_get(grid, cx, cy) == b"XMAS"[i as usize]
        })
    })
    .count() as u32
}

fn has_x_mas(grid: &[&[u8]], x: usize, y: usize) -> bool {
    let left_right = [
        safe_get(grid, x.wrapping_sub(1), y.wrapping_sub(1)),
        safe_get(grid, x + 1, y + 1),
    ];
    let right_left = [
        safe_get(grid, x + 1, y.wrapping_sub(1)),
        safe_get(grid, x.wrapping_sub(1), y + 1),
    ];

    [left_right, right_left]
        .iter()
        .all(|w| w == b"MS" || w == b"SM")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    let grid = input.lines().map(|row| row.as_bytes()).collect::<Vec<_>>();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == b'X' {
                count += count_xmas(&grid, x, y)
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    let grid = input.lines().map(|row| row.as_bytes()).collect::<Vec<_>>();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == b'A' && has_x_mas(&grid, x, y) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
