use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let mut total = 0;
    for &temperature in &lines {
        if temperature < 60 {
            total += 60 - temperature;
        }
    }
    println!("{total}");

    // part 2
    let mut total = 0;
    for &temperature in &lines {
        if temperature < 60 {
            total += 60 - temperature;
        } else if temperature > 60 {
            total += (temperature - 60) * 5;
        }
    }
    println!("{total}");

    // part 3
    let actual_temperatures = &lines[0..lines.len() / 2];
    let preferred_temperatures = &lines[lines.len() / 2..];
    let mut total = 0;
    for (temperature, preferred) in actual_temperatures.iter().zip(preferred_temperatures) {
        if temperature < preferred {
            total += preferred - temperature;
        } else if preferred < temperature {
            total += (temperature - preferred) * 5;
        }
    }
    println!("{total}");
}
