use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Unknown,
    Left,
    Right,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    // part 1
    println!(
        "{}",
        lines[0..lines.len() - 401]
            .iter()
            .filter(|&&l| l == "o-|" || l == "|-o")
            .count()
    );

    // part 2
    let mut total = 0;
    let mut side = Side::Unknown;
    for l in lines[0..lines.len() - 1].iter().rev() {
        if l.trim() == "o-|" {
            match side {
                Side::Unknown => side = Side::Left,
                Side::Left => {}
                Side::Right => {
                    side = Side::Left;
                    total += 1;
                }
            }
        } else if l.trim() == "|-o" {
            match side {
                Side::Unknown => side = Side::Right,
                Side::Left => {
                    side = Side::Right;
                    total += 1;
                }
                Side::Right => {}
            }
        }
    }
    println!("{total}");

    // part 3
    let mut total = 0;
    'outer: loop {
        let mut side = Side::Unknown;
        let mut prev = 0;
        for i in (0..lines.len() - 1).rev() {
            if lines[i].trim() == "o-|" {
                match side {
                    Side::Unknown => side = Side::Left,
                    Side::Left => {}
                    Side::Right => {
                        side = Side::Left;
                        lines[prev] = "|";
                    }
                }
                prev = i;
            } else if lines[i].trim() == "|-o" {
                match side {
                    Side::Unknown => side = Side::Right,
                    Side::Left => {
                        side = Side::Right;
                        lines[prev] = "|";
                    }
                    Side::Right => {}
                }
                prev = i;
            } else if lines[i].trim() == r"/|\" {
                if side == Side::Unknown {
                    break 'outer;
                }
                lines[prev] = "|";
            }
        }
        total += 1;
    }
    println!("{total}");
}
