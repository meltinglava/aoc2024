use aoc_runner_derive::aoc;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self},
    sequence::{delimited, separated_pair},
};

fn mul(input: &str) -> nom::IResult<&str, (u64, u64)> {
    delimited(
        tag("mul("),
        separated_pair(complete::u64, tag(","), complete::u64),
        tag(")"),
    )(input)
}

fn enabled_tag(input: &str) -> nom::IResult<&str, bool> {
    let (i, string) = alt((tag("do()"), tag("don't()")))(input)?;
    Ok((i, string == "do()"))
}

#[aoc(day03, part1)]
fn part1(mut input: &str) -> u64 {
    let mut ans = 0;
    while !input.is_empty() {
        match mul(input) {
            Ok((i, (a, b))) => {
                input = i;
                ans += a * b;
            }
            Err(_) => {
                input = &input[1..];
            }
        }
    }
    ans
}

#[aoc(day03, part2)]
fn part2(mut input: &str) -> u64 {
    let mut ans = 0;
    let mut enabled = true;
    while !input.is_empty() {
        match mul(input) {
            Ok((i, (a, b))) => {
                input = i;
                if enabled {
                    ans += a * b;
                }
            }
            Err(_) => {
                if let Ok((i, e)) = enabled_tag(input) {
                    input = i;
                    enabled = e;
                } else {
                    input = &input[1..];
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        let value = part1(&INPUT1);
        assert_eq!(value, 161);
    }

    #[test]
    fn test_part_2() {
        let value = part2(&INPUT2);
        assert_eq!(value, 48);
    }
}
