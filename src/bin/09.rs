use itertools::Itertools;
use std::cmp::PartialEq;

advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SectorId {
    Free,
    File(usize),
}

#[derive(Copy, Clone, Debug)]
struct Sector {
    id: SectorId,
    start_block: usize,
    length: u32,
}

impl Sector {
    fn is_free(&self) -> bool {
        match self.id {
            SectorId::Free => true,
            SectorId::File(_) => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum DiskBlock {
    Free,
    Taken(usize), // id of occupying file
}

impl DiskBlock {
    fn is_free(&self) -> bool {
        match self {
            DiskBlock::Free => true,
            DiskBlock::Taken(_) => false,
        }
    }
}

fn get_disk_map(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|length| {
            length
                .to_digit(10)
                .expect(&format!("Non-digit char {length}"))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = get_disk_map(input);

    let disk_size: u32 = disk_map.iter().sum();

    let mut disk = vec![DiskBlock::Free; disk_size as usize];

    let mut i = 0;

    for (d_idx, &length) in disk_map.iter().enumerate() {
        if d_idx % 2 == 0 {
            for _ in 0..length {
                disk[i] = DiskBlock::Taken(d_idx / 2);
                i += 1;
            }
        } else {
            i += length as usize;
        }
    }

    let disk2 = disk.clone();
    let mut free_blocks = disk2
        .iter()
        .enumerate()
        .filter(|(_, block)| block.is_free());

    for (block_idx, block) in disk2
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, block)| !block.is_free())
    {
        match block {
            DiskBlock::Free => {
                panic!("Got free block??")
            }
            DiskBlock::Taken(_) => {
                let (free_block_idx, _) = free_blocks.next().unwrap();

                if block_idx <= free_block_idx {
                    break;
                }

                disk[free_block_idx] = *block;
                disk[block_idx] = DiskBlock::Free;
            }
        }
    }

    let checksum = disk
        .iter()
        .enumerate()
        .map_while(|(block_idx, block)| match block {
            DiskBlock::Free => None,
            DiskBlock::Taken(file_id) => Some((block_idx * file_id) as u64),
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = get_disk_map(input);

    let sector_count = disk_map.len();

    let mut disk = Vec::with_capacity(sector_count);

    let mut i = 0;

    for (d_idx, &length) in disk_map.iter().enumerate() {
        if d_idx % 2 == 0 {
            disk.push(Sector {
                id: SectorId::File(d_idx / 2),
                start_block: i,
                length,
            });
        } else {
            disk.push(Sector {
                id: SectorId::Free,
                start_block: i,
                length,
            });
        }

        i += length as usize;
    }

    let disk2 = disk.clone();

    for sector in disk2.iter().rev().filter(|sec| !sec.is_free()) {
        let free_sector = disk
            .iter()
            .copied()
            .find_position(|sec| sec.is_free() && sec.length >= sector.length);

        match free_sector {
            None => {}
            Some((idx, free_sec)) => {
                let remaining = free_sec.length - sector.length;

                let (s_idx, _) = disk
                    .iter()
                    .find_position(|sec| sec.id == sector.id)
                    .unwrap();

                if s_idx < idx {
                    continue;
                }

                disk[s_idx] = Sector {
                    id: SectorId::Free,
                    start_block: sector.start_block,
                    length: sector.length,
                };
                disk[idx] = Sector {
                    id: sector.id,
                    start_block: free_sec.start_block,
                    length: sector.length,
                };
                disk.insert(
                    idx + 1,
                    Sector {
                        id: SectorId::Free,
                        start_block: free_sec.start_block + sector.length as usize,
                        length: remaining,
                    },
                )
            }
        }
    }

    let sum = disk
        .iter()
        .map(|sector| match sector.id {
            SectorId::Free => 0,
            SectorId::File(file_id) => {
                let mut sum = 0;

                for block in sector.start_block..(sector.start_block + sector.length as usize) {
                    sum += (file_id * block) as u64;
                }

                sum
            }
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
