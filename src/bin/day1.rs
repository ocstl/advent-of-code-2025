use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./inputs/day1.txt";
const SLOTS: i32 = 100;
const DIAL_START: i32 = 50;

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

fn read_instructions(input: &str) -> Result<Vec<Instruction>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn part1(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .scan(DIAL_START, |state, instruction| {
            match instruction {
                Instruction::Right(val) => *state = (*state + val).rem_euclid(SLOTS),
                Instruction::Left(val) => *state = (*state - val).rem_euclid(SLOTS),
            }
            Some(*state)
        })
        .filter(|dial| *dial == 0)
        .count() as i32
}

fn part2(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .fold((DIAL_START, 0), |state, instruction| {
            match instruction {
                Instruction::Right(val) => {
                    let dial = state.0 + val;
                    let clicks = state.1 + dial / SLOTS;
                    (dial % SLOTS, clicks)
                }
                Instruction::Left(val) => {
                    let dial = state.0 - val;
                    let clicks = state.1 + dial.div_euclid(SLOTS).abs();

                    // We need to adjust for two cases when turning left:
                    //  - if we were already at 0, we have already counted that click;
                    //  - if we are at 0, we need to count that click.
                    let dial = dial.rem_euclid(SLOTS);
                    let clicks =
                        clicks - if state.0 == 0 { 1 } else { 0 } + if dial == 0 { 1 } else { 0 };
                    (dial, clicks)
                }
            }
        })
        .1
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
