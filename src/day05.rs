use indexmap::IndexSet;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::iterator,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::convert_error_to_owned;

#[derive(Debug)]
struct PageSetup {
    rule: IndexSet<(u64, u64)>,
    produce: Vec<IndexSet<u64>>,
}

fn parse_pair(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(complete::u64, tag("|"), complete::u64)(input)
}

fn parse_ordering_rules(input: &str) -> IResult<&str, IndexSet<(u64, u64)>> {
    let mut ordering_iterator = iterator(input, terminated(parse_pair, newline));
    let set = ordering_iterator.collect();
    let res = ordering_iterator.finish();
    res.map(move |(s, _)| (s, set))
}

fn parse_produce_line(input: &str) -> IResult<&str, IndexSet<u64>> {
    separated_list1(tag(","), complete::u64)(input).map(|(s, n)| (s, n.into_iter().collect()))
}

fn parse_produce(input: &str) -> IResult<&str, Vec<IndexSet<u64>>> {
    separated_list1(newline, parse_produce_line)(input)
}

fn parse_input(input: &str) -> Result<PageSetup, nom::Err<Error<&str>>> {
    separated_pair(parse_ordering_rules, newline, parse_produce)(input)
        .map(|(_, (rule, produce))| PageSetup { rule, produce })
}

#[aoc_generator(day05)]
fn parse_actual_input(input: &str) -> Result<PageSetup, nom::Err<Error<String>>> {
    parse_input(input).map_err(convert_error_to_owned)
}

fn page_follows_rules(pages: &PageSetup, p: &IndexSet<u64>) -> bool {
    pages
        .rule
        .iter()
        .map(|rule| {
            let start_index = p.get_full(&rule.0)?.0;
            let end_index = p.get_full(&rule.1)?.0;
            Some(start_index < end_index)
        })
        .all(|n| n != Some(false))
}

#[aoc(day05, part1)]
fn part_1(pages: &PageSetup) -> u64 {
    pages
        .produce
        .iter()
        .filter(|p| page_follows_rules(pages, p))
        .map(|n| n.get_index(n.len() / 2).unwrap())
        .sum()
}

#[aoc(day05, part2)]
fn part_2(pages: &PageSetup) -> u64 {
    pages
        .produce
        .iter()
        .filter(|p| !page_follows_rules(pages, p))
        .map(|p| {
            let mut line = p.clone();
            let mut rules = pages.rule.iter().cycle();
            loop {
                let rule = rules.next().unwrap();
                let start_index = match line.get_full(&rule.0) {
                    Some(s) => s.0,
                    None => continue,
                };
                let end_index = match line.get_full(&rule.1) {
                    Some(s) => s.0,
                    None => continue,
                };
                if start_index > end_index {
                    line.swap_indices(start_index, end_index);
                    if page_follows_rules(pages, &line) {
                        break line;
                    }
                }
            }
        })
        .map(|n| *n.get_index(n.len() / 2).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_parsing() {
        parse_input(&INPUT).unwrap();
    }

    #[test]
    fn test_part_1() {
        let input = parse_input(&INPUT).unwrap();
        let ans = part_1(&input);
        assert_eq!(ans, 143);
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(&INPUT).unwrap();
        let ans = part_2(&input);
        assert_eq!(ans, 123);
    }
}
