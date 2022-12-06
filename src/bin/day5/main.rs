fn main() {
    let mut input_sections = include_str!("input.txt").split("\n\n");
    let stacks_str = input_sections.next().unwrap();
    let instructions_str = input_sections.next().unwrap();

    let stacks = parse_stacks(stacks_str);
    let instructions = instructions_str
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect();

    println!("Part1: {}", part1(&stacks, &instructions));
    println!("Part2: {}", part2(&stacks, &instructions));
}

fn parse_stacks(stacks_str: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(8); 9];

    // Get useful info from the rows
    let rows = stacks_str.lines().map(|line| {
        line.chars()
            .enumerate()
            .filter(|(i, _)| i % 4 == 1)
            .map(|(_, c)| if c == ' ' { None } else { Some(c) })
            .collect::<Vec<_>>()
    });

    // Stick that useful info into the stacks
    for row in rows.rev() {
        for (i, maybe_c) in row.into_iter().enumerate() {
            if let Some(c) = maybe_c {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

fn part1(stacks: &Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    // Don't modify the input
    let mut stacks = stacks.clone();

    // Follow the instructions
    for instruction in instructions {
        for _ in 0..instruction.count {
            let moved_crate = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(moved_crate);
        }
    }

    // Assemble string
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn part2(stacks: &Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    // Don't modify the input
    let mut stacks = stacks.clone();

    // Follow the instrutions
    for instruction in instructions {
        let split_point = stacks[instruction.from].len() - instruction.count;
        let mut moved_crates = stacks[instruction.from].split_off(split_point);
        stacks[instruction.to].append(&mut moved_crates);
    }

    // Assemble string
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(instruction_str: &str) -> Self {
        let mut instruction_iter = instruction_str.split_whitespace();
        // Fetch useful info from instruction line (skipping over the words)
        instruction_iter.next();
        let item = instruction_iter.next().unwrap().parse().unwrap();
        instruction_iter.next();
        let from = instruction_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        instruction_iter.next();
        let to = instruction_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        Instruction {
            count: item,
            from,
            to,
        }
    }
}
