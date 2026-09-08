use std::fs;
use std::range::Range;

use num_traits::PrimInt;

const SIDE: usize = 5;

fn cartesian<T>(ranges: &[Range<T>]) -> Vec<Vec<T>>
where
    Range<T>: IntoIterator<Item = T>,
    T: PrimInt,
{
    let mut result = Vec::new();
    let mut values = ranges.iter().map(|r| r.start).collect::<Vec<_>>();
    loop {
        result.push(values.clone());
        let mut j = values.len() - 1;
        loop {
            values[j] = values[j] + T::one();
            if values[j] < ranges[j].end {
                break;
            }
            if j == 0 {
                return result;
            }
            values[j] = ranges[j].start;
            j -= 1;
        }
    }
}

fn make_lines(dim: usize) -> Vec<Vec<Vec<usize>>> {
    let directions = cartesian(&vec![Range::from(-1..2); dim]);

    let mut result = Vec::new();

    for dir in directions {
        // make directions unique by ensuring that the first non-zero component
        // is positive
        let Some(non_zero) = dir.iter().find(|p| **p != 0) else {
            // skip entry that is no direction at all (i.e. where all components
            // are 0)
            continue;
        };
        if *non_zero < 0 {
            continue;
        }

        // determine valid starting coordinates for this direction
        let ranges: Vec<Range<usize>> = dir
            .iter()
            .map(|c| match c {
                -1 => Range::from(SIDE - 1..SIDE),
                0 => Range::from(0..SIDE),
                1 => Range::from(0..1),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        // for each possible starting coordinate, perform vector addition to
        // obtain all lines
        for sc in cartesian(&ranges) {
            let mut line = Vec::new();
            for n in 0..SIDE as isize {
                line.push(
                    sc.iter()
                        .zip(dir.iter())
                        .map(|(c, d)| c.wrapping_add_signed(d * n))
                        .collect::<Vec<_>>(),
                );
            }
            result.push(line);
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let (numbers, cards) = input.split_once("\n\n").unwrap();
    let numbers = numbers
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let orig_cards = cards
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for dim in 2..=4 {
        let mut cards = orig_cards.clone();
        let lines = make_lines(dim as usize);
        for n in &numbers {
            // cross out numbers
            for v in &mut cards {
                if v == n {
                    *v = i64::MAX;
                }
            }

            // count bingos
            let n_objects = cards.len() / SIDE.pow(dim);
            let mut bingos = 0;
            for object in 0..n_objects {
                for line in &lines {
                    let mut found = true;
                    for coord in line {
                        let mut idx = 0;
                        let mut factor = 1;
                        for c in coord {
                            idx += c * factor;
                            factor *= SIDE;
                        }
                        idx += object * factor;
                        if cards[idx] != i64::MAX {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        bingos += 1;
                    }
                }
            }

            if bingos >= 5 {
                println!("{n}");
                break;
            }
        }
    }
}
