use std::fs;
use std::range::Range;

fn cartesian2d<T>(xr: Range<T>, yr: Range<T>) -> Vec<Vec<T>>
where
    Range<T>: IntoIterator<Item = T>,
    T: Copy,
{
    let mut result = Vec::new();
    for y in yr {
        for x in xr {
            result.push(vec![x, y]);
        }
    }
    result
}

fn cartesian3d<T>(xr: Range<T>, yr: Range<T>, zr: Range<T>) -> Vec<Vec<T>>
where
    Range<T>: IntoIterator<Item = T>,
    T: Copy,
{
    let mut result = Vec::new();
    for z in zr {
        for y in yr {
            for x in xr {
                result.push(vec![x, y, z]);
            }
        }
    }
    result
}

fn cartesian4d<T>(xr: Range<T>, yr: Range<T>, zr: Range<T>, tr: Range<T>) -> Vec<Vec<T>>
where
    Range<T>: IntoIterator<Item = T>,
    T: Copy,
{
    let mut result = Vec::new();
    for t in tr {
        for z in zr {
            for y in yr {
                for x in xr {
                    result.push(vec![x, y, z, t]);
                }
            }
        }
    }
    result
}

fn make_lines(dim: u8) -> Vec<Vec<Vec<usize>>> {
    let directions = match dim {
        2 => cartesian2d(Range::from(-1..2), Range::from(-1..2)),
        3 => cartesian3d(Range::from(-1..2), Range::from(-1..2), Range::from(-1..2)),
        4 => cartesian4d(
            Range::from(-1..2),
            Range::from(-1..2),
            Range::from(-1..2),
            Range::from(-1..2),
        ),
        _ => unreachable!(),
    };

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
                -1 => Range::from(4..5),
                0 => Range::from(0..5),
                1 => Range::from(0..1),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let starting_coordinates = match dim {
            2 => cartesian2d(ranges[0], ranges[1]),
            3 => cartesian3d(ranges[0], ranges[1], ranges[2]),
            4 => cartesian4d(ranges[0], ranges[1], ranges[2], ranges[3]),
            _ => unreachable!(),
        };

        for sc in starting_coordinates {
            let mut line = Vec::new();
            for n in 0..5 {
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
        .lines()
        .map(|c| {
            c.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .chunks_exact(5)
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    let mut cards = orig_cards.clone();
    let lines2d = make_lines(2);
    for n in &numbers {
        for card in &mut cards {
            for row in card {
                for v in row {
                    if v == n {
                        *v = i64::MAX;
                    }
                }
            }
        }

        let mut bingos = 0;
        for card in &cards {
            for line in &lines2d {
                let mut found = true;
                for coord in line {
                    if card[coord[0]][coord[1]] != i64::MAX {
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

    // part 2
    let mut cards = orig_cards
        .chunks_exact(5)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    let lines3d = make_lines(3);
    for n in &numbers {
        for card in &mut cards {
            for slice in card {
                for row in slice {
                    for v in row {
                        if v == n {
                            *v = i64::MAX;
                        }
                    }
                }
            }
        }

        let mut bingos = 0;
        for card in &cards {
            for line in &lines3d {
                let mut found = true;
                for coord in line {
                    if card[coord[0]][coord[1]][coord[2]] != i64::MAX {
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

    // part 3
    let mut cards = orig_cards
        .chunks_exact(5)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
        .chunks_exact(5)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    let lines4d = make_lines(4);
    for n in &numbers {
        for card in &mut cards {
            for cube in card {
                for slice in cube {
                    for row in slice {
                        for v in row {
                            if v == n {
                                *v = i64::MAX;
                            }
                        }
                    }
                }
            }
        }

        let mut bingos = 0;
        for card in &cards {
            for line in &lines4d {
                let mut found = true;
                for coord in line {
                    if card[coord[0]][coord[1]][coord[2]][coord[3]] != i64::MAX {
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
