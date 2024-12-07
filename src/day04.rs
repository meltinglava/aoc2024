use std::ops::Not;

use aoc_runner_derive::aoc;

use crate::grid::{Direction, Grid};

pub fn input_generator<const N: usize>(input: &str) -> Result<Grid<N, char>, String> {
    let mut grid = Grid::default();
    let mut count = 0;
    input.lines().enumerate().for_each(|(line_nr, line)| {
        line.chars().enumerate().for_each(|(char_nr, c)| {
            grid[(char_nr, line_nr)] = c;
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

fn part1<const N: usize>(grid: &Grid<N, char>) -> usize {
    let directions = Direction::all();
    let search_for = ['X', 'M', 'A', 'S'];
    grid.iter()
        .filter(|(_, c)| *c == &'X')
        .map(|(pos, _)| {
            directions
                .iter()
                .filter(|dir| {
                    let mut search_pos = Some(pos);
                    let valid = search_for
                        .iter()
                        .map(|search| {
                            let found = grid.get(search_pos?)?;
                            search_pos = dir.step(search_pos?, N);
                            Some(found == search)
                        })
                        .all(|x| x == Some(true));
                    valid
                })
                .count()
        })
        .sum()
}

#[aoc(day04, part1)]
fn solve_part1(input: &str) -> Result<usize, String> {
    Ok(part1(&input_generator::<140>(input)?))
}

fn part2<const N: usize>(grid: &Grid<N, char>) -> usize {
    let directions = [Direction::NorthEast, Direction::SouthEast];
    grid.iter()
        .filter(|(_, c)| *c == &'A')
        .filter(|(pos, _)| {
            directions
                .iter()
                .map(|dir| {
                    let direction = grid.get(dir.step(*pos, N)?)?;
                    let opposite = grid.get(dir.not().step(*pos, N)?)?;
                    Some(
                        direction == &'M' && opposite == &'S'
                            || direction == &'S' && opposite == &'M',
                    )
                })
                .all(|x| x == Some(true))
        })
        .count()
}

#[aoc(day04, part2)]
fn solve_part2(input: &str) -> Result<usize, String> {
    Ok(part2(&input_generator::<140>(input)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &'static str = indoc!(
        "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "
    );

    #[test]
    fn test_part_1() {
        let parsed = input_generator::<10>(INPUT).unwrap();
        assert_eq!(part1(&parsed), 18);
    }

    #[test]
    fn test_part_2() {
        let parsed = input_generator::<10>(INPUT).unwrap();
        assert_eq!(part2(&parsed), 9);
    }
}
