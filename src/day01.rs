use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Err as NomErr,
};

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Result<Vec<(u64, u64)>, nom::Err<Error<String>>> {
    separated_list1(
        newline,
        separated_pair(complete::u64, tag("   "), complete::u64),
    )(input)
        .map(|(_, o)| o)
        .map_err(convert_error_to_owned)
}

/// Convert `nom::Err<Error<&str>>` to `nom::Err<Error<String>>`
fn convert_error_to_owned(e: NomErr<Error<&str>>) -> NomErr<Error<String>> {
    match e {
        NomErr::Incomplete(needed) => NomErr::Incomplete(needed),
        NomErr::Error(err) => NomErr::Error(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
        NomErr::Failure(err) => NomErr::Failure(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
    }
}

#[aoc(day01, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input.iter().copied().unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day01, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    let (left, raw_right): (Vec<u64>, Vec<u64>) = input.iter().copied().unzip();
    let mut right = HashMap::new();
    raw_right
        .into_iter()
        .for_each(|v| *right.entry(v).or_insert(0) += 1);
    left.into_iter()
        .filter_map(|n| right.get(&n).map(|v| v * n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &'static str = indoc!(
        "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "
    );

    #[test]
    fn test_part_1() {
        let values = input_generator(&INPUT).unwrap();
        assert_eq!(values.len(), 6);
        assert_eq!(part1(&values), 11);
    }

    #[test]
    fn test_part_2() {
        let values = input_generator(&INPUT).unwrap();
        assert_eq!(part2(&values), 31);
    }
}
