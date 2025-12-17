use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    println!("{}", lines.iter().map(|l| l.len() / 2).sum::<usize>());

    // part 2
    println!(
        "{}",
        lines
            .iter()
            .map(|l| l.len() / 2)
            .filter(|l| l.is_multiple_of(2))
            .sum::<usize>()
    );

    // part 3
    println!(
        "{}",
        lines
            .iter()
            .filter(|l| !l.contains("ne"))
            .map(|l| l.len() / 2)
            .sum::<usize>()
    );
}
