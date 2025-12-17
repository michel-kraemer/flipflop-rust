use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut green = 0;
    let mut cost = 0;
    let mut counts: HashMap<(i64, i64, i64), i64> = HashMap::new();
    for l in input.lines() {
        let mut n = l.split(',').map(|v| v.parse::<i64>().unwrap());
        let (r, g, b) = (n.next().unwrap(), n.next().unwrap(), n.next().unwrap());
        *counts.entry((r, g, b)).or_default() += 1;
        if r == g || r == b || g == b {
            // special
            cost += 10;
        } else if r > g && r > b {
            // red
            cost += 5;
        } else if g > r && g > b {
            // green
            green += 1;
            cost += 2;
        } else if b > r && b > g {
            // blue
            cost += 4;
        }
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_unstable_by_key(|c| -c.1);

    println!("{},{},{}", counts[0].0.0, counts[0].0.1, counts[0].0.2);
    println!("{green}");
    println!("{cost}");
}
