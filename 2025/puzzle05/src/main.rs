use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let bytes = input.trim().as_bytes();

    let mut index = vec![(usize::MAX, usize::MAX); 256];
    for (i, &b) in bytes.iter().enumerate() {
        if index[b as usize].0 == usize::MAX {
            index[b as usize].0 = i;
        } else {
            index[b as usize].1 = i;
        }
    }
    let mut seen = vec![false; 256];

    let mut total1 = 0;
    let mut total3 = 0;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        seen[b as usize] = true;

        let next = if index[b as usize].0 == i {
            index[b as usize].1
        } else {
            index[b as usize].0
        };

        let dist = next.abs_diff(i);
        total1 += dist;
        if b.is_ascii_uppercase() {
            total3 -= dist as i64;
        } else {
            total3 += dist as i64;
        }

        i = next + 1;
    }
    println!("{total1}");

    for &b in bytes {
        if !seen[b as usize] {
            print!("{}", b as char);
            seen[b as usize] = true;
        }
    }
    println!();

    println!("{total3}");
}
