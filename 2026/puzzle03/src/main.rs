use std::fs;

fn score(password: &str, additional_rules: bool) -> usize {
    let bytes = password.as_bytes();

    // lowercase character
    let mut score = bytes.iter().any(|b| b.is_ascii_lowercase()) as usize;

    // uppercase character
    score += bytes.iter().any(|b| b.is_ascii_uppercase()) as usize;

    // digit
    score += bytes.iter().any(|b| b.is_ascii_digit()) as usize;

    if additional_rules {
        // longest sequence
        let mut i = 0;
        let mut longest_sequence = 0;
        while i < bytes.len() {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] == bytes[i] {
                j += 1;
            }
            longest_sequence = longest_sequence.max(j - i);
            i = j;
        }
        if longest_sequence >= 3 {
            score += longest_sequence * longest_sequence;
        }

        // seven
        let mut contains_seven = false;
        for b in bytes {
            if b.is_ascii_digit() {
                if *b == b'7' {
                    contains_seven = true;
                } else {
                    contains_seven = false;
                    break;
                }
            }
        }
        if contains_seven {
            score += 7;
        }

        if password.contains("red") || password.contains("green") || password.contains("blue") {
            score *= 3;
        }
    }

    score * bytes.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // part 1
    println!(
        "{}",
        lines
            .iter()
            .max_by_key(|password| score(password, false))
            .unwrap()
    );

    // part 2
    println!(
        "{}",
        lines
            .iter()
            .max_by_key(|password| score(password, true))
            .unwrap()
    );

    // part 3
    let mut max = 0;
    for append in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        let total = lines
            .iter()
            .map(|password| score(&format!("{password}{append}"), true))
            .sum::<usize>();
        max = max.max(total);
    }
    println!("{max}");
}
