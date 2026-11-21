use advent_of_code_2025::grid::Grid;
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = "./inputs/day12.txt";

type Present = Grid<bool>;

struct Region {
    grid: Grid<bool>,
    presents: Vec<usize>,
}

impl Region {
    fn can_fit(&self, presents: &[Present]) -> bool {
        // Presents are always 3x3 grids. If we can fit enough 3x3 grids, we do not need to do
        // anymore work.
        if (self.grid.width() / 3) * (self.grid.height() / 3) >= self.presents.iter().sum() {
            return true;
        }

        // TODO: All of it.
        false
    }
}

impl FromStr for Region {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, presents) = s.trim().split_once(':').unwrap();
        let (x, y) = size.trim().split_once('x').unwrap();
        let x = x.parse()?;
        let y = y.parse()?;

        let grid = Grid::new(y, x, vec![false; x * y]).unwrap();
        let presents = presents
            .split_whitespace()
            .map(|p| p.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self { grid, presents })
    }
}

fn parse_input(input: &str) -> Result<(Vec<Present>, Vec<Region>), Box<dyn Error>> {
    let (presents, regions) = input.rsplit_once("\n\n").unwrap();
    let regions = regions
        .lines()
        .map(|l| l.parse::<Region>())
        .collect::<Result<Vec<Region>, _>>()?;
    let presents = presents
        .split("\n\n")
        .map(|p| {
            let grid = p
                .lines()
                .skip(1)
                .flat_map(|l| l.chars())
                .map(|c| c == '#')
                .collect();
            Grid::new(3, 3, grid).unwrap()
        })
        .collect();

    Ok((presents, regions))
}

fn part1(presents: &[Present], regions: &[Region]) -> usize {
    regions.iter().filter(|r| r.can_fit(presents)).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (presents, regions) = parse_input(&input)?;

    let part1 = part1(&presents, &regions);
    println!("First answer: {}", part1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        0:\n\
        ###\n\
        ##.\n\
        ##.\n\
        \n\
        1:\n\
        ###\n\
        ##.\n\
        .##\n\
        \n\
        2:\n\
        .##\n\
        ###\n\
        ##.\n\
        \n\
        3:\n\
        ##.\n\
        ###\n\
        ##.\n\
        \n\
        4:\n\
        ###\n\
        #..\n\
        ###\n\
        \n\
        5:\n\
        ###\n\
        .#.\n\
        ###\n\
        \n\
        4x4: 0 0 0 0 2 0\n\
        12x5: 1 0 1 0 2 2\n\
        12x5: 1 0 1 0 3 2\
    ";

    #[test]
    fn test_part1() {
        let (presents, regions) = parse_input(INPUT).unwrap();
        let result = part1(&presents, &regions);

        const EXPECTED: usize = 5;
        assert_eq!(EXPECTED, result);
    }
}
