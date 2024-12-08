use std::collections::VecDeque;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};

struct Equation {
    target: u64,
    numbers: VecDeque<u64>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(tag(" "), complete::u64),
    )(input)
    .map(|(rest, (target, numbers))| {
        (
            rest,
            Equation {
                target,
                numbers: numbers.into_iter().collect(),
            },
        )
    })
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(complete::newline, parse_equation)(input)
}

#[aoc_generator(day07)]
fn input_generator(input: &str) -> Vec<Equation> {
    parse_input(input).unwrap().1
}

fn solvable_part1(equation: &Equation) -> bool {
    let mut values = equation.numbers.clone();
    let first = values.pop_front().unwrap();
    if values.is_empty() {
        return first == equation.target;
    }
    if first > equation.target {
        return false;
    }
    let second = values.front().unwrap();
    let mut adding = values.clone();
    *adding.front_mut().unwrap() = first + second;
    if solvable_part1(&Equation {
        target: equation.target,
        numbers: adding,
    }) {
        true
    } else {
        *values.front_mut().unwrap() = first * second;
        solvable_part1(&Equation {
            target: equation.target,
            numbers: values,
        })
    }
}

fn solvable_part2(equation: &Equation) -> bool {
    let mut values = equation.numbers.clone();
    let first = values.pop_front().unwrap();
    if values.is_empty() {
        return first == equation.target;
    }
    if first > equation.target {
        return false;
    }
    let second = values.front().unwrap();
    let mut adding = values.clone();
    *adding.front_mut().unwrap() = first + second;
    if solvable_part2(&Equation {
        target: equation.target,
        numbers: adding,
    }) {
        true
    } else {
        let mut multiplying = values.clone();
        *multiplying.front_mut().unwrap() = first * second;
        if solvable_part2(&Equation {
            target: equation.target,
            numbers: multiplying,
        }) {
            true
        } else {
            *values.front_mut().unwrap() = concatinate(first, *second);
            solvable_part2(&Equation {
                target: equation.target,
                numbers: values,
            })
        }
    }
}

fn concatinate(first: u64, second: u64) -> u64 {
    let offset = second.ilog10() + 1;
    first * 10u64.pow(offset) + second
}

#[aoc(day07, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|equation| solvable_part1(equation))
        .map(|equation| equation.target)
        .sum()
}

#[aoc(day07, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|equation| solvable_part2(equation))
        .map(|equation| equation.target)
        .sum()
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_parsing() {
        let input = input_generator(INPUT);
        assert_eq!(input.len(), 9);
        assert_eq!(input[0].target, 190);
        assert_eq!(input[0].numbers, vec![10, 19]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 11387);
    }
}
