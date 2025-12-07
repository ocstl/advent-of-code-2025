use advent_of_code_2025::grid::{Direction, Grid, Position};
use std::collections::{HashMap, HashSet};
use std::error::Error;

const INPUT: &str = "./inputs/day7.txt";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Empty,
    Splitter,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            '^' => Tile::Splitter,
            _ => Tile::Empty,
        }
    }
}

type TachyonManifold = Grid<Tile>;

fn parse_input(input: &str) -> TachyonManifold {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();

    let grid = input
        .lines()
        .flat_map(|line| line.chars().map(Tile::from))
        .collect();

    TachyonManifold::new(height, width, grid).unwrap()
}

fn part1(tachyon_manifold: &TachyonManifold) -> usize {
    let start_position = tachyon_manifold
        .iter()
        .find(|&(_, &tile)| tile == Tile::Start)
        .unwrap()
        .0;

    let mut to_do = vec![start_position];
    let mut trajectory = HashSet::new();
    let mut count = 0;

    while let Some(position) = to_do.pop() {
        if trajectory.insert(position) && position.y() < tachyon_manifold.height() {
            if tachyon_manifold.get(position) == Some(&Tile::Splitter) {
                count += 1;
                to_do.push((position + Direction::Left).unwrap());
                to_do.push((position + Direction::Right).unwrap());
            } else {
                to_do.push((position + Direction::Down).unwrap());
            }
        }
    }

    count
}

fn part2(tachyon_manifold: &TachyonManifold) -> usize {
    //  We can use memoization to build from the end up.
    let mut known_trajectories = HashMap::new();
    fn recursive_fn(
        start_position: Position,
        known: &mut HashMap<Position, usize>,
        tachyon_manifold: &TachyonManifold,
    ) -> usize {
        if !known.contains_key(&start_position) {
            //  3 possibilities:
            //      - we've reached the bottom (no more splitting timelines)
            //      - we've reached a splitter (explore both timelines)
            //      - not the bottom and not a splitter, proceed downward.
            let count = if start_position.y() == tachyon_manifold.height() {
                1
            } else if tachyon_manifold.get(start_position) == Some(&Tile::Splitter) {
                recursive_fn(
                    (start_position + Direction::Left).unwrap(),
                    known,
                    tachyon_manifold,
                ) + recursive_fn(
                    (start_position + Direction::Right).unwrap(),
                    known,
                    tachyon_manifold,
                )
            } else {
                recursive_fn(
                    (start_position + Direction::Down).unwrap(),
                    known,
                    tachyon_manifold,
                )
            };

            known.insert(start_position, count);
        }

        *known.get(&start_position).unwrap()
    }

    let start_position = tachyon_manifold
        .iter()
        .find(|&(_, &tile)| tile == Tile::Start)
        .unwrap()
        .0;

    recursive_fn(start_position, &mut known_trajectories, tachyon_manifold)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let manifold = parse_input(&input);

    let part1 = part1(&manifold);
    println!("First answer: {}", part1);

    let part2 = part2(&manifold);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\
    ";

    #[test]
    fn test_part1() {
        let manifold = parse_input(INPUT);
        let result = part1(&manifold);

        const EXPECTED: usize = 21;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let manifold = parse_input(INPUT);
        let result = part2(&manifold);

        const EXPECTED: usize = 40;
        assert_eq!(EXPECTED, result);
    }
}
