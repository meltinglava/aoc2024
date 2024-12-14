use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::grid::{Direction, Grid};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<Grid<55, usize>, String> {
    input_generator(input)
}

fn input_generator<const N: usize>(input: &str) -> Result<Grid<N, usize>, String> {
    let mut grid = Grid::default();
    let mut count = 0;
    input.lines().enumerate().for_each(|(line_nr, line)| {
        line.chars().enumerate().for_each(|(char_nr, c)| {
            grid[(char_nr, line_nr)] = c.to_digit(10).unwrap() as usize;
            count += 1;
        })
    });
    if count == N * N {
        Ok(grid)
    } else {
        Err(format!(
            "Parsed: {} characters, expected: {} characters",
            count,
            N * N
        ))
    }
}

fn step_to_nine<const N: usize>(grid: &Grid<N, usize>, pos: (usize, usize), value: usize, tailHeads: &mut HashSet<(usize, usize)>) -> usize {
    if grid[pos] == 9 && value == 9 {
        tailHeads.insert(pos);
        return 1;
    } else if grid[pos] != value {
        return 0;
    }
    Direction::cardinal().iter().filter_map(|d| d.step(pos, N)).map(|dir| {
        step_to_nine(grid, dir, value + 1, tailHeads)
    })
        .sum()
}

fn step_to_nine_part2<const N: usize>(grid: &Grid<N, usize>, pos: (usize, usize), value: usize) -> usize {
    if grid[pos] == 9 && value == 9 {
        return 1;
    } else if grid[pos] != value {
        return 0;
    }
    Direction::cardinal().iter().filter_map(|d| d.step(pos, N)).map(|dir| {
        step_to_nine_part2(grid, dir, value + 1)
    })
        .sum()
}

#[aoc(day10, part1)]
fn part1<const N: usize>(grid: &Grid<N, usize>) -> usize {
    grid
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(pos, _)| {
            let mut set = HashSet::new();
            step_to_nine(grid, pos, 0, &mut set);
            set.len()
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2<const N: usize>(grid: &Grid<N, usize>) -> usize {
    grid
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(pos, _)| {
            step_to_nine_part2(grid, pos, 0)
        })
        .sum()
}

#[cfg(test)]
mod tests_day10 {
    use super::*;

    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_parse() {
        let grid = input_generator::<8>(TEST_INPUT).unwrap();
        assert_eq!(grid[(0, 0)], 8);
        assert_eq!(grid[(7, 7)], 2);
    }

    #[test]
    fn test_part1() {
        let grid = input_generator::<8>(TEST_INPUT).unwrap();
        assert_eq!(part1(&grid), 36);
    }

    #[test]
    fn test_part2() {
        let grid = input_generator::<8>(TEST_INPUT).unwrap();
        assert_eq!(part2(&grid), 81);
    }
}
