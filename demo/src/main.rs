use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let sum = numbers.iter().sum::<i64>();
    println!("{}", sum);

    // part 2
    println!("{}", (sum as f64 / numbers.len() as f64).round());

    // part 3
    let mut number_counts: HashMap<i64, i64> = HashMap::new();
    let mut digit_counts = [0; 10];
    for &n in &numbers {
        *number_counts.entry(n).or_default() += 1;
        let mut m = n;
        while m > 0 {
            let d = m % 10;
            m /= 10;
            digit_counts[d as usize] += 1;
        }
    }

    let mut number_counts = number_counts.into_iter().collect::<Vec<_>>();
    number_counts.sort_unstable_by_key(|c| -c.1);
    let max_number = number_counts[0];
    let min_digit = digit_counts
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| *c)
        .unwrap();
    println!("{}{}", max_number.0, min_digit.0);
}
