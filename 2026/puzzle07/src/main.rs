use std::collections::VecDeque;
use std::fs;

fn play<E, H>(
    instructions: &[u8],
    sushi: &[(i64, i64)],
    mut on_eat: E,
    mut on_hit: H,
) -> VecDeque<(i64, i64)>
where
    E: FnMut((i64, i64), &mut VecDeque<(i64, i64)>),
    H: FnMut((i64, i64), &mut VecDeque<(i64, i64)>) -> bool,
{
    let mut head = (0, 0);
    let mut snake = VecDeque::new();
    snake.push_front(head);

    let mut si = 0;
    for i in instructions {
        match i {
            b'>' => head.0 += 1,
            b'<' => head.0 -= 1,
            b'^' => head.1 += 1,
            b'v' => head.1 -= 1,
            _ => unreachable!(),
        }
        snake.push_front(head);
        let old_tail = snake.pop_back().unwrap();

        if si < sushi.len() && sushi[si] == head {
            on_eat(old_tail, &mut snake);
            si += 1;
        } else if snake.iter().skip(1).any(|p| *p == head) && on_hit(head, &mut snake) {
            break;
        }
    }

    snake
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (instructions, sushi) = input.split_once("\n\n").unwrap();
    let instructions = instructions.as_bytes();
    let sushi = sushi
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    // part 1
    let mut total = 0;
    play(
        &instructions[0..instructions.len() / 2],
        &sushi,
        |_, _| {
            total += 1;
        },
        |_, _| true,
    );
    println!("{total}");

    // part 2
    let snake = play(
        instructions,
        &sushi,
        |old_tail, snake| {
            snake.push_back(old_tail);
        },
        |_, _| true,
    );
    println!("{}", snake.len());

    // part 3
    let mut hits = 0;
    let snake = play(
        instructions,
        &sushi,
        |old_tail, snake| {
            snake.push_back(old_tail);
        },
        |hit_pos, snake| {
            // cut tail
            while hit_pos != snake.pop_back().unwrap() {}

            // Cut one more segment from the tail. This is an inconsistency in
            // the problem statement in my opinion. From step 8 to 9 in the
            // example, the snake does not hit itself even though (4,1) is
            // occupied by the snake's tail and the snake's head moves to (4,1).
            // Apparently, the snake moves before the hit check. From step 17 to
            // 18, it eats one of its own segments. However, here, the segment
            // that should be eaten is determined before (!) the snake moves.
            // Why is not is able to eat its last segment then?
            snake.pop_back();

            hits += 1;

            false
        },
    );
    println!("{}", snake.len() * hits);
}
