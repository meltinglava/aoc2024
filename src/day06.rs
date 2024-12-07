use std::{cell::RefCell, collections::HashSet};

use crate::grid::Direction;

type Point = (usize, usize);

struct Maze {
    blocks: HashSet<Point>,
    start_pos: Point,
    size: Point,
}

#[aoc_generator(day06)]
fn parse(input: &str) -> Result<Maze, String> {
    let start_pos = RefCell::new(None);
    let start_pos = &start_pos;
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
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '^' {
                    *start_pos.borrow_mut() = Some((x, y));
                    None
                } else if c == '#' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    Ok(Maze {
        blocks,
        start_pos: start_pos
            .clone()
            .into_inner()
            .ok_or_else(|| "No start position found".to_string())?,
        size: (width, height),
    })
}

fn get_steps(maze: &Maze) -> Option<usize> {
    let mut direction = Direction::North;
    let mut pos = maze.start_pos;
    let mut visited = HashSet::new();
    let mut loop_finder = HashSet::new();
    visited.insert(pos);
    loop_finder.insert((pos, direction));
    assert_eq!(maze.size.0, maze.size.1);
    let size = maze.size.0;

    while let Some(next_step) = direction.step(pos, size) {
        if maze.blocks.contains(&next_step) {
            direction = direction.right_turn();
        } else {
            visited.insert(next_step);
            pos = next_step;
        }

        if !loop_finder.insert((pos, direction)) {
            return None;
        }
    }

    Some(visited.len())
}

#[aoc(day06, part1)]
fn part_1(maze: &Maze) -> usize {
    get_steps(maze).unwrap()
}

#[aoc(day06, part2)]
fn part_2(maze: &Maze) -> usize {
    (0..maze.size.1)
        .flat_map(|y| (0..maze.size.0).map(move |x| (x, y)))
        .filter(|&pos| !maze.blocks.contains(&pos))
        .filter(|&pos| pos != maze.start_pos)
        .filter(|&pos| {
            let mut m = maze.blocks.clone();
            m.insert(pos);
            get_steps(&Maze {
                blocks: m,
                start_pos: maze.start_pos,
                size: maze.size,
            })
            .is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_part_1() {
        let result = parse(TEST_INPUT).unwrap();
        assert_eq!(part_1(&result), 41);
    }

    #[test]
    fn test_part_2() {
        let result = parse(TEST_INPUT).unwrap();
        assert_eq!(part_2(&result), 6);
    }
}
