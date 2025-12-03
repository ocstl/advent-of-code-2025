use radixal::IntoDigits;
use std::error::Error;
use std::num::ParseIntError;
use std::ops::RangeInclusive;

const INPUT: &str = "./inputs/day2.txt";

fn parse_input(input: &str) -> Result<Vec<RangeInclusive<u64>>, ParseIntError> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").expect("Invalid range.");
            let start = start.parse()?;
            let end = end.parse()?;
            Ok(RangeInclusive::new(start, end))
        })
        .collect()
}

fn part1(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| {
            range.clone().filter(|x| {
                let divider = 10_u64.pow((x.nbr_decimal_digits() / 2) as u32);
                x / divider == x % divider
            })
        })
        .sum()
}

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| {
            range.clone().filter(|x| {
                let l = x.nbr_decimal_digits();
                // Retain only lengths that evenly divide the number.
                (1..=l / 2).filter(|size| l % size == 0).any(|size| {
                    // Use the divider to break the number in equal-sized pieces, then compare
                    // each new piece with the first one.
                    let divider = 10_u64.pow(size as u32);
                    let initial = x % divider;
                    let mut remainder = x / divider;
                    while remainder > 0 {
                        if remainder % divider != initial {
                            return false;
                        }
                        remainder /= divider;
                    }
                    true
                })
            })
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let ranges = parse_input(&input)?;

    let part1 = part1(&ranges);
    println!("First answer: {}", part1);

    let part2 = part2(&ranges);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124\
    ";

    #[test]
    fn test_part1() {
        let ranges = parse_input(INPUT).unwrap();
        let result = part1(&ranges);
        const EXPECTED: u64 = 1227775554;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let ranges = parse_input(INPUT).unwrap();
        let result = part2(&ranges);
        const EXPECTED: u64 = 4174379265;
        assert_eq!(EXPECTED, result);
    }
}
