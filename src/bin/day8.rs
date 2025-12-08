use petgraph::algo::{condensation, connected_components};
use petgraph::graph::UnGraph;
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = "./inputs/day8.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: Self) -> u64 {
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

fn part1(boxes: &[JunctionBox], nbr_connections: usize) -> u64 {
    let mut graph = UnGraph::<JunctionBox, u64>::new_undirected();
    for b in boxes {
        graph.add_node(*b);
    }

    let mut distances = BTreeMap::new();
    for (idx, n1) in graph.node_indices().enumerate() {
        for n2 in graph.node_indices().skip(idx + 1) {
            distances.insert(
                graph
                    .node_weight(n1)
                    .unwrap()
                    .distance(*graph.node_weight(n2).unwrap()),
                (n1, n2),
            );
        }
    }

    for (distance, (n1, n2)) in distances.into_iter().take(nbr_connections) {
        graph.add_edge(n1, n2, distance);
    }

    let mut nodes_count: Vec<u64> = condensation(graph, true)
        .node_weights()
        .map(|v| v.len() as u64)
        .collect();
    nodes_count.sort_unstable();
    nodes_count.iter().rev().take(3).product()
}

fn part2(boxes: &[JunctionBox]) -> u64 {
    let mut graph = UnGraph::<JunctionBox, u64>::new_undirected();
    for b in boxes {
        graph.add_node(*b);
    }

    let mut distances = BTreeMap::new();
    for (idx, n1) in graph.node_indices().enumerate() {
        for n2 in graph.node_indices().skip(idx + 1) {
            distances.insert(
                graph
                    .node_weight(n1)
                    .unwrap()
                    .distance(*graph.node_weight(n2).unwrap()),
                (n1, n2),
            );
        }
    }

    let mut wall_distance = 0;
    while connected_components(&graph) > 1 {
        let (distance, (n1, n2)) = distances.pop_first().unwrap();
        graph.add_edge(n1, n2, distance);
        wall_distance = graph.node_weight(n1).unwrap().x * graph.node_weight(n2).unwrap().x
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

        const EXPECTED: u64 = 40;
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
