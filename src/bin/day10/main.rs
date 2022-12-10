#![feature(iter_intersperse)]

fn main() {
    let instructions: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect();

    println!("Part1: {}", part1(&instructions));
    part2(&instructions);
}

fn part1(instructions: &[Instruction]) -> i64 {
    let mut output = 0;
    let mut video = VideoSystem::new();
    for instruction in instructions {
        let video_out = video.perform_instruction(instruction);
        for (clock, x) in video_out {
            if [20, 60, 100, 140, 180, 220].contains(&clock) {
                output += clock as i64 * x;
            }
        }
    }
    output
}

fn part2(instructions: &[Instruction]) {
    let mut output = [['.'; 40]; 6];
    let mut video = VideoSystem::new();

    for instruction in instructions {
        let video_out = video.perform_instruction(instruction);

        for (clock, x) in video_out {
            let position = clock - 1;
            if ((position % 40) as i64).abs_diff(x) <= 1 {
                output[position / 40][position % 40] = '#'
            }
        }
    }

    println!(
        "{}",
        output
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .intersperse("\n".to_string())
            .collect::<String>()
    );
}

struct VideoSystem {
    clock: usize,
    x: i64,
}

impl VideoSystem {
    fn new() -> Self {
        VideoSystem { clock: 0, x: 1 }
    }

    fn perform_instruction(&mut self, instruction: &Instruction) -> Vec<(usize, i64)> {
        self.clock += 1;
        let mut output = vec![(self.clock, self.x)];
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(value) => {
                self.clock += 1;
                output.push((self.clock, self.x));
                self.x += value;
            }
        }
        output
    }
}

enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn from_str(instruction: &str) -> Self {
        let mut split_instruction = instruction.split(" ");
        match split_instruction.next() {
            Some("addx") => Instruction::Addx(split_instruction.next().unwrap().parse().unwrap()),
            Some("noop") => Instruction::Noop,
            unexpected => panic!("Unexpected instruction {:?}", unexpected),
        }
    }
}
