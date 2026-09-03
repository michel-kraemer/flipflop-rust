use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid_lines = input.lines().collect::<Vec<_>>();
    let width = grid_lines[0].len();
    let height = grid_lines.len();
    let mut grid = grid_lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // part 1
    let mut pos = (0, 0);
    let mut seen = vec![false; grid.len()];
    let mut visited = 0;
    while !seen[pos.1 * width + pos.0] {
        visited += 1;
        seen[pos.1 * width + pos.0] = true;
        let b = grid[pos.1 * width + pos.0];
        match b {
            b'>' => pos.0 += 1,
            b'<' => pos.0 -= 1,
            b'v' => pos.1 += 1,
            b'^' => pos.1 -= 1,
            _ => unreachable!(),
        }
    }
    println!("{visited}");

    // part 2
    let mut max = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let i = y * width + x;
            let old = grid[i];
            for nb in *b"<>v^" {
                grid[i] = nb;

                let mut pos = (0, 0);
                seen.fill(false);
                let mut visited = 0;
                while !seen[pos.1 * width + pos.0] {
                    visited += 1;
                    seen[pos.1 * width + pos.0] = true;
                    let b = grid[pos.1 * width + pos.0];
                    match b {
                        b'>' => pos.0 += 1,
                        b'<' => pos.0 -= 1,
                        b'v' => pos.1 += 1,
                        b'^' => pos.1 -= 1,
                        _ => unreachable!(),
                    }
                }

                max = max.max(visited);
            }
            grid[i] = old;
        }
    }
    println!("{max}");

    // part 3
    let mut max = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let i = y * width + x;
            let old = grid[i];
            for nb in *b"<>v^" {
                grid[i] = nb;

                let mut pos = (0, 0);
                seen.fill(false);
                let mut visited = 0;
                let mut illegal = 0;

                while !seen[pos.1 * width + pos.0] || illegal < 3 {
                    if seen[pos.1 * width + pos.0] {
                        if pos.0 == 0 || pos.0 == width - 1 || pos.1 == 0 || pos.1 == height - 1 {
                            break;
                        }

                        let b = grid[pos.1 * width + pos.0];
                        match b {
                            b'>' => pos.1 += 1,
                            b'<' => pos.1 -= 1,
                            b'v' => pos.0 -= 1,
                            b'^' => pos.0 += 1,
                            _ => unreachable!(),
                        }

                        illegal += 1;
                    } else {
                        visited += 1;
                        seen[pos.1 * width + pos.0] = true;

                        let b = grid[pos.1 * width + pos.0];
                        match b {
                            b'>' => pos.0 += 1,
                            b'<' => pos.0 -= 1,
                            b'v' => pos.1 += 1,
                            b'^' => pos.1 -= 1,
                            _ => unreachable!(),
                        }
                    }
                }

                max = max.max(visited);
            }

            grid[i] = old;
        }
    }
    println!("{max}");
}
