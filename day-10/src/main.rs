use anyhow::Result;
use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-10/input.txt")?;

    println!("Puzzle 1 answer: {}", part_1(&input));

    println!("Puzzle 2 answer: ");
    // Prints stuff out to the screen, no need to return it
    part_2(&input);

    Ok(())
}

fn part_2(input: &str) {
    let mut crt_screen: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let timeline = &register_timeline(instructions(input).as_slice())[1..]; // Remove the first 1

    for (i, value) in timeline.iter().enumerate() {
        let x = i % 40;
        let y = (i / 40) % 6;

        crt_screen[y][x] = if *value == x as i64 || *value + 1 == x as i64 || *value - 1 == x as i64
        {
            '#'
        } else {
            '.'
        };
    }

    // Print the crt screen
    for line in crt_screen {
        println!("{}", line.iter().collect::<String>());
    }
}

fn part_1(input: &str) -> i64 {
    let instructions = instructions(input);
    let mut timeline = register_timeline(&instructions).into_iter();
    println!("{:?}", timeline);

    // Take all until the first probe
    timeline.nth(20 - 1).unwrap();

    timeline
        .step_by(40)
        .enumerate()
        .fold(0, |acc, (i, value)| acc + (40 * (i as i64) + 20) * value)
}

fn register_timeline(instructions: &[Instruction]) -> Vec<i64> {
    let mut timeline: Vec<i64> = vec![1];

    let mut tmp_reg = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Nop => {
                // Push the last value again
                timeline.push(*timeline.last().unwrap() + tmp_reg);
                tmp_reg = 0;
            }
            Instruction::Addx(value) => {
                // Addition takes two cycles
                timeline.push(*timeline.last().unwrap() + tmp_reg);
                timeline.push(*timeline.last().unwrap());
                tmp_reg = *value;
            }
        }
    }

    if tmp_reg != 0 {
        timeline.push(*timeline.last().unwrap() + tmp_reg);
    }

    timeline
}

// fn simulate(instructions: &[Instruction], stop_cycle: usize) -> i64 {
//     let mut register_x = 1;
//     let mut cycles = 0;
//     let mut collect_at_cycle = 20;
//     let mut signal_strengths = 0;

//     for instruction in instructions {
//         cycles += 1;

//         match instruction {
//             Instruction::Nop => {}
//             Instruction::Addx(value) => {
//                 // Do this before because we could have cycle update in middle of adding
//                 update(
//                     &mut register_x,
//                     &mut collect_at_cycle,
//                     cycles,
//                     stop_cycle,
//                     &mut signal_strengths,
//                 );

//                 // Addition takes two cycles
//                 cycles += 1;

//                 register_x += value;
//             }
//         }

//         update(
//             &mut register_x,
//             &mut collect_at_cycle,
//             cycles,
//             stop_cycle,
//             &mut signal_strengths,
//         )
//     }

//     signal_strengths
// }

// fn update(
//     register: &mut i64,
//     collect_at_cycle: &mut usize,
//     cycles: usize,
//     stop_cycle: usize,
//     signal_strengths: &mut i64,
// ) {
//     if cycles >= *collect_at_cycle && cycles <= stop_cycle {
//         *collect_at_cycle += 40;
//         *signal_strengths += *register * (cycles as i64);

//         println!(
//             "Cycle: {}, Register: {}, Signal Strength: {}",
//             cycles,
//             register,
//             *register * (cycles as i64)
//         );
//     }
// }

fn instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            match parts[0] {
                "noop" => Instruction::Nop,
                "addx" => Instruction::Addx(parts[1].parse().unwrap()),
                _ => unreachable!("Unknown instruction '{}'", parts[0]),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Nop,
    Addx(i64),
}
