use good_lp::{
    Expression, Solution, SolverModel, Variable, constraint, default_solver, variable, variables,
};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./inputs/day10.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    to_start: u32,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    const ON: char = '#';
}

impl FromStr for Machine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let to_start = iter
            .next()
            .unwrap()
            .trim_prefix('[')
            .trim_suffix(']')
            .char_indices()
            .fold(0, |acc, (idx, c)| {
                if c == Machine::ON {
                    acc ^ (1 << idx)
                } else {
                    acc
                }
            });

        let joltages: Vec<u32> = iter
            .next_back()
            .unwrap()
            .trim_prefix('{')
            .trim_suffix('}')
            .split(',')
            .map(|b| b.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;

        let buttons = iter
            .map(|b| {
                b.trim_prefix('(')
                    .trim_suffix(')')
                    .split(',')
                    .map(|b| b.parse::<usize>())
                    .collect::<Result<Vec<usize>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<usize>>, ParseIntError>>()?;

        Ok(Self {
            to_start,
            buttons,
            joltages,
        })
    }
}

fn part1(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| {
            // Convert the buttons to u32 (bit fields), then do a BFS.
            let buttons: Vec<u32> = machine
                .buttons
                .iter()
                .map(|button| button.iter().fold(0, |acc, x| acc ^ (1 << x)))
                .collect();

            let mut to_visit = VecDeque::new();
            to_visit.push_back((0, 0));
            let mut visited = HashSet::new();

            while let Some((presses, state)) = to_visit.pop_front() {
                if visited.insert(state) {
                    if state == machine.to_start {
                        return presses;
                    }

                    to_visit.extend(buttons.iter().map(|button| (presses + 1, button ^ state)))
                }
            }

            unreachable!()
        })
        .sum()
}

fn part2(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| {
            // Integer linear programming problem.
            let mut vars = variables!();
            let presses: Vec<Variable> = machine
                .buttons
                .iter()
                .map(|_| vars.add(variable().integer().min(0)))
                .collect();
            let objective: Expression = presses.iter().sum();

            // Build constraints for each joltage.
            let mut model = vars.minimise(objective).using(default_solver);
            for (idx, j) in machine.joltages.iter().enumerate() {
                let mut expr = Expression::from(0);
                for (idy, affected_joltage) in machine.buttons.iter().enumerate() {
                    if affected_joltage.contains(&idx) {
                        expr += presses[idy];
                    }
                }
                model = model.with(constraint!(expr == *j));
            }

            let solution = model.solve().unwrap();
            presses
                .iter()
                .map(|v| solution.value(*v).round() as u32)
                .sum::<u32>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let machines = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Machine>, _>>()?;

    let part1 = part1(&machines);
    println!("First answer: {}", part1);

    let part2 = part2(&machines);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\
    ";

    #[test]
    fn test_part1() {
        let machines = INPUT
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Machine>, _>>()
            .unwrap();
        let result = part1(&machines);

        const EXPECTED: u32 = 7;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let machines = INPUT
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Machine>, _>>()
            .unwrap();
        let result = part2(&machines);

        const EXPECTED: u32 = 33;
        assert_eq!(EXPECTED, result);
    }
}
