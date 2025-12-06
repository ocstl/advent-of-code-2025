use std::error::Error;

const INPUT: &str = "./inputs/day6.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("Unknown operation: {}", value),
        }
    }
}

impl TryFrom<u8> for Operation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'*' => Ok(Operation::Multiply),
            b'+' => Ok(Operation::Add),
            _ => Err(value),
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let operations: Vec<Operation> = lines
        .next_back()
        .expect("invalid input")
        .split_whitespace()
        .map(Operation::from)
        .collect();
    let mut values: Vec<Vec<u64>> = vec![vec![]; operations.len()];

    for line in lines {
        for (idx, value) in line.split_whitespace().enumerate() {
            values[idx].push(value.parse::<u64>().unwrap());
        }
    }

    operations
        .into_iter()
        .zip(values)
        .map(|(operation, v)| match operation {
            Operation::Add => v.iter().sum::<u64>(),
            Operation::Multiply => v.iter().product(),
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut lines: Vec<_> = input.lines().map(|line| line.bytes().rev()).collect();
    let operations = lines.pop().unwrap();

    let mut sum = 0;
    let mut values = Vec::new();

    for operation in operations {
        let new_value = lines.iter_mut().fold(0, |acc, line| {
            line.next()
                .and_then(|v| v.checked_sub(b'0'))
                .map_or(acc, |v| 10 * acc + u64::from(v))
        });
        values.push(new_value);

        if let Ok(operation) = Operation::try_from(operation) {
            match operation {
                Operation::Add => sum += values.drain(..).sum::<u64>(),
                Operation::Multiply => sum += values.drain(..).filter(|v| *v != 0).product::<u64>(),
            }
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let part1 = part1(&input);
    println!("First answer: {}", part1);

    let part2 = part2(&input);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        123 328  51 64 \n\
         45 64  387 23 \n\
          6 98  215 314\n\
        *   +   *   +  \
    ";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        const EXPECTED: u64 = 4277556;
        assert_eq!(EXPECTED, result);
    }
    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        const EXPECTED: u64 = 3263827;
        assert_eq!(EXPECTED, result);
    }
}
