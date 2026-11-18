use std::cmp::Reverse;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = "./inputs/day8.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap_or_default().parse()?;
        let y = iter.next().unwrap_or_default().parse()?;
        let z = iter.next().unwrap_or_default().parse()?;

        Ok(Self::new(x, y, z))
    }
}

type Circuit = HashSet<JunctionBox>;

fn part1(boxes: &[JunctionBox], nbr_connections: usize) -> usize {
    let mut circuits: Vec<Circuit> = boxes.iter().map(|b| HashSet::from([*b])).collect();
    let mut distances: Vec<(u64, JunctionBox, JunctionBox)> = boxes
        .iter()
        .enumerate()
        .flat_map(|(idx, a)| boxes.iter().skip(idx + 1).map(|b| (a.distance(b), *a, *b)))
        .collect();
    distances.sort_unstable_by_key(|(d, _, _)| *d);

    for &(_, first, second) in distances.iter().take(nbr_connections) {
        // Extract the first circuit.
        let mut first_circuit = circuits
            .iter()
            .position(|c| c.contains(&first))
            .map(|c| circuits.swap_remove(c))
            .unwrap();

        // If the first circuit already contains the second box, we can push it back. Otherwise,
        // grab the second circuit and combine them.
        // If we can find a second circuit, combine them. Otherwise, it means the second box is
        // already in the first circuit.
        if !first_circuit.contains(&second) {
            let second_circuit = circuits
                .iter()
                .position(|c| c.contains(&second))
                .map(|c| circuits.swap_remove(c))
                .unwrap();
            first_circuit.extend(second_circuit);
        }

        circuits.push(first_circuit);
    }

    let mut nodes_count: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    nodes_count.sort_unstable();
    nodes_count.iter().rev().take(3).product()
}

fn part2(boxes: &[JunctionBox]) -> u64 {
    let mut circuits: Vec<Circuit> = boxes.iter().map(|b| HashSet::from([*b])).collect();
    let mut distances: Vec<(u64, JunctionBox, JunctionBox)> = boxes
        .iter()
        .enumerate()
        .flat_map(|(idx, a)| boxes.iter().skip(idx + 1).map(|b| (a.distance(b), *a, *b)))
        .collect();
    distances.sort_unstable_by_key(|(d, _, _)| Reverse(*d));

    // Keep track of the last two boxes' result.
    let mut wall_distance = 0;
    while circuits.len() > 1 {
        let (_, first, second) = distances.pop().unwrap();
        wall_distance = first.x * second.x;

        // Extract the first circuit.
        let mut first_circuit = circuits
            .iter()
            .position(|c| c.contains(&first))
            .map(|c| circuits.swap_remove(c))
            .unwrap();

        // If the first circuit already contains the second box, we can push it back. Otherwise,
        // grab the second circuit and combine them.
        // If we can find a second circuit, combine them. Otherwise, it means the second box is
        // already in the first circuit.
        if !first_circuit.contains(&second) {
            let second_circuit = circuits
                .iter()
                .position(|c| c.contains(&second))
                .map(|c| circuits.swap_remove(c))
                .unwrap();
            first_circuit.extend(second_circuit);
        }

        circuits.push(first_circuit);
    }

    wall_distance
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let boxes = input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = part1(&boxes, 1000);
    println!("First answer: {}", part1);

    let part2 = part2(&boxes);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        162,817,812\n\
        57,618,57\n\
        906,360,560\n\
        592,479,940\n\
        352,342,300\n\
        466,668,158\n\
        542,29,236\n\
        431,825,988\n\
        739,650,466\n\
        52,470,668\n\
        216,146,977\n\
        819,987,18\n\
        117,168,530\n\
        805,96,715\n\
        346,949,466\n\
        970,615,88\n\
        941,993,340\n\
        862,61,35\n\
        984,92,344\n\
        425,690,689\
    ";

    #[test]
    fn test_part1() {
        let boxes = INPUT
            .lines()
            .map(JunctionBox::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part1(&boxes, 10);

        const EXPECTED: usize = 40;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let boxes = INPUT
            .lines()
            .map(JunctionBox::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = part2(&boxes);

        const EXPECTED: u64 = 25272;
        assert_eq!(EXPECTED, result);
    }
}
