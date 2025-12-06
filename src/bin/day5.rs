use std::error::Error;
use std::ops::RangeInclusive;

const INPUT: &str = "./inputs/day5.txt";

type Ingredient = u64;
type FreshIngredients = RangeInclusive<Ingredient>;

fn parse_input(input: &str) -> (Vec<FreshIngredients>, Vec<Ingredient>) {
    let (fresh_ingredients, available_ingredients) =
        input.split_once("\n\n").expect("invalid input");

    let fresh_ingredients = fresh_ingredients
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").expect("invalid input");
            left.parse::<Ingredient>().expect("invalid input")
                ..=right.parse::<Ingredient>().expect("invalid input")
        })
        .collect();

    let available_ingredients = available_ingredients
        .lines()
        .map(|line| line.parse().expect("invalid input"))
        .collect();

    (fresh_ingredients, available_ingredients)
}

fn part1(fresh_ingredients: &[FreshIngredients], available_ingredients: &[Ingredient]) -> usize {
    available_ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ingredients
                .iter()
                .any(|fresh| fresh.contains(ingredient))
        })
        .count()
}

fn part2(fresh_ingredients: &[FreshIngredients]) -> u64 {
    let mut ranges: Vec<FreshIngredients> = Vec::new();

    // For each range, keep union-ing the ranges that are overlapping (or contiguous)
    // until we find no more.
    for fresh_ingredient in fresh_ingredients {
        let mut new_range = fresh_ingredient.clone();
        while let Some(idx) = ranges.iter().position(|range| {
            //  Find if they overlap.
            range.contains(new_range.start())
                || range.contains(new_range.end())
                || new_range.contains(range.start())
                || new_range.contains(range.end())
                //  Also, if they are contiguous, though not overlapping.
                || new_range.start() - 1 == *range.end()
                || new_range.end() + 1 == *range.start()
        }) {
            let (start, end) = ranges.swap_remove(idx).into_inner();
            let (new_start, new_end) = new_range.into_inner();
            new_range = start.min(new_start)..=end.max(new_end);
        }
        ranges.push(new_range);
        ranges.sort_unstable_by_key(|range| *range.start());
    }

    ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (fresh_ingredients, available_ingredients) = parse_input(&input);

    let part1 = part1(&fresh_ingredients, &available_ingredients);
    println!("First answer: {}", part1);

    let part2 = part2(&fresh_ingredients);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\
    ";

    #[test]
    fn test_part1() {
        let (available_ingredients, fresh_ingredients) = parse_input(INPUT);
        let result = part1(&available_ingredients, &fresh_ingredients);

        const EXPECTED: usize = 3;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let (available_ingredients, _) = parse_input(INPUT);
        let result = part2(&available_ingredients);

        const EXPECTED: u64 = 14;
        assert_eq!(EXPECTED, result);
    }
}
