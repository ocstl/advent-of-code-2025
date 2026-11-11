use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./inputs/day1.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Right(i32),
    Left(i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("R", val) => Ok(Self::Right(val.parse()?)),
            ("L", val) => Ok(Self::Left(val.parse()?)),
            _ => panic!("Invalid instruction."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dial {
    state: i32,
}

impl Dial {
    const INITIAL_STATE: i32 = 50;
    const SLOTS: i32 = 100;

    fn turn(&mut self, instruction: Instruction) -> i32 {
        match instruction {
            Instruction::Left(x) => {
                // When using `div_euclid`, we have to be careful.
                // If we were already at 0, the click was counted previously, so we remove it.
                // If we are now at 0, that click is not counted by the division, so we include it.
                let clicks = if self.state == 0 { -1 } else { 0 };
                let new_state = self.state - x;
                self.state = new_state.rem_euclid(Self::SLOTS);
                clicks
                    + new_state.div_euclid(Self::SLOTS).abs()
                    + if self.state == 0 { 1 } else { 0 }
            }
            Instruction::Right(x) => {
                let new_state = self.state + x;
                self.state = new_state.rem_euclid(Self::SLOTS);
                new_state.div_euclid(Self::SLOTS)
            }
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            state: Self::INITIAL_STATE,
        }
    }
}

fn part1(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .scan(Dial::default(), |dial, &instruction| {
            dial.turn(instruction);
            Some(dial.state)
        })
        .filter(|&dial| dial == 0)
        .count() as i32
}

fn part2(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .scan(Dial::default(), |dial, &instruction| {
            Some(dial.turn(instruction))
        })
        .sum()
}

fn read_instructions(input: &str) -> Result<Vec<Instruction>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let instructions = read_instructions(&input)?;

    let part1 = part1(&instructions);
    println!("First answer: {}", part1);

    let part2 = part2(&instructions);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\
    ";

    #[test]
    fn test_part1() {
        const EXPECTED: i32 = 3;
        let instructions = read_instructions(INPUT).unwrap();
        let result = part1(&instructions);

        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        const EXPECTED: i32 = 6;
        let instructions = read_instructions(INPUT).unwrap();
        let result = part2(&instructions);

        assert_eq!(EXPECTED, result);
    }
}
