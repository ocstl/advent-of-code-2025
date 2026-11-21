use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = "./inputs/day11.txt";

type Paths<'p> = HashMap<&'p str, Vec<&'p str>>;
const YOU: &str = "you";
const OUT: &str = "out";
const DAC: &str = "dac";
const FFT: &str = "fft";
const SVR: &str = "svr";

fn parse_input(input: &str) -> Paths<'_> {
    input
        .lines()
        .map(|line| {
            let (device, out) = line.trim().split_once(':').unwrap();
            let out = out.split_whitespace().collect();
            (device, out)
        })
        .collect()
}

fn part1(devices: &Paths) -> usize {
    // Since we may find ourselves at the same device repeatedly, we can memoize the number of
    // paths from each device we can reach.
    let mut memoization = HashMap::new();
    memoization.insert(OUT, 1);

    fn recursive_fn<'a>(
        device: &'a str,
        devices: &'a Paths,
        memoization: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if !memoization.contains_key(device) {
            let count = devices
                .get(device)
                .unwrap()
                .iter()
                .map(|d| recursive_fn(d, devices, memoization))
                .sum();
            memoization.insert(device, count);
        }

        *memoization.get(device).unwrap()
    }

    recursive_fn(YOU, devices, &mut memoization)
}

fn part2(devices: &Paths) -> usize {
    // We can break the problem into subproblems, since we assume no cycles.
    // Two possible paths:
    //  - srv -> fft -> dac -> out
    //  - srv -> dac -> fft -> out
    // For the first one, we can first all paths from dac to out that do NOT contain fft. Then the
    // number of paths from fft to dac, then the number of paths from srv to fft that do not
    // contain dac.
    // Repeat for the second one.
    fn recursive_fn<'a>(
        device: &'a str,
        devices: &'a Paths,
        memoization: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if !memoization.contains_key(device) {
            let count = devices
                .get(device)
                .unwrap()
                .iter()
                .map(|d| recursive_fn(d, devices, memoization))
                .sum();
            memoization.insert(device, count);
        }

        *memoization.get(device).unwrap()
    }

    // First path. By setting the value at 0, we stop following that path and do not count it.
    let mut dac_out = HashMap::new();
    dac_out.insert(OUT, 1);
    dac_out.insert(FFT, 0);
    let paths = recursive_fn(DAC, devices, &mut dac_out);
    let mut fft_dac = HashMap::new();
    fft_dac.insert(OUT, 0);
    fft_dac.insert(DAC, paths);
    let paths = recursive_fn(FFT, devices, &mut fft_dac);
    let mut svr_fft = HashMap::new();
    svr_fft.insert(OUT, 0);
    svr_fft.insert(DAC, 0);
    svr_fft.insert(FFT, paths);

    let svr_fft_dac_out = recursive_fn(SVR, devices, &mut svr_fft);

    // Second path.
    let mut fft_out = HashMap::new();
    fft_out.insert(OUT, 1);
    fft_out.insert(DAC, 0);
    let paths = recursive_fn(FFT, devices, &mut fft_out);
    let mut dac_fft = HashMap::new();
    dac_fft.insert(OUT, 0);
    dac_fft.insert(FFT, paths);
    let paths = recursive_fn(DAC, devices, &mut dac_fft);
    let mut svr_dac = HashMap::new();
    svr_dac.insert(OUT, 0);
    svr_dac.insert(DAC, 0);
    svr_dac.insert(FFT, paths);

    let svr_dac_fft_out = recursive_fn(SVR, devices, &mut svr_dac);

    svr_fft_dac_out + svr_dac_fft_out
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let devices = parse_input(&input);

    let part1 = part1(&devices);
    println!("First answer: {}", part1);

    let part2 = part2(&devices);
    println!("Second answer: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        aaa: you hhh\n\
        you: bbb ccc\n\
        bbb: ddd eee\n\
        ccc: ddd eee fff\n\
        ddd: ggg\n\
        eee: out\n\
        fff: out\n\
        ggg: out\n\
        hhh: ccc fff iii\n\
        iii: out\
    ";

    const INPUT2: &str = "\
        svr: aaa bbb\n\
        aaa: fft\n\
        fft: ccc\n\
        bbb: tty\n\
        tty: ccc\n\
        ccc: ddd eee\n\
        ddd: hub\n\
        hub: fff\n\
        eee: dac\n\
        dac: fff\n\
        fff: ggg hhh\n\
        ggg: out\n\
        hhh: out\
    ";

    #[test]
    fn test_part1() {
        let devices = parse_input(INPUT);
        let result = part1(&devices);

        const EXPECTED: usize = 5;
        assert_eq!(EXPECTED, result);
    }

    #[test]
    fn test_part2() {
        let devices = parse_input(INPUT2);
        let result = part2(&devices);

        const EXPECTED: usize = 2;
        assert_eq!(EXPECTED, result);
    }
}
