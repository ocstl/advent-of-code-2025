use advent_of_code_2025::grid::{Grid, Position};
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = "./inputs/day4.txt";
const PAPER_ROLL: char = '@';

#[derive(Debug, Clone)]
struct PrintingDepartment(Grid<bool>);

impl PrintingDepartment {
    fn removable_rolls(&self) -> impl Iterator<Item = Position> {
        self.0.iter().filter_map(|(position, tile)| {
            if *tile
                && position
                    .extended_neighbours()
                    .filter_map(|neighbour| self.0.get(neighbour))
                    .filter(|tile| **tile)
                    .count()
                    < 4
            {
                Some(position)
            } else {
                None
            }
        })
    }

    fn remove_rolls(&mut self, positions: impl Iterator<Item = Position>) {
        for p in positions {
            *self.0.get_mut(p).unwrap() = false;
        }
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
            .map(|t| t == PAPER_ROLL)
            .collect();

        Ok(Self(Grid::new(height, width, grid).unwrap()))
    }
}

fn part1(printing_department: &PrintingDepartment) -> usize {
    printing_department.removable_rolls().count()
}

fn part2(printing_department: &PrintingDepartment) -> usize {
    let mut printing_department = printing_department.clone();
    std::iter::from_fn(move || {
        let removable: Vec<Position> = printing_department.removable_rolls().collect();
        if removable.is_empty() {
            return None;
        }

        let n = removable.len();
        printing_department.remove_rolls(removable.into_iter());
        Some(n)
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
