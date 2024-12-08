use std::collections::HashMap;

use itertools::Itertools;

type Point = (i64, i64);
type Antennas = HashMap<char, Vec<Point>>;

#[aoc_generator(day08)]
fn input_generator(input: &str) -> Result<(Point, Antennas), String> {
    let mut width = 0;
    let mut height = 0;

    let blocks = input
        .lines()
        .inspect(|line| {
            width = width.max(line.len());
            height += 1;
        })
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                c => Some((c, (x as i64, y as i64))),
            })
        })
        .fold(Antennas::new(), |mut a, (c, point)| {
            a.entry(c).or_default().push(point);
            a
        });
    Ok(((width as i64, height as i64), blocks))
}

#[aoc(day08, part1)]
fn part1(input: &(Point, Antennas)) -> usize {
    input
        .1
        .values()
        .flat_map(|v| v.iter().combinations(2))
        .flat_map(|v| v.into_iter().permutations(2))
        .map(|v| {
            let (x0, y0) = v[0];
            let (x1, y1) = v[1];
            (x0 + (x1 - x0) * 2, y0 + (y1 - y0) * 2)
        })
        .unique()
        .filter(|p| within_limits(*p, input.0))
        .count()
}

fn within_limits(p: Point, limit: Point) -> bool {
    (0..limit.0).contains(&p.0) && (0..limit.1).contains(&p.1)
}

#[aoc(day08, part2)]
fn part2(input: &(Point, Antennas)) -> usize {
    input
        .1
        .values()
        .flat_map(|v| v.iter().combinations(2))
        .flat_map(|v| v.into_iter().permutations(2))
        .flat_map(|v| {
            let mut hits: Vec<Point> = v.iter().copied().copied().collect_vec();
            let (x0, y0) = v[0];
            let (x1, y1) = v[1];
            let dx = x1 - x0;
            let dy = y1 - y0;
            let mut steps = 2..;
            let mut step = steps.next().unwrap();
            let mut point = (x0 + dx * step, y0 + dy * step);
            while within_limits(point, input.0) {
                hits.push(point);
                step = steps.next().unwrap();
                point = (x0 + dx * step, y0 + dy * step);
            }
            hits
        })
        .unique()
        .count()
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    const INPUT2: &str = indoc! {"
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
    "};

    #[test]
    fn test_parsing() {
        let i = input_generator(INPUT).unwrap();
        let count = i.1.values().flatten().count();
        assert_eq!(count, 7)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT).unwrap()), 14)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT).unwrap()), 34)
    }

    #[test]
    fn test_part2_input2() {
        assert_eq!(part2(&input_generator(INPUT2).unwrap()), 9)
    }
}
