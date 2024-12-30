use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_coords(input: &str) -> IResult<&str, (i64, i64)> {
    let x = preceded(alt((tag("X="), tag("X+"))), complete::i64);
    let y = preceded(alt((tag("Y="), tag("Y+"))), complete::i64);
    separated_pair(x, tag(", "), y)(input)
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, _) = tag("Button A: ")(input)?;
    let (input, a) = parse_coords(input)?;
    let (input, _) = tag("\nButton B: ")(input)?;
    let (input, b) = parse_coords(input)?;
    let (input, _) = tag("\nPrize: ")(input)?;
    let (input, prize) = parse_coords(input)?;

    Ok((input, ClawMachine { a, b, prize }))
}

fn parse_claw_machines(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(tag("\n\n"), parse_claw_machine)(input)
}

#[aoc_generator(day13, part1)]
fn input_generator(input: &str) -> Result<Vec<ClawMachine>, nom::Err<nom::error::Error<String>>> {
    parse_claw_machines(input)
        .map_err(crate::convert_error_to_owned)
        .map(|(_, machines)| machines)
}

#[aoc_generator(day13, part2)]
fn input_generator_part2(
    input: &str,
) -> Result<Vec<ClawMachine>, nom::Err<nom::error::Error<String>>> {
    let mut machines = parse_claw_machines(input)
        .map_err(crate::convert_error_to_owned)
        .map(|(_, machines)| machines)?;
    machines.iter_mut().for_each(|machine| {
        machine.prize = (
            machine.prize.0 + 10000000000000,
            machine.prize.1 + 10000000000000,
        );
    });
    Ok(machines)
}

// a = press a
// b = press b
// px = prize x
// py = prize y
// ax = distance a x
// ay = distance a y
// bx = distance b x
// by = distance b y
//
// a * ax + b * bx = px
// a * ay + b * by = py
// a = (px * by - py * bx) / (ax * by - ay * bx)
// f(b) =

fn calqulate_tokens(claw_machine: &ClawMachine, part2: bool) -> Option<i64> {
    let (ax, ay) = claw_machine.a;
    let (bx, by) = claw_machine.b;
    let (px, py) = claw_machine.prize;

    let a_nominator = px * by - py * bx;
    let a_denominator = ax * by - ay * bx;
    let b_nominator = px * ay - py * ax;
    let b_denominator = bx * ay - by * ax;

    if a_denominator == 0 || b_denominator == 0 {
        return None;
    }

    if a_nominator % a_denominator != 0 || b_nominator % b_denominator != 0 {
        return None;
    }

    let a = (px * by - py * bx) / (ax * by - ay * bx);
    let b = (px * ay - py * ax) / (bx * ay - by * ax);

    if !part2 && (a >= 100 || b >= 100) {
        return None;
    }

    Some(3 * a + b)
}

#[aoc(day13, part1)]
fn part1(machines: &[ClawMachine]) -> i64 {
    machines
        .iter()
        .filter_map(|c| calqulate_tokens(c, false))
        .sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[ClawMachine]) -> i64 {
    machines
        .iter()
        .filter_map(|c| calqulate_tokens(c, true))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_parse() {
        let (_, machines) = parse_claw_machines(TEST_INPUT).unwrap();
        assert_eq!(machines.len(), 4);
    }

    #[test]
    fn test_coords() {
        parse_coords("X=8400, Y=5400").unwrap();
        parse_coords("X+8400, Y+5400").unwrap();
    }

    #[test]
    fn test_calqulate_tokes() {
        let machine = ClawMachine {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        };
        assert_eq!(calqulate_tokens(&machine, false), Some(280));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(TEST_INPUT).unwrap()), 480);
    }
}
