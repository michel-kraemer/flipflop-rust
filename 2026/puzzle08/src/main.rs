use std::fs;

fn count_part1(stoat: u8, rules: &[Vec<u8>], depth: usize, cache: &mut [usize]) -> usize {
    if depth == 0 {
        return 1;
    }

    let ci = (depth - 1) * 26 + (stoat - b'A') as usize;
    let c = cache[ci];
    if c != usize::MAX {
        return c;
    }

    let mut result = 0;
    for r in rules {
        if r[0] == stoat {
            for ns in &r[1..] {
                result += count_part1(*ns, rules, depth - 1, cache);
            }
            break;
        }
    }

    cache[ci] = result;

    result
}

fn count_parts_2_3(
    stoat1: u8,
    stoat2: u8,
    depth: usize,
    rules: &[Vec<u8>],
    cache: &mut [usize],
) -> usize {
    if depth == 0 {
        return 2;
    }

    let ci = (depth - 1) * 26 * 26 + (stoat1 - b'A') as usize * 26 + (stoat2 - b'A') as usize;
    let c = cache[ci];
    if c != usize::MAX {
        return c;
    }

    let mut result = 0;
    for r in rules {
        if (r[0] == stoat1 && r[1] == stoat2) || (r[0] == stoat2 && r[1] == stoat1) {
            // count each pair individually and subtract overlap
            result += count_parts_2_3(stoat1, r[2], depth - 1, rules, cache) - 1;
            for [ns1, ns2] in r[2..].array_windows() {
                result += count_parts_2_3(*ns1, *ns2, depth - 1, rules, cache) - 1;
            }
            result += count_parts_2_3(r[r.len() - 1], stoat2, depth - 1, rules, cache);
            break;
        }
    }

    cache[ci] = result;

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let rules = lines
        .into_iter()
        .map(|l| l.replace(' ', "").bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // part 1
    let mut cache = [usize::MAX; 26 * 7];
    println!(
        "{}",
        count_part1(b'A', &rules, 7, &mut cache) + count_part1(b'B', &rules, 7, &mut cache)
    );

    // part 2
    let mut cache = [usize::MAX; 26 * 26 * 7];
    println!("{}", count_parts_2_3(b'A', b'B', 7, &rules, &mut cache));

    // part 3
    let mut cache = [usize::MAX; 26 * 26 * 21];
    println!("{}", count_parts_2_3(b'A', b'B', 21, &rules, &mut cache));
}
