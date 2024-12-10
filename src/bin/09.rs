use std::collections::BTreeSet;

advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug)]
struct File {
    id: usize,
    start_block: usize,
    length: usize,
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

fn get_disk_map(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|length| {
            length
                .to_digit(10)
                .unwrap_or_else(|| panic!("Non-digit char {length}")) as usize
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = get_disk_map(input);

    let disk_size = disk_map.iter().sum();

    let mut disk = vec![DiskBlock::Free; disk_size];

    let mut i = 0;

    for (d_idx, &length) in disk_map.iter().enumerate() {
        if d_idx % 2 == 0 {
            for _ in 0..length {
                disk[i] = DiskBlock::Taken(d_idx / 2);
                i += 1;
            }
        } else {
            i += length;
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

    let mut files = Vec::with_capacity(sector_count / 2);
    let mut free_space: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 10];

    let mut block = 0;

    for (d_idx, &length) in disk_map.iter().enumerate() {
        if d_idx % 2 == 0 {
            files.push(File {
                id: d_idx / 2,
                start_block: block,
                length,
            });
        } else {
            free_space[length].insert(block);
        }

        block += length;
    }

    for file in files.iter_mut().rev() {
        let free_sector = free_space
            .iter()
            .enumerate()
            .filter(|&(length, _)| length >= file.length)
            .filter_map(|(sec_length, block_starts)| {
                let first = block_starts.first();
                first
                    .is_some_and(|&block| block <= file.start_block)
                    .then(|| (sec_length, *first.unwrap()))
            })
            .min_by_key(|sec| sec.1);

        if let Some((sec_length, free_sec_start_block)) = free_sector {
            file.start_block = free_sec_start_block;

            free_space[sec_length].remove(&free_sec_start_block);

            if sec_length > file.length {
                let remaining = sec_length - file.length;

                free_space[remaining].insert(free_sec_start_block + file.length);
            }
        }
    }

    let sum = files
        .iter()
        .map(|file| {
            let mut sum = 0;

            for block in file.start_block..(file.start_block + file.length) {
                sum += (file.id * block) as u64;
            }

            sum
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
