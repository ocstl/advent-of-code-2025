use advent_of_code_2025::grid::{Grid, Position};
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = "./inputs/day4.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Paper,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Paper),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PrintingDepartment(Grid<Tile>);

impl PrintingDepartment {
    fn removable_paper_rolls(&self) -> impl Iterator<Item = Position> {
        self.0.iter().filter_map(|(position, tile)| {
            if *tile == Tile::Paper
                && position
                    .extended_neighbours()
                    .filter_map(|neighbour| self.0.get(neighbour))
                    .filter(|&&t| t == Tile::Paper)
                    .count()
                    < 4
            {
                Some(position)
            } else {
                None
            }
        })
    }
}

impl FromStr for PrintingDepartment {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let grid = s
            .lines()
            .flat_map(|line| line.chars())
            .map(Tile::try_from)
            .collect::<Result<Vec<Tile>, _>>()?;

        Ok(Self(Grid::new(height, width, grid).unwrap()))
    }
}

fn part1(department: &PrintingDepartment) -> usize {
    department.removable_paper_rolls().count()
}

fn part2(department: &PrintingDepartment) -> usize {
    let mut department = department.clone();
    std::iter::from_fn(move || {
        let removable: Vec<Position> = department.removable_paper_rolls().collect();
        if removable.is_empty() {
            return None;
        }

        let count = removable.len();
        for position in removable {
            if let Some(tile) = department.0.get_mut(position) {
                *tile = Tile::Empty;
            }
        }

        Some(count)
    })
    .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let department: PrintingDepartment = input.parse().expect("Invalid input.");

    let part1 = part1(&department);
    println!("First answer: {}", part1);

    let part2 = part2(&department);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\
    ";

    #[test]
    fn test_part1() {
        let department: PrintingDepartment = INPUT.parse().unwrap();
        let result = part1(&department);
        const EXPECTED: usize = 13;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let department: PrintingDepartment = INPUT.parse().unwrap();
        let result = part2(&department);
        const EXPECTED: usize = 43;
        assert_eq!(EXPECTED, result);
    }
}
