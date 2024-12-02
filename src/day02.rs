use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    error::Error,
    multi::separated_list1,
};

use crate::convert_error_to_owned;

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<i64>>, nom::Err<Error<String>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" "),
            complete::i64
        ),
    )(input)
    .map(|(_, v)| v)
    .map_err(convert_error_to_owned)
}

#[aoc(day02, part1)]
fn count_safe(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|line| {
            line.windows(2)
                .map(|pair| (pair[0].cmp(&pair[1]), pair[0].abs_diff(pair[1])))
                .try_reduce(|acc, cmp| {
                    if acc.0 == cmp.0 && cmp.0 != Ordering::Equal && acc.1 <= 3 && cmp.1 <= 3 {
                        Some(cmp)
                    } else {
                        None
                    }
                })
                .is_some()
        })
        .count()
}

#[aoc(day02, part2)]
fn count_dampner_safe(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|line| {
            if count_safe(&[line.to_vec()]) == 1 {
                true
            } else {
                for index in 0..line.len() {
                    let mut line = line.to_vec();
                    line.remove(index);
                    if count_safe(&[line]) == 1 {
                        return true;
                    }
                }
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        1 2 3 4 4
        1 1 2 3 4
    "};

    #[test]
    fn test_part_1() {
        let values = input_generator(&INPUT).unwrap();
        assert_eq!(values.len(), 8);
        assert_eq!(count_safe(&values), 2);
    }

    #[test]
    fn test_part_2() {
        let values = input_generator(&INPUT).unwrap();
        assert_eq!(values.len(), 8);
        assert_eq!(count_dampner_safe(&values), 6);
    }
}
