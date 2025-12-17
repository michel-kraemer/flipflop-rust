use std::fs;

const LUT_U64: [u64; 21] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
    3628800,
    39916800,
    479001600,
    6227020800,
    87178291200,
    1307674368000,
    20922789888000,
    355687428096000,
    6402373705728000,
    121645100408832000,
    2432902008176640000,
];

fn factorial(n: u64) -> u64 {
    if n as usize >= LUT_U64.len() {
        panic!("Unable to calculate factorial of n = {n} because it's result is too large for u64");
    }
    LUT_U64[n as usize]
}

/// Closed formula for the number of d-dimensional lattice paths. See also
/// https://en.wikipedia.org/wiki/Lattice_path and
/// https://math.stackexchange.com/a/203726
fn count(sizes: &[u64]) -> u64 {
    factorial(sizes.iter().map(|s| s - 1).sum())
        / sizes.iter().map(|s| factorial(s - 1)).product::<u64>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut total1 = 0;
    let mut total2 = 0;
    let mut total3 = 0;
    for l in input.lines() {
        let p = l.split_once(' ').unwrap();
        let (a, b) = (p.0.parse::<u64>().unwrap(), p.1.parse::<u64>().unwrap());
        total1 += count(&[a, b]);
        total2 += count(&[a, b, a]);
        total3 += count(&vec![b; a as usize]);
    }
    println!("{total1}");
    println!("{total2}");
    println!("{total3}");
}
