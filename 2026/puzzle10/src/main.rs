use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

use rayon::prelude::*;

enum Instruction {
    Load(u16, u8),
    Copy(u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Mod(u8, u8, u8),
    Inc(u8),
    Dec(u8),
    Jmp(usize),
    Jz(u8, usize),
    Jnz(u8, usize),
}

fn run(instructions: &[Instruction], labels: &HashMap<usize, usize>, regs: &mut [u16]) -> bool {
    let mut ip = 0;
    let mut executed = 0;

    while ip < instructions.len() {
        executed += 1;
        if executed > 5000000 {
            return true;
        }

        match instructions[ip] {
            Instruction::Load(val, dst) => regs[dst as usize] = val,

            Instruction::Copy(src, dst) => regs[dst as usize] = regs[src as usize],

            Instruction::Add(src1, src2, dst) => {
                regs[dst as usize] = regs[src1 as usize].wrapping_add(regs[src2 as usize]);
            }

            Instruction::Sub(src1, src2, dst) => {
                regs[dst as usize] = regs[src1 as usize].wrapping_sub(regs[src2 as usize]);
            }

            Instruction::Mul(src1, src2, dst) => {
                regs[dst as usize] = regs[src1 as usize].wrapping_mul(regs[src2 as usize]);
            }

            Instruction::Mod(src1, src2, dst) => {
                if regs[src2 as usize] == 0 {
                    regs[dst as usize] = 0;
                } else {
                    regs[dst as usize] = regs[src1 as usize] % regs[src2 as usize];
                }
            }

            Instruction::Inc(dst) => regs[dst as usize] = regs[dst as usize].wrapping_add(1),

            Instruction::Dec(dst) => regs[dst as usize] = regs[dst as usize].wrapping_sub(1),

            Instruction::Jmp(id) => {
                ip = labels[&id];
                continue;
            }

            Instruction::Jz(src, id) => {
                if regs[src as usize] == 0 {
                    ip = labels[&id];
                    continue;
                }
            }

            Instruction::Jnz(src, id) => {
                if regs[src as usize] != 0 {
                    ip = labels[&id];
                    continue;
                }
            }
        }

        ip += 1;
    }

    false
}

fn find_longest_pattern(diffs: Vec<u16>) -> Vec<u16> {
    'outer: for len in 1..diffs.len() {
        let pattern = &diffs[0..len];
        let mut i = pattern.len();
        while i < diffs.len() {
            let end = (i + pattern.len()).min(diffs.len());
            if diffs[i..end] != pattern[0..end - i] {
                continue 'outer;
            }
            i = end;
        }
        return pattern.to_vec();
    }

    panic!("No pattern found in {diffs:?}!");
}

fn run_range(
    instructions: &[Instruction],
    labels: &HashMap<usize, usize>,
    r0range: RangeInclusive<u16>,
    r1range: RangeInclusive<u16>,
) -> u64 {
    r1range
        .into_par_iter()
        .map(|r1| {
            let mut total = 0;

            // find the first 32 numbers for which the program runs
            // indefinitely, record the first number and the differences between
            // numbers
            let mut infinites = 0;
            let mut first = None;
            let mut last = None;
            let mut diffs = Vec::new();
            for r0 in r0range.clone() {
                let mut regs = vec![0u16; 16];
                regs[0] = r0;
                regs[1] = r1;

                if run(instructions, labels, &mut regs) {
                    if let Some(last) = last {
                        diffs.push(r0 - last);
                    }
                    last = Some(r0);

                    if first.is_none() {
                        first = last;
                    }

                    infinites += 1;
                    if infinites == 32 {
                        break;
                    }
                }
            }

            // try to find a repeating pattern in `diffs`
            let pattern = find_longest_pattern(diffs);

            // extrapolate all numbers for which the program would run
            // indefinitely
            let mut j = first.unwrap() as u64;
            let mut di = pattern.into_iter().cycle();
            while j <= *r0range.end() as u64 {
                total += 1;
                j += di.next().unwrap() as u64;
            }

            total
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();

    // parse
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    for l in lines {
        if let Some(rest) = l.strip_prefix("ba") {
            let parts = rest.split("ne").collect::<Vec<_>>();
            instructions.push(match parts[0].len() / 2 {
                0 => Instruction::Load((parts[1].len() / 2) as u16, (parts[2].len() / 2) as u8),

                1 => Instruction::Copy((parts[1].len() / 2) as u8, (parts[2].len() / 2) as u8),

                2 => Instruction::Add(
                    (parts[1].len() / 2) as u8,
                    (parts[2].len() / 2) as u8,
                    (parts[3].len() / 2) as u8,
                ),

                3 => Instruction::Sub(
                    (parts[1].len() / 2) as u8,
                    (parts[2].len() / 2) as u8,
                    (parts[3].len() / 2) as u8,
                ),

                4 => Instruction::Mul(
                    (parts[1].len() / 2) as u8,
                    (parts[2].len() / 2) as u8,
                    (parts[3].len() / 2) as u8,
                ),

                5 => Instruction::Mod(
                    (parts[1].len() / 2) as u8,
                    (parts[2].len() / 2) as u8,
                    (parts[3].len() / 2) as u8,
                ),

                6 => Instruction::Inc((parts[1].len() / 2) as u8),

                7 => Instruction::Dec((parts[1].len() / 2) as u8),

                8 => Instruction::Jmp(parts[1].len() / 2),

                9 => Instruction::Jz((parts[1].len() / 2) as u8, parts[2].len() / 2),

                10 => Instruction::Jnz((parts[1].len() / 2) as u8, parts[2].len() / 2),

                _ => panic!("Unknown instruction type: {}", parts[0].len() / 2),
            });
        } else if l.starts_with("be") {
            let id = (l.len() - 2) / 2;
            labels.insert(id, instructions.len());
        } else {
            panic!("Syntax error: {l}");
        }
    }

    // part 1
    let mut regs = vec![0u16; 16];
    run(&instructions, &labels, &mut regs);
    println!("{}", regs[0]);

    // part 2
    println!("{}", run_range(&instructions, &labels, 0..=99, 0..=0));

    // part 3
    println!("{}", run_range(&instructions, &labels, 0..=65535, 0..=15));
}
