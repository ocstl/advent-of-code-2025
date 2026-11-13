use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./inputs/day3.txt";

struct Bank(Vec<u64>);

impl Bank {
    fn maximum_joltage(&self, nbr_batteries: usize) -> u64 {
        // We want the leftmost maximum joltage, but also to keep enough batteries on the right
        // to have the required number of batteries.
        let mut joltage = 0;
        let mut available_batteries: &[u64] = self.0.as_ref();

        // Since `max` retains the last element that is the maximum, we use a reverse iterator.
        // Also, skip the number of batteries we need on the right.
        // We can then use the index to restrict the slice.
        for battery in 1..=nbr_batteries {
            let (idx, j) = available_batteries
                .iter()
                .enumerate()
                .rev()
                .skip(nbr_batteries - battery)
                .max_by_key(|&(_, joltage)| joltage)
                .unwrap();
            joltage = joltage * 10 + j;
            available_batteries = &available_batteries[idx + 1..];
        }

        joltage
    }
}

impl FromStr for Bank {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| c.to_digit(10).expect("Invalid joltage.").into())
                .collect(),
        ))
    }
}

fn part1(banks: &[Bank]) -> u64 {
    banks.iter().map(|b| b.maximum_joltage(2)).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    banks.iter().map(|b| b.maximum_joltage(12)).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let banks: Vec<Bank> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let part1 = part1(&banks);
    println!("First answer: {}", part1);

    let part2 = part2(&banks);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\n\
    ";

    #[test]
    fn test_part1() {
        let banks: Vec<Bank> = INPUT
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let result = part1(&banks);
        const EXPECTED: u64 = 357;
        assert_eq!(EXPECTED, result);
    }
    #[test]
    fn test_part2() {
        let banks: Vec<Bank> = INPUT
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let result = part2(&banks);
        const EXPECTED: u64 = 3121910778619;
        assert_eq!(EXPECTED, result);
    }
}
