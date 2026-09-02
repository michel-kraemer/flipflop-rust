use std::fs;

const LEN: usize = 100;

fn mov(pos: &mut usize, dir: u8) {
    if dir == b'<' {
        *pos = (*pos + LEN - 1) % LEN;
    } else {
        *pos = (*pos + 1) % LEN;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let instructions = input.trim().as_bytes();

    // part 1
    let mut wall_temperatures = vec![0_usize; LEN];
    let mut pos = 0;
    for &i in instructions {
        mov(&mut pos, i);
        wall_temperatures[pos] += 1;
    }
    let max = wall_temperatures
        .iter()
        .enumerate()
        .max_by_key(|(_, t)| *t)
        .unwrap();
    println!("{}", (max.0 + 1) * max.1);

    // part 2
    let mut robot_pos = 0;
    let mut wall_pos = 0;
    let mut total = 0;
    for (&robot_instruction, &wall_instruction) in
        instructions.iter().zip(instructions.iter().rev())
    {
        mov(&mut robot_pos, robot_instruction);
        mov(&mut wall_pos, wall_instruction);
        if robot_pos == wall_pos {
            total += 1;
        }
    }
    println!("{total}");

    // part 3
    let mut robot_pos = 0;
    let mut wall_positions = (0..LEN).collect::<Vec<_>>();
    let mut wall_temperatures = vec![0; LEN];
    for (&robot_instruction, &wall_instruction) in
        instructions.iter().zip(instructions.iter().rev())
    {
        mov(&mut robot_pos, robot_instruction);
        for i in 0..LEN {
            mov(&mut wall_positions[i], wall_instruction);
            if robot_pos == wall_positions[i] {
                wall_temperatures[i] += 1;
            }
        }
    }
    let max = wall_temperatures
        .iter()
        .enumerate()
        .max_by_key(|(_, t)| *t)
        .unwrap();
    println!("{}", (max.0 + 1) * max.1);
}
