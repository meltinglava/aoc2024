use aoc_runner_derive::{aoc, aoc_generator};
use nom::{character::complete::{self, space1}, error::Error, multi::separated_list1};

use crate::convert_error_to_owned;

use dp_macro::dp;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Result<Vec<u64>, nom::Err<Error<String>>> {
    separated_list1(space1, complete::u64)(input).map_err(convert_error_to_owned).map(|(_, v)| v)
}

#[dp]
fn solve(x: u64, steps: u64) -> u64 {
    if steps == 0 {
        return 1
    } else if x == 0 {
        return solve(1, steps - 1)
    }
    let number_of_digits = x.ilog10() + 1;
    if number_of_digits % 2 == 0 {
        let split = 10u64.pow(number_of_digits / 2);
        let split_value = x % split;
        solve(x / split, steps-1) + solve(split_value, steps-1)
    } else {
        solve(x * 2024, steps-1)
    }
}

fn solve_part(input: &[u64], steps: u64) -> u64 {
    input.iter().map(|x| solve(*x, steps)).sum()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    solve_part(input, 25)

}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    solve_part(input, 75)
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_parse() {
        assert_eq!(parse_input(EXAMPLE).unwrap(), vec![125, 17]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE).unwrap()), 55312);
    }
}
