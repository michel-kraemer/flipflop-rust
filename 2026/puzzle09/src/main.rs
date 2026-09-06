use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;

// Right, Down, Left, Up
pub const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    steps: usize,
    x: usize,
    y: usize,
    last_portal: Option<(usize, usize)>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn shoot(
    x: usize,
    y: usize,
    dx: i64,
    dy: i64,
    grid: &[u8],
    width: usize,
    height: usize,
) -> (usize, usize) {
    let mut x = x as i64;
    let mut y = y as i64;
    while (x + dx) >= 0
        && ((x + dx) as usize) < width
        && (y + dy) >= 0
        && ((y + dy) as usize) < height
        && grid[(y + dy) as usize * width + (x + dx) as usize] != b'#'
    {
        x += dx;
        y += dy;
    }
    (x as usize, y as usize)
}

fn dijkstra<F>(
    start: (usize, usize),
    end: (usize, usize),
    grid: &[u8],
    width: usize,
    height: usize,
    skip_ahead: F,
) -> usize
where
    F: Fn(State, &mut [usize], &mut BinaryHeap<State>),
{
    let mut queue = BinaryHeap::new();
    queue.push(State {
        steps: 0,
        x: start.0,
        y: start.1,
        last_portal: None,
    });

    let mut best = vec![usize::MAX; grid.len()];
    while let Some(state) = queue.pop() {
        if (state.x, state.y) == end {
            return state.steps;
        }

        // try to skip ahead
        skip_ahead(state, &mut best, &mut queue);

        // walk normally
        for (dx, dy) in DIRS {
            let nx = state.x as i64 + dx;
            let ny = state.y as i64 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] != b'#'
                && best[ny as usize * width + nx as usize] > state.steps + 1
            {
                best[ny as usize * width + nx as usize] = state.steps + 1;
                queue.push(State {
                    steps: state.steps + 1,
                    x: nx as usize,
                    y: ny as usize,
                    last_portal: None,
                });
            }
        }
    }

    panic!("No shortest path found");
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

    // find start and end
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x, y);
            } else if grid[y * width + x] == b'E' {
                end = (x, y);
            }
        }
    }

    // part 1 - BFS
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    let mut seen = vec![false; grid.len()];
    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == end {
            println!("{steps}");
            break;
        }
        for (dx, dy) in DIRS {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] != b'#'
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // part 2 - Dijkstra's
    println!(
        "{}",
        dijkstra(start, end, &grid, width, height, |state, best, queue| {
            // try to skip ahead by teleporting
            for (dx, dy) in DIRS {
                let (nx, ny) = shoot(state.x, state.y, dx, dy, &grid, width, height);
                if (nx, ny) != (state.x, state.y) && best[ny * width + nx] > state.steps + 1 {
                    best[ny * width + nx] = state.steps + 1;
                    queue.push(State {
                        steps: state.steps + 1,
                        x: nx,
                        y: ny,
                        last_portal: None,
                    });
                }
            }
        })
    );

    // part 3 - Dijkstra's
    // Insights:
    // 1. Since it doesn't make sense to go back, it is sufficient to only keep
    //    the last portal used in `State`
    // 2. Instead of creating portals in walls, we can just create them on the
    //    floor (in front of the wall) and add one step when we go through them.
    //    This allows us to save states, as shooting in different directions may
    //    lead to a portal being created at the same place (i.e. beneath the
    //    current position).
    println!(
        "{}",
        dijkstra(start, end, &grid, width, height, |state, best, queue| {
            // Try to skip ahead. First, we check if there already is a portal
            // beneath us or if we can create a portal beneath us (by shooting a
            // wall next to us). We then shoot in all four directions and check
            // if going through the portal gives us a better `best` score. If
            // there already was a portal beneath us, going through it takes 2
            // steps (creating the other portal + going through). Otherwise, it
            // takes 3 steps (creating the portal beneath us + creating the
            // other portal + going through).
            let mut skip_steps = None;
            if state.last_portal == Some((state.x, state.y)) {
                // there is a portal beneath us
                skip_steps = Some(2);
            } else {
                // check if we can create a portal beneath us
                for (dx, dy) in DIRS {
                    let nx = state.x as i64 + dx;
                    let ny = state.y as i64 + dy;
                    if nx >= 0
                        && (nx as usize) < width
                        && ny >= 0
                        && (ny as usize) < height
                        && grid[ny as usize * width + nx as usize] == b'#'
                    {
                        skip_steps = Some(3);
                        break;
                    }
                }
            }

            // if there is a portal beneath us or if we can create one, shoot in
            // all four directions and try to skip ahead
            if let Some(skip_steps) = skip_steps {
                for (dx, dy) in DIRS {
                    // shoot until the laser hits a wall
                    let (nx, ny) = shoot(state.x, state.y, dx, dy, &grid, width, height);

                    if (nx, ny) != (state.x, state.y)
                        && best[ny * width + nx] > state.steps + skip_steps
                    {
                        // skipping ahead is possible
                        best[ny * width + nx] = state.steps + skip_steps;
                        queue.push(State {
                            steps: state.steps + skip_steps,
                            x: nx,
                            y: ny,
                            last_portal: Some((nx, ny)),
                        });
                    }
                }
            }
        })
    );
}
