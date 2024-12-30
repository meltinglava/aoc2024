use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn step(&self, grid_size: (i64, i64)) -> Robot {
        let (x, y) = self.position;
        let (vx, vy) = self.velocity;
        Robot {
            position: (
                (x + vx).rem_euclid(grid_size.0),
                (y + vy).rem_euclid(grid_size.1),
            ),
            velocity: self.velocity,
        }
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) = tuple((
        preceded(
            tag("p="),
            separated_pair(complete::i64, tag(","), complete::i64),
        ),
        preceded(
            tag(" v="),
            separated_pair(complete::i64, tag(","), complete::i64),
        ),
    ))(input)?;
    Ok((input, Robot { position, velocity }))
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Result<Vec<Robot>, nom::Err<nom::error::Error<String>>> {
    separated_list1(newline, parse_robot)(input)
        .map_err(crate::convert_error_to_owned)
        .map(|(_, v)| v)
}

#[aoc(day14, part1)]
fn part1(robots: &[Robot]) -> usize {
    hundred_seconds(robots, (101, 103))
}

fn hundred_seconds(robots: &[Robot], grid_size: (i64, i64)) -> usize {
    let mut quadrants = [0, 0, 0, 0];
    robots
        .iter()
        .map(|robot| robot_position_after_n_seconds(robot, grid_size, 100))
        .for_each(|pos| {
            let (x, y) = pos;
            let quadrant = (x.cmp(&(grid_size.0 / 2)), y.cmp(&(grid_size.1 / 2)));
            match quadrant {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[1] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[2] += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
                _ => {}
            }
        });
    quadrants.into_iter().product()
}

fn robot_position_after_n_seconds(robot: &Robot, grid_size: (i64, i64), n: usize) -> (i64, i64) {
    let (x, y) = robot.position;
    let (vx, vy) = robot.velocity;
    let mut pos = (x, y);
    (0..n).for_each(|_| {
        pos = (
            (pos.0 + vx).rem_euclid(grid_size.0),
            (pos.1 + vy).rem_euclid(grid_size.1),
        );
    });
    pos
    // let (x, y) = robot.position;
    // let (vx, vy) = robot.velocity;
    // ((x + 100 * vx) % grid_size.0, (y + 100 * vy) % grid_size.1)
}

fn _print_tree(tree: &HashSet<(i64, i64)>, grid_size: (i64, i64)) {
    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            if tree.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn is_candidate_tree(tree: &HashSet<(i64, i64)>, _grid_size: (i64, i64)) -> bool {
    // Check for consecutive positions along the y-axis (columns)
    let mut column_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for &(x, y) in tree {
        column_map.entry(x).or_default().push(y);
    }
    for y_values in column_map.values() {
        let mut sorted_y = y_values.clone();
        sorted_y.sort_unstable();
        let mut consecutive_count = 1;
        for i in 1..sorted_y.len() {
            if sorted_y[i] == sorted_y[i - 1] + 1 {
                consecutive_count += 1;
                if consecutive_count >= 10 {
                    return true;
                }
            } else {
                consecutive_count = 1;
            }
        }
    }

    // Check for consecutive positions along the x-axis (rows)
    let mut row_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for &(x, y) in tree {
        row_map.entry(y).or_default().push(x);
    }
    for x_values in row_map.values() {
        let mut sorted_x = x_values.clone();
        sorted_x.sort_unstable();
        let mut consecutive_count = 1;
        for i in 1..sorted_x.len() {
            if sorted_x[i] == sorted_x[i - 1] + 1 {
                consecutive_count += 1;
                if consecutive_count >= 10 {
                    return true;
                }
            } else {
                consecutive_count = 1;
            }
        }
    }

    false
}

fn step_tree(robots: &[Robot], grid_size: (i64, i64)) -> Vec<Robot> {
    robots.iter().map(|robot| robot.step(grid_size)).collect()
}

fn find_christmas_tree(robots: &[Robot], grid_size: (i64, i64)) -> usize {
    let mut step = 1;
    let mut tree = step_tree(robots, grid_size);
    loop {
        let tree_set = HashSet::from_iter(tree.iter().map(|robot| robot.position));
        if is_candidate_tree(&tree_set, grid_size) {
            return step;
        }
        tree = step_tree(&tree, grid_size);
        step += 1;
    }
}

#[aoc(day14, part2)]
fn part2(robots: &[Robot]) -> usize {
    find_christmas_tree(robots, (101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = indoc::indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn test_parse() {
        let res = parse_input(INPUT);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 12);
        assert_eq!(res[0].position, (0, 4));
        assert_eq!(res[0].velocity, (3, -3));
    }

    #[test]
    fn test_part1() {
        let res = parse_input(INPUT).unwrap();
        assert_eq!(hundred_seconds(&res, (11, 7)), 12);
    }

    #[test]
    fn test_steps() {
        let input = "p=2,4 v=2,-3";
        let robot = parse_input(input).unwrap().pop().unwrap();
        for i in 0..5 {
            println!(
                "{}: {:?}",
                i,
                robot_position_after_n_seconds(&robot, (11, 7), i)
            );
        }
        assert_eq!(robot_position_after_n_seconds(&robot, (11, 7), 4), (10, 6));
    }
}
