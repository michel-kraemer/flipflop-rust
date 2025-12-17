use std::fs;

fn sum(pieces: &[(i64, i64)], diagonal: bool) -> u64 {
    let mut result = 0;
    let mut pos = (0i64, 0i64);
    for &p in pieces {
        let dist = if diagonal {
            pos.0.abs_diff(p.0).max(pos.1.abs_diff(p.1))
        } else {
            pos.0.abs_diff(p.0) + pos.1.abs_diff(p.1)
        };
        result += dist;
        pos = p;
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut pieces = input
        .lines()
        .map(|l| {
            let p = l.split_once(',').unwrap();
            (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    // part 1
    println!("{}", sum(&pieces, false));

    // part 2
    println!("{}", sum(&pieces, true));

    // part 3
    pieces.sort_unstable_by_key(|p| p.0 + p.1);
    println!("{}", sum(&pieces, true));
}
