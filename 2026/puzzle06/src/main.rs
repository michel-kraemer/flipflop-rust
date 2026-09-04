use std::fs;

// Right, Down, Left, Up
pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Check if a number is prime using [Wheel
/// factorization](https://en.wikipedia.org/wiki/Wheel_factorization). This test
/// is not as fast as Miller-Rabin or the like, but it gets the job done.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6; // numbers divisible by 2 and 3 don't have to be tested again
    }

    true
}

fn fill(
    start: (usize, usize),
    clockwise: bool,
    seen: &mut [Option<bool>],
    grid: &[u8],
    width: usize,
    height: usize,
    part: u8,
) {
    let mut queue = vec![(start.0, start.1, clockwise)];
    let mut inputs = Vec::new();
    let mut inner_seen = vec![None; grid.len()];
    let mut count = 0;
    if part < 4 {
        inner_seen[start.1 * width + start.0] = Some(clockwise);
    }

    while let Some((x, y, cw)) = queue.pop() {
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && (nx as usize) < width && ny >= 0 && (ny as usize) < height {
                let nx = nx as usize;
                let ny = ny as usize;
                let idx = ny * width + nx;
                if (grid[idx] == b'#' || grid[idx] == b'3') && inner_seen[ny * width + nx].is_none()
                {
                    inner_seen[ny * width + nx] = Some(!cw);
                    count += 1;
                    queue.push((nx, ny, !cw));
                } else if grid[idx].is_ascii_lowercase() {
                    inputs.push((nx, ny, cw));
                }
            }
        }
    }

    if part == 4 && is_prime(count) {
        return;
    }

    for (i, s) in inner_seen.into_iter().enumerate() {
        if let Some(s) = s {
            seen[i] = Some(s);
        }
    }

    if part > 1 {
        for (ix, iy, icw) in inputs {
            let output = grid[iy * width + ix].to_ascii_uppercase();
            for y in 0..height {
                for x in 0..width {
                    if grid[y * width + x] == output {
                        fill(
                            (x, y),
                            icw,
                            seen,
                            grid,
                            width,
                            height,
                            if part == 3 { 4 } else { part },
                        );
                    }
                }
            }
        }
    }
}

fn count(seen: &[Option<bool>], grid: &[u8], width: usize, height: usize) -> u64 {
    let mut result = 0u64;
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'*' {
                continue;
            }
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && (nx as usize) < width
                    && ny >= 0
                    && (ny as usize) < height
                    && let Some(cw) = seen[ny as usize * width + nx as usize]
                {
                    result <<= 1;
                    if cw {
                        result |= 1;
                    }
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let grid_lines = input.lines().collect::<Vec<_>>();
    let width = grid_lines[0].len();
    let height = grid_lines.len();
    let grid = grid_lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // find start
    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x, y);
                break;
            }
        }
    }

    for part in 1..=3 {
        let mut seen = vec![None; grid.len()];
        fill(start, false, &mut seen, &grid, width, height, part);
        println!("{}", count(&seen, &grid, width, height));
    }
}
