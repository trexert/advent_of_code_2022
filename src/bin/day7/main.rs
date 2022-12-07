use lib::PositionBinary;
use std::collections::HashMap;

fn main() {
    let shell_output = include_str!("input.txt");

    let mut dir_walker = DirWalker::default();
    let directory_sizes = dir_walker.size_directories(shell_output);
    println!("Part1: {}", part1(directory_sizes));
    println!("Part2: {}", part2(directory_sizes));
}

fn part1(directory_sizes: &HashMap<String, u64>) -> u64 {
    directory_sizes
        .iter()
        .filter_map(|(_, size)| if *size <= 100_000 { Some(size) } else { None })
        .sum()
}

fn part2(directory_sizes: &HashMap<String, u64>) -> u64 {
    let min_size = directory_sizes["/"] - 40_000_000;

    let mut sorted_sizes: Vec<_> = directory_sizes
        .iter()
        .map(|(_, dir_size)| *dir_size)
        .collect();
    sorted_sizes.sort_unstable();

    sorted_sizes[sorted_sizes
        .as_slice()
        .position_binary(|size| *size >= min_size)
        .unwrap()]
}

#[derive(Debug, Default)]
struct DirWalker {
    directory_sizes: HashMap<String, u64>,
    cwd: Vec<&'static str>,
}

impl DirWalker {
    fn size_directories<'a>(&'a mut self, shell_output: &'static str) -> &'a HashMap<String, u64> {
        for line in shell_output.lines() {
            let split_line: Vec<_> = line.split_whitespace().collect();
            match (split_line[0], split_line[1]) {
                ("$", "cd") => self.handle_cd(split_line[2]),
                ("$", "ls") => (),
                ("dir", _) => (),
                _ => self.handle_file(split_line[0].parse().unwrap()),
            }
            // println!("{}, {:?}", line, self);
        }

        &self.directory_sizes
    }

    fn handle_cd(&mut self, dir: &'static str) {
        if dir == ".." {
            self.cwd.pop();
        } else {
            self.cwd.push(dir);
        }
    }

    fn handle_file(&mut self, file_size: u64) {
        let mut dir_to_update = vec![];
        for dir in &self.cwd {
            dir_to_update.push(dir.to_string());
            let fq_dir_name = dir_to_update.join("/");
            *self.directory_sizes.entry(fq_dir_name).or_default() += file_size;
        }
    }
}
