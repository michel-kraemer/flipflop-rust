use std::fs;

const SKY: i64 = 1000;
const FRAME: i64 = 500;

fn count(
    n_pictures: i64,
    n_seconds: i64,
    cycle_len: i64,
    birds: &mut [(i64, i64, i64, i64)],
) -> i64 {
    let center = SKY / 2;
    let min = center - FRAME / 2;
    let max = center + FRAME / 2;
    let n_steps = n_seconds % cycle_len;

    let mut result = 0;
    let mut pi = 0;
    while pi < n_pictures {
        let mut all_zero = true;
        for b in birds.iter_mut() {
            b.0 = (b.0 + b.2 * n_steps).rem_euclid(SKY);
            b.1 = (b.1 + b.3 * n_steps).rem_euclid(SKY);
            if b.0 >= min && b.1 >= min && b.0 < max && b.1 < max {
                result += 1;
            }
            if b.0 != 0 || b.1 != 0 {
                all_zero = false;
            }
        }
        pi += 1;
        if all_zero {
            result *= n_pictures / pi;
            pi = n_pictures - n_pictures % pi;
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut birds = Vec::new();
    for l in input.lines() {
        let speed = l.split_once(',').unwrap();
        let speedx = speed.0.parse::<i64>().unwrap();
        let speedy = speed.1.parse::<i64>().unwrap();
        birds.push((0i64, 0i64, speedx, speedy));
    }

    // find cycle
    let mut bird_clones = birds.clone();
    let cycle_len = (1i64..)
        .find(|_| {
            let mut all_zero = true;
            for b in bird_clones.iter_mut() {
                b.0 = (b.0 + b.2).rem_euclid(SKY);
                b.1 = (b.1 + b.3).rem_euclid(SKY);
                if b.0 != 0 || b.1 != 0 {
                    all_zero = false;
                }
            }
            all_zero
        })
        .unwrap();

    println!("{}", count(1, 100, cycle_len, &mut birds.clone()));
    println!("{}", count(1000, 3600, cycle_len, &mut birds.clone()));
    println!("{}", count(1000, 31_556_926, cycle_len, &mut birds.clone()));
}
