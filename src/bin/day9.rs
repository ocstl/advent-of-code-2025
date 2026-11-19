use advent_of_code_2025::grid::{Grid, Position};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::num::ParseIntError;

const INPUT: &str = "./inputs/day9.txt";

fn parse_input(input: &str) -> Result<Vec<Position>, ParseIntError> {
    input
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(',')?;
            Some(
                a.parse()
                    .and_then(|x| b.parse().map(|y| Position::new(x, y))),
            )
        })
        .collect()
}

fn part1(red_tiles: &[Position]) -> usize {
    // Try all pairs, not forgetting to add 1 because of opposite corners (tile surface).
    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(idx, a)| {
            red_tiles
                .iter()
                .skip(idx + 1)
                .map(|b| (a.x().abs_diff(b.x()) + 1) * (a.y().abs_diff(b.y()) + 1))
        })
        .max()
        .unwrap_or_default()
}

fn part2(red_tiles: &[Position]) -> usize {
    // The example is small enough, but the actual data is too large to fill. So we'll 2D compress
    // it first. Add a 0 for the x and y coordinates, as these will be outside the polygon. We also
    // add an extra set of coordinates at the end for the same reason.
    let (mut xs, mut ys) = red_tiles.iter().fold((vec![0], vec![0]), |mut acc, pos| {
        acc.0.push(pos.x());
        acc.1.push(pos.y());
        acc
    });
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    xs.push(xs.last().unwrap() + 1);
    ys.push(ys.last().unwrap() + 1);

    let xmapping = xs
        .iter()
        .enumerate()
        .map(|(idx, c)| (*c, idx))
        .collect::<HashMap<_, _>>();
    let ymapping = ys
        .iter()
        .enumerate()
        .map(|(idx, c)| (*c, idx))
        .collect::<HashMap<_, _>>();

    let mapped_tiles: Vec<Position> = red_tiles
        .iter()
        .map(|pos| Position::new(xmapping[&pos.x()], ymapping[&pos.y()]))
        .collect();

    // Create a grid with the compressed positions and add in the red tiles.
    let lx = xs.len();
    let ly = ys.len();
    let mut grid = Grid::new(ly, lx, vec![false; ly * lx]).unwrap();

    for (a, b) in mapped_tiles.iter().zip(mapped_tiles.iter().cycle().skip(1)) {
        if a.x() == b.x() {
            for y in a.y().min(b.y())..=a.y().max(b.y()) {
                *grid.get_mut(Position::new(a.x(), y)).unwrap() = true;
            }
        } else {
            for x in a.x().min(b.x())..=a.x().max(b.x()) {
                *grid.get_mut(Position::new(x, a.y())).unwrap() = true;
            }
        }
    }

    // Fill the interior. We're going to use the exterior points we created to find those not to
    // fill.
    let mut to_visit = vec![Position::new(0, 0)];
    let mut visited = HashSet::new();

    while let Some(pos) = to_visit.pop() {
        if visited.insert(pos) {
            to_visit.extend(pos.neighbours().filter(|p| !grid.get(*p).unwrap_or(&true)));
        }
    }

    // Recreate the grid, then remove the outside points.
    grid = Grid::new(ly, lx, vec![true; ly * lx]).unwrap();
    for p in visited {
        *grid.get_mut(p).unwrap() = false;
    }

    // For each pair of red tiles, we can now check whether all edges are contained in the
    // polygon. We can sort them by size, so we can terminate early.
    let mut rectangles: Vec<(usize, Position, Position)> = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(idx, a)| {
            red_tiles.iter().skip(idx + 1).map(|b| {
                (
                    (a.x().abs_diff(b.x()) + 1) * (a.y().abs_diff(b.y()) + 1),
                    *a,
                    *b,
                )
            })
        })
        .collect();

    rectangles.sort_unstable();

    while let Some((size, a, b)) = rectangles.pop() {
        let ax = *xmapping.get(&a.x()).unwrap();
        let ay = *ymapping.get(&a.y()).unwrap();
        let bx = *xmapping.get(&b.x()).unwrap();
        let by = *ymapping.get(&b.y()).unwrap();

        let minx = ax.min(bx);
        let maxx = ax.max(bx);
        let miny = ay.min(by);
        let maxy = ay.max(by);

        if (minx..=maxx)
            .flat_map(|x| (miny..=maxy).map(move |y| Position::new(x, y)))
            .all(|p| *grid.get(p).unwrap())
        {
            return size;
        }
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let red_tiles = parse_input(input.trim())?;

    let part1 = part1(&red_tiles);
    println!("First answer: {}", part1);

    let part2 = part2(&red_tiles);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3\
    ";

    #[test]
    fn test_part1() {
        let red_tiles = parse_input(INPUT).unwrap();
        let result = part1(&red_tiles);

        const EXPECTED: usize = 50;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let red_tiles = parse_input(INPUT).unwrap();
        let result = part2(&red_tiles);

        const EXPECTED: usize = 24;
        assert_eq!(EXPECTED, result);
    }
}
