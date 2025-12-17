use std::fs;

#[derive(Debug)]
struct Move {
    direction: i64,
    len: usize,
}

struct Fib(Vec<i64>);

impl Fib {
    fn new() -> Self {
        Self(vec![0, 1])
    }

    fn get(&mut self, n: usize) -> i64 {
        while self.0.len() < n + 1 {
            self.0
                .push(self.0[self.0.len() - 2] + self.0[self.0.len() - 1]);
        }
        self.0[n]
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let moves = input
        .trim()
        .as_bytes()
        .iter()
        .fold(Vec::new(), |mut acc: Vec<Move>, e| {
            let d = if *e == b'^' { 1 } else { -1 };
            if let Some(last) = acc.last_mut()
                && last.direction == d
            {
                last.len += 1;
            } else {
                acc.push(Move {
                    direction: d,
                    len: 1,
                });
            }
            acc
        });

    // part 1
    let mut height = 0;
    let mut max = 0;
    for m in &moves {
        height += m.direction * m.len as i64;
        max = max.max(height);
    }
    println!("{max}");

    // part 2
    let mut height = 0;
    let mut max = 0;
    for m in &moves {
        height += m.direction * (m.len * (m.len + 1) / 2) as i64;
        max = max.max(height);
    }
    println!("{max}");

    // part 3
    let mut height = 0;
    let mut max = 0;
    let mut fib = Fib::new();
    for m in &moves {
        height += m.direction * fib.get(m.len);
        max = max.max(height);
    }
    println!("{max}");
}
