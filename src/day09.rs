use aoc_runner_derive::{aoc, aoc_generator};

use std::{convert::identity, iter::repeat};

type Disk = Vec<Option<usize>>;
type DiskSlice = [Option<usize>];

#[aoc_generator(day09)]
fn parse_input(input: &str) -> Disk {
    let mut value = true;
    let mut bit_values = 0..;
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .flat_map(|d| {
            let v = if value {
                Some(bit_values.next().unwrap())
            } else {
                None
            };
            value = !value;
            repeat(v)
                .take(d)
        })
        .collect()
}

fn compact_disk_p1(d: &mut DiskSlice) {
    let mut left = 0;
    let mut right = d.len() - 1;
    while left < right {
        if d[left].is_none() {
            while d[right].is_none() {
                right -= 1;
            }
            d.swap(left, right);
            right -= 1;
        }
        left += 1;
    }
}

fn compact_disk_p2(d: &mut DiskSlice) {
    // Initialize the `right` pointer to the last index of the slice.
    let mut right = d.len() - 1;

    // Outer loop: Continue processing until the `right` pointer reaches 0.
    while 0 < right {
        let mut left = 0; // Initialize the `left` pointer to the start of the slice.

        // Find the rightmost `Some` value by decrementing `right` until it points to a `Some`.
        while d[right].is_none() {
            right -= 1;
        }

        // Store the value at the `right` pointer and initialize `width` to 1.
        let v_right = d[right];
        let mut width = 1;
        right -= 1; // Move `right` one position to the left.

        // Count contiguous `Some` values that match `v_right` to determine the `width` of the block.
        while d[right] == v_right && 0 != right {
            width += 1;
            right -= 1;
        }

        // Inner loop: Use the `left` pointer to find empty (`None`) spaces to fill with the block.
        while left < right {
            // Advance `left` to the next `None`.
            while d[left].is_some() {
                left += 1;
            }

            if left >= right {
                break;
            }

            let mut l_width = 1; // Width of the current empty block at `left`.
            left += 1; // Move past the current `None`.

            // Count the size of the empty (`None`) block starting at `left`.
            while left < right && d[left].is_none() && l_width < width {
                l_width += 1;
                left += 1;
            }

            // If the size of the empty block matches the `width` of the `Some` block...
            if l_width == width && left - width < right {
                // Move the block of `Some` values to fill the empty block.
                for i in 0..width {
                    d.swap(left - width + i, right + i + 1);
                }
            }
        }
    }
}

#[aoc(day09, part1)]
fn part1(input: &DiskSlice) -> usize {
    let mut disk: Disk = input.to_vec();
    compact_disk_p1(&mut disk);
    disk
        .into_iter()
        .filter_map(identity)
        .enumerate()
        .map(|(i, o)| i * o)
        .sum()
}

#[aoc(day09, part2)]
fn part2(input: &DiskSlice) -> usize {
    let mut disk: Disk = input.to_vec();
    compact_disk_p2(&mut disk);
    disk
        .into_iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|v| (i, v)))
        .map(|(i, o)| i * o)
        .sum()
}

#[cfg(test)]
mod test_day09 {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    fn print_disk(d: &DiskSlice) -> String {
        d.iter().map(|o| o.map(|v| v.to_string()).unwrap_or(".".to_string())).collect()
    }

    #[test]
    fn test_parse() {
        let parsed = parse_input(EXAMPLE);
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        let parsed_str = print_disk(&parsed);
        assert_eq!(parsed_str, expected);
    }

    #[test]
    fn test_compact_p1() {
        let mut parsed = parse_input(EXAMPLE);
        let expected = "0099811188827773336446555566..............";
        compact_disk_p1(&mut parsed);
        let parsed_str = print_disk(&parsed);
        assert_eq!(parsed_str, expected);
    }

    #[test]
    fn test_part1() {
        let parsed = parse_input(EXAMPLE);
        let part1_result = part1(&parsed);
        assert_eq!(part1_result, 1928);
    }

    #[test]
    fn test_compact_p2() {
        let mut parsed = parse_input(EXAMPLE);
        let expected = "00992111777.44.333....5555.6666.....8888..";
        compact_disk_p2(&mut parsed);
        let parsed_str = print_disk(&parsed);
        assert_eq!(parsed_str, expected);
    }

    #[test]
    fn test_part2() {
        let parsed = parse_input(EXAMPLE);
        let part2_result = part2(&parsed);
        assert_eq!(part2_result, 2858);
    }

    #[test]
    fn test_p2_value() {
        let correct = 6239783302560;
        //            6239783431260;
        let parsed = parse_input(include_str!("../input/2024/day9.txt").trim());
        let part2_result = part2(&parsed);
        assert_eq!(part2_result, correct);
    }
}
